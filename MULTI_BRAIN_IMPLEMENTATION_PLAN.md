# Multi-Brain Implementation Plan

## Goal
Implement a ModernClaw-native multi-brain workspace system in a way that preserves:
- local-first control
- markdown-based brain files
- SQLite conversation persistence
- explicit Brain workflows
- clear user ownership

## Success Definition
The first milestone is simple:
Users can create, select, and use multiple isolated local brains cleanly.

That means:
- a brain registry exists
- an active brain can be selected
- each brain has its own workspace
- each brain has its own conversation history
- Memory, Brain, and Chat all switch with the active brain

## Non-Goals For V1
- orchestration between brains
- cross-brain routing
- team sharing
- billing restrictions
- shared memory pools
- per-brain everything

## Core Architecture Rule
Do not treat this as persona switching.
Treat it as full workspace switching.

## Proposed Data Model

### Agents Table
Add an `agents` table with at least:
- `agent_id`
- `name`
- `description`
- `created_at`
- `updated_at`
- `workspace_path`
- `default_model`

### Conversations
Add `agent_id` to `conversations`.
Messages can stay linked to conversations, but conversation queries must be filtered by active `agent_id`.

## Proposed Workspace Shape
```text
agents/
  default/
    SOUL.md
    USER.md
    MEMORY.md
    memory/
    knowledge/
    curator/
    tools/
  rosie/
    SOUL.md
    USER.md
    MEMORY.md
    memory/
    knowledge/
    curator/
    tools/
  triton/
    SOUL.md
    USER.md
    MEMORY.md
    memory/
    knowledge/
    curator/
    tools/
```

## Migration Rule
The existing single-brain workspace becomes the first default brain.

Do not force a risky file migration in the first pass.
The safest first move is to register the current workspace as the initial default brain.

## Implementation Phases

### Phase 0: Baseline Setup
- copy current ModernClaw code into this repo
- confirm baseline app still runs
- create tracking docs
- preserve a baseline commit before architecture changes

### Phase 1: Data Foundations
- add `agents` table
- add `agent_id` to conversations
- add backend agent registry service
- add agent-aware workspace resolution
- seed default brain from current workspace

### Phase 2: Active-Brain Infrastructure
- add active-brain store in frontend
- make memory store brain-aware
- make conversation store brain-aware
- make chat context builder brain-aware
- namespace Brain builder state by `agent_id`

### Phase 3: Brain Selector UX
- add active brain selector
- show active brain in shell/header
- add `Create Brain` flow
- generate starter workspace files for new brain
- restore the latest conversation when switching into a brain that already has chat history
- keep streamed responses attached to the brain and conversation that started them
- add safe delete-brain support for non-baseline brains
- keep baseline/default mechanics internal instead of making them a user-facing concept

### Phase 4: Full Workspace Integration
- Memory view becomes brain-scoped
- Brain view becomes brain-scoped
- curator flow becomes brain-scoped
- daily logs become brain-scoped
- knowledge import becomes brain-scoped

### Phase 5: Quality Of Life
- conversation rename improvements for multi-brain management
- brain rename
- archive brain
- stronger empty-state guidance for new brains
- clearer per-brain default-model behavior

## Hardest Technical Areas
- workspace resolution
- frontend store reload discipline on brain switch
- persistence migration
- Brain builder state isolation
- keeping the user oriented in the UI
- model ownership clarity between global settings and the active brain

## V1 Acceptance Criteria
- default single-brain behavior still works
- second brain can be created
- switching brains reloads the app context cleanly
- conversations do not bleed between brains
- memory files do not bleed between brains
- Brain workflows remain isolated per brain
- switching to an existing brain returns the user to a real conversation instead of an unnecessary blank chat
- switching brains during a streamed response does not leak output into the wrong brain

## Build Discipline
Do not widen scope until the previous phase feels solid.
The clean order is:
1. data model
2. state switching
3. UX
4. deeper integration
5. polish

## Documentation Discipline
- update `MULTI_BRAIN_STATUS.md` before or with each checkpoint commit
- only change this plan when scope, sequencing, or priorities materially shift
- keep the status doc as the day-to-day source of truth and keep this plan as the stable roadmap

