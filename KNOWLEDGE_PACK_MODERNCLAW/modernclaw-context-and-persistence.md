# ModernClaw Context and Persistence

## Purpose
This file explains how ModernClaw stores information and how it builds the live prompt context sent to the model.

## Two Persistence Layers
### Markdown Workspace
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- voice tool folders

### SQLite Database
Used for:
- conversations
- messages
- settings

## Current Database Tables
- `conversations`
- `messages`
- `settings`
- `schema_version`

## Workspace Initialization
The memory service ensures the workspace exists and creates folders for:
- `memory`
- `knowledge`
- `curator`
- `tools/piper/voices`
- `tools/whisper/models`

## Context Build Order
When a chat request is sent, ModernClaw builds context from:
1. `SOUL.md`
2. `USER.md`
3. `MEMORY.md`
4. today's daily log
5. each top-level knowledge markdown file
6. recent conversation history that still fits the token budget

## Knowledge Loading Rule
This is critical:
- only top-level markdown files in `knowledge/` are loaded
- nested folders are ignored by the current implementation

That is why expert brains like Rosie need compact flat compiled knowledge files rather than deep nested documentation trees.

## Token Budget Behavior
The backend estimates tokens roughly from character length and reserves a portion of the configured context window for the model's response. Older history is trimmed to fit.

## Current Runtime Storage Identity
Visible product branding is `ModernClaw`, but runtime storage still resolves under `LocalAI` on Windows and `localai` on Linux.

## Why This Matters
This layer is the real operating system of the app. Any future fetcher, compiler, or self-improvement workflow has to respect these storage and context rules unless the code changes.
