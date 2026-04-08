# Voice System Summary

## Purpose
The voice system gives ModernClawMulti a local speech loop.

It currently supports:
- speech-to-text with Whisper
- text-to-speech with Piper
- in-chat transcription into the composer
- read-aloud for assistant messages
- playback pause/resume/stop behavior
- brain-specific voice choice on top of one shared machine-level install
- speech normalization before Piper playback

## Where To Find It In The App
### Chat
- mic button in the composer when voice input is enabled
- `Read Aloud` on assistant messages when voice output is enabled

### Settings
Voice setup lives in `Settings` under:
- Voice Output
- Voice Input

## Current Voice Truth
### Output
- machine-level Piper executable path
- brain-specific approved voice choice
- current approved voices: `Amy (Female)` and `Joe (Male)`
- speech-friendly text normalization before playback

### Input
- machine-level Whisper executable path
- active-brain Whisper model and language preferences when needed

## How Voice Output Works
1. User clicks `Read Aloud` on an assistant message.
2. The frontend normalizes the message into speech-friendly text.
3. Rust launches Piper and generates a WAV file.
4. The frontend receives audio bytes.
5. The app plays the audio locally.

## Important Notes
- voice dependency delivery is still manual
- the app can auto-create folders and default paths, but it does not yet bundle or download Piper or Whisper automatically
- multiple brains can keep different voice choices while sharing one machine-level Piper and Whisper install
