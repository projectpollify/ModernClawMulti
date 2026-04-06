# ModernClaw Memory Workspace

## Purpose
The Memory view exposes the live markdown workspace behind the assistant. This is where the user can directly inspect and edit the files that shape behavior, context, and persistent knowledge.

## Where To Find It
Open the `Memory` view from the sidebar.

## Core Files
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

## Other Workspace Areas
- daily logs under `memory/`
- top-level knowledge files under `knowledge/`
- curator folders under `curator/`
- voice tool folders under `tools/`

## Main Memory View Functions
- open the storage folder
- refresh the current memory snapshot
- edit and save core files
- inspect daily logs
- inspect knowledge files

## Knowledge Files
The Memory view can list top-level knowledge markdown files. This reflects the same flat loading rule used by the runtime context builder.

## Daily Logs
The system can keep daily notes that become part of live context for the current day.

## Why This Matters
Memory is one of ModernClaw's core differentiators. The user can see and control what the assistant is being shaped by instead of treating the context system as a black box.

## User Guidance
1. Open `Memory`.
2. Review `SOUL.md` to shape behavior and tone.
3. Review `USER.md` to shape user context.
4. Review `MEMORY.md` for current priorities and durable notes.
5. Add compact top-level knowledge files for domain expertise.
