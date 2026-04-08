# ModernClaw Context and Persistence

## Purpose
This file explains how ModernClawMulti stores information and how it builds the live prompt context sent to the model.

## Two Persistence Layers
### Markdown Workspace
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- shared voice tool folders
- per-brain workspaces under `agents/<brain>/`

### SQLite Database
Used for:
- agents
- conversations
- messages
- settings
- schema version

## Current Database Shape
The current app is no longer well-described as a single-brain database.
Current important truth:
- an `agents` registry exists
- conversations are scoped by `agent_id`
- the active brain is tracked in settings
- brain-level model and voice preferences can be persisted on the agent record

## Workspace Initialization
The memory service ensures the workspace exists and creates folders for:
- `memory`
- `knowledge`
- `curator`
- `tools/piper/voices`
- `tools/whisper/models`

Important current truth:
- the LocalAI app-data root still exists
- the baseline Rosie brain uses that root workspace
- additional brains live under `agents/<brain>/`
- shared Piper and Whisper tool folders still resolve under the LocalAI root

## Context Build Order
When a chat request is sent, ModernClawMulti builds context from the active brain workspace:
1. `SOUL.md`
2. `USER.md`
3. `MEMORY.md`
4. today's daily log
5. each top-level knowledge markdown file
6. recent conversation history for the active brain that still fits the token budget

## Knowledge Loading Rule
This is critical:
- only top-level markdown files in `knowledge/` are loaded
- nested folders are ignored by the current implementation

That is why expert brains still need compact flat compiled knowledge files rather than deep nested trees.

## Token Budget Behavior
The backend estimates tokens roughly from character length and reserves a portion of the configured context window for the model's response. Older history is trimmed to fit.

## Current Runtime Storage Identity
Visible product branding is `ModernClawMulti`, but runtime storage still resolves under `LocalAI` on Windows and `localai` on Linux.

## Why This Matters
This layer is the operating system of the app. Any future fetcher, support brain, or wizard workflow has to respect these storage and context rules unless the code changes.
