# ModernClaw Curator System — Configuration Reference

This document records the live operational configuration of the ModernClaw Curator
pipeline as built. Use it to replicate, debug, or hand off the system.

For pipeline architecture, build state, risks, and build order, see:
→ `NOTEBOOKLM-CURATOR-WORKFLOW.md`

Last updated: 2026-04-09

---

## System Overview

Two Cowork agents share the same folder system and produce identical output formats.
The `## Source` field in each request form determines which agent handles it.

```
Request Form (Source: web research)  →  modernclaw-curator-agent        (auto, every 15 min)
Request Form (Source: NotebookLM)    →  modernclaw-notebooklm-curator   (manual trigger)
Request Form (Source: manual)        →  no agent — user places package directly in staged/

All three paths write to:  agents/joe/curator/staged/
                                       ↓
                           ModernClaw Curator Inbox (refresh to see)
                                       ↓
                           Approve → knowledge/   |   Reject → rejected/
```

---

## 1. Required Cowork Folder Connections

Both folders must be connected in Cowork for the agents to function. If a session
restarts and agents stop working, reconnecting these two is the first thing to check.

| Name               | Windows Path                                          | Purpose                                     |
|--------------------|-------------------------------------------------------|---------------------------------------------|
| `LocalAI`          | `C:\Users\pento\AppData\Roaming\LocalAI`              | ModernClaw workspace root — all curator I/O |
| `com.localai.app`  | `C:\Users\pento\AppData\Roaming\com.localai.app`      | App database — used to resolve active agent |

Mount paths on the Linux sandbox side change with every Cowork session.
The agents resolve them dynamically at runtime using glob patterns — no manual updates needed.

---

## 2. ModernClaw App Configuration

### Database location
```
C:\Users\pento\AppData\Roaming\com.localai.app\data.db
```

### Currently selected agent
```
active_agent_id = joe
```

### All registered agents

| Agent ID  | Name  | Workspace Path                                         | DB status field | Currently selected |
|-----------|-------|--------------------------------------------------------|-----------------|-------------------|
| `default` | Rosie | `C:\Users\pento\AppData\Roaming\LocalAI`               | active          | No                |
| `joe`     | Joe   | `C:\Users\pento\AppData\Roaming\LocalAI\agents\joe`    | active          | **Yes**           |

Note on the status field: `status = active` in the database means the agent is
registered and enabled — not that it is the currently selected brain. Only one
agent is selected at a time, determined by the `active_agent_id` settings key.
Curator automation always follows `active_agent_id`, not the status field.

---

## 3. Curator Folder Structure

All curator I/O runs inside the selected agent's workspace. For the current
selected agent (Joe), the full paths are:

| Folder          | Windows Path                                                    | Managed by     |
|-----------------|-----------------------------------------------------------------|----------------|
| `requests/`     | `...\agents\joe\curator\requests\`                              | User / agents  |
| `in-progress/`  | `...\agents\joe\curator\in-progress\`                           | Agents only    |
| `staged/`       | `...\agents\joe\curator\staged\`                                | Agents / user  |
| `approved/`     | `...\agents\joe\curator\approved\`                              | ModernClaw app |
| `rejected/`     | `...\agents\joe\curator\rejected\`                              | ModernClaw app |
| `archive/`      | `...\agents\joe\curator\archive\`                               | Agents only    |

All paths are under `C:\Users\pento\AppData\Roaming\LocalAI`.

### Staged package format
```
staged/
  └── [request-topic-slug]-[YYYYMMDD-HHMMSS]/
        ├── curated-knowledge.md    ← main knowledge file (required)
        ├── meta.json               ← title, summary, tags, source (required)
        └── assets/                 ← screenshots, generated charts (optional)
```

---

## 4. Request Form Routing

The `## Source` field in the request form controls which agent picks it up.

| Source value      | Agent that processes it                   | Trigger        |
|-------------------|-------------------------------------------|----------------|
| `web research`    | `modernclaw-curator-agent`                | Automatic      |
| `NotebookLM`      | `modernclaw-notebooklm-curator`           | Manual         |
| `manual`          | None — see Manual Mode below              | None           |
| Absent or blank   | `modernclaw-curator-agent` (default)      | Automatic      |

### Manual mode flow

When `Source: manual`, neither agent runs. This branch is for packages the user
produces directly — a pre-built `curated-knowledge.md` and `meta.json` that do
not need research or browser extraction.

To use manual mode:
1. Create a package folder directly in `staged/` following the standard naming pattern
2. Add `curated-knowledge.md` and `meta.json` using the standard schemas
3. Open ModernClaw → Brain → Curator Inbox → Refresh
4. The package appears and is ready to approve or reject

Manual mode is how the system was first validated. It has no dependencies on
Cowork tasks, Chrome, or mounted folders.

---

## 5. Scheduled Tasks

Both tasks live in the Cowork **Scheduled** sidebar.
Canonical rebuild reference copies of the prompts are in `CURATOR_TASKS/` in this repo.

### Task 1 — modernclaw-curator-agent (Web Research)

| Setting        | Value                                                |
|----------------|------------------------------------------------------|
| Task ID        | `modernclaw-curator-agent`                           |
| Schedule       | Every 15 minutes — cron `*/15 * * * *`               |
| Handles        | `Source: web research` and requests with no Source field |
| Skips          | `Source: NotebookLM`                                 |
| Tools required | WebSearch, WebFetch, file R/W, Bash, Python sqlite3  |
| Chrome needed  | No                                                   |

Windows task file (machine-local, not in repo):
```
C:\Users\pento\Documents\Claude\Scheduled\modernclaw-curator-agent\SKILL.md
```

Repo rebuild reference:
```
CURATOR_TASKS\modernclaw-curator-agent.md
```

### Task 2 — modernclaw-notebooklm-curator (NotebookLM + Chrome)

| Setting        | Value                                                |
|----------------|------------------------------------------------------|
| Task ID        | `modernclaw-notebooklm-curator`                      |
| Schedule       | Manual only — no automatic schedule                  |
| Handles        | `Source: NotebookLM` only                            |
| Skips          | All other source types                               |
| Tools required | Chrome browser tools, file R/W, Bash, Python sqlite3 |
| Chrome needed  | Yes — extension must be active, user must be logged into Google |

Windows task file (machine-local, not in repo):
```
C:\Users\pento\Documents\Claude\Scheduled\modernclaw-notebooklm-curator\SKILL.md
```

Repo rebuild reference:
```
CURATOR_TASKS\modernclaw-notebooklm-curator.md
```

**NotebookLM task build state:** The task is configured and the prompt is complete.
The Chrome extraction path has not been tested end-to-end against a live notebook.
It is currently external-automation-driven — it depends on the browser being open,
the extension being active, and the user being logged in. Treat it as designed and
wired but not yet production-validated. See `NOTEBOOKLM-CURATOR-WORKFLOW.md` for
full build state and remaining gaps.

---

## 6. Dynamic Workspace Resolver

Both agents run this Python block at the start of every run to find the correct
workspace paths without relying on hardcoded session IDs.

```python
import sqlite3, shutil, os, tempfile, re, glob

localai_paths = glob.glob("/sessions/*/mnt/LocalAI")
appdata_paths = glob.glob("/sessions/*/mnt/com.localai.app")

if not localai_paths:
    raise SystemExit("ERROR: LocalAI folder is not mounted in Cowork")
if not appdata_paths:
    raise SystemExit("ERROR: com.localai.app folder is not mounted in Cowork")

localai_mount = localai_paths[0]
appdata_mount = appdata_paths[0]

# Copy DB to avoid locking the live file
db_src = f"{appdata_mount}/data.db"
tmp = tempfile.mktemp(suffix=".db")
shutil.copy2(db_src, tmp)

conn = sqlite3.connect(tmp)
cur = conn.cursor()
cur.execute("SELECT value FROM settings WHERE key = 'active_agent_id'")
active_id = cur.fetchone()[0]
cur.execute("SELECT name, workspace_path FROM agents WHERE agent_id = ?", (active_id,))
agent_name, win_workspace = cur.fetchone()
conn.close(); os.unlink(tmp)

# Convert Windows path to Linux mount path
parts = re.split(r'[/\\]', win_workspace)
localai_idx = next(i for i, p in enumerate(parts) if p == 'LocalAI')
relative = '/'.join(p for p in parts[localai_idx+1:] if p)
workspace = f"{localai_mount}/{relative}" if relative else localai_mount

REQUESTS_DIR   = f"{workspace}/curator/requests"
INPROGRESS_DIR = f"{workspace}/curator/in-progress"
STAGED_DIR     = f"{workspace}/curator/staged"
ARCHIVE_DIR    = f"{workspace}/curator/archive"
```

If `active_agent_id` is changed inside ModernClaw (switching the active brain),
all subsequent agent runs automatically follow the new active workspace.

---

## 7. meta.json Schema

```json
{
  "title": "Short descriptive title (3–8 words)",
  "summary": "2–3 sentences on what the package contains and what it is for.",
  "source": "web research | NotebookLM | manual",
  "notebook_url": "https://notebooklm.google.com/... (NotebookLM packages only)",
  "tags": ["tag1", "tag2", "tag3"],
  "request_topic": "short-slug-matching-request-form",
  "created_at": "2026-04-09T19:16:15Z"
}
```

Fields surfaced in the Curator Inbox UI: `title`, `summary`, `tags`, `request_topic`.
The `summary` field is the most important — it determines whether a reviewer
can approve without opening the full document.

---

## 8. Replication Checklist

Use this when rebuilding on a new machine or after a reset.

- [ ] Connect `C:\Users\pento\AppData\Roaming\LocalAI` in Cowork
- [ ] Connect `C:\Users\pento\AppData\Roaming\com.localai.app` in Cowork
- [ ] Confirm `agents/joe/curator/` subfolders exist: requests, in-progress, staged, approved, rejected, archive
- [ ] Copy `TEMPLATE-knowledge-request.md` from `CURATOR_TASKS/` into `agents/joe/curator/requests/`
- [ ] Recreate `modernclaw-curator-agent` using the prompt in `CURATOR_TASKS/modernclaw-curator-agent.md`
- [ ] Recreate `modernclaw-notebooklm-curator` using the prompt in `CURATOR_TASKS/modernclaw-notebooklm-curator.md`
- [ ] Run each task once manually via "Run now" to pre-approve tool permissions
- [ ] Confirm `active_agent_id` in the database matches the brain you want curator to target
- [ ] Drop a test request with `Source: web research` and confirm it lands in `staged/` and appears in Curator Inbox
- [ ] Validate manual mode by placing a hand-built package in `staged/` and confirming it appears after Refresh
