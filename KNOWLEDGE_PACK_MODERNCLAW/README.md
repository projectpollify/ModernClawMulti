# ModernClaw Knowledge Pack

## Purpose
This folder is the flat compiled knowledge pack for the current `ModernClawMulti` app.

Every markdown file in this folder is intentionally top-level and compact so it can be copied directly into the live `knowledge/` folder used by a brain such as Rosie, a future support brain, or another domain-expert brain.

## Why This Exists
The app still loads only top-level markdown files from the live knowledge folder. It does not recurse into nested subfolders.

That means:
- the richer `SOURCE_SUMMARIES/` pack is the human-maintained source layer
- this folder is the runtime-oriented compiled layer

## Current Truth
These compiled docs now reflect the current multi-brain app rather than the older single-brain blueprint.

They cover:
- active brain selection
- create and delete brain flows
- per-brain conversation and memory isolation
- per-brain model persistence
- hybrid global and brain-specific settings behavior
- shared machine-level Piper and Whisper tools with brain-specific voice choice
- current Gemma-first product direction for this repo

## Recommended Usage
Copy the files in this folder into the live knowledge directory used by the target brain:
- `%APPDATA%\LocalAI\knowledge` on Windows today

Do not nest these files in subfolders if you want the current app to load them.

## Files
- `modernclaw-overview.md`
- `modernclaw-navigation.md`
- `modernclaw-onboarding.md`
- `modernclaw-models.md`
- `modernclaw-chat.md`
- `modernclaw-memory.md`
- `modernclaw-brain.md`
- `modernclaw-curator.md`
- `modernclaw-voice.md`
- `modernclaw-settings.md`
- `modernclaw-context-and-persistence.md`
