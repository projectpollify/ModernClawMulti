# ModernClaw Progress

## Purpose

This file tracks the differentiated product track in `LocalAI-Next`, now operating under the ModernClaw brand direction.

The original MVP remains in the separate `LocalAI` workspace.
This file is only for the extension product work.

## Current Status

ModernClaw is now a working brain-building and voice-enabled layer on top of the validated Local AI foundation.

What is real right now:
- Brain suggestions UI
- persistent Brain state
- dedicated `Question Queue`
- accepted / deferred / dismissed lifecycle
- Brain activity history
- guided Brain setup
- structured knowledge intake
- curator staging pipeline and Curator Inbox
- local Piper voice output in settings and chat
- local Whisper voice input into the composer
- pause / resume / stop voice playback controls
- approved Piper voice presets with app-managed default paths
- a defined model stack:
  - floor model: `nchapman/dolphin3.0-qwen2.5:3b`
  - stronger fallback: `dolphin3:8b`
  - experimental heavy lane: `gemma4:e4b`
- Windows-validated local app flow through Ollama
- successful frontend and Rust verification (`npm run build`, `cargo check`)

## Milestones

### X00 - Track Setup

- Status: Complete
- Created separate `LocalAI-Next` workspace
- Preserved the original MVP in `LocalAI`
- Added `MVP_EXTENSION_PLAN.md`
- Added this progress tracker

### X01 - Suggestion System Foundation

- Status: Complete
- Added Brain suggestion data model and store
- Added `BrainView` and `SuggestionCard`
- Seeded profile, memory, knowledge, and behavior suggestions
- Wired the Brain view into the app shell

### X02 - First Progressive Profile Loop

- Status: Complete
- Added draft-answer flow for profile questions
- Accepting the `profile-role` suggestion writes structured content into `USER.md`
- Accepting the `memory-projects` suggestion writes structured content into `MEMORY.md`
- Verified live in the app

### X03 - Persistent Brain Queue Workflow

- Status: Complete
- Persisted Brain suggestion state across restarts
- Added dedicated `Question Queue`
- Added accepted / deferred sections
- Verified defer + restart behavior live

### X04 - Brain History And Deeper Knowledge Intake

- Status: Complete
- Added persistent Brain activity history
- Added richer knowledge intake with summary, tags, and source
- Added recent knowledge additions view in Brain
- Verified build success and live workflow behavior

### X05 - Guided Brain Setup

- Status: Complete
- Added guided setup for `SOUL.md`, `USER.md`, and `MEMORY.md`
- Added setup readiness badges
- Added activity logging for guided setup
- Verified live file updates in the app

### X06 - Curator Inbox And Staged Pipeline

- Status: Complete
- Added backend `curator/` pipeline support
- Added staged curator package listing
- Added `Import to Knowledge` and `Reject` actions
- Added `Curator Inbox` to Brain
- Fixed Brain memory initialization so curator packages load on first Brain visit
- Added explicit Curator Inbox refresh control for packages added outside the app
- Verified build success and live staged-package behavior

### X07 - Local Voice Output

- Status: Complete
- Added Piper-backed local voice output commands in Tauri
- Added voice settings for Piper executable path, voice model path, status, and test playback
- Added `Read Aloud` on assistant messages in chat
- Verified live local speech playback in the app

### X08 - Voice Input And Playback Polish

- Status: Complete
- Added Whisper-backed local voice input into the chat composer
- Added pause / resume / stop controls for voice playback
- Fixed false end-of-playback error handling in the app audio layer
- Verified live local transcription and playback control behavior

### X09 - Curated Voice And Model Defaults

- Status: Complete
- Added approved Piper voice presets (`Lessac`, `Amy`, `Joe`)
- Added app-managed default voice tool paths under the memory workspace
- Set `nchapman/dolphin3.0-qwen2.5:3b` as the supported floor model
- Kept `dolphin3:8b` as the stronger fallback model
- Updated onboarding and settings copy to reflect the supported model lane
- Confirmed `gemma4:e4b` works as an additional experimental Ollama model on the main machine

## Verification Summary

Most recent verified checks:
- `npm run build`
- `cargo check`
- live Brain queue behavior
- live guided setup behavior
- live Curator Inbox behavior
- live Piper voice output behavior
- live Whisper voice input behavior
- live pause / resume playback behavior
- live Gemma 4 model availability through Ollama

## Documentation Status

Updated in this pass:
- root `README.md`
- root `RUNBOOK.md`
- `local-ai/README.md`
- this progress file
- roadmap sync
- model-selection plan sync

## Current Gaps / Watch Items

- No automated tests yet for Brain or curator workflows
- Knowledge and curator content added outside the app still requires refresh
- Bundle size warning remains from Vite chunking
- App/package naming in code still reflects the older `local-ai` scaffold and `LocalAI` app-data root
- Voice dependency delivery is still manual:
  - folders are provisioned automatically
  - Piper / Whisper executables and models are not auto-downloaded yet
- Clean-machine voice setup still needs a guided install/download path
- Multi-brain support is still planning-stage, not implemented
- Gemma 4 is available for experimentation but is not the recommended baseline lane

## Recommended Next Focus

If continuing feature work, the strongest next paths are:
1. decide whether to build a lightweight guided voice dependency setup flow next
2. use the current app to build the next real expert brain (Janet)
3. continue fresh-machine install validation before adding more complexity