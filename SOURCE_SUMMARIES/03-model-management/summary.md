# Model Management Summary

## Purpose
Model management controls which local Ollama model ModernClaw uses and helps the user install or remove supported models.

## Where To Find It In The App
### Header
The active model selector lives in the top header bar.

### Settings
The full model management experience is in `Settings` under:
- default model selection
- model download area
- installed model cards

### Onboarding
Initial model setup also appears during onboarding.

## Current Product Strategy
ModernClaw is not trying to support every possible model equally.
It currently prefers:
- floor model: `nchapman/dolphin3.0-qwen2.5:3b`
- fallback model: `dolphin3:8b`
- heavier models can still be installed, but are not the main guided path

## How It Works
### Frontend components
- `src/components/models/ModelSelector.tsx`
- `src/components/models/ModelDownloader.tsx`
- `src/components/models/ModelCard.tsx`
- `src/components/models/ModelList.tsx`
- `src/components/models/ModelInfo.tsx`

### Store
- `src/stores/modelStore.ts`

### Service
- `src/services/ollama.ts`

### Backend commands
- `check_ollama_status`
- `list_models`
- `pull_model`
- `delete_model`
- `chat_send`

## User-Facing Behavior
### Header model selector
- shows `Ollama Offline` if Ollama is not running
- otherwise opens a dropdown of installed models
- selecting a model changes the active chat model

### Settings model section
Allows the user to:
- choose the default model
- refresh the model list
- download the floor or fallback model
- optionally download a custom model by name
- view installed models
- delete installed models from model cards

## Persistence
The model store persists the currently selected model in local storage.
Settings also persist a default model value.

## User Instructions
### Refresh model list
1. Open `Settings`.
2. Click `Refresh Models`.

### Change active model
1. Use the header dropdown.
2. Choose an installed model.

### Set the default model
1. Open `Settings`.
2. Go to the `Model` section.
3. Change `Default Model`.

### Download a supported model
1. Open `Settings`.
2. In `Model Management`, click the desired curated model.
3. Wait for the download to finish.

## Important Notes
- Model management depends entirely on Ollama being available.
- If Ollama storage is broken or a model is missing, the app may show no selectable models.
- The app is intentionally tuned around smaller, responsive local models rather than the biggest possible model.
