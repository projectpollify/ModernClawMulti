# Brain Builder Summary

## Purpose
The Brain view is ModernClaw’s differentiated workflow layer.

Instead of silently changing memory, it proposes structured improvements to the assistant’s brain and lets the user review, defer, accept, or dismiss them.

## Where To Find It In The App
Open the `Brain` view from the sidebar.

Main sections include:
- guided setup
- question queue
- pending suggestions
- deferred suggestions
- accepted suggestions
- knowledge intake
- recent brain activity
- recent knowledge additions

## Main Components
- `src/components/brain/BrainView.tsx`
- `src/components/brain/SuggestionCard.tsx`
- `src/components/brain/QuestionQueueCard.tsx`

## Store and Persistence
- `src/stores/suggestionStore.ts`

The Brain state persists locally through Zustand `persist` middleware.
This includes:
- suggestion status
- draft answers
- activity log
- recent knowledge records

## Key Features
### Guided Setup
Writes starter sections into:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

The user fills in:
- primary role
- what they want help with
- collaboration style
- current priorities

### Question Queue
A dedicated place for profile questions the brain wants answered.

Example seed question:
- clarify your primary role

### Pending Suggestions
General proposals that do not belong in the question queue.
Examples include:
- memory structure suggestions
- knowledge workflow suggestions
- behavior shaping ideas

### Deferred and Accepted Tracking
Suggestions can be moved through a visible lifecycle:
- pending
- deferred
- accepted
- dismissed

### Brain Activity History
Every meaningful action can create a lightweight log entry.
This makes the brain feel reviewable instead of temporary.

### Knowledge Intake
The Brain can create new structured knowledge files using:
- title
- summary
- tags
- source
- content

These are saved into `knowledge/`.

## How It Works
### Seed suggestions
The suggestion store starts with a default set of seed suggestions.

### Applying profile changes
Some suggestions write directly into specific files.
For example:
- profile answers can update `USER.md`
- project tracking can update `MEMORY.md`

### Knowledge save flow
Brain knowledge intake creates markdown in the `knowledge/` folder and records the event in recent knowledge and activity history.

## User Instructions
### Use guided setup
1. Open `Brain`.
2. Fill the four guided setup fields.
3. Click `Apply Guided Setup`.

### Answer a question queue item
1. Open `Brain`.
2. In `Question Queue`, type an answer.
3. Apply or defer it.

### Save a knowledge file
1. Open the `Knowledge Intake` section.
2. Fill in title and content at minimum.
3. Add summary, tags, or source if useful.
4. Click `Save Knowledge File`.

## Important Notes
- Brain state survives restart.
- The Brain is intentionally explicit and review-based.
- This is the layer that makes ModernClaw feel like a brain-building product instead of only a chat app.
