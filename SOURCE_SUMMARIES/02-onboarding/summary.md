# Onboarding Summary

## Purpose
Onboarding prepares a first-time user to run ModernClaw successfully.

It is meant to confirm:
- Ollama is available
- at least one supported model is installed
- the local memory workspace exists
- the user understands the basic local-first setup

## Where To Find It In The App
On a fresh app state, onboarding appears before the main shell.

It includes five steps:
- welcome
- Ollama check
- model setup
- memory initialization
- completion

## How It Works
### Main component
- `src/components/onboarding/OnboardingFlow.tsx`

### Steps
- `WelcomeStep.tsx`
- `OllamaStep.tsx`
- `ModelStep.tsx`
- `MemoryStep.tsx`
- `CompleteStep.tsx`

### Store
- `src/stores/onboardingStore.ts`

The app decides whether to show onboarding based on whether onboarding has been completed.

## What Each Step Does
### Welcome
Explains the product at a high level:
- private by default
- works offline
- custom memory
- model control

### Ollama Check
Checks whether Ollama is reachable on localhost.
If not, it instructs the user to install/start Ollama.

### Model Step
Guides the user toward the supported floor model first.
Current curated path:
- floor model: `nchapman/dolphin3.0-qwen2.5:3b`
- optional fallback: `dolphin3:8b`

### Memory Step
Initializes the local brain files and shows the storage path.
It verifies the presence of:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

### Complete Step
Marks onboarding as finished and hands the user into the full app.

## User Instructions
### Complete first-time setup
1. Click `Get Started`.
2. Let ModernClaw verify Ollama.
3. Download the recommended model if needed.
4. Confirm the memory files are initialized.
5. Finish onboarding.

## Important Notes
- Onboarding is opinionated toward the floor model strategy.
- It is not meant to expose every possible configuration option.
- Users can restart onboarding later from Settings.
