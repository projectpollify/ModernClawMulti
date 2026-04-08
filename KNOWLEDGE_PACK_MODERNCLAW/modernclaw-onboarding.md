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
Introduces ModernClawMulti as a private, local-first workspace with editable memory, direct model control, and a multi-brain system that opens after the initial setup is complete.

### Ollama
Checks whether the local Ollama service is available on `localhost:11434`.

### Model
Guides the user toward the current baseline lane for this repo and product track:
- `gemma4:e4b`

### Memory
Initializes the markdown workspace and explains the memory path. This step creates the initial brain-file environment.

### Complete
Marks onboarding finished and sends the user into the full multi-brain app shell.

## Why This Matters
Onboarding is not just a cosmetic tour. It is the first-time bootstrap for the baseline brain workspace that later multi-brain features build on.
