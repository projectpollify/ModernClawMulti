# ModernClaw Chat and Conversations

## Purpose
Chat is the direct interaction layer with the currently selected local model. It combines live prompt context, persisted conversation history, optional token display, and voice controls.

## Where To Find It
Open the main app and stay in the default `Chat` view.

## Main Elements
- message list
- text composer
- send button
- mic button when voice input is enabled
- read-aloud controls on assistant messages when voice output is enabled
- error banner for chat or voice failures

## Message Flow
When the user sends a message:
1. the app makes sure there is an active conversation
2. it builds prompt context from brain files, daily log, knowledge, and recent conversation history
3. it sends the request through the Rust backend to Ollama
4. streamed chunks update the current assistant response live
5. final messages can be persisted if history is enabled

## Conversation History
If `saveConversationHistory` is enabled:
- conversations are stored in SQLite
- messages are stored in SQLite
- the sidebar conversation list survives restart

If it is disabled:
- history is cleared from persistent storage usage in the live flow
- chat behaves more like a temporary workspace

## Titles
New conversations begin as `New Chat`. After the first user message, the app can generate a better title from that prompt.

## Voice in Chat
### Voice Input
If voice input is enabled and Whisper is configured, the mic button records locally, transcribes locally, and places the transcript into the text box for review.

### Voice Output
If voice output is enabled and Piper is configured, assistant messages can be read aloud with pause and resume support.

## User Guidance
1. Pick a model.
2. Start or select a conversation.
3. Type or dictate a prompt.
4. Read or listen to the answer.
5. Use the sidebar to return to older persisted conversations.
