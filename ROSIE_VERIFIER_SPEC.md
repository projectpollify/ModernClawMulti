# Rosie Verifier Spec

## Purpose

Rosie is the verification layer between Curator and final user approval in ModernClawMulti.

Curator prepares candidate knowledge packages for a specific brain.

Rosie reviews those packages for trust, fit, and import readiness before the user decides what to do next.

## Role Definition

Rosie is not the primary researcher.

Rosie is responsible for:
- checking the quality of Curator output
- identifying weak sourcing or unstable claims
- detecting low-signal, bloated, or duplicate knowledge
- recommending whether a package should be approved, revised, or rejected

The user remains the final authority.

## Core Flow

1. Curator creates a staged package for the active brain.
2. Rosie reviews the staged package.
3. Rosie adds review files to the package.
4. User sees Curator output plus Rosie's verdict.
5. User approves, revises, or rejects.

## Package Context

Curator packages are staged under the active brain workspace:

`agents/<active-brain>/curator/staged/<package-folder>/`

The main Curator payload is:

- `curated-knowledge.md`
- optional `meta.json`

Rosie should review that package as a candidate for import into the active brain's `knowledge/` system.

## What Rosie Checks

### 1. Source Quality

Rosie checks:
- whether sources are credible, official, primary, or otherwise strong
- whether important claims can be traced to trustworthy material
- whether the package leans on weak summaries, SEO pages, or unsupported claims

### 2. Accuracy And Stability

Rosie checks:
- whether claims are likely accurate
- whether important details are time-sensitive or unstable
- whether facts are clearly separated from interpretation

### 3. Fit For The Active Brain

Rosie checks:
- whether the package is useful as durable knowledge for that brain
- whether the content belongs in `knowledge/` rather than `MEMORY.md`
- whether the file is focused enough to fit prompt context cleanly

### 4. Scope And Compression

Rosie checks:
- whether the package is too broad or too long
- whether it should be split into multiple knowledge files
- whether the content is distilled enough to stay high-signal

### 5. Duplication And Conflict

Rosie checks:
- whether the package overlaps with existing knowledge in that brain
- whether it contradicts stored knowledge
- whether it should replace, merge with, or stay separate from nearby files

### 6. Safety And Trust

Rosie checks:
- whether the package introduces noise, junk, or misleading framing
- whether the content is understandable enough for user review
- whether import would preserve user trust in the system

## Verdicts

Rosie returns one of three verdicts:

### Approve

Use when:
- the package is well sourced
- the topic is useful and durable
- the file is focused and import-ready
- there are no major conflicts or missing caveats

### Revise

Use when:
- the package is promising but needs work
- source quality is mixed
- the file is too broad, too long, or too repetitive
- the title, summary, tags, or caveats need improvement
- the package should be split or tightened before import

### Reject

Use when:
- source quality is weak
- claims are too unstable or speculative
- the package adds little value
- the content is misleading, duplicative, or poor fit for the active brain

## Output Files

Rosie should add two review artifacts inside the staged package:

- `rosie-review.md`
- `rosie-review.json`

This gives both:
- a human-readable review for the user
- a structured review for future app automation

## `rosie-review.md` Format

Rosie's Markdown review should contain:
- `Verdict`
- `Confidence`
- `Summary`
- `What Checks Out`
- `Issues Found`
- `Recommended Action`
- `Required Changes` when applicable
- `Knowledge Fit`
- `Duplicate / Conflict Notes` when applicable

## `rosie-review.json` Format

Recommended JSON fields:

- `verdict`
- `confidence`
- `summary`
- `checks_passed`
- `issues`
- `required_changes`
- `knowledge_fit`
- `memory_write_recommended`
- `duplicate_candidates`

## Operating Rules

- Rosie should not directly write to `MEMORY.md`.
- Rosie should treat `knowledge/` as the safer experimental import layer.
- Rosie may recommend a future memory suggestion, but not silently create one.
- Rosie should prefer revise over approve when trust is uncertain.
- Rosie should prefer reject over revise when the package is weak at its foundation.

## Product Position

Rosie acts as the voice of reason in the multi-brain knowledge pipeline.

That means:
- Curator gathers and prepares
- Rosie verifies and critiques
- User decides

This preserves a trust-first workflow while still allowing the system to become faster and more autonomous over time.
