# ModernClaw

ModernClaw is a private, local-first brain creator for local AI agents.

This repository tracks the differentiated product line built on top of the earlier Local AI desktop foundation. The app source lives in `local-ai/`, while the repo root holds project docs and private planning material.

## Current Product Direction

ModernClaw is not just a desktop chat client. It is evolving into a system for building and growing local AI brains through:
- guided brain setup
- reviewable Brain suggestions
- progressive question queues
- durable memory and knowledge files
- activity history
- staged curator imports for research and knowledge growth
- local voice input and output
- curated model and voice defaults for a practical local setup

## What Is Implemented Now

Working in this repo today:
- Brain suggestions writing into `USER.md` and `MEMORY.md`
- persistent Brain state across restarts
- dedicated `Question Queue`
- accepted / deferred suggestion lifecycle
- richer knowledge intake with summary, source, and tags
- recent Brain activity timeline
- guided Brain setup for `SOUL.md`, `USER.md`, and `MEMORY.md`
- curator staging pipeline with `Curator Inbox`, `Import to Knowledge`, and `Reject`
- Piper-backed local voice output with `Read Aloud`
- Whisper-backed local voice input into the composer
- pause / resume voice playback controls
- approved Piper voice presets and app-managed default voice paths
- Windows-validated local app flow through Ollama

## Current Model Strategy

Supported lane right now:
- floor model: `nchapman/dolphin3.0-qwen2.5:3b`
- stronger fallback: `dolphin3:8b`

Additional validated experiment on the main machine:
- `gemma4:e4b`

Gemma 4 is available for experimentation, but it is not the recommended baseline model because it is much heavier than the floor setup.

## Verification Status

Verified recently:
- `npm run build`
- `cargo check`
- live Brain workflow tests
- guided Brain setup writing to the right files
- curator inbox loading and staged package handling
- local Piper voice output in settings and chat
- local Whisper voice input into the composer
- pause / resume playback behavior
- Gemma 4 availability through Ollama

## Repo Layout

- `local-ai/`: app source
- `NEXT_PROGRESS.md`: current progress tracker
- `MVP_EXTENSION_PLAN.md`: original extension plan
- `RUNBOOK.md`: bring-up and recovery steps

## Bring-Up

See [RUNBOOK.md](./RUNBOOK.md).

Short version:

```powershell
cd "C:\Users\pento\Desktop\LocalAI-Next\local-ai"
npm run tauri:dev
```

Ollama must be installed and running separately.

## Notes

Several deeper strategy and future-feature documents are intentionally kept private and ignored from git in this workspace.