# RUNBOOK

## Purpose

This runbook records the bring-up, recovery, and basic verification steps for the ModernClaw workspace.

## Current Workspace

- Repo root: `C:\Users\pento\Desktop\LocalAI-Next`
- App source: `C:\Users\pento\Desktop\LocalAI-Next\local-ai`

## Daily Bring-Up

1. Make sure Ollama is installed.
2. Start Ollama if it is not already running.
3. Start the app in Tauri dev mode.

### Commands

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai"
npm run tauri:dev
```

If Ollama is not already running, start it in a separate PowerShell window:

```powershell
ollama serve
```

If Ollama says the port is already in use, it is usually already running.

## Build Verification

### Frontend + TypeScript

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai"
npm run build
```

### Rust / Tauri Backend

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai\src-tauri"
cargo check
```

## Packaged Build

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai"
npm run tauri:build
```

## Common Recovery Steps

### App folder was renamed or moved

Symptoms may include:
- Tauri build paths pointing at an old location
- plugin permission path errors
- stale build output referencing the wrong folder

Fix:

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai\src-tauri"
cargo clean
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai"
npm run tauri:dev
```

### Sidebar appears lost

Current behavior should be safer now because the hidden sidebar state is no longer persisted between launches.

If the app still gets into a strange UI state, restart dev mode.

## Voice Setup Notes

### Current Voice Features

Working now:
- local voice output through Piper
- local voice input through Whisper
- pause / resume / stop playback controls

### Current Default Voice Tool Layout

The app provisions default folders under the runtime memory base path:
- `...\tools\piper\`
- `...\tools\piper\voices\`
- `...\tools\whisper\`
- `...\tools\whisper\models\`

Important current limitation:
- the folders are auto-created
- Piper / Whisper executables and model files are **not** auto-downloaded yet
- clean-machine setup still requires manual dependency placement or installation

### Main-Machine Working Paths

Current working local voice setup on the main machine:
- Piper executable: `C:\Tools\piper\piper.exe`
- Piper voice model: `C:\Tools\piper\voices\en_US-lessac-medium.onnx`
- Whisper executable: `C:\Tools\whisper\release\whisper-cli.exe`
- Whisper model: `C:\Tools\whisper\models\ggml-base.en.bin`

Important note:
- the matching `en_US-lessac-medium.onnx.json` file must sit beside the `.onnx` model file

### Approved Voice Presets

Current approved Piper voice presets in the app:
- `Lessac`
- `Amy`
- `Joe`

## Brain / Curator Notes

- Brain data loads from the local memory workspace managed by the app.
- Curator staged packages are read from the local `curator/staged/` area under the app data folder.
- Packages added outside the app appear in the Curator Inbox after refresh.

## Local Data Location

The app still uses the older `LocalAI` app-data root for runtime memory and knowledge files, including:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- `memory/`
- `knowledge/`
- `curator/`
- `tools/`

## Current Model Stack

Current tested model stack on the main machine:
- floor model: `nchapman/dolphin3.0-qwen2.5:3b`
- stronger fallback: `dolphin3:8b`
- heavy experimental lane: `gemma4:e4b`

Use the 3B model as the practical baseline.
Use 8B Dolphin as the stronger default fallback.
Treat Gemma 4 as an experimental heavy option, not the floor lane.

## Quick Audit Commands

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next"
git status --short
```

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next"
git log --oneline -10
```