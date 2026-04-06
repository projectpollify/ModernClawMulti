# ModernClaw Curator Inbox

## Purpose
The Curator Inbox is the review surface for staged external research or compiled packages before they are imported into live knowledge.

## Where To Find It
It appears inside the `Brain` view as `Curator Inbox`.

## What It Shows
Staged packages prepared in the curator workflow. Each package can expose summary information and review actions.

## Current Actions
- Refresh
- Import to Knowledge
- Reject

## Storage Layout
The memory service initializes these curator folders:
- `curator/requests`
- `curator/in-progress`
- `curator/staged`
- `curator/approved`
- `curator/rejected`
- `curator/archive`

## Import Behavior
Approved staged packages are imported into live knowledge instead of giving external automation direct write access to the production knowledge layer.

## Why This Matters
This is the safety layer between raw source gathering and the assistant's live brain. It keeps the system reviewable and reduces uncontrolled context pollution.

## User Guidance
1. Open `Brain`.
2. Go to `Curator Inbox`.
3. Refresh if external packages were added.
4. Review the staged package.
5. Import trusted content into knowledge or reject it.
