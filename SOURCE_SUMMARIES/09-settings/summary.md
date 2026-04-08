# Settings Summary

## Purpose
Settings controls the main user-configurable behavior of ModernClawMulti.

This now includes both:
- global machine/app settings
- brain-specific settings that belong to the active brain

## Where To Find It In The App
Open `Settings` from the sidebar or the header gear button.

## Main Component
- `src/components/settings/SettingsView.tsx`

## Stores and Backend
- `src/stores/settingsStore.ts`
- `src/stores/agentStore.ts`
- `src/services/settings.ts`
- `src/services/agents.ts`
- Rust settings commands
- SQLite `settings` table
- `agents` table for brain-level model and voice preferences

## Settings Categories
### Global / Machine-Level
- theme
- context window size
- stream responses
- send on enter
- show token count
- save conversation history
- memory folder display
- Piper executable path
- Whisper executable path
- onboarding restart and reset actions

### Brain-Level
- preferred model for the active brain
- enable voice output
- approved voice selection
- Piper voice model path
- enable voice input
- Whisper model path
- Whisper language

## How It Works
Settings are loaded at app startup and reloaded when the active brain changes.

The system now combines:
1. global app settings from the `settings` table
2. active brain settings from the `agents` table
3. default shared voice paths derived from the LocalAI tools folder

That means the user sees one Settings view, but some values are shared machine-wide and others belong only to the active brain.

## Important Behavior
### Save Conversation History
If turned off:
- conversations are not kept in SQLite
- the app clears stored conversation state from the UI layer

### Model Ownership
- the app-wide default model is a fallback
- the active brain's saved model takes priority when present

### Voice Ownership
- Piper and Whisper executable paths remain machine-level by default
- voice choice and voice model path can belong to the active brain
- multiple brains can share one Piper/Whisper install while sounding different

### Reset All Settings
Reset returns the global settings to defaults and re-resolves the shared voice paths.
It does not mean the app becomes a single-brain app again.

## User Instructions
### Check the active brain's storage context
1. Open `Settings`.
2. Find the `Storage` section.
3. Confirm the memory folder path reflects the current brain workspace.

### Change the active brain's voice
1. Open `Settings`.
2. Go to `Voice Output`.
3. Choose the approved voice for the active brain.
4. Refresh output status and test voice.

### Change the active brain's model
1. Use the header model selector for fast switching.
2. Use Settings when you want to inspect broader model management.

## Important Notes
- Settings is no longer a purely global config surface
- the current app uses a hybrid model: some settings are machine-level, some are brain-level
- the voice sections remain the most setup-heavy parts of the app because dependency delivery is still manual
