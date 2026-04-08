# EXPRESSIVE_SPEECH_SHAPING_PLAN

## Purpose

This document captures the next voice-polish direction for ModernClawMulti after the successful introduction of `prepareTextForSpeech()`.

The current system already improves voice playback by cleaning raw assistant text before it is sent to Piper. The next step is not new TTS training. The next step is adding a light expressive speech-shaping layer so different brains can sound more natural and more distinct while still using the same local Piper runtime.

## Current State

What is already real in ModernClawMulti:
- raw assistant text is normalized before Piper playback
- markdown emphasis markers are stripped
- list bullets are converted into more natural spoken pauses
- code fences and inline formatting are softened or omitted
- links and decorative symbols are reduced so Piper does not read them literally
- Rosie and Mia already use different validated Piper voices

Current pipeline:

```text
assistant message
-> prepareTextForSpeech(rawMessage)
-> Piper
-> local audio playback
```

This is already a major improvement over sending raw markdown directly to TTS.

## Product Insight

The biggest wins in voice quality are currently coming from text preparation, not from retraining the TTS model.

That means ModernClaw should treat spoken output as a separate representation of the message:
- chat text remains rich and expressive for reading
- speech text becomes cleaner and more prosody-friendly for listening

This allows the app to preserve the full assistant response in the UI while still making read-aloud feel more natural.

## Future Direction

The next voice-polish layer should be an expressive speech-shaping system built on top of the current normalization pipeline.

Instead of only:

```ts
prepareTextForSpeech(rawMessage)
```

we can later evolve to:

```ts
prepareTextForSpeech(rawMessage, style)
```

Where `style` might be:
- `neutral`
- `warm`
- `playful`
- `direct`

This does not need to change the underlying Piper engine.
It changes how text is prepared before it is spoken.

## Why This Is The Right Next Layer

Benefits:
- lower complexity than training or replacing the TTS engine
- controllable from app logic
- brain-specific without needing separate TTS installs
- safer than trying to inject emotion directly into model weights
- works with the current local-first architecture

This is the right approach because the current product already supports:
- multiple brains
- brain-specific identity
- brain-specific voice choice
- local speech playback

Expressive speech shaping would become the next logical differentiation layer.

## Proposed Architecture

### Current Function

```ts
prepareTextForSpeech(rawMessage)
```

### Future Function

```ts
prepareTextForSpeech(rawMessage, style)
```

### Suggested Signature

```ts
type SpeechStyle = 'neutral' | 'warm' | 'playful' | 'direct';

function prepareTextForSpeech(rawMessage: string, style: SpeechStyle = 'neutral'): string
```

### Suggested Ownership

The style should be associated with the active brain, not the global app.

That means eventually each brain could have:
- a Piper voice choice
- a speech style

Example:
- Rosie
  - voice: `Amy (Female)`
  - style: `warm`
- Mia
  - voice: `Joe (Male)`
  - style: `playful`

## Style Behavior Ideas

### Neutral
Goal:
- plain
- clean
- informative
- least transformed

Behavior:
- remove formatting noise
- preserve sentence structure
- moderate pause insertion
- avoid extra flourish

Best for:
- support brains
- product explainer brains
- documentation brains

### Warm
Goal:
- softer
- friendlier
- calmer

Behavior:
- shorten harsh or abrupt sentence transitions
- favor softer sentence endings
- slightly longer pauses after greetings and reassurance
- avoid reading highly technical clutter when it is not useful in speech

Best for:
- support brain
- assistant-style personalities
- coaching brains

### Playful
Goal:
- more animated
- more personality
- lighter cadence

Behavior:
- turn stage-direction style text into cleaner spoken energy rather than literal symbols
- compress awkward decorative wording
- favor shorter, lively sentence pacing
- reduce stiff list formatting

Best for:
- mascot-style agents
- pet or companion brains
- more casual personalities

### Direct
Goal:
- efficient
- clipped
- high-signal

Behavior:
- reduce filler
- compress transitions
- shorten pauses
- prioritize content over warmth

Best for:
- operations brains
- technical brains
- task-oriented agents

## Suggested Transformation Rules

These are examples of what can eventually vary by style.

### Sentence Length
- `neutral`: leave mostly intact
- `warm`: break long sentences into shorter spoken phrases
- `playful`: prefer short energetic sentences
- `direct`: aggressively compress long phrasing

### List Handling
- `neutral`: convert bullets to mild pauses
- `warm`: convert bullets to softer spoken transitions
- `playful`: keep rhythm lively and less formal
- `direct`: flatten into concise sequential statements

### Punctuation Shaping
- `neutral`: standard cleanup
- `warm`: allow more pauses and smoother transitions
- `playful`: tolerate slightly more dramatic short pacing
- `direct`: minimize pause-heavy flourish

### Decorative Or Emotional Markers
- `neutral`: strip them
- `warm`: convert some into natural spoken tone hints when safe
- `playful`: keep the spirit, but not the literal symbols
- `direct`: strip nearly all of them

### Code And Links
- `neutral`: omit or simplify
- `warm`: omit and keep flow friendly
- `playful`: omit with a softer filler phrase
- `direct`: omit immediately

## Example Transformations

### Raw Input
```text
*Wags tail excitedly* Hi Shawn! Here are the steps:
- Open Settings
- Click Refresh Output Status
- Test Voice
```

### Neutral Output
```text
Hi Shawn. Here are the steps. Open Settings. Click Refresh Output Status. Test Voice.
```

### Warm Output
```text
Hi Shawn. Here are the steps. First, open Settings. Then click Refresh Output Status. After that, test the voice.
```

### Playful Output
```text
Hi Shawn. Okay, here we go. Open Settings. Click Refresh Output Status. Then test the voice.
```

### Direct Output
```text
Open Settings. Click Refresh Output Status. Test Voice.
```

## Proposed Config Model

This should eventually be brain-level state, not app-global.

### Candidate Agent Fields

Possible future `agents` fields:
- `speech_style`
- `speech_cleanup_level`
- `speech_code_handling`
- `speech_link_handling`

### Minimum Useful Version

A minimal first implementation only needs:
- `speech_style`

Example values:
- `neutral`
- `warm`
- `playful`
- `direct`

### Optional Future Add-Ons

Possible later controls:
- `omit_code_blocks_in_speech: boolean`
- `omit_links_in_speech: boolean`
- `compress_long_messages_for_speech: boolean`
- `speech_max_sentence_length: number`
- `speech_pause_intensity: 'low' | 'medium' | 'high'`

These should not be built until the simple style system proves useful.

## Suggested Implementation Phases

### Phase A: Keep The Current Win
- preserve current `prepareTextForSpeech(rawMessage)` behavior
- do not widen scope yet

### Phase B: Add Style Parameter
- add `style` argument
- implement four small rule branches
- keep the transforms deterministic and easy to reason about

### Phase C: Store Style Per Brain
- save `speech_style` on the brain record
- use the active brain style automatically during `Read Aloud`

### Phase D: Add Optional UI
- show a `Speech Style` selector in Settings for the active brain
- choices: `Neutral`, `Warm`, `Playful`, `Direct`

### Phase E: Refine With Real Use
- test Rosie, Mia, and future support brains
- listen for over-processing
- keep the system light and predictable

## What Not To Do Yet

Do not:
- try to train a new emotional TTS model yet
- add complicated SSML-like controls unless the runtime clearly benefits
- overfit speech rules before more live use
- let spoken output drift too far from the actual message meaning

The speech layer should improve listening, not rewrite truth.

## Success Criteria

This future polish track is successful when:
- read-aloud sounds more natural than raw markdown playback
- different brains feel distinct without separate TTS installations
- the text still stays faithful to the original meaning
- the feature is brain-specific but simple enough for noobs
- the app can support expressive speech without becoming fragile

## Recommendation

Do not build this immediately.

Current recommendation:
- keep the current normalization pipeline
- log expressive speech shaping as a future polish track
- revisit it after more Phase 4 validation and after core product truths are more stable

When ready, the first real step should be:
- add `prepareTextForSpeech(rawMessage, style)`
- implement `neutral`, `warm`, `playful`, and `direct`
- test them on Rosie, Mia, and a future support brain
