# Chat and Conversations Summary

## Purpose
The chat system is the main interactive conversation surface in ModernClawMulti.

It handles:
- message input
- local model inference through Ollama
- streaming responses
- conversation history
- optional voice transcription into the composer
- optional read-aloud for assistant messages
- per-brain conversation isolation

## Where To Find It In The App
The Chat view is the main working area after onboarding.

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
- `src/stores/agentStore.ts`

### Services
- `src/services/ollama.ts`
- `src/services/context.ts`
- `src/services/history.ts`

## Current Multi-Brain Behavior
- each brain has its own conversation list
- switching brains restores the latest conversation for that brain when one exists
- switching brains only shows the selected brain's chat history
- streamed output remains attached to the brain and conversation that started it

## How Sending a Message Works
1. User types into the composer.
2. If no conversation exists yet, one is created for the active brain.
3. The user message is appended to UI state.
4. If conversation history is enabled, the message is written to SQLite.
5. Context is built from the active brain's workspace files and knowledge.
6. The prompt is sent to Ollama.
7. Response chunks stream back.
8. The final assistant message is added to the active conversation.
9. If history is enabled, the assistant message is written to SQLite.

## Important Notes
- chat depends on the active brain's model selection
- if Ollama cannot find the model, chat will fail
- read-aloud now uses a speech-normalized version of assistant text so Piper does not speak raw markdown literally
