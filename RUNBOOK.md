# RUNBOOK

## Purpose

This runbook records the bring-up, recovery, and basic verification steps for the ModernClawMulti workspace.

## Current Workspace

- Repo root: `C:\Users\pento\Desktop\ModernClawMulti`
- App source: `C:\Users\pento\Desktop\ModernClawMulti\local-ai`

## Daily Bring-Up

1. Make sure Ollama is installed.
2. Start Ollama if it is not already running.
3. Start the app in Tauri dev mode.

### Commands

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
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
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
npm run build
```

### Rust / Tauri Backend

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai\src-tauri"
cargo check
```

## Packaged Build

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
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
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai\src-tauri"
cargo clean
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
npm run tauri:dev
```

### Sidebar appears lost

If the app gets into a strange UI state, restart dev mode.

## Voice Setup Notes

### Current Voice Features

Working now:
- local voice output through Piper
- local voice input through Whisper
- pause / resume / stop playback controls
- brain-specific Piper voice choice layered on top of a shared machine-level install

### Current Default Voice Tool Layout

The app provisions shared folders under the LocalAI app-data root:
- `%APPDATA%\LocalAI\tools\piper\`
- `%APPDATA%\LocalAI\tools\piper\voices\`
- `%APPDATA%\LocalAI\tools\whisper\`
- `%APPDATA%\LocalAI\tools\whisper\models\`

Important current limitation:
- the folders are auto-created
- Piper / Whisper executables and model files are **not** auto-downloaded yet
- clean-machine setup still requires manual dependency placement or installation

### Current Validated Piper Setup

Current validated Piper files on this machine:
- Piper executable: `C:\Tools\piper\piper.exe`
- Amy model: `%APPDATA%\LocalAI\tools\piper\voices\en_US-amy-medium.onnx`
- Amy metadata: `%APPDATA%\LocalAI\tools\piper\voices\en_US-amy-medium.onnx.json`
- Joe model: `%APPDATA%\LocalAI\tools\piper\voices\en_US-joe-medium.onnx`
- Joe metadata: `%APPDATA%\LocalAI\tools\piper\voices\en_US-joe-medium.onnx.json`

Current validated Whisper setup:
- Whisper executable: `C:\Tools\whisper\release\whisper-cli.exe`
- Whisper model: `%APPDATA%\LocalAI\tools\whisper\models\ggml-base.en.bin`

### Approved Voice Presets

Current approved Piper voice presets in the app:
- `Amy (Female)`
- `Joe (Male)`

### Brain Voice Validation

Validated live:
- Rosie uses `Amy (Female)`
- Mia uses `Joe (Male)`
- both brains share the same machine-level Piper install
- model and voice selection persist when switching brains

## Brain / Curator Notes

- Brain data loads from the active local brain workspace managed by the app.
- Curator staged packages are read from the active brain's `curator/staged/` area.
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
- `agents/`

## Current Model Stack

Current tested baseline on this machine:
- baseline model: `gemma4:e4b`

## Quick Audit Commands

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti"
git status --short
```

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti"
git log --oneline -10
```
