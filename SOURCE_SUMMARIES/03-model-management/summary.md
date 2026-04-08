# Model Management Summary

## Purpose
Model management controls which local Ollama model each brain uses and helps the user install, remove, and switch supported models.

## Where To Find It In The App
### Header
The active model selector lives in the top header bar.

### Settings
The fuller model management experience is in `Settings` under:
- app-wide fallback model selection
- model download area
- installed model cards

### Onboarding
Initial model guidance still appears during onboarding.

## Current Product Strategy
ModernClawMulti is no longer best described by the older 3B-first lane.
For this repo and product track, the cleaner current truth is:
- baseline lane: `gemma4:e4b`
- heavier or alternate models can still be installed if wanted
- the app-wide default model is now a fallback, not the most important source of truth
- each brain can persist its own preferred model

## How It Works
### Frontend components
- `src/components/models/ModelSelector.tsx`
- `src/components/models/ModelDownloader.tsx`
- `src/components/models/ModelCard.tsx`
- `src/components/models/ModelList.tsx`
- `src/components/models/ModelInfo.tsx`

### Stores
- `src/stores/modelStore.ts`
- `src/stores/agentStore.ts`

### Service
- `src/services/ollama.ts`
- `src/services/agents.ts`

### Backend commands
- `check_ollama_status`
- `list_models`
- `pull_model`
- `delete_model`
- `chat_send`
- `agent_update_default_model`

## User-Facing Behavior
### Header model selector
- shows installed models returned by Ollama
- changes the active model for the current brain
- brain switching restores that brain's saved model when present

### Settings model section
Allows the user to:
- choose the app-wide fallback model
- refresh the model list
- download curated or custom models
- view installed models
- delete removable models from model cards

## Persistence
There are now two layers:
- `modelStore` keeps the current active selection in the frontend
- `agentStore` persists a preferred model to the active brain record

That means model ownership is no longer purely global.

## User Instructions
### Refresh model list
1. Open `Settings`.
2. Click `Refresh Models`.

### Change the active model for the current brain
1. Use the header model selector.
2. Choose an installed model.

### Set the app-wide fallback model
1. Open `Settings`.
2. Go to the `Model` section.
3. Change `Default Model`.

## Important Notes
- model management still depends entirely on Ollama being available
- if Ollama storage is broken or a model is missing, the app may show no selectable models
- the current product truth in this repo is Gemma-first, not the older 3B-first ladder
