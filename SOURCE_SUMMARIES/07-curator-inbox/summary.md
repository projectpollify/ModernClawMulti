# Curator Inbox Summary

## Purpose
The Curator Inbox is the review layer between raw external research and live knowledge.

It exists so research can be prepared safely before it reaches the active brain.

## Where To Find It In The App
Open `Brain` and look for the `Curator Inbox` section.

## Current Multi-Brain Truth
The Curator Inbox is now brain-scoped.
That means:
- staged packages belong to the active brain workspace
- importing a package affects only the active brain
- switching brains should not show another brain's staged or imported curator content

## What It Shows
It lists staged curator packages prepared in the filesystem under the current brain's curator workflow.

Each package can include:
- title
- summary
- source label
- tags
- request topic
- created timestamp

## Main Component
- `src/components/brain/CuratorInbox.tsx`

## Important Notes
- Curator packages added outside the app appear after refresh
- this is a staging-and-review model, not autonomous live mutation
- the Curator Inbox is now part of the active brain workflow, not one global inbox
