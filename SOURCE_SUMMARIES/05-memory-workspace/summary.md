# Memory Workspace Summary

## Purpose
The Memory workspace is where the user inspects and edits the markdown files that define the active brain's long-term context.

It remains one of the clearest and most transparent parts of the app.

## Where To Find It In The App
Open the `Memory` view from the sidebar.

Visible sections:
- core memory files
- daily logs
- knowledge files
- brain-aware workspace guidance

## What It Controls
### Core files
- `SOUL.md`
- `USER.md`
- `MEMORY.md`

### Daily logs
- `memory/YYYY-MM-DD.md`

### Knowledge references
- top-level markdown files in `knowledge/`

## Main Components
- `src/components/memory/MemoryView.tsx`
- `src/components/memory/MemoryFileCard.tsx`
- `src/components/memory/MemoryEditor.tsx`
- `src/components/memory/DailyLogList.tsx`
- `src/components/memory/DailyLogViewer.tsx`
- `src/components/memory/DailyLogComposer.tsx`
- `src/components/memory/KnowledgeFiles.tsx`

## Store and Service
- `src/stores/memoryStore.ts`
- `src/services/memory.ts`
- Rust memory service in `src-tauri/src/services/memory.rs`

## Current Multi-Brain Behavior
- Memory always reflects the active brain
- daily logs are isolated per brain
- knowledge files are isolated per brain
- switching brains reloads the workspace instead of leaving stale content visible
- baseline Rosie uses the root LocalAI workspace, while created brains use `agents/<brain>/`

## Important Notes
- knowledge loading is still flat: only top-level `.md` files in `knowledge/` are loaded
- Memory is still the clearest view into what the current brain actually knows
- the current app now makes Memory feel visibly brain-scoped, not just technically scoped
