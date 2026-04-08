# ModernClaw Voice System

## Purpose
The voice system adds fully local speech input and output to the workspace.

## Where To Find It
- chat composer for voice input
- assistant message controls for voice output
- Settings view for voice configuration and testing

## Voice Output
### Stack
- Piper for text-to-speech

### What It Does
The user can click `Read Aloud` on an assistant message. ModernClaw sends the text to Piper, receives audio data, and plays it locally in the app.

### Current Playback Controls
- Read Aloud
- Pause Reading
- Resume Reading
- Stop reading behavior through playback control state

## Voice Input
### Stack
- Whisper CLI for speech-to-text

### What It Does
The user can record locally from the chat composer. The app converts the recording to WAV, sends it to Whisper, and places the transcript into the text box for review before sending.

## Settings and Defaults
Voice settings include:
- enable voice output
- Piper executable path
- approved voice preset
- Piper voice model path
- enable voice input
- Whisper executable path
- Whisper model path
- transcription language

ModernClawMulti now uses one shared machine-level Piper and Whisper install while allowing each brain to keep its own voice selection.

Current approved Piper voices:
- `Amy (Female)`
- `Joe (Male)`

## Important Current Limitation
ModernClaw currently prepares the folders and expected paths, but dependency delivery is still manual. Piper, Whisper, and their models still need to be installed or placed into those folders.

## Why This Matters
A local voice loop makes the assistant feel much more alive while preserving the product's owned-intelligence and offline direction.
