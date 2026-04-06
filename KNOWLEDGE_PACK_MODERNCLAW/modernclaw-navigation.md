# ModernClaw Navigation

## Purpose
The app shell gives the user a stable workspace for switching between chat, memory editing, brain-building workflows, and settings.

## Where To Find It
The shell appears after onboarding is complete.

Main navigation lives in the left sidebar.

## Main Areas
- `Conversations` list in the sidebar
- `New Chat` action in the sidebar footer area
- `Memory` view
- `Brain` view
- `Settings` view

## Header Controls
The top header includes:
- sidebar collapse toggle
- current view title and subtitle
- model selector dropdown
- theme toggle
- settings shortcut

## Sidebar Behavior
The sidebar can collapse to a narrow rail instead of disappearing entirely. This was intentionally designed to prevent the user from losing navigation access.

## Model Selector Behavior
The model selector in the header shows currently installed Ollama models. If Ollama is offline, it shows an offline state instead of a normal list.

## Navigation Rules
- If onboarding is incomplete, the onboarding flow replaces the normal shell.
- If conversation history is enabled, the sidebar conversation list persists across restarts.
- If conversation history is disabled, the chat behaves more like an ephemeral session.

## User Guidance
Use the shell this way:
1. Chat for direct model interaction.
2. Memory to edit the live brain files and knowledge files.
3. Brain to improve the system through guided workflows.
4. Settings to control model, voice, behavior, privacy, and storage behavior.
