# ModernClawMulti Progress

## Purpose

This file tracks the multi-brain experimental product track in `ModernClawMulti`.

## Current Status

ModernClawMulti is now a working multi-brain layer on top of the validated ModernClaw foundation.

What is real right now:
- create / switch / delete brain flows
- active-brain conversation isolation
- active-brain Memory, Brain, Curator, Daily Log, and Knowledge isolation
- latest-conversation restore on brain switch
- stream isolation across brain switches
- per-brain model persistence
- per-brain Piper and Whisper preference persistence
- shared machine-level Piper and Whisper install paths
- validated Rosie and Mia Piper voice split:
  - Rosie -> `Amy (Female)`
  - Mia -> `Joe (Male)`
- successful frontend and Rust verification (`npm run build`, `cargo check`)

## Current Phase

- Phase 4: Full Workspace Integration

## Most Recent Milestone

### M04 - Brain-Scoped Voice Settings On Shared Machine Tools

- Status: Complete
- Added brain-level voice settings to the `agents` record
- Kept executable paths machine-level while allowing brain-specific model and voice preferences
- Fixed shared voice path resolution so brains under `agents/<brain>` still use the shared `%APPDATA%\LocalAI\tools` install
- Simplified approved Piper voices to `Amy (Female)` and `Joe (Male)`
- Validated Rosie with Amy and Mia with Joe live in the app

## Verification Summary

Most recent verified checks:
- `npm run build`
- `cargo check`
- live multi-brain switching
- live conversation isolation
- live memory and knowledge isolation
- live curator import isolation
- live brain-specific model persistence
- live brain-specific Piper voice selection with Rosie and Mia

## Documentation Status

Updated in this pass:
- root `README.md`
- root `RUNBOOK.md`
- `local-ai/README.md`
- `MULTI_BRAIN_STATUS.md`
- `MULTI_BRAIN_IMPLEMENTATION_PLAN.md`
- this progress file

## Current Gaps / Watch Items

- No automated tests yet for multi-brain workflows
- Knowledge and curator content added outside the app still requires refresh
- Bundle size warning remains from Vite chunking
- Voice dependency delivery is still manual on clean machines
- Whisper validation still needs a fresh-machine confirmation pass after the Piper cleanup
- Wizard and support-brain layers are still intentionally deferred until the workspace truths feel stable

## Recommended Next Focus

If continuing feature work, the strongest next paths are:
1. continue targeted Phase 4 validation on the remaining less-traveled surfaces
2. decide whether Whisper settings need any further per-brain polish after the Piper pass
3. avoid wizard/support work until the remaining workspace truths feel stable
