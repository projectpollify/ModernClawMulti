# ModernClaw Voice System

## Purpose
The voice system adds fully local speech input and output to the workspace.

## Where To Find It
- chat composer for voice input
- assistant message controls for voice output
- Settings view for voice configuration and testing

## Current Voice Truth
- Whisper is used for speech-to-text
- Piper is used for text-to-speech
- Piper and Whisper executables are shared machine-level tools
- brains can keep different voice preferences on top of that shared install
- current approved Piper voices are `Amy (Female)` and `Joe (Male)`
- assistant text is normalized for speech before Piper playback

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

## Important Current Limitation
Dependency delivery is still manual. Piper, Whisper, and their models still need to be installed or placed into the expected folders.

## Why This Matters
A local voice loop makes the assistant feel much more alive while preserving the product's owned-intelligence and offline direction. The new speech-normalization layer also makes read-aloud much more natural than raw markdown playback.
