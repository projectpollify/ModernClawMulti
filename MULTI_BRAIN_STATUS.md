# Multi-Brain Status

## Project State
Status: Late Phase 3, approaching Phase 4

ModernClawMulti now has a real working multi-brain loop, not just the foundation. Users can create, switch, use, and delete isolated brains from the shell, and the app now behaves much more calmly during brain switches.

## Current Objective
Close out the remaining high-value Phase 3 polish, then move into Phase 4 workspace-integration work without widening scope unnecessarily.

## Current Phase
Phase 3: Brain Selector UX and switching polish

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
- live second-brain test completed successfully with a distinct Mia brain persona
- baseline brain now presents as `Rosie` instead of exposing the internal default naming
- duplicate display-name guardrails added for new brains
- existing duplicate names are shown with IDs when needed so they remain distinguishable
- delete-brain support added for non-baseline brains
- delete-brain guardrails added so the baseline brain is protected and the last remaining brain cannot be deleted
- dark-mode brain-selector readability bug fixed
- switching brains now restores the latest conversation for that brain when one exists
- brain switching no longer dumps the user into a blank chat unless the target brain truly has no conversations
- in-flight chat streaming is now isolated to the brain and conversation that started it
- switching brains during a long response no longer leaks streamed output into the wrong brain
- active-brain model selection now persists to the brain record itself
- switching brains now restores each brain's saved model without forcing a full workspace reload
- model picker and model cards now clearly act on the active brain instead of behaving like loose global selectors
- empty chat guidance is now brain-aware and explains why a brain may start with no conversation
- settings copy now distinguishes between app-wide fallback model behavior and brain-level model choice

## What The App Can Do Now
- seed and resolve the baseline Rosie brain
- maintain an active brain in backend settings
- scope conversations by active brain in the backend
- load memory context from the active brain workspace
- show a visible active-brain selector in the shell
- create a new brain from the shell and switch into it
- delete an unused non-baseline brain from the shell
- reload conversation and memory state when the active brain changes
- restore the latest conversation for the newly selected brain when one exists
- keep streamed responses attached to the conversation and brain that started them
- persist a preferred model to the active brain and restore it on switch
- show clearer brain-aware empty chat guidance when a brain has no conversations yet
- show the active brain more clearly in Memory, Brain, and sidebar surfaces

## What Is Still Missing
- deeper Phase 4 brain-scoped integration checks for curator, daily logs, and knowledge-import surfaces
- conversation rename improvements for multi-brain management
- optional brain rename and archive flows
- final judgment on whether the remaining polish is light enough to mark Phase 3 complete

## Current Limits
The core create, switch, isolate, and brain-level model loop is now working. The main remaining gaps are mostly around deeper workspace-surface validation and a few quality-of-life management flows.

## Next
- review curator, daily-log, and knowledge-import behavior under real multi-brain switching
- improve conversation-management polish for multi-brain use
- decide when Phase 3 is complete enough to call Phase 4 underway
- evaluate whether the remaining polish is light enough to move into Phase 4

## Immediate Risks
- stale frontend state in deeper workspace surfaces that have not yet been stressed as hard as chat switching
- user confusion if later workspace features still assume a single-brain mental model
- unfinished management polish making the system feel rougher than the underlying architecture really is

## Guardrails
- keep v1 narrow
- do not add advanced automation yet
- keep one-brain default behavior safe
- prefer explicit reloads over clever hidden state tricks
- update this status doc before or with every checkpoint commit so repo truth stays current

## Definition Of Progress
This phase will feel truly solid when:
- the user can create and manage multiple brains in the UI without confusion
- switching between brains is obvious and calm
- Memory, Brain, and conversations stay isolated without surprises
- the remaining model, management, and workspace polish no longer makes the app feel experimental
