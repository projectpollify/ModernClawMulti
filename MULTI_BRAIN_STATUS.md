# Multi-Brain Status

## Project State
Status: Phase 4 underway

ModernClawMulti now has a real working multi-brain loop with stable create, switch, delete, conversation restore, stream isolation, brain-level model persistence, and increasingly mature brain-scoped workspace behavior.

## Current Objective
Continue Phase 4 by validating and hardening the remaining brain-scoped workspace flows while preserving the calm, predictable behavior already achieved in Chat, Memory, Brain, Settings, Curator, and model switching.

## Current Phase
Phase 4: Full Workspace Integration

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
- inline conversation rename added to the sidebar for multi-brain conversation management
- sidebar empty state now names the active brain directly when it has no conversations
- daily log isolation validated across Rosie and Mia
- knowledge storage isolation validated across Rosie and Mia
- curator inbox isolation validated across Rosie and Mia
- transient Brain-view feedback now resets on brain switch so success banners do not leak between brains
- Brain guided-setup drafts, knowledge-intake drafts, and curator action feedback now reset cleanly on brain switch
- daily-log composer and viewer now reset or reload cleanly when switching brains mid-flow
- Curator inbox UI now explicitly names the active brain
- Daily Logs UI now explicitly indicates entries belong to the active brain
- Knowledge Files UI now explicitly indicates files belong to the active brain
- deeper workspace surfaces now feel visibly brain-scoped, not just technically scoped
- Settings now reload with the active brain so workspace path information updates correctly on switch
- curator import and reject actions were fixed after a Tauri payload mismatch blocked the buttons
- curator import was re-validated end to end and imported knowledge stays isolated to the correct brain
- active-brain workspace resolution is now the governing rule for external Curator integration as well as internal app state
- Curator automation now needs to target the active brain workspace instead of assuming the top-level `LocalAI\curator\` root
- brain-level voice settings added for output/input preferences and model paths
- shared machine-level Piper and Whisper defaults now resolve correctly even when the active brain lives under `agents/<brain>`
- old brain-local tool paths now fall back safely to the shared machine install instead of staying pinned to empty per-brain `tools/` folders
- approved Piper voice list simplified to `Amy (Female)` and `Joe (Male)`
- Rosie voice output validated with `Amy (Female)`
- speech normalization added before Piper playback so markdown and formatting artifacts are not spoken literally
- Joe support brain created as a live dedicated tech support and customer service brain
- Joe workspace seeded with support-specific `SOUL.md`, `USER.md`, `MEMORY.md`, a support playbook, and the refreshed ModernClaw knowledge pack
- Joe voice output validated with `Joe (Male)`
- Rosie and Joe now have distinct validated female/male voice identities in the live app

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
- persist brain-specific Piper and Whisper preferences on top of a shared machine-level install
- rename conversations inline from the sidebar
- show clearer brain-aware empty guidance in chat and sidebar surfaces
- show the active brain more clearly in Memory, Brain, Curator, Daily Logs, Knowledge Files, and sidebar surfaces
- keep Brain and daily-log draft surfaces from carrying stale state across brain switches
- keep Settings workspace information aligned with the currently active brain
- import staged curator knowledge into the correct brain workspace without cross-brain bleed
- require external Curator staging to land in the active brain workspace if the packages are meant to appear in the live Curator Inbox
- give Rosie and Joe different validated Piper voices without requiring separate Piper installs
- run Joe as a live support and customer service brain with his own workspace, support files, and ModernClaw knowledge pack

## What Is Still Missing
- broader Phase 4 stress-testing of all remaining brain-scoped workflows under real use
- final review of whether any hidden single-brain assumptions remain in less frequently used surfaces
- cleanup of any stale docs or helper folders that still imply one shared root Curator inbox
- optional archive-brain support if later needed
- formal product entry points for the support brain
- the full wizard system for guided brain creation and refinement

## Current Limits
The structural multi-brain system is working. The remaining work is no longer about proving the architecture. It is about continuing to validate, harden, and polish the full workspace experience so new feature layers are built on stable truths.

Joe already exists as a live runtime brain, but the support layer is not yet fully productized as a built-in help surface with dedicated entry points and guided flows.

## Next
- continue targeted Phase 4 validation under real use
- keep watching for stale state in less frequently used workspace surfaces after this latest voice/settings pass
- avoid adding the full wizard system until the remaining workspace truths feel stable
- keep the support-brain concept documented and grounded in current product truth while the broader feature layer matures
- checkpoint stability improvements as they are proven

## Immediate Risks
- stale frontend state in deeper workspace surfaces that have not yet been stressed as hard as chat, memory, and brain switching
- future features accidentally reintroducing single-brain assumptions
- moving into wizard/support product layers before the remaining workspace truths are stable enough

## Guardrails
- keep v1 narrow
- do not add advanced automation yet
- keep one-brain default behavior safe
- prefer explicit reloads over clever hidden state tricks
- update this status doc before or with every checkpoint commit so repo truth stays current

## Definition Of Progress
Phase 4 will feel healthy when:
- every major workspace surface behaves as obviously brain-scoped
- remaining hidden bleed risks are squeezed out through real use
- new feature work can rely on stable multi-brain truths instead of assumptions
