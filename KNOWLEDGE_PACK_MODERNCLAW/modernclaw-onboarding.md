# ModernClaw Onboarding

## Purpose
Onboarding gets a new installation into a usable state without forcing the user to guess the architecture.

## Where To Find It
The onboarding flow appears automatically until the onboarding-complete flag is set.

## Steps
- Welcome
- Ollama
- Model
- Memory
- Complete

## Step Details
### Welcome
Introduces ModernClaw as a private, local-first workspace with editable memory and direct model control.

### Ollama
Checks whether the local Ollama service is available on `localhost:11434`. This step confirms the model provider is reachable before the rest of setup continues.

### Model
Pushes the user toward a proven baseline setup. The current opinionated recommendation is a small floor model first, with `dolphin3:8b` offered as an optional stronger fallback.

### Memory
Initializes the markdown workspace and explains the memory path. This step creates the initial brain-file environment.

### Complete
Marks onboarding finished and sends the user into the full app shell.

## What Onboarding Creates
At minimum, the memory service ensures these files exist:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

It also creates key folders for:
- memory logs
- knowledge files
- curator staging
- voice tools

## Why This Matters
Onboarding is not just a cosmetic tour. It is the first-time system bootstrap for the brain workspace.
