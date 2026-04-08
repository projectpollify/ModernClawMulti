# Onboarding Summary

## Purpose
Onboarding prepares a first-time user to run ModernClawMulti successfully.

It is meant to confirm:
- Ollama is available
- at least one usable local model is installed
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

## What Each Step Does
### Welcome
Explains the product at a high level:
- private by default
- local-first
- brain files and model control
- multi-brain system comes after onboarding, not before it

### Ollama Check
Checks whether Ollama is reachable on localhost.
If not, it instructs the user to install or start Ollama.

### Model Step
Guides the user toward the current baseline lane for this repo and product track.
Current practical guidance:
- baseline lane: `gemma4:e4b`

### Memory Step
Initializes the local brain files and shows the storage path.
It verifies the presence of:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

### Complete Step
Marks onboarding as finished and hands the user into the full multi-brain app shell.

## Important Notes
- onboarding still bootstraps the baseline workspace first
- multi-brain flows begin after onboarding is complete
- users can restart onboarding later from Settings
