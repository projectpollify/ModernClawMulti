# ModernClaw Models

## Purpose
Model management lets the user choose, download, remove, and switch local Ollama models inside the app.

## Where To Find It
- Header model selector
- `Settings` view under model controls
- onboarding model step

## Main Supported Pattern
ModernClaw is tuned around a fast floor model first.

Current model lanes:
- `nchapman/dolphin3.0-qwen2.5:3b` as the floor setup
- `dolphin3:8b` as the stronger fallback
- optional custom models if the user's hardware allows them

## Current Heavy Experimental Lane
The app can also use larger compatible Ollama models, including `gemma4:e4b`, if the user installs them.

## Header Model Selector
The header selector shows installed models returned by Ollama. It allows quick switching of the active model without leaving the current view.

## Settings Model Management
The Settings view includes:
- default model selection
- model refresh
- opinionated download recommendations
- optional custom model download
- installed model list
- delete support for removable models

## How It Works
The frontend asks Ollama for the installed model list. The selected current model is persisted locally. If the stored model disappears, the app can fall back to the first installed model that still exists.

## User Guidance
1. Install or pull a supported model.
2. Refresh models in Settings if needed.
3. Select the active model from the header dropdown or Settings.
4. Keep the floor model installed even if you experiment with larger models.

## Important Notes
- The app depends on Ollama being online.
- Model storage location is managed by Ollama, not by ModernClaw itself.
- Heavier models can work, but the product is still optimized around a smaller responsive baseline.
