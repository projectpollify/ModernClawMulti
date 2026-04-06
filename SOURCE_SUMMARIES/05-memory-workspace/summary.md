# Memory Workspace Summary

## Purpose
The Memory workspace is where the user inspects and edits the core markdown files that define the assistant’s long-term brain.

It is the most transparent part of ModernClaw’s architecture.

## Where To Find It In The App
Open the `Memory` view from the sidebar.

Visible sections:
- Open Brain overview
- core memory files
- daily logs
- knowledge files

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

## How It Works
### Initialization
When Memory opens, the store calls `memory_initialize()`.
That ensures the base folder and expected files/folders exist.

### Default folder structure
The backend creates:
- base memory folder
- `memory/`
- `knowledge/`
- `curator/`
- `tools/piper/voices`
- `tools/whisper/models`

### File loading
The store reads:
- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- daily log list
- knowledge file list
- curator staged packages
- current memory base path

## User Instructions
### Edit a core brain file
1. Open `Memory`.
2. Click into the file card/editor for `SOUL.md`, `USER.md`, or `MEMORY.md`.
3. Save the file.

### View daily logs
1. Open `Memory`.
2. Use the daily log list.
3. Select a date to inspect its log.

### Add knowledge manually outside the app
1. Open the memory folder.
2. Place a markdown file in `knowledge/`.
3. Return to the app and refresh Memory.

### Open the storage folder
1. In the Memory view, click `Open Folder`.

## Important Notes
- The Memory workspace is the clearest view into what the assistant actually knows.
- Knowledge loading is flat: only top-level `.md` files in `knowledge/` are loaded.
- This is the main foundation for future expert brain creation.
