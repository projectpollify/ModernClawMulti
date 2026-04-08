# ModernClaw Chat

## Purpose
Chat is the main interactive conversation surface for the active brain.

## Where To Find It
- main chat view
- sidebar conversation list
- message composer at the bottom

## Current Multi-Brain Truth
- each brain has its own conversation history
- switching brains restores the latest conversation for that brain when one exists
- switching brains should not leak streamed output into the wrong brain
- new chat starts a new conversation for the active brain, not for all brains

## How It Works
1. user types into the composer
2. if needed, a new conversation is created for the active brain
3. context is built from the active brain's markdown files, logs, and knowledge
4. the prompt is sent to Ollama
5. streamed chunks are shown in the active conversation
6. final messages are persisted if history is enabled

## Voice Hooks
- voice input can transcribe into the composer
- read-aloud can speak assistant messages
- read-aloud now uses speech-normalized text so raw markdown is not spoken literally

## Important Notes
- chat depends on the active brain's selected model
- if history is enabled, persistence is isolated per brain
