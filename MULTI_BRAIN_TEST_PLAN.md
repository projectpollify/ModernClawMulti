# Multi-Brain Test Plan

## Purpose
This file defines the tests that matter for the multi-brain experiment.

## Test Philosophy
Do not test everything at once.
Test in layers and keep the single-brain case healthy while adding multi-brain capability.

## Phase 0 Tests: Baseline Repo
### Test 0.1
The copied baseline app starts successfully.

### Test 0.2
Existing single-brain behavior matches the current stable app.

## Phase 1 Tests: Data Foundations
### Test 1.1
A default agent record exists.

### Test 1.2
The existing workspace resolves correctly as the default brain.

### Test 1.3
Existing conversations are associated with the default brain without visible regression.

## Phase 2 Tests: Active Brain Switching
### Test 2.1
Changing the active brain reloads conversations.

### Test 2.2
Changing the active brain reloads Memory data.

### Test 2.3
Changing the active brain reloads Brain builder state.

### Test 2.4
No stale conversation or memory data from brain A appears in brain B.

## Phase 3 Tests: Create Brain Flow
### Test 3.1
User can create a new brain.

### Test 3.2
New workspace folders are created.

### Test 3.3
Starter `SOUL.md`, `USER.md`, and `MEMORY.md` are created.

### Test 3.4
The new brain becomes selectable immediately.

## Phase 4 Tests: Full Workspace Isolation
### Test 4.1
Editing `SOUL.md` in brain A does not affect brain B.

### Test 4.2
Knowledge files added in brain A do not appear in brain B.

### Test 4.3
Curator staged packages in brain A do not appear in brain B.

### Test 4.4
Daily logs are isolated per brain.

### Test 4.5
Conversation list in sidebar reflects only the active brain.

## Phase 5 Tests: UX Clarity
### Test 5.1
The active brain is always visible.

### Test 5.2
Users can rename conversations without confusion.

### Test 5.3
Users do not lose orientation when switching between brains.

## Failure Signals To Watch For
- stale state after brain switch
- mixed conversations across brains
- mixed memory files across brains
- Brain builder suggestions leaking between brains
- confusing or hidden active-brain state
- migration bugs in the default brain

## Final V1 Pass Condition
The experiment passes v1 if:
- multiple brains are usable
- each brain feels isolated
- the app remains understandable
- the original single-brain value is preserved inside the new architecture
