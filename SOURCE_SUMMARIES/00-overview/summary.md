# ModernClaw Source Summaries Overview

## Purpose
This folder is a source-knowledge pack for ModernClaw itself.

It exists so a future fetcher, compiler, or domain-expert brain like Rosie can understand the app from the inside out without re-reading the full codebase every time.

These summaries are based on the actual current implementation in the `local-ai` Tauri/React application.

## What This Doc Set Covers
- the app shell and navigation
- onboarding
- model management
- chat and conversation persistence
- memory workspace
- brain builder workflows
- curator staging/import
- voice input and output
- settings
- context building and persistence architecture

## How to Use This Folder
Use the numbered feature folders in order.

Recommended reading order:
1. app shell and navigation
2. onboarding
3. model management
4. chat and conversations
5. memory workspace
6. brain builder
7. curator inbox
8. voice system
9. settings
10. context and persistence

## Current App Shape
ModernClaw currently has four main app views:
- Chat
- Memory
- Brain
- Settings

If onboarding has not been completed yet, the app shows the onboarding flow instead of the main shell.

## Important Architectural Truths
- ModernClaw is a Tauri desktop app with a React frontend and Rust backend.
- The frontend talks to Rust through Tauri commands.
- Markdown brain files live in the local app data memory path.
- Conversation history lives in SQLite when history is enabled.
- Ollama remains the current model provider.
- Voice uses Whisper for input and Piper for output.

## Current Internal Storage Split
### Markdown-based workspace
Used for:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- curator folders
- voice tool folders

### SQLite
Used for:
- conversations
- messages
- settings

## Important Current Limitation
Knowledge loading is currently flat and top-level only.
ModernClaw loads top-level markdown files from the `knowledge` folder, not nested subfolders.

That means source knowledge for Rosie should eventually be compiled into compact flat files rather than a deep folder tree.
