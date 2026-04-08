# Brain Builder Summary

## Purpose
The Brain view is still ModernClawMulti's differentiated workflow layer.

It proposes structured improvements to the active brain and lets the user review, defer, accept, dismiss, or stage knowledge changes explicitly.

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
- Curator Inbox

## Main Components
- `src/components/brain/BrainView.tsx`
- `src/components/brain/SuggestionCard.tsx`
- `src/components/brain/QuestionQueueCard.tsx`
- `src/components/brain/CuratorInbox.tsx`

## Store and Persistence
- `src/stores/suggestionStore.ts`

Important current truth:
- Brain state is namespaced by active brain
- Brain drafts and feedback now reset cleanly on brain switch

## Key Features
### Guided Setup
Writes starter sections into the active brain's:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

### Question Queue
A dedicated place for the current brain's missing context questions.

### Suggestions
Suggestions move through a visible lifecycle:
- pending
- deferred
- accepted
- dismissed

### Knowledge Intake
The Brain can create new structured knowledge files for the active brain using:
- title
- summary
- tags
- source
- content

### Curator Inbox
The Brain now also holds the staging-and-review layer for curated external knowledge.

## Important Notes
- Brain remains explicit and review-based
- Brain is now strongly brain-scoped instead of behaving like one global review surface
- this is still one of the main layers that makes the product feel like a brain-building system instead of only a chat app
