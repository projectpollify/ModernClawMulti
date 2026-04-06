# Voice System Summary

## Purpose
The voice system gives ModernClaw a local speech loop.

It currently supports:
- speech-to-text with Whisper
- text-to-speech with Piper
- in-chat transcription into the composer
- read-aloud for assistant messages
- playback pause/resume/stop behavior

## Where To Find It In The App
### Chat
- mic button in the composer when voice input is enabled
- `Read Aloud` on assistant messages when voice output is enabled

### Settings
Voice setup lives in `Settings` under:
- Voice Output
- Voice Input

## Main Components
- `src/components/chat/MessageInput.tsx`
- `src/components/chat/MessageBubble.tsx`
- `src/components/settings/SettingsView.tsx`

## Store and Services
- `src/stores/voiceStore.ts`
- `src/services/voice.ts`
- `src/lib/audio.ts`
- Rust backend commands in `src-tauri/src/commands/voice.rs`

## Current Voice Stack
### Output
- Piper executable
- approved Piper voices
- generated WAV playback in the frontend audio layer

### Input
- Whisper CLI executable
- Whisper model file
- browser microphone capture
- WAV conversion
- local transcription

## How Voice Output Works
1. User clicks `Read Aloud` on an assistant message.
2. The frontend calls `voice_speak` through Tauri.
3. Rust launches Piper and generates a WAV file.
4. The frontend receives audio bytes.
5. The app plays the audio locally.
6. The user can pause, resume, or stop playback.

## How Voice Input Works
1. User clicks the mic button.
2. Browser microphone capture starts.
3. User clicks again to stop.
4. The audio blob is converted to WAV.
5. The frontend calls Whisper through Tauri.
6. The transcript is returned.
7. The transcript is inserted into the chat composer.
8. The user reviews and sends it manually.

## Voice Settings
### Output
- enable voice output
- approved voice preset
- Piper executable path
- Piper model path
- output status check
- test voice

### Input
- enable voice input
- Whisper executable path
- Whisper model path
- input language
- input status check

## User Instructions
### Enable voice output
1. Open `Settings`.
2. Turn on `Enable Voice Output`.
3. Configure Piper path and voice model path if needed.
4. Refresh output status.
5. Test voice.

### Use read-aloud
1. Open a chat with assistant messages.
2. Click `Read Aloud`.
3. Use `Pause Reading` or `Resume Reading` as needed.

### Enable voice input
1. Open `Settings`.
2. Turn on `Enable Voice Input`.
3. Configure Whisper path and model path if needed.
4. Refresh input status.

### Use microphone transcription
1. Open `Chat`.
2. Click the mic button.
3. Speak.
4. Click again to stop.
5. Review the inserted transcript in the composer.
6. Send manually.

## Important Notes
- Voice dependency delivery is still manual.
- The app can auto-create folders and default paths, but it does not yet bundle/download Piper or Whisper automatically.
- Voice works fully locally when dependencies are present.
