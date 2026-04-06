# ModernClaw Overview

## Purpose
ModernClaw is a local-first desktop workspace for building and using persistent AI brains. It combines chat, editable brain files, structured knowledge, model control, and voice into one controllable system.

## Main User Outcome
The app helps the user create a private AI system that remembers useful context, can be shaped through markdown files, and can be improved over time instead of resetting to a generic assistant every session.

## Main Views
- Chat
- Memory
- Brain
- Settings

## Core Stack
- Tauri desktop shell
- React frontend
- Rust backend
- Ollama for local model inference
- SQLite for conversations, messages, and settings
- Markdown files for brain files, knowledge, curator staging, and voice-tool folders

## Brain Files
ModernClaw always centers around three core markdown files:
- `SOUL.md` for assistant behavior and identity
- `USER.md` for the user's context
- `MEMORY.md` for active priorities, projects, and durable notes

## Live Knowledge Rule
ModernClaw currently loads only top-level markdown files inside the live `knowledge/` folder. Nested folders are ignored by the current loader.

## Voice Stack
- Whisper for speech-to-text
- Piper for text-to-speech

## Current Model Strategy
The app is tuned around a small fast floor model first, with heavier models remaining optional.

Current tested lanes include:
- `nchapman/dolphin3.0-qwen2.5:3b` as the floor model
- `dolphin3:8b` as a stronger fallback
- `gemma4:e4b` as a heavier experimental lane

## Product Identity
ModernClaw is best understood as a brain creator for local AI agents. The point is not only chatting with a model. The point is building owned intelligence around real context.
