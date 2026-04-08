# Context and Persistence Summary

## Purpose
This summary explains what ModernClawMulti persists, how it builds prompts, and where the app stores different kinds of information.

This is the backbone that ties the multi-brain user experience together.

## Two Main Persistence Layers
### 1. Markdown Brain Workspace
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- shared voice tool folders under the LocalAI root
- per-brain workspace folders under `agents/<brain>/`

### 2. SQLite Database
Stored as `data.db` in app data.
Used for:
- agents
- conversations
- messages
- settings
- schema version tracking

## Current Database Shape
The current database now includes:
- `agents`
- `conversations`
- `messages`
- `settings`
- `schema_version`

Important relationship:
- conversations are scoped by `agent_id`
- the active brain is stored in settings
- model and voice preferences can live on the brain record

## Current Workspace Behavior
The Rust memory service still initializes the workspace and ensures key folders exist.

Important current truth:
- the app still uses the LocalAI app-data root
- the baseline Rosie brain uses the root workspace
- additional brains live under `agents/<brain>/`
- shared Piper and Whisper folders still resolve under the LocalAI root `tools/` path

## Context Building Algorithm
When chat sends a message, the app builds a system prompt from the active brain workspace:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- today's log
- each top-level knowledge file as a separate `Knowledge Reference`

Then it appends recent conversation history for the active brain until the token budget is reached.

## Current Knowledge Loading Rule
This still matters a lot:
- ModernClawMulti currently loads only top-level markdown files in `knowledge/`
- nested subfolders are not part of live context loading

That means knowledge for expert agents should still be compiled into compact top-level files.

## Conversation Persistence Behavior
If `saveConversationHistory` is on:
- conversations are stored in SQLite
- messages are stored in SQLite
- the sidebar history survives restarts
- history is isolated per brain

If it is off:
- the app behaves more like a temporary session
- the conversation store is cleared when settings load

## Important Commands and Wiring
### Tauri command registration
See:
- `src-tauri/src/lib.rs`

Major command groups:
- chat and model commands
- agent commands
- history commands
- memory commands
- settings commands
- voice commands

## Important Current App Data Identity
Even though the visible branding is `ModernClawMulti`, runtime storage still resolves under:
- `LocalAI` on Windows
- `localai` on Linux

That means app data still lives under a `LocalAI`-named folder today.

## Important Notes
- this layer is now multi-brain infrastructure, not just single-brain persistence
- the product's brain-builder identity still depends on keeping this architecture understandable and controllable
- future Rosie, support-brain, and wizard workflows need to respect the current flat knowledge-loading model unless the code changes
