# ModernClawMulti Mac vs Windows Diff Summary

Compared repos:
- Mac reference: `C:\Users\Admin\Desktop\ModernClawMacMulti`
- Windows target: `C:\Users\Admin\Desktop\ModernClawMultifinal`

## Executive Summary

The Mac multi repo is not just a skin update. It is the newer multi-brain product line with a provider-aware runtime layer, a direct-engine path on macOS, stronger setup/status UX, richer chat response instrumentation, and a small persistence/schema upgrade for feedback notes. The Windows multi repo is the older Ollama-first version.

The biggest gap is runtime architecture:
- Windows multi is hard-wired to Ollama in frontend services, onboarding, setup UI, Tauri state, and Rust backend services.
- Mac multi introduces a provider-aware layer that can use `llama.cpp` on macOS while still keeping Ollama code paths for non-mac targets.

The second biggest gap is product polish:
- Mac multi has a more mature setup flow, richer chat metrics and feedback capture, cleaner model handling, and branding/theme polish.

## Scope Snapshot

High-level diff size from direct file comparison:
- frontend `src`: 47 changed files, about 1,897 insertions and 274 deletions
- backend `src-tauri/src`: 17 changed files, about 1,198 insertions and 110 deletions

Files present in Mac multi but missing in Windows multi:
- `src/lib/providerConfig.ts`
- `src/services/setup.ts`
- `src/hooks/useSetupActions.ts`
- `src/components/chat/MessageMetricsRow.tsx`
- `src/components/chat/ThinkingPulse.tsx`
- `src/components/models/ModelDownloadProgressCard.tsx`
- `src/assets/brand/modernclaw-icon.png`
- `src/assets/brand/modernclaw-wordmark.png`
- `src-tauri/src/services/llama_cpp.rs`
- `src-tauri/src/services/provider.rs`
- `src-tauri/src/commands/setup.rs`
- `src-tauri/src/migrations/006_message_feedback_note.sql`
- `src-tauri/tauri.windows.conf.json`

Windows-only file in `local-ai` that is not present in Mac multi:
- `.github/workflows/release.yml`

## Main Differences

### 1. Runtime / Provider Layer

Windows multi:
- Uses `OllamaService` directly in Tauri app state.
- Frontend talks to `ollamaApi` only.
- Onboarding and setup copy assume Ollama is the model runtime.

Mac multi:
- Adds `providerConfig.ts` to define provider-aware labels, URLs, and preferred-model resolution.
- Adds `ProviderService` in Rust, with `LlamaCppService` on macOS and `OllamaService` elsewhere.
- Adds `setup.rs` direct-engine commands for engine startup and model switching.
- Adds direct-engine settings fields for executable path and GGUF model path.

Practical implication:
- Porting this is not a one-file swap. The engine change spans frontend labels, setup actions, model store behavior, Rust state wiring, and startup commands.

### 2. Setup / Onboarding Flow

Windows multi:
- `OllamaStep.tsx` is explicitly Ollama-branded.
- `SetupStatusPanel.tsx` is mostly a passive checklist with refresh/open-folder actions.
- There is no dedicated frontend setup action layer for provider-specific behavior.

Mac multi:
- Keeps the `OllamaStep` file name, but behavior is provider-aware.
- Adds `useSetupActions.ts` and `services/setup.ts` so setup can open docs, start the provider, confirm models, and initialize the workspace.
- `SetupStatusPanel.tsx` is much more action-oriented, with next-step buttons and provider-specific guidance.
- Model/setup language is aware of direct engine vs Ollama.

Practical implication:
- The Mac setup flow is materially more complete and should be treated as a product-flow port, not just wording cleanup.

### 3. Settings + Model Management

Windows multi:
- `AppSettings` has no direct-engine path fields.
- `modelStore.ts` only supports simple Ollama status/load/download/delete behavior.
- `ModelDownloader.tsx` is a simpler Gemma-first downloader with a generic pulse bar.

Mac multi:
- `AppSettings` adds `directEngineExecutablePath`, `directEngineModelPath`, and `showResponseMetrics`.
- `modelStore.ts` adds preferred-model resolution and structured download progress tracking.
- `ModelDownloadProgressCard.tsx` adds explicit progress UI.
- `SettingsView.tsx` includes provider-aware model controls and settings affordances.

Practical implication:
- Model and settings behavior should be ported together with the runtime layer. Otherwise the UI will still behave like Ollama even if the backend is changed.

### 4. Chat UX / Feedback / Metrics

Windows multi:
- Assistant messages support thumbs up/down only.
- No metrics row component.
- No thinking pulse component.
- Backend message schema stores `feedback` but not a note.
- Rust chat response type is missing `prompt_eval_count` and `finish_reason`.

Mac multi:
- Adds `MessageMetricsRow.tsx` and `ThinkingPulse.tsx`.
- `MessageBubble.tsx`, `StreamingBubble.tsx`, and `TypingIndicator.tsx` are upgraded.
- Assistant feedback supports a written `feedback_note`.
- Migration `006_message_feedback_note.sql` adds the schema column.
- Rust types add `prompt_eval_count`, `finish_reason`, and `feedback_note`.
- `message_repo.rs` reads/writes `feedback_note` and updates feedback with note text.

Practical implication:
- This is a clean, portable second phase after engine migration. It improves product quality without changing the multi-brain architecture.

### 5. Branding / Theme / Layout Polish

Windows multi:
- Older visual treatment.
- No dedicated brand assets in `src/assets/brand`.
- Sidebar and brain selector are older variants.

Mac multi:
- Adds brand icon and wordmark assets.
- `index.css` is substantially upgraded.
- `Sidebar.tsx`, `BrainSelector.tsx`, `Header.tsx`, and onboarding steps are more polished.

Practical implication:
- The Mac repo carries a real design pass, not just runtime work. This can be ported independently after engine/setup parity is reached.

### 6. Backend / App Wiring

Windows multi:
- `src-tauri/src/lib.rs` manages `OllamaService` directly.
- No `setup` command module is registered.
- Agent bootstrapping uses `ensure_default_agent`.

Mac multi:
- `src-tauri/src/lib.rs` manages `ProviderService`.
- Registers setup commands including direct-engine switching.
- Agent bootstrapping uses `ensure_base_profiles`.

Practical implication:
- The Mac repo reflects a more evolved multi-brain backend baseline even outside the engine work.

### 7. Repo Organization

Windows multi repo root:
- older product tree with planning docs and curator content at the root
- app lives in `local-ai`

Mac multi repo root:
- standardized repo layout with `.github`, `docs`, `website`, and `local-ai`

Practical implication:
- This matters for long-term repo hygiene, but it should not be mixed into the direct engine port unless explicitly requested.

## Recommended Port Order

1. Port the runtime/provider layer first.
   - Rust: `llama_cpp.rs`, `provider.rs`, `commands/setup.rs`, `lib.rs`, related type changes.
   - Frontend: `providerConfig.ts`, `services/setup.ts`, `useSetupActions.ts`, settings/model/setup wiring.

2. Port setup and onboarding behavior next.
   - `SetupStatusPanel.tsx`
   - `setupStatus.ts`
   - `OllamaStep.tsx` behavior and copy
   - `ModelStep.tsx`
   - `SettingsView.tsx`

3. Port chat instrumentation and feedback improvements.
   - `MessageMetricsRow.tsx`
   - `ThinkingPulse.tsx`
   - `MessageBubble.tsx`
   - `StreamingBubble.tsx`
   - `TypingIndicator.tsx`
   - migration `006_message_feedback_note.sql`
   - backend/frontend message types and repos

4. Port branding and layout polish last.
   - `index.css`
   - sidebar / header / brain selector
   - brand assets

## What Not To Blindly Copy

- Do not copy the Mac runtime exactly as-is and stop there.
  - It is still conditional by platform and still carries Ollama fallback logic outside macOS.
- Do not mix repo-structure cleanup with engine migration.
- Do not port only the backend engine files without the provider-aware setup/settings/model flow.

## Bottom Line

The Mac multi repo is the newer product baseline.

The most important differences to port into Windows multi are:
1. provider-aware direct-engine architecture
2. setup/onboarding flow improvements
3. model/settings behavior for direct engine
4. chat metrics + feedback-note persistence
5. branding and UI polish

The Windows multi repo should be treated as the older Ollama-first base, not as feature-parity with the Mac multi line.
