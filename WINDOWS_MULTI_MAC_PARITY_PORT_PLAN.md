# Windows Multi Mac-Parity Port Plan

## Purpose

This plan describes how to bring `ModernClawMultifinal` on Windows up to functional parity with `ModernClawMacMulti`, while preserving Windows-specific process handling and avoiding unnecessary repo-layout churn.

Compared repos:
- Mac reference: `C:\Users\Admin\Desktop\ModernClawMacMulti`
- Windows target: `C:\Users\Admin\Desktop\ModernClawMultifinal`

Related reference note:
- `C:\Users\Admin\Desktop\ModernClawMultifinal\MAC_MULTI_DIFF_SUMMARY.md`

## Goal

Make the Windows premium multi-brain app match the Mac multi app in the areas that matter functionally:
- direct-engine support and provider-aware runtime behavior
- setup and onboarding experience
- model and settings behavior
- chat metrics and richer response feedback
- polished multi-brain UI behavior

## Definition of Done

Windows multi will be considered at parity when all of the following are true:

1. The app no longer depends on Ollama as the primary runtime path.
2. The app can start and talk to the direct engine on Windows.
3. Multi-brain behavior still works after the engine swap.
4. Setup and onboarding match the Mac multi experience in practical user flow.
5. Chat UI includes response metrics, thinking visuals, and feedback notes.
6. Model selection and settings behavior match the Mac multi product behavior.
7. The app passes build, startup, and live-chat verification on Windows.

## Non-Goals

The following are explicitly out of scope unless requested later:
- full repo-structure standardization to match the Mac repo root
- redesigning multi-brain architecture beyond what is required for parity
- major new features that do not already exist in the Mac multi reference
- packaging/distribution redesign during the runtime migration itself
- packaged installer behavior parity beyond keeping the existing Windows app buildable

## Guiding Rules

1. Treat `ModernClawMacMulti` as the behavioral source of truth.
2. Do not blindly copy Mac code when it carries platform-specific shortcuts.
3. Keep Windows-safe process management for engine start/stop/restart.
4. Keep each milestone independently testable.
5. Commit at clear checkpoints after each stable milestone.

## Current Gap Summary

The Windows multi repo is the older Ollama-first line.

The Mac multi repo already includes:
- provider-aware runtime logic
- direct-engine setup commands
- setup action hooks and provider-aware setup copy
- direct-engine settings fields
- richer model resolution behavior
- response metrics and thinking pulse UI
- feedback-note schema and persistence
- broader visual and branding polish

The biggest difference is not visual. It is the runtime layer and the product flow around it.

## Port Strategy Overview

Port in this order:

1. Runtime and provider layer
2. Setup and onboarding parity
3. Model/settings parity
4. Chat metrics and feedback-note persistence
5. Visual and branding polish
6. Full Windows verification and cleanup

This order is important because setup, chat UX, and settings all depend on the runtime/provider layer being correct first.

---

## Phase 1 - Runtime / Provider Layer

### Objective

Replace the Windows multi app's hard Ollama dependency with the provider-aware direct-engine architecture used by Mac multi.

### Source Files To Port From Mac Multi

Frontend:
- `local-ai/src/lib/providerConfig.ts`
- `local-ai/src/services/setup.ts`
- `local-ai/src/hooks/useSetupActions.ts`
- `local-ai/src/types/settings.ts`
- `local-ai/src/stores/modelStore.ts`
- `local-ai/src/components/settings/SettingsView.tsx`
- `local-ai/src/components/setup/SetupStatusPanel.tsx`
- `local-ai/src/components/onboarding/OllamaStep.tsx`
- `local-ai/src/components/onboarding/ModelStep.tsx`
- `local-ai/src/lib/setupStatus.ts`

Backend:
- `local-ai/src-tauri/src/services/llama_cpp.rs`
- `local-ai/src-tauri/src/services/provider.rs`
- `local-ai/src-tauri/src/commands/chat.rs`
- `local-ai/src-tauri/src/commands/setup.rs`
- `local-ai/src-tauri/src/commands/mod.rs`
- `local-ai/src-tauri/src/lib.rs`
- `local-ai/src-tauri/src/types/mod.rs`
- `local-ai/src-tauri/src/services/mod.rs`

Phase 1 owns the provider/runtime wiring required to compile, boot, and complete a real direct-engine chat round-trip.

If a setup/onboarding file is touched during Phase 1, keep that work limited to provider/runtime scaffolding only.
Phase 2 still owns the full user-facing setup/onboarding parity pass.

### Windows-Specific Adaptation Requirements

Do not port the Mac runtime literally.

Required Windows adaptations:
- use `llama-server.exe` path detection that matches the Windows machine
- use Windows-safe process start/stop behavior instead of `pkill`
- prefer tracked PID shutdown or `taskkill`-based fallback
- use Windows file paths for GGUF and optional `mmproj`
- ensure the engine health check is against `http://127.0.0.1:8080/v1/models`
- keep startup polling instead of relying on blind delay

### Output Of Phase 1

After Phase 1:
- Windows multi can resolve a provider-aware model layer
- Tauri state no longer points only to `OllamaService`
- chat commands route through the provider layer instead of directly through `OllamaService`
- setup commands exist for engine startup and model switching
- settings can hold direct-engine executable and model paths

### Acceptance Criteria

- `cargo check` passes
- `npm run build` passes
- Windows multi can detect the direct engine
- Windows multi can list models from the direct engine
- Windows multi can send a successful chat request through the direct engine

---

## Phase 2 - Setup and Onboarding Parity

### Objective

Bring the setup and onboarding product flow up to Mac multi parity so Windows users are guided through the correct engine path.

### Key Areas To Port

- `local-ai/src/components/setup/SetupStatusPanel.tsx`
- `local-ai/src/hooks/useSetupActions.ts`
- `local-ai/src/services/setup.ts`
- `local-ai/src/lib/setupStatus.ts`
- `local-ai/src/components/onboarding/OllamaStep.tsx`
- `local-ai/src/components/onboarding/ModelStep.tsx`
- `local-ai/src/components/onboarding/CompleteStep.tsx`
- `local-ai/src/components/onboarding/WelcomeStep.tsx`

### Required Behavior

Windows multi should:
- stop presenting Ollama as the primary requirement when direct engine is intended
- show provider-aware next actions
- expose setup buttons that actually advance the user
- show engine/model/workspace readiness clearly
- avoid dead-end status banners where possible

### Windows Notes

- naming can stay temporarily compatible during migration if command names still reference Ollama internally
- displayed product behavior should point users toward the direct engine
- if Windows autostart behavior differs from Mac, document and implement the Windows-safe equivalent rather than copying Mac assumptions

### Acceptance Criteria

- Onboarding directs the user into the direct-engine flow, not the old Ollama-first flow
- Setup panel shows actionable next steps
- A user can complete setup without guessing what to do next

---

## Phase 3 - Model and Settings Parity

### Objective

Align model selection, model discovery, and settings behavior with the Mac multi product.

### Key Areas To Port

- `local-ai/src/types/settings.ts`
- `local-ai/src/stores/settingsStore.ts`
- `local-ai/src/stores/modelStore.ts`
- `local-ai/src/services/ollama.ts`
- `local-ai/src/components/models/ModelSelector.tsx`
- `local-ai/src/components/models/ModelDownloader.tsx`
- `local-ai/src/components/models/ModelDownloadProgressCard.tsx`
- `local-ai/src/components/models/ModelCard.tsx`
- `local-ai/src/components/models/ModelList.tsx`
- `local-ai/src/components/models/ModelInfo.tsx`
- `local-ai/src/components/settings/SettingsView.tsx`

### Behavior To Match

- direct-engine settings fields exist and persist correctly
- preferred model selection logic behaves like Mac multi
- curated model behavior is consistent
- model selection is stable across refresh/restart
- setup and settings show the same truth about model availability
- model download/progress UI behavior is consistent where supported

### Important Porting Rule

Do not port Mac model-management wording only.
Port the state logic and resolution rules together with the UI.

### Acceptance Criteria

- settings persist the correct engine/model values
- current model remains stable after restart
- selector and setup state agree on which model is active
- model information is accurate in the UI

---

## Phase 4 - Chat Metrics and Feedback Parity

### Objective

Bring the richer assistant-response instrumentation and feedback capture from Mac multi into Windows multi.

### Files To Port

Frontend:
- `local-ai/src/services/ollama.ts` (extend response payload typing after Phase 3 adds download-progress support)
- `local-ai/src/components/chat/MessageMetricsRow.tsx`
- `local-ai/src/components/chat/ThinkingPulse.tsx`
- `local-ai/src/components/chat/MessageBubble.tsx`
- `local-ai/src/components/chat/StreamingBubble.tsx`
- `local-ai/src/components/chat/TypingIndicator.tsx`
- `local-ai/src/components/chat/MessageList.tsx`
- `local-ai/src/stores/chatStore.ts`
- `local-ai/src/types/index.ts`
- `local-ai/src/types/database.ts`

Backend:
- `local-ai/src-tauri/src/migrations/006_message_feedback_note.sql`
- `local-ai/src-tauri/src/services/database.rs`
- `local-ai/src-tauri/src/services/message_repo.rs`
- `local-ai/src-tauri/src/commands/history.rs`
- `local-ai/src-tauri/src/types/mod.rs`
- `local-ai/src-tauri/src/commands/chat.rs` (augment metrics/finish-reason fields on top of the Phase 1 provider routing)

### Functional Differences To Close

Mac multi adds:
- response metrics row
- context usage display
- finish-reason support
- optional prompt/completion metrics support
- `feedback_note` persistence for negative feedback
- richer assistant message behavior

### Windows Port Notes

- The direct-engine backend should expose at least the metrics fields the Mac UI depends on
- If exact parity fields are not immediately available, port the UI with the supported subset first, then fill missing fields
- Database migration order must remain clean and forward-only

### Acceptance Criteria

- assistant messages show metrics when enabled
- negative feedback can store and reload a note
- history reload preserves feedback note and feedback state
- streaming/thinking visuals behave correctly

---

## Phase 5 - Visual and Branding Parity

### Objective

Bring the Mac multi design polish into Windows multi after the behavior is already stable.

### Areas To Port

- `local-ai/src/index.css`
- `local-ai/src/assets/brand/modernclaw-icon.png`
- `local-ai/src/assets/brand/modernclaw-wordmark.png`
- `local-ai/src/components/layout/Sidebar.tsx`
- `local-ai/src/components/layout/Header.tsx`
- `local-ai/src/components/layout/BrainSelector.tsx`
- any associated onboarding and model-card presentation improvements

### Acceptance Criteria

- Windows multi visually matches the Mac multi line closely
- layout remains stable on Windows desktop sizes
- no regression in readability or interaction flow

---

## Phase 6 - Full Verification and Stabilization

### Objective

Prove that Windows multi behaves like Mac multi in the critical product flows.

### Verification Checklist

#### Build / Static Verification

- `npm run build`
- `cargo check`

#### Runtime Verification

1. App starts successfully on Windows.
2. Setup recognizes the direct engine.
3. A supported model is visible.
4. A brain can send and receive a chat reply.
5. Multi-brain switching still works.
6. Daily log and memory context still load correctly.
7. Response metrics render correctly.
8. Feedback note save/load works.
9. Voice settings still save and load correctly.
10. Restarting the app preserves the correct setup state.

#### Product Verification

- onboarding is understandable
- setup does not dead-end
- the active model shown in settings/setup/chat is consistent
- old Ollama-first assumptions are removed from the critical user path

---

## Recommended Checkpoint Commits

Use checkpoint commits at these points:

1. `multi: add provider-aware direct engine runtime`
2. `multi: port setup and onboarding flow`
3. `multi: port direct engine model and settings behavior`
4. `multi: add response metrics and feedback notes`
5. `multi: port mac multi design polish`
6. `multi: finalize windows parity verification`

---

## Main Risks and Mitigations

### Risk 1 - Multi-brain regressions during engine swap

Mitigation:
- keep agent and workspace logic untouched unless a provider change requires it
- verify brain switching after each milestone

### Risk 2 - Windows process control instability

Mitigation:
- implement Windows-native engine lifecycle management
- verify restart and model switching with real live tests

### Risk 3 - UI says one thing while backend does another

Mitigation:
- port provider-aware setup, settings, and model state together
- avoid partial UI-only ports

### Risk 4 - Mac code contains platform assumptions

Mitigation:
- use Mac behavior as the spec, not literal code as the spec
- adapt every process/path assumption for Windows explicitly

---

## Immediate Next Step

Start Phase 1 in `ModernClawMultifinal`:
- port provider/runtime files
- wire the direct-engine settings layer
- get the Windows multi app talking to the direct engine before touching broader UI polish

## Bottom Line

To make Windows multi functionally match Mac multi, the port should be treated as a structured product migration, not a loose diff chase.

The correct order is:
1. runtime/provider layer
2. setup/onboarding flow
3. model/settings behavior
4. chat metrics + feedback persistence
5. visual polish
6. full Windows verification
