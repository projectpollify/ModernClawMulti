# MVP Extension Plan

## Purpose

This document defines the original plan for extending the Local AI MVP into a more differentiated product without disturbing the existing working app.

The current MVP remains intact as a stable baseline.

This document is now best read as the founding extension thesis.
The active sequencing has since moved into `ROADMAP.md`, because much of the early extension plan is already complete.

## Status Snapshot

### Completed From This Plan

Already built in `LocalAI-Next` / ModernClaw:
- separate cloned workspace
- Brain suggestion system
- question queue
- accepted / deferred workflow
- persistent Brain state
- profile updates writing into `USER.md`
- memory updates writing into `MEMORY.md`
- richer knowledge intake
- Brain activity history
- guided Brain setup
- curator staging pipeline and Curator Inbox

### Still Relevant From This Plan

Still ahead:
- model selection and onboarding strategy
- cowork / curator automation integration
- deeper knowledge management
- multi-brain thinking
- voice and variant systems later

---

## Product Direction

The current Local AI app is useful, real, and shippable, but it is not yet differentiated enough to be the final product story by itself.

The extension direction remains:
- keep instant local chat
- keep editable memory and knowledge files
- add progressive personalization
- add agent-assisted brain building
- let the AI become more useful over time without forcing a long setup flow at first launch

Core thesis:

The app should help build a custom local AI brain around the user over time.

That means:
- fast start for users who want to use it immediately
- optional guided setup for users who want stronger personalization up front
- ongoing background/cowork assistance that improves `USER.md`, `MEMORY.md`, and `knowledge/` over time

## Non-Goal

We are not trying to turn the current MVP into a giant all-at-once platform rewrite.

We are not adding every possible future concept immediately.

We are selecting the smallest set of high-leverage additions that make the app feel novel and worth talking about.

## Separation Strategy

### Keep Current MVP Untouched

The current repo/workspace remains:
- the stable MVP
- the current Windows-validated product
- the fallback if new experiments fail

### Create A New Working Copy

That strategy is already in place.

Current structure:

```text
Desktop/
  LocalAI/                  # frozen/current MVP
  LocalAI-Next/             # differentiated product track / ModernClaw
```

Important rule remains:
- do not destabilize the original MVP while exploring the differentiated product line

## New Product Layer

The extension product was built around three personalization layers.

### Layer 1: Fast Start

Goal:
- user can begin chatting immediately
- no long questionnaire required
- app works on first open even if the user skips setup

### Layer 2: Guided Brain Setup

Goal:
- help users who want stronger personalization up front
- generate a better first-pass `USER.md`, `MEMORY.md`, and `SOUL.md`

Status:
- now implemented in first version

### Layer 3: Ongoing Brain Builder

Goal:
- build personalization over time
- use a cowork/research/curation agent to identify missing context and improve the local brain

Requirements:
- detect missing user context
- propose profile questions
- gather source documents
- generate suggested knowledge files
- propose updates to memory files
- keep the user in control

This remains one of the main differentiators.

## Core Feature Additions

### Feature 1: Progressive Profile Builder

Status:
- implemented in first working form

### Feature 2: Brain Suggestions Queue

Status:
- implemented in first working form

### Feature 3: Knowledge Intake / Source Gathering

Status:
- implemented in richer first working form

### Feature 4: Cowork Brain Builder Agent

Status:
- not yet implemented end to end
- staging pipeline now exists
- curator automation is the natural next step

Important design rule remains:
- the agent should propose, not silently mutate

## Proposed Build Order

The original build order was:
- clone workspace
- define suggestion model
- define profile question flow
- define knowledge intake
- define cowork integration

That original sequence largely happened.

For the active next sequence, use `ROADMAP.md`.

## UX Rules

These still remain correct:
- user can always just chat
- advanced personalization is available but not forced
- AI improvement feels progressive and helpful
- suggestions are reviewable
- important actions are understandable

## Success Criteria

The extension product is successful if:
- users can still start instantly without friction
- users who want personalization can improve the AI quickly
- the app gathers better context over time
- the system produces a brain that feels increasingly customized
- the product becomes clearly more interesting than a generic local chat client

## Current Recommendation

This document has served its purpose well.

The best next planning references are now:
- `NEXT_PROGRESS.md`
- `ROADMAP.md`
- the private strategy and feature planning docs in this workspace

Use this file as the founding extension brief, not the active execution tracker.
