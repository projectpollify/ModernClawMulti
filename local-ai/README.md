# ModernClawMulti App

This app is the current ModernClawMulti application surface built on the original local-first desktop foundation.

It keeps the core ModernClaw brain-building model, but extends it into a working multi-brain workspace experiment.

## What The App Does

- local chat with Ollama models
- streamed assistant responses
- persistent conversation history stored locally
- editable memory files: `SOUL.md`, `USER.md`, `MEMORY.md`
- daily logs stored in `memory/YYYY-MM-DD.md`
- knowledge files loaded from `knowledge/*.md`
- Brain suggestions that write into local brain files
- persistent Brain suggestion state across restarts
- dedicated `Question Queue`
- activity history for Brain changes
- guided Brain setup for `SOUL.md`, `USER.md`, and `MEMORY.md`
- structured knowledge intake with summary, source, and tags
- curator staging pipeline with `Curator Inbox`, `Import to Knowledge`, and `Reject`
- local voice output through Piper with `Read Aloud`
- local voice input through Whisper into the composer
- pause / resume / stop playback controls
- approved Piper voice presets
- shared machine-level voice tool paths with brain-specific voice choice layered on top
- onboarding flow for first launch
- settings for theme, default model, privacy, prompt budget, and voice
- multi-brain create, switch, delete, and restore flows

## What Is Verified

This project has been manually verified on Windows for:

- `npm run tauri:dev`
- `npm run build`
- `cargo check`
- local Ollama chat
- streamed responses
- conversation persistence
- settings persistence
- daily log creation in the app and daily-log prompt-context use
- knowledge-file prompt-context use
- Brain question queue and persistence
- Brain activity history
- guided Brain setup writing to the right files
- Curator Inbox loading and staged import workflow
- local Piper voice output from settings and assistant replies
- local Whisper voice input into the composer
- pause / resume playback behavior
- multi-brain create/switch/delete flows
- conversation isolation across Rosie and Mia
- knowledge and curator isolation across Rosie and Mia
- brain-specific Piper voice selection with Rosie on `Amy (Female)` and Mia on `Joe (Male)`
- Windows production packaging with `npm run tauri:build`

## Windows-First Status

This app is currently Windows-first.

- Windows: validated
- macOS: planning is separated, not validated here
- Linux: not validated here

## Ollama Requirement

ModernClawMulti requires Ollama to be installed separately.

Current expectation:

1. install Ollama from [ollama.com](https://ollama.com)
2. pull at least one model
3. make sure Ollama is running before using chat

Current baseline lane:
- `gemma4:e4b`

## Voice Setup Reality

Current behavior:
- the app creates the shared voice-tool folders under `%APPDATA%\LocalAI\tools`
- Piper and Whisper files are not auto-downloaded yet
- clean-machine voice setup still requires manual dependency placement or installation
- brains can keep different voice choices while sharing one machine-level Piper and Whisper install

Current approved Piper voice presets:
- `Amy (Female)`
- `Joe (Male)`

Required Piper files for the current validated setup:
- `en_US-amy-medium.onnx`
- `en_US-amy-medium.onnx.json`
- `en_US-joe-medium.onnx`
- `en_US-joe-medium.onnx.json`

## Development

```bash
npm install
npm run tauri:dev
```

For the full Windows bring-up flow, see [RUNBOOK.md](../RUNBOOK.md).

## Build Commands

```bash
# Frontend only
npm run build

# Tauri dev
npm run tauri:dev

# Production build for the current platform
npm run tauri:build

# Platform-specific build targets
npm run tauri:build:mac
npm run tauri:build:win
npm run tauri:build:linux
```

## Memory System

The app stores local Markdown context files under the LocalAI app-data memory folder, including:

- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs in `memory/YYYY-MM-DD.md`
- knowledge references in `knowledge/*.md`
- curator staging folders in `curator/`
- shared voice tool folders in `tools/`
- per-brain workspaces under `agents/<brain>/`

## Known Limitations

- Ollama is a required external dependency
- only Windows has been validated so far
- knowledge files added outside the app appear after refresh
- curator packages added outside the app appear after refresh
- voice dependency delivery is still manual on clean machines
- the app still uses the original `LocalAI` app-data root and deeper scaffold identifiers internally
- knowledge files are loaded directly into prompt context; there is no selective retrieval yet
- daily logs are user-created entries, not automatic conversation summaries
- macOS signing and notarization are not configured yet
- Windows code signing is not configured yet

## First Launch

1. Start Ollama on your machine.
2. Open the app.
3. Confirm a model is installed.
4. Confirm Piper and Whisper paths if you want voice.
5. Start building and switching brains.
