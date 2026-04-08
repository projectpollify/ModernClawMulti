# App Shell and Navigation Summary

## Purpose
The shell is the permanent frame around the multi-brain ModernClawMulti experience.

It gives the user:
- sidebar navigation
- conversation access
- active brain visibility
- brain switching and creation
- model access
- theme access
- view switching between Chat, Memory, Brain, and Settings

## Where To Find It In The App
After onboarding is complete, the app opens into the main shell.

Visible areas:
- left side: sidebar and conversation list
- top bar: header controls
- center: active view content

## User-Facing Features
### Sidebar
The sidebar contains:
- conversations for the active brain
- new chat action
- Memory view button
- Brain view button
- Settings view button

It supports two states:
- expanded sidebar
- collapsed rail

### Header
The header contains:
- sidebar toggle button
- active view title and subtitle
- active brain selector
- create brain action
- model selector
- theme toggle
- settings shortcut button

## Multi-Brain Navigation Truth
The shell is now multi-brain aware.

That means:
- switching brains changes the active workspace
- switching brains changes the conversation list
- switching brains restores the latest conversation for that brain when one exists
- switching brains should not leak streamed output or draft state into the wrong brain
- deleting a non-baseline brain is handled through brain management, not conversation management

## How It Works
### Frontend entry point
The app uses:
- `src/App.tsx`
- `src/components/layout/AppShell.tsx`
- `src/components/layout/Sidebar.tsx`
- `src/components/layout/Header.tsx`
- `src/components/layout/BrainSelector.tsx`
- `src/components/layout/ConversationList.tsx`

### View routing model
There is no router library.
The app uses a lightweight view store and conditionally renders the active panel.

The active views are:
- `chat`
- `memory`
- `brain`
- `settings`

## Stores Involved
- `src/stores/uiStore.ts`
- `src/stores/agentStore.ts`
- `src/stores/conversationStore.ts`
- `src/stores/modelStore.ts`

## User Instructions
### Switch brains
1. Open the brain selector in the header.
2. Choose an existing brain.
3. The shell reloads that brain's conversations, memory context, and active model preference.

### Create a brain
1. Use the `Create Brain` action in the header.
2. Enter a unique brain name.
3. The app creates the workspace and switches into it.

### Switch views
1. Open the sidebar if collapsed.
2. Click `Memory`, `Brain`, or `Settings`.
3. Use `New Chat` to start a new conversation for the current brain.

## Important Notes
- the shell is always present after onboarding
- the active brain is now a first-class shell concept
- the model selector remains in the header because the current brain's active model affects chat immediately
- Settings can be opened either from the sidebar or the header gear button
