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
- validated Rosie and Joe Piper voice split:
  - Rosie -> `Amy (Female)`
  - Joe -> `Joe (Male)`
- thumbs up / thumbs down reply feedback with local persistence
- a visible character budget in chat input
- a small response-feedback summary in Settings
- shared setup-readiness checks in onboarding, Settings, and chat
- image attachments in the composer and conversation UI
- workspace-backed attachment storage instead of SQLite image blobs
- successful frontend and Rust verification (`npm run build`, `cargo check`)

## Current Phase

- Phase 4: Full Workspace Integration

## Most Recent Milestone

### M06 - Setup Readiness And Image Input

- Status: Complete
- Added a shared setup-readiness layer across onboarding, Settings, and chat
- Added required vs optional setup classification for core chat readiness
- Added drag-drop and picker-based image attachments
- Added image previews in the composer and image rendering in chat history
- Added workspace attachment storage and attachment persistence in history
- Added multimodal image routing into the Ollama request path

## Verification Summary

Most recent verified checks:
- `npm run build`
- `cargo check`
- live multi-brain switching
- live conversation isolation
- live memory and knowledge isolation
- live curator import isolation
- live brain-specific model persistence
- live brain-specific Piper voice selection with Rosie and Joe
- live Curator package staging into the active brain workspace
- image attachment compile path from UI to backend storage

## Build-State Snapshot

Built now:
- active-brain Curator staging and import
- active-brain model and voice preference persistence
- persistent thumbs up / thumbs down feedback
- lightweight local feedback summary
- Curator request-template and NotebookLM workflow documentation
- shared setup-readiness checks for required and optional dependencies
- image attachments copied into the active brain workspace
- image rendering in chat history and multimodal request routing

Partially built:
- Curator as a one-box intake interpreter is a defined direction, but not yet a first-class in-app flow
- NotebookLM workflow is documented, but still depends on external Cowork and browser automation
- Rosie verification is designed, but not yet integrated into the live Curator review path
- dedicated setup navigation inside the multi-brain shell is not finished even though readiness surfaces are live
- audio understanding is still transcript-first and not yet routed through the new attachment pipeline

Not built yet:
- true NotebookLM extraction inside the live Curator pipeline
- in-app Rosie review surfacing for Curator packages
- stronger knowledge provenance, rollback, edit, and removal tooling
- deeper feedback reporting beyond the compact Settings summary
- audio-note attachment handling through Whisper plus saved attachment metadata

## Documentation Status

Updated in this pass:
- root `README.md`
- root `RUNBOOK.md`
- this progress file

Curator automation documentation already in repo:
- `CURATOR-SYSTEM-CONFIG.md` - live operational config, folder paths, task settings, replication checklist
- `NOTEBOOKLM-CURATOR-WORKFLOW.md` - pipeline design, build state, risks
- `CURATOR_TASKS/` - rebuild reference copies of both Cowork task prompts and the request form template

## Current Gaps / Watch Items

- No automated tests yet for multi-brain workflows
- Knowledge and curator content added outside the app still requires refresh
- External Curator automation must follow the active brain workspace path rather than the top-level `LocalAI\curator\` root
- Bundle size warning remains from Vite chunking
- Voice dependency delivery is still manual on clean machines
- Whisper validation still needs a fresh-machine confirmation pass after the Piper cleanup
- NotebookLM extraction still depends on external automation and browser state
- Rosie verification is not yet visible in the app workflow
- Wizard and support-brain layers are still intentionally deferred until the workspace truths feel stable
- audio-note capture and transcript attachment flow is the next multimodal gap

## Recommended Next Focus

If continuing feature work, the strongest next paths are:
1. add the audio-note MVP by saving audio files, transcribing with Whisper, and sending transcript plus attachment metadata through the same pipeline as images
2. build the real NotebookLM extraction path that turns browser output into staged Curator packages
3. decide whether Curator and Rosie should become first-class brain-scoped automation layers in the app itself
4. add stronger knowledge-management recovery tools before considering lower-friction auto-import behavior
