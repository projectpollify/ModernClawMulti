import { useConversationStore } from '@/stores/conversationStore';
import { useViewStore } from '@/stores/uiStore';
import { cn } from '@/lib/utils';
import type { Conversation } from '@/types';

export function ConversationList() {
  const conversations = useConversationStore((state) => state.conversations);
  const currentId = useConversationStore((state) => state.currentId);
  const selectConversation = useConversationStore((state) => state.selectConversation);
  const deleteConversation = useConversationStore((state) => state.deleteConversation);
  const setView = useViewStore((state) => state.setView);

  const recentConversations = [...conversations].sort(
    (a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
  );

  if (recentConversations.length === 0) {
    return (
      <div className="rounded-lg border border-dashed border-border/80 bg-background/70 px-3 py-4 text-sm text-muted-foreground">
        No conversations yet
      </div>
    );
  }

  const grouped = groupByDate(recentConversations);

  const handleSelect = async (id: string) => {
    await selectConversation(id);
    setView('chat');
  };

  const handleDelete = async (id: string) => {
    await deleteConversation(id);
  };

  return (
    <div className="space-y-4">
      {Object.entries(grouped).map(([label, items]) => (
        <div key={label}>
          <h3 className="mb-2 px-2 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">
            {label}
          </h3>
          <div className="space-y-1">
            {items.map((conversation) => (
              <ConversationItem
                key={conversation.id}
                conversation={conversation}
                isActive={conversation.id === currentId}
                onSelect={() => handleSelect(conversation.id)}
                onDelete={() => handleDelete(conversation.id)}
              />
            ))}
          </div>
        </div>
      ))}
    </div>
  );
}

interface ConversationItemProps {
  conversation: Conversation;
  isActive: boolean;
  onSelect: () => void;
  onDelete: () => void;
}

function ConversationItem({
  conversation,
  isActive,
  onSelect,
  onDelete,
}: ConversationItemProps) {
  return (
    <div
      className={cn(
        'group flex w-full items-center gap-2 rounded-md px-2 py-2 text-left transition-colors',
        isActive
          ? 'bg-accent text-accent-foreground'
          : 'text-foreground hover:bg-accent hover:text-accent-foreground'
      )}
    >
      <button onClick={onSelect} className="min-w-0 flex-1 text-left">
        <p className="truncate text-sm font-medium">{conversation.title}</p>
        {conversation.preview ? (
          <p className="truncate text-xs text-muted-foreground">{conversation.preview}</p>
        ) : null}
      </button>

      <button
        onClick={(event) => {
          event.stopPropagation();
          onDelete();
        }}
        className="rounded p-1 opacity-0 transition-opacity hover:bg-background/80 group-hover:opacity-100"
        aria-label={`Delete ${conversation.title}`}
      >
        <TrashIcon className="h-4 w-4" />
      </button>
    </div>
  );
}

function groupByDate(conversations: Conversation[]): Record<string, Conversation[]> {
  const now = new Date();
  const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
  const yesterday = new Date(today.getTime() - 86400000);
  const weekAgo = new Date(today.getTime() - 7 * 86400000);

  const groups: Record<string, Conversation[]> = {
    Today: [],
    Yesterday: [],
    'This Week': [],
    Older: [],
  };

  for (const conversation of conversations) {
    const updatedAt = new Date(conversation.updatedAt);

    if (updatedAt >= today) {
      groups.Today.push(conversation);
    } else if (updatedAt >= yesterday) {
      groups.Yesterday.push(conversation);
    } else if (updatedAt >= weekAgo) {
      groups['This Week'].push(conversation);
    } else {
      groups.Older.push(conversation);
    }
  }

  return Object.fromEntries(
    Object.entries(groups).filter(([, items]) => items.length > 0)
  );
}

function TrashIcon({ className }: { className?: string }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24">
      <path
        strokeLinecap="round"
        strokeLinejoin="round"
        strokeWidth={2}
        d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
      />
    </svg>
  );
}
