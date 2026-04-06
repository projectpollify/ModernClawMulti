# Curator Inbox Summary

## Purpose
The Curator Inbox is the review layer between raw external research and live knowledge.

It exists so research can be prepared safely before it reaches the active brain.

## Where To Find It In The App
Open `Brain` and look for the `Curator Inbox` section.

## What It Shows
It lists staged curator packages prepared in the filesystem under the curator workflow.

Each package can include:
- title
- summary
- source label
- tags
- request topic
- created timestamp

## Main Component
- `src/components/brain/CuratorInbox.tsx`

## Backend Support
Curator package handling is built into the Rust memory service.
Relevant folders are created automatically:
- `curator/requests`
- `curator/in-progress`
- `curator/staged`
- `curator/approved`
- `curator/rejected`
- `curator/archive`

## Current Package Format
A staged package lives in its own folder and may include files like:
- `curated-knowledge.md`
- `meta.json`
- source notes
- request notes

## User Actions
### Refresh
Refresh the list of staged packages.

### Import to Knowledge
Moves the package into live usage by:
- reading `curated-knowledge.md`
- writing a new file into `knowledge/`
- moving the package from `staged` to `approved`

### Reject
Moves the package from `staged` to `rejected`.

## User Instructions
### Import a staged package
1. Open `Brain`.
2. Find the package in `Curator Inbox`.
3. Click `Import to Knowledge`.

### Reject a staged package
1. Open `Brain`.
2. Find the package.
3. Click `Reject`.

## Important Notes
- Curator packages added outside the app appear after refresh.
- This is a staging-and-review model, not autonomous live mutation.
- The Curator Inbox is a foundation for future research/auto-improvement workflows.
