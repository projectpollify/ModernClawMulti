# DOC_PACK_AUDIT

## Purpose

This audit compares the current documentation packs against the actual current structure of `ModernClawMulti`.

Audited folders:
- `SOURCE_SUMMARIES/`
- `KNOWLEDGE_PACK_MODERNCLAW/`

Reference implementation used for comparison:
- `local-ai/src/`
- current multi-brain app behavior already validated live in this repo

## Short Verdict

The two doc packs are still valuable, but they no longer describe the app's final structure accurately.

They are best understood as:
- strong pre-multi-brain source material
- partially updated product knowledge
- not yet a trustworthy final map of the current app

Main problem areas:
- model strategy drift
- missing multi-brain architecture
- missing brain-scoped settings and voice behavior
- older shell/navigation assumptions
- context/persistence docs still describing the old database shape

## What Is Still Strong

These truths still hold across the doc packs:
- the app is Tauri + React + Rust
- the main views are still Chat, Memory, Brain, and Settings
- the app still uses Ollama
- the app still uses markdown brain files
- the flat top-level `knowledge/*.md` loading rule is still important
- Piper is still used for voice output
- Whisper is still used for voice input
- the Brain / Curator / Memory concepts themselves are still fundamentally correct

## Main Drift Categories

### 1. Model Strategy Is Out Of Date

The packs still describe the older product lane:
- `nchapman/dolphin3.0-qwen2.5:3b` floor model
- `dolphin3:8b` fallback
- `gemma4:e4b` as a heavier experiment

That is no longer the strongest current truth for this repo.

Current repo/product direction is now much closer to:
- Gemma 4 as the real baseline lane for this experimental product track
- older 3B/8B guidance is no longer the clean current story

Files with model drift:
- `SOURCE_SUMMARIES/02-onboarding/summary.md`
- `SOURCE_SUMMARIES/03-model-management/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-models.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-overview.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-onboarding.md`

### 2. The Packs Mostly Describe A Pre-Multi-Brain App

The current codebase is now a real multi-brain app with:
- brain selector
- create brain flow
- delete brain flow
- active brain state
- per-brain conversation isolation
- per-brain memory/workspace isolation
- per-brain model persistence
- per-brain voice preferences

Most of the packs do not reflect this.

Major missing truths:
- header `BrainSelector`
- `agentStore`
- active brain switching behavior
- restore latest conversation per brain
- stream isolation per brain
- delete-brain support
- baseline Rosie brain concept

Files most affected:
- `SOURCE_SUMMARIES/00-overview/summary.md`
- `SOURCE_SUMMARIES/01-app-shell-and-navigation/summary.md`
- `SOURCE_SUMMARIES/03-model-management/summary.md`
- `SOURCE_SUMMARIES/09-settings/summary.md`
- `SOURCE_SUMMARIES/10-context-and-persistence/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-overview.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-navigation.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-models.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-settings.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-context-and-persistence.md`

### 3. Settings Documentation Is Now Incomplete

Current app behavior is more nuanced than the packs describe.

Current truth:
- some settings remain global
- model choice is persisted per brain
- voice preferences are now persisted per brain
- machine-level Piper and Whisper executable paths remain shared
- the voice UI is no longer just a simple global config surface

The settings docs mostly still describe a single settings layer.

Files most affected:
- `SOURCE_SUMMARIES/09-settings/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-settings.md`

### 4. Context/Persistence Docs Miss The New Database Shape

The current app is no longer well-described by:
- `conversations`
- `messages`
- `settings`
- `schema_version`

That was the earlier truth.

Current repo now also includes:
- `agents` registry
- active agent resolution
- `agent_id` conversation scoping
- brain-level model persistence
- brain-level voice settings
- multi-brain workspace paths under `agents/<brain>/`

The persistence docs still explain the older single-brain architecture too heavily.

Files most affected:
- `SOURCE_SUMMARIES/10-context-and-persistence/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-context-and-persistence.md`

### 5. Navigation Docs Are Incomplete

The app shell docs still mostly describe:
- sidebar
- header model selector
- theme toggle
- settings button

But the current final shell also includes:
- brain selector
- create-brain flow
- active brain identity in the header
- delete-brain support tied to brain management

Files most affected:
- `SOURCE_SUMMARIES/01-app-shell-and-navigation/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-navigation.md`

### 6. Voice Docs Are Better Than The Rest, But Still Not Final Everywhere

The voice docs were updated more recently, so they are in better shape.

Still, the final current truth is now:
- shared machine-level Piper and Whisper install
- brain-specific voice choice and voice model preferences
- validated Rosie = Amy / Mia = Joe
- text normalization now happens before Piper playback through `prepareTextForSpeech()`

That last point is brand new and is not reflected in the source pack.

Files needing the most voice updates:
- `SOURCE_SUMMARIES/08-voice-system/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-voice.md`

## File-By-File Risk Level

### SOURCE_SUMMARIES

#### 00-overview
Status: partially outdated

Still useful for:
- high-level product shape

Needs updates for:
- multi-brain structure
- current model truth
- active brain architecture

#### 01-app-shell-and-navigation
Status: outdated

Needs updates for:
- brain selector
- create/delete brain flows
- active brain visibility
- shell behavior under brain switching

#### 02-onboarding
Status: outdated

Needs updates for:
- current baseline model lane
- current product framing

#### 03-model-management
Status: outdated

Needs updates for:
- Gemma-first reality
- per-brain model persistence
- header model selector behavior in a multi-brain app

#### 04-chat-and-conversations
Status: mostly useful but incomplete

Likely still strong on basic chat flow, but should be updated for:
- per-brain conversation isolation
- restore latest conversation behavior
- stream isolation across brain switches

#### 05-memory-workspace
Status: mostly useful but incomplete

Needs updates for:
- per-brain memory workspace behavior
- active brain workspace switching

#### 06-brain-builder
Status: mostly useful but incomplete

Needs updates for:
- active brain ownership of Brain workflows
- reset behavior on brain switch

#### 07-curator-inbox
Status: mostly useful but incomplete

Needs updates for:
- active brain curator scoping
- import isolation per brain

#### 08-voice-system
Status: partially outdated

Needs updates for:
- shared machine-level tool paths
- per-brain voice preference persistence
- speech normalization pipeline before Piper playback

#### 09-settings
Status: outdated

Needs updates for:
- global versus brain-specific settings split
- per-brain model and voice behavior

#### 10-context-and-persistence
Status: outdated

Needs updates for:
- `agents` table
- `agent_id` scoping
- multi-brain workspace layout

### KNOWLEDGE_PACK_MODERNCLAW

#### modernclaw-overview
Status: outdated

Needs updates for:
- multi-brain app identity
- current model story

#### modernclaw-navigation
Status: outdated

Needs updates for:
- brain selector
- create/delete brain flows
- active brain shell behavior

#### modernclaw-onboarding
Status: outdated

Needs updates for:
- Gemma-first baseline guidance

#### modernclaw-models
Status: outdated

Needs updates for:
- per-brain model persistence
- current baseline model truth

#### modernclaw-chat
Status: probably useful but should be refreshed

Needs updates for:
- restore latest conversation per brain
- stream isolation during brain switching

#### modernclaw-memory
Status: probably useful but should be refreshed

Needs updates for:
- per-brain workspace switching

#### modernclaw-brain
Status: probably useful but should be refreshed

Needs updates for:
- Brain workflow ownership by active brain
- draft reset behavior on switch

#### modernclaw-curator
Status: probably useful but should be refreshed

Needs updates for:
- curator import isolation per brain

#### modernclaw-voice
Status: closer to current than most, but not final

Needs updates for:
- current Amy/Joe-only approved voices
- speech normalization pipeline
- per-brain voice preferences on shared machine tools

#### modernclaw-settings
Status: outdated

Needs updates for:
- split between global settings and brain-specific settings

#### modernclaw-context-and-persistence
Status: outdated

Needs updates for:
- `agents`
- `agent_id`
- multi-brain workspace layout
- brain-scoped settings/model/voice persistence

## Most Trustworthy Docs Right Now

The most trustworthy docs in the two packs today are not the ones that describe the whole app. The most trustworthy parts are the ones describing concepts that survived the transition:
- flat knowledge loading rule
- markdown brain files
- Brain / Curator / Memory concept split
- general voice stack (Piper + Whisper)

## Least Trustworthy Docs Right Now

The least trustworthy docs are the ones that present the current app as if it were still a single-brain system with the older model lane:
- model docs
- onboarding docs
- settings docs
- context/persistence docs
- navigation overview docs

## Recommendation

Do not treat these two folders as the final truth source for the current app anymore.

Recommended next step:
1. keep them as useful seed material
2. create a focused refresh pass specifically for the current multi-brain truth
3. update the highest-drift files first:
   - overview
   - navigation
   - onboarding
   - models
   - settings
   - context-and-persistence

## Priority Refresh Order

### Highest Priority
- `SOURCE_SUMMARIES/00-overview/summary.md`
- `SOURCE_SUMMARIES/01-app-shell-and-navigation/summary.md`
- `SOURCE_SUMMARIES/03-model-management/summary.md`
- `SOURCE_SUMMARIES/09-settings/summary.md`
- `SOURCE_SUMMARIES/10-context-and-persistence/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-overview.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-navigation.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-models.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-settings.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-context-and-persistence.md`

### Medium Priority
- `SOURCE_SUMMARIES/02-onboarding/summary.md`
- `SOURCE_SUMMARIES/04-chat-and-conversations/summary.md`
- `SOURCE_SUMMARIES/05-memory-workspace/summary.md`
- `SOURCE_SUMMARIES/06-brain-builder/summary.md`
- `SOURCE_SUMMARIES/07-curator-inbox/summary.md`
- `SOURCE_SUMMARIES/08-voice-system/summary.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-onboarding.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-chat.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-memory.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-brain.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-curator.md`
- `KNOWLEDGE_PACK_MODERNCLAW/modernclaw-voice.md`

## Bottom Line

Yes, these folders hold the blueprint for the app before it kept evolving.

They are still useful.
But they are no longer a trustworthy final reflection of what ModernClawMulti actually is today.

The current app has moved beyond them in some of the most important areas:
- multi-brain architecture
- model ownership
- voice ownership
- settings ownership
- context and persistence shape

They should now be treated as:
- partially valid source material
- not final product truth
