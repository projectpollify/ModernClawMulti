# Settings Summary

## Purpose
Settings control the main user-configurable behavior of ModernClaw.

This includes:
- appearance
- model defaults
- context window size
- chat behavior
- voice setup
- privacy/history
- storage path visibility
- reset and onboarding restart actions

## Where To Find It In The App
Open `Settings` from the sidebar or the header gear button.

## Main Component
- `src/components/settings/SettingsView.tsx`

## Store and Backend
- `src/stores/settingsStore.ts`
- `src/services/settings.ts`
- Rust settings commands
- SQLite `settings` table

## Settings Categories
### Appearance
- theme: system, light, dark

### Model
- default model
- context window size

### Behavior
- stream responses
- send on enter
- show token count

### Voice Output
- enable output
- approved voice preset
- Piper executable path
- Piper model path
- output status and test controls

### Voice Input
- enable input
- Whisper executable path
- Whisper model path
- whisper language
- input status controls

### Privacy
- save conversation history

### Storage
- memory folder display
- open folder shortcut

### Recovery Actions
- restart onboarding
- reset all settings

## How It Works
Settings are loaded at app startup.
The settings store:
1. initializes the memory service
2. fetches persisted settings
3. resolves the real memory path
4. auto-fills default voice paths when specific paths are missing

## Important Behavior
### Save Conversation History
If turned off:
- conversations are not kept in SQLite
- the app clears stored conversation state from the UI layer

### Reset All Settings
Reset returns settings to defaults and re-resolves the voice paths based on the current memory path.

### Restart Onboarding
Lets the user re-run the first-run setup flow without reinstalling the app.

## User Instructions
### Change the default model
1. Open `Settings`.
2. Change the `Default Model` dropdown.

### Change theme
1. Open `Settings`.
2. Select system, light, or dark.

### Check the memory folder
1. Open `Settings`.
2. Find the `Storage` section.
3. Click `Open`.

## Important Notes
- Settings are part of the main app identity and should stay understandable to non-technical users.
- The voice sections are currently the most setup-heavy parts of the app.
