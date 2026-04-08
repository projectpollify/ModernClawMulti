# ModernClawMulti

ModernClawMulti is the multi-brain experimental branch of ModernClaw.

It keeps the local-first brain workspace model, but extends it so users can create, switch, and manage multiple isolated AI brains inside one app.

## Current Focus

This repo is currently focused on finishing the multi-brain foundation and polish before the broader wizard system is built on top of it.

The support-brain concept is already being validated live through Joe, but the full support entry points and wizard bridge still come later.

Working now:
- create brain
- switch brain
- delete non-baseline brain
- restore the latest conversation per brain
- isolate streamed replies per brain
- persist model choice per brain
- isolate Memory, Brain, Curator, Daily Logs, and Knowledge Files per brain
- support brain-specific voice preferences on top of one shared machine-level Piper and Whisper install
- normalize read-aloud speech before Piper playback so spoken responses sound more natural

## Current Brain Defaults

Current baseline shape:
- baseline brain name: `Rosie`
- baseline model lane: `gemma4:e4b`
- baseline curated voice: `Amy (Female)`

Current validated support-brain example:
- `Joe`
- role: ModernClaw tech support and customer service brain
- curated voice: `Joe (Male)`

## Voice Setup Reality

Voice works, but dependency delivery is still manual.

Current working expectation on Windows:
- shared Piper executable path at the machine level
- shared Whisper executable path at the machine level
- per-brain voice model selection layered on top of that shared install

Current approved Piper voices in this repo:
- `Amy (Female)`
- `Joe (Male)`

Required Piper files in the shared voices folder:
- `en_US-amy-medium.onnx`
- `en_US-amy-medium.onnx.json`
- `en_US-joe-medium.onnx`
- `en_US-joe-medium.onnx.json`

Expected shared folder:
- `%APPDATA%\LocalAI\tools\piper\voices`

## Verification

Recently verified in this repo:
- `npm run build`
- `cargo check`
- live multi-brain switching
- conversation isolation
- memory isolation
- curator import isolation
- brain-specific model persistence
- brain-specific Piper voice selection with Rosie and Joe
- speech text normalization before Piper playback
- Joe seeded as a live support and customer service brain with its own workspace and knowledge pack

## Repo Layout

- `local-ai/`: app source
- `MULTI_BRAIN_IMPLEMENTATION_PLAN.md`: roadmap for the experiment
- `MULTI_BRAIN_STATUS.md`: current truth and checkpoint status
- `MULTI_BRAIN_TEST_PLAN.md`: test coverage notes
- `JOE_SUPPORT_BRAIN_BLUEPRINT.md`: repo-side record of Joe's support-brain seed and operating shape

## Run

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
npm run tauri:dev
```

Ollama must be installed and running separately.
