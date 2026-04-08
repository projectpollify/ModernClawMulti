# ModernClaw Overview

## Purpose
ModernClawMulti is a local-first desktop workspace for building and using multiple persistent AI brains. It combines chat, editable brain files, structured knowledge, model control, and voice into one controllable multi-brain system.

## Main User Outcome
The app helps the user create private AI brains that remember useful context, can be shaped through markdown files, and can be switched cleanly without losing their separate conversations, memory, or preferences.

## Main Views
- Chat
- Memory
- Brain
- Settings

## Main Multi-Brain Layer
The current app is not just one workspace anymore. It now includes:
- active brain selection
- create brain flow
- delete non-baseline brain flow
- isolated conversation history per brain
- isolated memory workspace per brain
- per-brain model preference
- per-brain voice preference on top of shared machine-level tools

## Core Stack
- Tauri desktop shell
- React frontend
- Rust backend
- Ollama for local model inference
- SQLite for agents, conversations, messages, and settings
- Markdown files for brain files, knowledge, curator staging, and workspace content

## Brain Files
ModernClawMulti still centers around three core markdown files per brain:
- `SOUL.md` for assistant behavior and identity
- `USER.md` for the user's context
- `MEMORY.md` for active priorities, projects, and durable notes

## Live Knowledge Rule
ModernClawMulti still loads only top-level markdown files inside the live `knowledge/` folder. Nested folders are ignored by the current loader.

## Voice Stack
- Whisper for speech-to-text
- Piper for text-to-speech
- shared machine-level Piper and Whisper install paths
- brain-specific voice selection on top of that shared install

## Current Model Strategy
For this repo and product track, the clean current baseline lane is:
- `gemma4:e4b`

The app-wide default model still exists as a fallback, but the active brain's saved model takes priority when present.

## Product Identity
ModernClawMulti is best understood as a multi-brain creator for local AI agents. The point is not only chatting with one model. The point is building owned intelligence around real context and switching between distinct brains cleanly.
