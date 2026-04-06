# Chat and Conversations Summary

## Purpose
The chat system is the main interactive conversation surface in ModernClaw.

It handles:
- message input
- local model inference through Ollama
- streaming responses
- conversation history
- per-conversation model association
- optional voice transcription into the composer
- optional read-aloud for assistant messages

## Where To Find It In The App
The Chat view is the default main working area after onboarding.

Main visible parts:
- conversation list in the sidebar
- message list in the center
- composer at the bottom
- optional streaming content during generation

## Main Components
- `src/components/chat/ChatView.tsx`
- `src/components/chat/MessageList.tsx`
- `src/components/chat/MessageBubble.tsx`
- `src/components/chat/MessageInput.tsx`
- `src/components/chat/MessageContent.tsx`
- `src/components/chat/ErrorBanner.tsx`
- `src/components/chat/EmptyState.tsx`
- `src/components/chat/StreamingBubble.tsx`

## Stores and Services
### Stores
- `src/stores/chatStore.ts`
- `src/stores/conversationStore.ts`

### Services
- `src/services/ollama.ts`
- `src/services/context.ts`
- `src/services/history.ts`

### Backend
- `chat_send`
- `build_context`
- conversation and message commands in `commands/history.rs`

## How Sending a Message Works
1. User types into the composer.
2. If no conversation exists yet, one is created automatically.
3. The user message is appended to UI state.
4. If conversation history is enabled, the message is written to SQLite.
5. The app builds context from:
   - `SOUL.md`
   - `USER.md`
   - `MEMORY.md`
   - today's log
   - knowledge files
   - recent conversation history
6. The prompt is sent to Ollama.
7. Response chunks stream back through a Tauri event channel.
8. The final assistant message is added to the conversation.
9. If history is enabled, the assistant message is written to SQLite.

## Conversation History
Conversation history uses SQLite when `Save Conversation History` is enabled.

Persisted data includes:
- conversations
- messages
- timestamps
- associated model
- message counts
- preview snippets

## User Instructions
### Send a normal chat message
1. Open `Chat`.
2. Type your message.
3. Press `Enter` or click send.

### Use multiline input
1. Use `Shift+Enter` when `send on enter` is enabled.
2. Or disable `send on enter` in Settings and use Ctrl/Cmd+Enter to send.

### Switch conversations
1. Use the conversation list in the sidebar.
2. Click any saved conversation.

### Start a new conversation
1. Click `New Chat` in the sidebar.

### Delete a conversation
1. Use the conversation list controls.
2. Delete the selected conversation.

## Voice Hooks In Chat
### Voice input
If enabled, a mic button appears beside the send button.
Recorded speech is transcribed and inserted into the composer.
It does not auto-send.

### Voice output
Assistant bubbles can show `Read Aloud`, `Pause Reading`, or `Resume Reading`.

## Important Notes
- Chat depends on a currently selected model.
- If Ollama cannot find the model, the chat will fail.
- Conversation persistence is controlled by a settings toggle.
