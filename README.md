# ModernClawMulti

ModernClawMulti is the current multi-brain development lane for ModernClaw, a local-first desktop workspace for building, shaping, and operating persistent AI brains.

This app combines chat, editable brain files, structured knowledge, curated imports, local model control, and voice into one system where different brains can live side by side without sharing conversations, memory, or preferences by accident.

## What ModernClawMulti Is

ModernClawMulti is not just a chat client with saved history. It is a workspace for building owned AI agents around durable context.

Each brain can have its own:
- identity and behavior
- conversations
- memory files
- knowledge files
- curated research flow
- preferred model
- preferred voice

The result is a product where one brain can act as an internal product architect and another can act as a dedicated customer support agent, while both remain grounded in the same local-first system.

## Current Product Shape

The app currently supports a working multi-brain loop with real isolation between brains.

Validated today:
- create a new brain
- switch between brains from the header
- delete unused non-baseline brains
- restore the most recent conversation per brain
- keep streaming replies attached to the brain and conversation that started them
- maintain separate conversations, memory, knowledge, and curator state per brain
- persist a preferred model per brain
- persist voice preferences per brain on top of shared machine-level Piper and Whisper installs
- normalize assistant text before text-to-speech playback so spoken replies sound more natural

Current live role split:
- `Rosie`: product development and architect brain
- `Joe`: customer service and app support brain

## Main Views

### Chat
The direct conversation surface for the active brain.

Current behavior:
- brain-scoped conversations
- latest conversation restore on switch
- isolated streaming responses
- read-aloud support for assistant replies

### Memory
The editable brain workspace.

Current behavior:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily logs
- knowledge files
- workspace content scoped to the active brain

### Brain
The structured refinement and intake surface.

Current behavior:
- guided setup area
- knowledge intake
- curator inbox review
- brain-scoped draft and feedback reset on switch

### Settings
The system contract for app-wide and brain-specific configuration.

Current behavior:
- hybrid global and brain-level settings
- app-wide defaults for machine and UX behavior
- brain-level model and voice preferences
- storage visibility
- voice testing and readiness checks

## Core Brain Model

Every brain is still built around the same durable file pattern:
- `SOUL.md`: identity, behavior, and operating style
- `USER.md`: user and business context
- `MEMORY.md`: active priorities, durable facts, and working context

This structure is the foundation of how ModernClaw shapes agent behavior without hiding the core context from the user.

## Knowledge Model

ModernClawMulti still uses a flat runtime knowledge rule.

Current rule:
- only top-level markdown files in the live `knowledge/` folder are loaded
- nested folders are ignored by the current loader

This keeps the live knowledge pack compact and predictable.

## Curator Model

Curator acts as a staged knowledge review layer.

Current behavior:
- packages can be staged for review
- staged packages can be imported into the active brain's knowledge
- imports are brain-scoped
- curator state is isolated across brains

Important current integration detail:
- the live Curator inbox follows the active brain workspace path
- when external automation prepares curator packages, it must target the active brain workspace rather than assuming one shared root curator folder
- in practice that means the live inbox may resolve under `LocalAI/agents/<active-brain>/curator/` rather than the top-level `LocalAI/curator/` root

## Models

ModernClawMulti currently uses a Gemma-first baseline.

Current supported baseline lane:
- `gemma4:e4b`

Important rule:
- the app-wide default model is only a fallback
- if the active brain has its own saved model, that brain-level choice takes priority

## Voice System

The voice system is fully local.

Current stack:
- Whisper for speech-to-text
- Piper for text-to-speech

Current voice behavior:
- Piper and Whisper executables are shared at the machine level
- brains can keep their own voice preferences on top of that shared install
- approved Piper voices in this repo are:
  - `Amy (Female)`
  - `Joe (Male)`
- assistant text is normalized before Piper playback so markdown artifacts are not spoken literally

Current live example:
- Rosie uses `Amy (Female)`
- Joe uses `Joe (Male)`

## Product Truths That Matter

The current app is built around these rules:
- brain name should match agent name
- the baseline seeded brain is `Rosie`
- support and product-architecture roles can live in separate brains
- shared machine-level tooling can still support distinct brain-level identities
- current validated product truth should win over stale plans or older docs

## Current Support-Brain Reality

Joe already exists as a live support brain in the runtime app.

What that means now:
- support can be modeled as a dedicated brain
- Joe has his own workspace, knowledge pack, and voice
- the support layer is validated in practice

What it does not mean yet:
- there is not yet a formal built-in `Get Help` entry point
- the support brain is not fully productized as a surfaced feature
- the wizard system is still future work

## Current Strengths

ModernClawMulti is already strong in these areas:
- multi-brain isolation
- editable local brain files
- grounded knowledge packs
- brain-specific model persistence
- brain-specific voice identity
- local-first trust model
- support for real product-specialist brains instead of only generic assistants

## Current Limits

The app is real and usable, but still in active development.

Current limits include:
- dependency delivery for Piper and Whisper is still manual
- broader wizard-driven brain creation is not built yet
- support-brain entry points are not fully productized yet
- deeper phase-4 validation is still ongoing in less-traveled surfaces

## Technology Stack

Core stack:
- Tauri
- React
- TypeScript
- Rust
- SQLite
- Ollama
- Piper
- Whisper
- Markdown brain files and knowledge files

## Repository Layout

- `local-ai/`: app source
- `SOURCE_SUMMARIES/`: source-layer feature documentation
- `KNOWLEDGE_PACK_MODERNCLAW/`: compiled runtime-oriented knowledge pack
- `CURATOR_REQUESTS/`: helper/reference folder for curator request-form work, not the live runtime inbox
- `CURATOR_STAGED/`: helper/reference folder for staged-package work, not the live runtime inbox
- `MULTI_BRAIN_IMPLEMENTATION_PLAN.md`: implementation roadmap
- `MULTI_BRAIN_STATUS.md`: current truth and validation status
- `MULTI_BRAIN_TEST_PLAN.md`: testing notes
- `JOE_SUPPORT_BRAIN_BLUEPRINT.md`: support-brain runtime blueprint
- `ROSIE_PRODUCT_ARCHITECT_BLUEPRINT.md`: architect-brain runtime blueprint

## Requirements

To run the app locally you currently need:
- Node.js
- Rust toolchain
- Ollama installed and running
- a supported local model available in Ollama

For voice features you also need:
- Piper installed or placed in the expected machine-level path
- Whisper installed or placed in the expected machine-level path
- required Piper voice model files
- required Whisper model files

## Run In Development

```powershell
cd "C:\Users\pento\Desktop\ModernClawMulti\local-ai"
npm install
npm run tauri:dev
```

## Voice File Expectations On Windows

Current shared Piper voices folder:
- `%APPDATA%\LocalAI\tools\piper\voices`

Required Piper files for the approved voice pair:
- `en_US-amy-medium.onnx`
- `en_US-amy-medium.onnx.json`
- `en_US-joe-medium.onnx`
- `en_US-joe-medium.onnx.json`

## Direction

The current priority is to finish stabilizing the multi-brain foundation so future layers are built on trustworthy product truths.

Near-future layers:
- support-brain productization
- guided wizard system for building custom brains
- stronger onboarding for non-technical users
- continued product-quality and rollout hardening

## Why This Repo Exists

ModernClawMulti exists so the multi-brain architecture can be explored aggressively without destabilizing the base product direction.

The goal is not to create a throwaway experiment. The goal is to build a version of ModernClaw that is strong enough to become the future mainline once its truths are stable.
