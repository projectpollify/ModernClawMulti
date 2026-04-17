import { useMemo, useState } from 'react';
import { useAgentStore } from '@/stores/agentStore';
import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/Button';
import { useModelStore } from '@/stores/modelStore';

function slugifyBrainName(value: string) {
  return value
    .toLowerCase()
    .trim()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .slice(0, 48);
}

function uniqueBrainId(base: string, existingIds: string[]) {
  if (!base) {
    return '';
  }

  if (!existingIds.includes(base)) {
    return base;
  }

  let counter = 2;
  let candidate = `${base}-${counter}`;

  while (existingIds.includes(candidate)) {
    counter += 1;
    candidate = `${base}-${counter}`;
  }

  return candidate;
}

export function BrainSelector() {
  const agents = useAgentStore((state) => state.agents);
  const activeAgent = useAgentStore((state) => state.activeAgent);
  const isLoading = useAgentStore((state) => state.isLoading);
  const storeError = useAgentStore((state) => state.error);
  const setActiveAgent = useAgentStore((state) => state.setActiveAgent);
  const createAgent = useAgentStore((state) => state.createAgent);
  const deleteAgent = useAgentStore((state) => state.deleteAgent);
  const currentModel = useModelStore((state) => state.currentModel);

  const [isCreating, setIsCreating] = useState(false);
  const [brainName, setBrainName] = useState('');
  const [brainPurpose, setBrainPurpose] = useState('');
  const [localError, setLocalError] = useState<string | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isDeleting, setIsDeleting] = useState(false);

  const value = activeAgent?.agentId ?? '';
  const existingIds = useMemo(() => agents.map((agent) => agent.agentId), [agents]);
  const normalizedExistingNames = useMemo(
    () => new Set(agents.map((agent) => agent.name.trim().toLowerCase())),
    [agents]
  );
  const duplicateNameCounts = useMemo(() => {
    const counts = new Map<string, number>();
    for (const agent of agents) {
      counts.set(agent.name, (counts.get(agent.name) ?? 0) + 1);
    }
    return counts;
  }, [agents]);
  const suggestedId = uniqueBrainId(slugifyBrainName(brainName), existingIds);
  const canDeleteActiveBrain = Boolean(activeAgent && activeAgent.agentId !== 'default' && agents.length > 1);
  const optionStyle = { color: '#0f172a', backgroundColor: '#ffffff' };

  const handleCreateBrain = async () => {
    const trimmedName = brainName.trim();
    const trimmedPurpose = brainPurpose.trim();

    if (!trimmedName) {
      setLocalError('Give the new brain a name first.');
      return;
    }

    if (normalizedExistingNames.has(trimmedName.toLowerCase())) {
      setLocalError('That brain name is already in use. Pick a different display name so the selector stays clear.');
      return;
    }

    if (!suggestedId) {
      setLocalError('Could not generate a valid brain ID from that name.');
      return;
    }

    setIsSubmitting(true);
    setLocalError(null);

    try {
      await createAgent({
        agentId: suggestedId,
        name: trimmedName,
        description: trimmedPurpose || undefined,
        defaultModel: currentModel ?? undefined,
      });
      await setActiveAgent(suggestedId);
      setBrainName('');
      setBrainPurpose('');
      setIsCreating(false);
    } catch (error) {
      setLocalError(String(error));
    } finally {
      setIsSubmitting(false);
    }
  };

  const handleDeleteBrain = async () => {
    if (!activeAgent || !canDeleteActiveBrain) {
      return;
    }

    const confirmed = window.confirm(
      `Delete brain "${activeAgent.name}"?\n\nThis will remove its conversations and local workspace files. This cannot be undone.`
    );

    if (!confirmed) {
      return;
    }

    setIsDeleting(true);
    setLocalError(null);

    try {
      await deleteAgent(activeAgent.agentId);
    } catch (error) {
      setLocalError(String(error));
    } finally {
      setIsDeleting(false);
    }
  };

  return (
    <div className="relative flex items-center gap-2">
      <div className="flex min-w-[240px] items-center gap-3 rounded-2xl border border-border bg-secondary/60 px-3 py-2 backdrop-blur">
        <div className="flex h-9 w-9 items-center justify-center rounded-xl bg-primary/10 text-xs font-semibold uppercase tracking-[0.16em] text-primary">
          {activeAgent?.name?.slice(0, 2) ?? 'MB'}
        </div>
        <div className="min-w-0 flex-1">
          <p className="text-[10px] font-semibold uppercase tracking-[0.18em] text-muted-foreground">
            Active Brain
          </p>
          <select
            value={value}
            onChange={(event) => void setActiveAgent(event.target.value)}
            disabled={isLoading || !agents.length}
            className={cn(
              'mt-1 w-full bg-transparent text-sm font-medium text-foreground outline-none',
              'disabled:cursor-not-allowed disabled:text-muted-foreground'
            )}
            aria-label="Select active brain"
          >
            {agents.length ? null : <option value="">Loading brains...</option>}
            {agents.map((agent) => {
              const showId = (duplicateNameCounts.get(agent.name) ?? 0) > 1;
              const optionLabel = showId ? `${agent.name} (${agent.agentId})` : agent.name;

              return (
                <option key={agent.agentId} value={agent.agentId} style={optionStyle}>
                  {optionLabel}
                </option>
              );
            })}
          </select>
        </div>
      </div>

      <Button
        variant="outline"
        size="sm"
        className="rounded-xl"
        onClick={() => {
          setIsCreating((value) => !value);
          setLocalError(null);
        }}
      >
        Create Brain
      </Button>

      {canDeleteActiveBrain ? (
        <Button
          variant="outline"
          size="sm"
          className="rounded-xl border-red-500/30 text-red-600 hover:bg-red-500/10 hover:text-red-600"
          onClick={() => void handleDeleteBrain()}
          disabled={isDeleting || isLoading}
        >
          {isDeleting ? 'Deleting...' : 'Delete Brain'}
        </Button>
      ) : null}

      {isCreating ? (
        <div className="absolute right-0 top-full z-50 mt-2 w-[340px] rounded-2xl border border-border bg-background p-4 shadow-2xl">
          <div className="mb-3">
            <h3 className="text-sm font-semibold">Create Brain</h3>
            <p className="mt-1 text-xs text-muted-foreground">
              Create a new isolated workspace with its own SOUL, USER, MEMORY, knowledge, and curator flow.
            </p>
          </div>

          <div className="space-y-3">
            <label className="block">
              <span className="mb-1 block text-xs font-medium text-muted-foreground">Name</span>
              <input
                value={brainName}
                onChange={(event) => setBrainName(event.target.value)}
                placeholder="Rosie"
                className="w-full rounded-xl border border-border bg-secondary/40 px-3 py-2 text-sm outline-none focus:border-primary/40"
              />
            </label>

            <label className="block">
              <span className="mb-1 block text-xs font-medium text-muted-foreground">Purpose</span>
              <textarea
                value={brainPurpose}
                onChange={(event) => setBrainPurpose(event.target.value)}
                placeholder="Marketing strategist, launch planner, and product explainer"
                rows={3}
                className="w-full rounded-xl border border-border bg-secondary/40 px-3 py-2 text-sm outline-none focus:border-primary/40"
              />
            </label>

            <div className="rounded-xl border border-border bg-secondary/30 px-3 py-2 text-xs text-muted-foreground">
              <p className="font-medium text-foreground">Brain ID</p>
              <p className="mt-1 font-mono">{suggestedId || 'waiting-for-name'}</p>
              <p className="mt-1">The new brain will start with the current selected model if one is active.</p>
            </div>

            {localError || storeError ? (
              <div className="rounded-xl border border-red-500/20 bg-red-500/10 px-3 py-2 text-xs text-red-600">
                {localError ?? storeError}
              </div>
            ) : null}

            <div className="flex items-center justify-end gap-2">
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  setIsCreating(false);
                  setLocalError(null);
                }}
              >
                Cancel
              </Button>
              <Button size="sm" onClick={() => void handleCreateBrain()} disabled={isSubmitting || isLoading}>
                {isSubmitting ? 'Creating...' : 'Create and Switch'}
              </Button>
            </div>
          </div>
        </div>
      ) : null}
    </div>
  );
}


