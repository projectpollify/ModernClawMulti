# ModernClaw Models

## Purpose
Model management lets the user choose, download, remove, and switch local Ollama models inside the app.

## Where To Find It
- Header model selector
- `Settings` view under model controls
- onboarding model step

## Main Supported Pattern
ModernClawMulti is now best described as Gemma-first for this repo and product track.

Current practical truth:
- baseline lane: `gemma4:e4b`
- the app-wide default model acts as a fallback
- the active brain can persist its own preferred model
- alternate or heavier models can still be installed if the user's hardware allows them

## Header Model Selector
The header selector shows installed models returned by Ollama. It allows quick switching of the active model for the current brain without leaving the current view.

## Settings Model Management
The Settings view includes:
- app-wide fallback model selection
- model refresh
- download recommendations
- optional custom model download
- installed model list
- delete support for removable models

## How It Works
The frontend asks Ollama for the installed model list. The active model is tracked in the frontend, but the current brain can also persist its own preferred model. If the stored model disappears, the app can fall back to the first installed model that still exists.

## User Guidance
1. Install or pull a supported model.
2. Refresh models in Settings if needed.
3. Select the active model from the header dropdown or Settings.
4. Understand that the current brain's saved model takes priority over the app-wide fallback when present.

## Important Notes
- the app depends on Ollama being online
- model storage location is managed by Ollama, not by ModernClawMulti itself
- the earlier 3B/8B lane is no longer the clean current story for this repo
