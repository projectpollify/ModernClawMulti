# Context and Persistence Summary

## Purpose
This summary explains what ModernClaw persists, how it builds prompts, and where the app stores different kinds of information.

This is the backbone that ties the user-facing features together.

## Two Main Persistence Layers
### 1. Markdown Brain Workspace
Stored in the local app data memory folder.
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- voice tool folders

### 2. SQLite Database
Stored as `data.db` in app data.
Used for:
- conversations
- messages
- settings

## Current Database Tables
From the current migration:
- `conversations`
- `messages`
- `settings`
- `schema_version`

## Current Markdown Workspace Behavior
The Rust memory service initializes the workspace and creates:
- base path
- `memory`
- `knowledge`
- `curator`
- `tools/piper/voices`
- `tools/whisper/models`

## Context Building Algorithm
When chat sends a message, the app builds a system prompt from:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- today’s log
- each top-level knowledge file as a separate `Knowledge Reference`

Then it appends recent conversation history until the token budget is reached.

## Current Knowledge Loading Rule
This matters a lot:
- ModernClaw currently loads only top-level markdown files in `knowledge/`
- nested subfolders are not part of live context loading

That means knowledge for expert agents should be compiled into compact top-level files.

## Conversation Persistence Behavior
If `saveConversationHistory` is on:
- conversations are stored in SQLite
- messages are stored in SQLite
- sidebar history survives restarts

If it is off:
- the app behaves more like a temporary session
- conversation store is cleared when settings load

## Important Commands and Wiring
### Tauri command registration
See:
- `src-tauri/src/lib.rs`

Major command groups:
- chat and model commands
- history commands
- memory commands
- settings commands
- voice commands

## Important Current App Data Identity
Even though the visible branding is `ModernClaw`, the app still uses the older runtime storage name:
- `LocalAI` on Windows
- `localai` on Linux

That means app data still resolves under a `LocalAI`-named folder today.

## User Instructions
### Inspect what the app knows
1. Open `Memory`.
2. Review `SOUL.md`, `USER.md`, and `MEMORY.md`.
3. Inspect top-level knowledge files.

### Verify that history is persisted
1. Ensure `Save Conversation History` is enabled in Settings.
2. Send messages in chat.
3. Restart the app.
4. Confirm conversations still appear in the sidebar.

## Important Notes
- This layer is the real infrastructure of ModernClaw.
- The product’s “brain builder” identity depends on keeping this architecture understandable and controllable.
- Future Rosie knowledge and fetcher workflows should respect the current flat knowledge-loading model unless the code changes.
