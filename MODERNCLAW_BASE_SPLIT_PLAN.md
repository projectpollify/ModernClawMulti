# ModernClaw Base Split Plan

## Purpose

This document defines how to create `ModernClawBase` from the current `ModernClawMulti` product, and how to separate the free open-source edition from the flagship advanced edition without damaging product trust.

This plan replaces the older assumption that the original `ModernClaw` repo should become the base product as-is.

Current truth:
- `ModernClawMulti` is now the most complete, most current, most validated version of the app.
- `ModernClawBase` should be derived from the current `ModernClawMulti` product truth, not from an older pre-multi branch state.

---

## Executive Decision

### Product Roles

`ModernClawMulti`
- flagship app
- most advanced working version
- current source of product truth
- lane for premium, advanced, and multi-brain workflows

`ModernClawBase`
- free and open-source edition
- clean single-brain version of the product
- trustworthy core workspace for local AI use
- built from the current `ModernClawMulti` snapshot, then simplified

### Key Product Principle

Do not build `ModernClawBase` from the older repo state.

Build `ModernClawBase` from the current `ModernClawMulti` implementation, because `ModernClawMulti` now contains the better truths:
- stronger product language
- Gemma-first model strategy
- better voice pipeline behavior
- better settings clarity
- better doc accuracy
- more mature workspace logic
- stronger product identity

---

## Why This Split Makes Sense

Trying to treat the old single-repo history as the real base product creates drift.

The current app truth now lives in `ModernClawMulti`.
That means the clean move is:

1. treat `ModernClawMulti` as the current flagship and truth source
2. derive `ModernClawBase` from that truth
3. remove the advanced/premium layers from the base edition
4. keep the resulting open-core app strong enough to stand on its own

This keeps the product family coherent.

---

## Product Family Model

### ModernClawBase

A clean, single-brain, local-first AI workspace.

What it should feel like:
- understandable
- useful
- not crippled
- good enough to recommend on its own
- easy to support around one clear baseline model lane

### ModernClawMulti

The advanced edition.

What it should feel like:
- more powerful
- more configurable
- better for people managing multiple roles or agents
- a clear upgrade for users who need scale, separation, and advanced workflows

---

## What Must Stay True In Base

The free version must still feel like a real product.

It should preserve the core ModernClaw identity:
- local-first
- editable brain files
- durable context
- knowledge ingestion
- model-backed chat
- practical voice support
- approachable workspace structure

The free version should not feel like:
- a teaser shell
- a broken demo
- a paywall funnel with no real usefulness

---

## Recommended ModernClawBase Feature Set

### Core Workspace
- single-brain chat experience
- conversation history
- editable `SOUL.md`, `USER.md`, and `MEMORY.md`
- daily logs
- knowledge file ingestion
- Memory view
- Brain view
- Settings view

### Brain Building
- core Brain builder workflows
- guided setup area if it remains stable enough
- knowledge intake
- Curator staging if it remains part of the essential brain-building model

### Model Layer
- Ollama integration
- one clean baseline model lane: `gemma4:e4b`
- simple model refresh flow
- no confusing buffet of default model lanes in the free product

### Voice Layer
- basic voice output and input support
- shared machine-level Piper and Whisper setup
- speech normalization before Piper playback
- one clean voice story in the free product

### Product UX
- stable onboarding
- stable settings
- clear storage behavior
- good documentation
- clear install/setup instructions

---

## Recommended ModernClawMulti Feature Set

ModernClawMulti should keep the advanced layers that create a strong upgrade path.

### Advanced Workspace Model
- multiple brains
- create brain
- switch brain
- delete brain
- isolated workspace per brain
- separate model preference per brain
- separate voice identity per brain
- separate role design per brain

### Advanced Product Role Model
- `Rosie` as internal product-development / architect brain
- `Joe` as customer support / service brain
- future role-specialized brains as needed

### Advanced Workflow Layer
- support-brain productization
- formal help entry points
- wizard system
- future guided brain creation and refinement
- advanced workspace management tools

### Advanced Voice Layer
- multiple approved curated voices
- per-brain voice identity
- future expressive speech shaping
- potential premium voice packs later

### Advanced Knowledge Layer
- richer knowledge compilation workflows
- more powerful source ingestion
- future fetcher/automation workflows
- advanced templates and expert brain packs

---

## Features That Should Stay Out Of Base At First

These are the strongest candidates to keep in `ModernClawMulti` or future paid lanes:
- multi-brain management
- separate support and architect brains as surfaced product features
- per-brain model switching
- per-brain voice identity
- advanced brain lifecycle management
- wizard system
- premium knowledge ingestion features
- automation and recurring workflows
- expert business brain packs
- enterprise or team workflow layers

---

## Paid Feature Strategy

The clean monetization rule is:

Charge for:
- convenience
- scale
- specialization
- power-user control
- curated value
- saved time

Do not charge for:
- core trust
- core usability
- the basic app being genuinely useful

### Strong Paid Feature Categories

#### 1. Multi-Brain Workspace
This is one of the strongest premium anchors.

Includes:
- multiple brains
- role separation
- separate memory and knowledge per brain
- separate model and voice identity per brain

#### 2. Brain Management Pack
- archive brain
- clone brain
- duplicate as template
- export/import brain
- snapshots and restore points
- compare brains

#### 3. Workspace Organization Pack
This is where conversation rename fits.

Recommended bundle:
- conversation rename
- pinned conversations
- folders or tags
- archive conversation
- better search
- starred chats

Important recommendation:
- do not sell conversation rename by itself
- it is better as part of a broader organization and productivity pack

#### 4. Wizard System
A premium guided workflow layer.

Examples:
- full brain creation wizard
- personality wizard
- memory refinement wizard
- knowledge import wizard
- voice setup wizard
- model setup wizard

#### 5. Premium Knowledge Tools
- website-to-knowledge import
- PDF/doc import cleanup
- structured source fetching
- dedupe/conflict handling
- compiled knowledge packaging

#### 6. Upgraded Model Lanes
This does not mean charging for the open model itself.
It means charging for the validated integration and premium experience around stronger model lanes.

Examples:
- stronger recommended model profiles
- hardware-aware setup guidance
- per-brain advanced model routing
- better support for heavier local setups

#### 7. Premium Voice Layer
- additional approved curated voices
- better voice presets
- future expressive speech shaping
- improved setup experience

#### 8. Expert Brain Packs
- customer support brain templates
- marketing brain packs
- founder/operator packs
- vertical-specific expert packs

#### 9. Automation Layer
- recurring summaries
- scheduled knowledge refresh
- automated curator intake
- workflow helpers

#### 10. Business and Team Layers
- client-separated brains
- SOP packs
- business support brains
- team-ready workflows
- enterprise connectors

---

## Features That Should Probably Stay Free

To preserve trust, `ModernClawBase` should likely keep:
- one usable brain
- chat
- conversation history
- editable brain files
- knowledge ingestion
- Brain view
- stable settings
- Gemma 4 baseline lane
- basic voice support
- the local-first trust model

These are core identity features, not premium upsells.

---

## Conversation Rename Decision

Current recommendation:
- do not sell chat renaming as a standalone feature
- bundle it into a paid workspace organization pack if it stays outside base

Why:
- by itself it feels too small and petty to monetize
- bundled with pins, folders, archive, search, and starred items, it becomes a real quality-of-life upgrade

---

## Support Brain Decision

Current truth:
- Joe already exists and works as a live support brain in `ModernClawMulti`
- support as a dedicated brain is validated

Recommendation:
- keep the current dedicated support-brain productization in `ModernClawMulti` for now
- let `ModernClawBase` stay simpler and single-brain initially
- later decide whether a lightweight built-in support mode belongs in base

This prevents the base product from immediately inheriting multi-role complexity.

---

## Wizard Decision

Current truth:
- the wizard is important
- but it should sit on top of stable product truths

Recommendation:
- keep full wizard work in the advanced lane first
- let `ModernClawBase` focus on a simpler, stable single-brain experience initially
- later promote a lightweight wizard into base only if it becomes a strong part of the core product identity

---

## Recommended Technical Split Strategy

### Step 1: Use ModernClawMulti As The Source Snapshot
Create `ModernClawBase` from the current `ModernClawMulti` codebase.

Not from:
- the old original repo state
- stale docs
- pre-multi assumptions

### Step 2: Simplify To A Single-Brain App
Remove or hide:
- brain selector
- create/switch/delete brain UI
- multi-brain role model in the user-facing shell
- advanced per-brain identity surfaces

Keep:
- the mature workspace logic
- the improved voice behavior
- the improved product docs and language
- the Gemma-first defaults

### Step 3: Simplify Settings For Base
Keep settings understandable around one brain.

Base should likely present:
- one clear model lane
- one clear voice path story
- one clear workspace story

### Step 4: Rewrite Docs Around The Base Identity
ModernClawBase docs should describe:
- one brain
- one workspace
- one baseline model lane
- one approachable setup path

### Step 5: Preserve Upgrade Story
The migration from Base to Multi should feel natural.

A user should be able to understand:
- Base is the clean single-brain edition
- Multi is the advanced workspace edition

---

## Recommended Base vs Multi Feature Matrix

### ModernClawBase
- single brain
- chat
- conversation history
- `SOUL.md`, `USER.md`, `MEMORY.md`
- daily logs
- knowledge files
- Brain view
- Curator if kept as core
- Settings
- Ollama integration
- Gemma 4 baseline
- basic voice support
- speech normalization
- clean onboarding and docs

### ModernClawMulti
- all base capabilities plus:
- multi-brain management
- separate product roles across brains
- per-brain model persistence
- per-brain voice identity
- support brain as dedicated product specialist
- future wizard system
- advanced knowledge workflows
- future automation and expert packs

---

## Documentation Strategy

### ModernClawBase Docs
Keep docs disciplined and public-facing.

Recommended:
- `README.md`
- install/setup guide
- runbook
- progress/status doc
- short architecture summary only if it remains current

### ModernClawMulti Docs
Keep docs lean, truthful, and operational.

Recommended:
- feature truth docs
- split-plan docs
- status docs
- blueprint docs for special brains
- implementation plans for premium/advanced layers

---

## Repo Strategy

### ModernClawMulti
This remains:
- the flagship app
- the current source of truth
- the advanced and premium lane

### ModernClawBase
This becomes:
- a new repo
- derived from current `ModernClawMulti`
- the free open-source edition
- the trust anchor for the wider product family

### Future Repos
Create only when the feature is real enough to justify it.

Examples:
- `ModernClawAutomation`
- `ModernClawExperts`
- `ModernClawEnterprise`

---

## Immediate Recommended Next Steps

1. Treat `ModernClawMulti` as the source of truth going forward.
2. Stop thinking of the older repo state as the future base by default.
3. Create a new `ModernClawBase` repo from the current `ModernClawMulti` snapshot.
4. Remove the multi-brain product layer from that new base repo.
5. Keep Gemma 4 as the one clean base model lane.
6. Decide which quality-of-life features stay in Multi as premium differentiators.
7. Keep the base app respectable enough that users trust the whole product family.

---

## Guiding Principle

`ModernClawBase` should be:
- open
- useful
- simple
- stable
- trustworthy

`ModernClawMulti` should be:
- more powerful
- more specialized
- more convenient
- more scalable
- clearly worth paying for

That is the cleanest way to turn the current app family into a product family without confusing users or weakening the open-core story.
