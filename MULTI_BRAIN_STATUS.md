# Multi-Brain Status

## Project State
Status: Phase 3 brain-selector UX underway

ModernClawMulti now has the Phase 1 backend foundations, active-brain infrastructure, and the first usable create/switch brain UX.

## Current Objective
Harden the new create/switch brain flow and validate a real second-brain end-to-end pass.

## Current Phase
Phase 3: Brain Selector UX

## Completed
- project folder created
- repo name chosen: `ModernClawMulti`
- working doc set created
- baseline ModernClaw code copied into this repo
- local git repo initialized
- GitHub remote connected
- baseline commit created and pushed
- frontend dependencies installed
- `npm run build` passed on baseline
- `cargo check` passed on baseline
- `agents` migration created
- `agents` backend repository added
- `agent_id` added to conversations in the new migration path
- default brain seeding implemented in backend startup
- active-agent resolution implemented in backend workspace loading
- chat context builder updated to resolve memory from the active agent workspace
- memory commands updated to resolve the active workspace dynamically
- conversation listing and search updated to scope by active agent
- frontend agent types and API service added
- frontend active-brain store added
- Brain builder store namespaced by active brain
- app startup now loads active brain state
- app startup now reloads Memory and conversations when the active brain changes
- first visible brain selector added to the header
- create-brain flow added to the selector with isolated workspace creation and switch
- Memory, Brain, and shell copy now show the active brain more explicitly
- frontend build passes after active-brain selector and reload wiring
- backend build still passes after active-brain selector and reload wiring

## What The App Can Do Now
- seed and resolve a default brain
- maintain an active brain in backend settings
- scope conversations by active brain in the backend
- load memory context from the active brain workspace
- show a visible active-brain selector in the shell
- create a new brain from the shell and switch into it
- reload conversation and memory state when the active brain changes
- show the active brain more clearly in Memory, Brain, and sidebar surfaces

## What Is Still Missing
- frontend conversation rename improvements for multi-brain management
- stronger empty-state guidance when only the default brain exists
- frontend model override behavior per brain beyond basic wiring
- live end-to-end manual switching test with more than one real brain

## Current Limits
The app can now create and switch brains from the shell, but it still needs a real end-to-end validation pass with a second brain before we can trust the isolation fully.

## Next
- test a second real brain end to end
- improve empty states and naming polish around multi-brain behavior
- tighten per-brain model defaults and fallback behavior
- confirm no state bleed across Memory, Brain, and conversations

## Immediate Risks
- stale frontend state if a later store does not fully react to brain switching
- hidden state bleed in areas not yet explicitly reloaded
- subtle model-selection confusion if per-brain defaults are not obvious

## Guardrails
- keep v1 narrow
- do not add advanced automation yet
- keep one-brain default behavior safe
- prefer explicit reloads over clever hidden state tricks

## Definition Of Progress
This phase will feel truly solid when:
- the user can create a second brain in the UI
- switching between brains is obvious and calm
- Memory, Brain, and conversations stay isolated without surprises

