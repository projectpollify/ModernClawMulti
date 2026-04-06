# App Shell and Navigation Summary

## Purpose
The app shell is the permanent frame around the main ModernClaw experience.

It provides:
- sidebar navigation
- header controls
- model selector access
- theme toggle access
- view switching between Chat, Memory, Brain, and Settings

## Where To Find It In The App
After onboarding is complete, the app opens into the main shell.

Visible areas:
- left side: sidebar
- top bar: header
- center: active view content

## User-Facing Features
### Sidebar
The sidebar contains:
- conversations
- new chat action
- Memory view button
- Brain view button
- Settings view button

It supports two states:
- expanded sidebar
- collapsed rail

This was intentionally changed from a fully disappearing sidebar to a safer collapsed rail so users cannot lose navigation completely.

### Header
The header contains:
- sidebar toggle button
- active view title
- model selector dropdown
- theme toggle
- settings shortcut button

## How It Works
### Frontend entry point
The app uses:
- `src/App.tsx`
- `src/components/layout/AppShell.tsx`
- `src/components/layout/Sidebar.tsx`
- `src/components/layout/Header.tsx`
- `src/components/layout/MainContent.tsx`

### View routing model
There is no router library here.
The app uses a lightweight view store and conditionally renders the active panel.

The active views are:
- `chat`
- `memory`
- `brain`
- `settings`

## Stores Involved
- `src/stores/uiStore.ts`
  - active view state
  - sidebar open/collapsed state

## User Instructions
### Switch views
1. Open the sidebar if collapsed.
2. Click `Memory`, `Brain`, or `Settings`.
3. `New Chat` resets the main work area back toward the chat flow.

### Collapse or expand the sidebar
1. Click the hamburger button in the header.
2. When collapsed, use the left rail controls to re-expand.

## Important Notes
- The shell is always present after onboarding.
- The model selector sits in the header because model choice affects chat globally.
- Settings can be opened either from the sidebar or the header gear button.
