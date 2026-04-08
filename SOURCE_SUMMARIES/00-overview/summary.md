# ModernClaw Source Summaries Overview

## Purpose
This folder is the human-maintained source knowledge pack for the current `ModernClawMulti` app.

It exists so a future fetcher, compiler, support brain, or expert brain can understand the real product shape without rereading the full codebase every time.

## What This Doc Set Covers
- app shell and navigation
- onboarding
- model management
- chat and conversation behavior
- memory workspace
- brain builder workflows
- curator staging and import
- voice input and output
- settings
- context and persistence architecture

## Current Product Shape
ModernClawMulti is no longer just a single-brain local AI workspace.
It is now a real multi-brain desktop app with:
- Chat
- Memory
- Brain
- Settings
- active brain switching
- create brain flow
- delete non-baseline brain flow
- per-brain conversations
- per-brain memory workspace
- per-brain model preference
- per-brain voice preference on top of shared machine-level tools

If onboarding has not been completed yet, the onboarding flow still replaces the main shell.

## Most Important Architectural Truths
- the app is a Tauri desktop app with a React frontend and Rust backend
- the frontend talks to Rust through Tauri commands
- markdown brain files still live in the local app-data workspace
- conversation history still lives in SQLite when history is enabled
- Ollama is still the model provider
- Whisper is still used for speech-to-text
- Piper is still used for text-to-speech
- an `agents` registry now drives multi-brain behavior
- the active brain controls which workspace, conversations, model preference, and voice preference are in play

## Current Internal Storage Split
### Markdown Workspace
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- shared voice tool folders under the LocalAI root
- brain-specific workspaces under `agents/<brain>/`

### SQLite
Used for:
- agents
- conversations
- messages
- settings
- schema version tracking

## Current Model Truth
For this repo and this product track, the clean current baseline story is Gemma-first.

Current practical truth:
- baseline lane: `gemma4:e4b`
- model choice is persisted per brain
- app-wide default model still exists as a fallback, but the active brain's saved model takes priority

## Current Voice Truth
Current practical truth:
- Piper and Whisper are machine-level installs
- approved Piper voices are currently `Amy (Female)` and `Joe (Male)`
- brains can keep different voice choices while sharing the same machine-level install
- speech output is normalized before Piper playback so raw markdown is not spoken literally

## Important Current Limitation
Knowledge loading is still flat and top-level only.
ModernClawMulti loads top-level markdown files from the live `knowledge/` folder, not nested subfolders.

That means expert knowledge for Rosie or future support brains should still be compiled into compact top-level files rather than deep nested trees.
