# ModernClaw Navigation

## Purpose
The app shell gives the user a stable workspace for switching between chat, memory editing, brain-building workflows, settings, and the active brain itself.

## Where To Find It
The shell appears after onboarding is complete.

Main navigation lives in the left sidebar, while active brain controls live in the header.

## Main Areas
- `Conversations` list in the sidebar for the active brain
- `New Chat` action in the sidebar footer area
- `Memory` view
- `Brain` view
- `Settings` view

## Header Controls
The top header includes:
- sidebar collapse toggle
- current view title and subtitle
- active brain selector
- create brain action
- model selector dropdown
- theme toggle
- settings shortcut

## Brain Navigation Rules
- switching brains changes the active workspace
- switching brains changes the visible conversation list
- switching brains restores the latest conversation for that brain when one exists
- streamed output should remain attached to the brain and conversation that started it
- deleting a brain is separate from deleting a conversation

## Sidebar Behavior
The sidebar can collapse to a narrow rail instead of disappearing entirely. This was intentionally designed to prevent the user from losing navigation access.

## Model Selector Behavior
The model selector in the header shows currently installed Ollama models. It acts on the active brain's model preference rather than behaving like a purely global selector.

## User Guidance
Use the shell this way:
1. Chat for direct interaction with the current brain.
2. Memory to edit that brain's live files and knowledge.
3. Brain to improve that brain through guided workflows.
4. Settings to control model, voice, behavior, privacy, and storage behavior.
5. Header brain selector to move between Rosie, Mia, or other brains.
