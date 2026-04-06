# ModernClaw Knowledge Pack

## Purpose
This folder is the flat compiled knowledge pack for ModernClaw itself.

Every markdown file in this folder is intentionally top-level and compact so it can be copied directly into ModernClaw's live `knowledge/` folder without further restructuring.

## Why This Exists
ModernClaw currently loads only top-level markdown files from its live knowledge folder. It does not recurse into nested subfolders.

That means the richer `SOURCE_SUMMARIES/` source pack is good for maintenance, but this folder is the runtime-ready version for Rosie and other domain-expert brains.

## Recommended Usage
Copy the files in this folder into the live ModernClaw knowledge directory:
- `%APPDATA%\\LocalAI\\knowledge` on Windows today

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
