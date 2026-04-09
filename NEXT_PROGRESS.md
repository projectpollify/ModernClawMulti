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
- thumbs up / thumbs down reply feedback with local persistence
- a visible character budget in chat input
- a small response-feedback summary in Settings
- successful frontend and Rust verification (`npm run build`, `cargo check`)

## Current Phase

- Phase 4: Full Workspace Integration

## Most Recent Milestone

### M05 - Feedback Signals And Curator Workflow Documentation

- Status: Complete
- Added persistent reply feedback for assistant messages
- Added a compact response-feedback summary view in Settings
- Added a visible chat character budget
- Added `gemma4:e2b` as a lighter supported Gemma 4 option alongside the main lane
- Added NotebookLM workflow documentation and Curator request-template files to the repo

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
- live Curator package staging into the active brain workspace

## Build-State Snapshot

Built now:
- active-brain Curator staging and import
- active-brain model and voice preference persistence
- persistent thumbs up / thumbs down feedback
- lightweight local feedback summary
- Curator request-template and NotebookLM workflow documentation

Partially built:
- Curator as a one-box intake interpreter is a defined direction, but not yet a first-class in-app flow
- NotebookLM workflow is documented, but still depends on external Cowork and browser automation
- Rosie verification is designed, but not yet integrated into the live Curator review path

Not built yet:
- true NotebookLM extraction inside the live Curator pipeline
- in-app Rosie review surfacing for Curator packages
- stronger knowledge provenance, rollback, edit, and removal tooling
- deeper feedback reporting beyond the compact Settings summary

## Documentation Status

Updated in this pass:
- root `README.md`
- root `RUNBOOK.md`
- `local-ai/README.md`
- `MULTI_BRAIN_STATUS.md`
- `MULTI_BRAIN_IMPLEMENTATION_PLAN.md`
- this progress file
- `NOTEBOOKLM-CURATOR-WORKFLOW.md`

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

## Recommended Next Focus

If continuing feature work, the strongest next paths are:
1. build the real NotebookLM extraction path that turns browser output into staged Curator packages
2. decide whether Curator and Rosie should become first-class brain-scoped automation layers in the app itself
3. add stronger knowledge-management recovery tools before considering lower-friction auto-import behavior
4. avoid wizard/support work until the remaining workspace truths feel stable
