# NotebookLM -> ModernClaw Curator Workflow

## The Big Idea

NotebookLM is a world-class source gatherer and synthesizer. ModernClaw is where the
resulting knowledge lives and gets used. Cowork is the bridge between them - pulling
content out of NotebookLM, transforming it into ModernClaw's knowledge format, and
staging it for review. This document maps the full pipeline.

---

## Full Pipeline Overview

```
[ User drops sources into NotebookLM ]
           ->
[ NotebookLM generates summaries, briefings, Q&As, mind maps ]
           ->
[ Chrome Agent extracts content from NotebookLM UI ]
           ->
[ Transformation Agent rebuilds content in ModernClaw format ]
           ->
[ Curator Package lands in agents/joe/curator/staged/ ]
           ->
[ User reviews and approves in ModernClaw Curator Inbox ]
           ->
[ Knowledge lives in ModernClaw - used by Joe in chat and context ]
```

---

## Stage 1 - Source Intake (User -> NotebookLM)

The user loads raw source material into a NotebookLM notebook. This is NotebookLM
doing what it does best: ingesting and indexing.

Supported source types:
- PDFs (reports, whitepapers, research papers)
- Google Docs
- Web URLs
- YouTube videos
- Copied text

No agent involvement here. The user builds the notebook manually. The more focused
the notebook (one topic per notebook), the better the output.

---

## Stage 2 - Generation (NotebookLM)

Once sources are loaded, NotebookLM can generate structured outputs on demand.
These are the outputs the Chrome agent will target:

| NotebookLM Output    | What It Contains                          | Extractable? |
|----------------------|-------------------------------------------|--------------|
| Briefing Doc         | Full prose summary of all sources         | Yes - clean text |
| FAQ                  | Q&A format summary                        | Yes - structured |
| Study Guide          | Key concepts and explanations             | Yes - structured |
| Timeline             | Chronological event list                  | Yes - list format |
| Table of Contents    | Hierarchical topic map                    | Yes - outline format |
| Mind Map             | Visual node diagram                       | Partial - text labels extractable, layout is visual |
| Chat Q&A             | Direct answers to specific questions      | Yes - on demand |

**The agent's job in this stage:** trigger the generation of the right output type,
then read what NotebookLM produces.

---

## Stage 3 - Chrome Extraction (Cowork -> Chrome -> NotebookLM)

The Chrome Agent opens the NotebookLM notebook tab and:

1. Navigates to the correct notebook (by URL or by finding it on the home screen)
2. Requests generation of the target output type (clicks the button)
3. Reads the generated content from the page DOM
4. For visual elements (mind maps): takes a screenshot AND extracts visible text labels
5. Asks targeted follow-up questions in the NotebookLM chat if needed
6. Captures all extracted content as structured raw material

**What triggers this stage:** a Curator Request form with a `Source: NotebookLM`
field pointing to a specific notebook URL.

**Output of this stage:** raw extracted content (text, structured data, screenshot
paths) passed to the Transformation Agent.

---

## Stage 4 - Transformation (Cowork Processing)

This is where the "NotebookLM version" becomes the "ModernClaw version."

### Text content
- Rewritten in ModernClaw's knowledge style (distilled, focused, no filler)
- Restructured using ModernClaw's document format (Overview, Sections, Key Takeaways, Sources)
- NotebookLM's framing and branding stripped out

### Tables and structured data
- Converted to clean Markdown tables
- Or broken into scannable bullet sections if tables are too wide

### Visual content (graphs, mind maps, infographics)
Three tiers depending on what was captured:

**Tier 1 - Text-reconstructible:**
If the visual is a mind map or diagram with readable text labels, the agent extracts
the labels and rebuilds as a Markdown outline or nested list. Clean, importable,
no image dependency.

**Tier 2 - Data-driven chart:**
If the visual represents quantitative data (bar chart, line graph, etc.) and the
underlying numbers are extractable from the source text, the agent generates a
fresh version using Python (matplotlib) or a Markdown table. ModernClaw gets
clean data, not a screenshot.

**Tier 3 - Image capture:**
If the visual cannot be reconstructed (complex infographic, photograph-based),
the agent packages the screenshot as a reference file alongside the knowledge
document with a note explaining what it shows.

---

## Stage 5 - Package Output (Curator Format)

The Transformation Agent produces a standard Curator package:

```
agents/joe/curator/staged/
  \-- [topic-slug]-[timestamp]/
        |-- curated-knowledge.md    <- main knowledge file
        |-- meta.json               <- title, summary, tags, source, created_at
        \-- assets/                 <- optional: screenshots, generated charts
              \-- [image files]
```

`meta.json` for NotebookLM-sourced packages includes:
```json
{
  "title": "...",
  "summary": "...",
  "source": "NotebookLM",
  "notebook_url": "https://notebooklm.google.com/notebook/...",
  "tags": [...],
  "request_topic": "...",
  "created_at": "..."
}
```

---

## Stage 6 - Review and Import (ModernClaw Curator Inbox)

No change from the existing Curator flow. User opens ModernClaw -> Brain -> Curator
Inbox -> Refresh. Package appears. User reads the summary and approves or rejects.

---

## What Needs to Be Built

| Component | Status | Notes |
|---|---|---|
| Curator Agent (base) | Built | Watches requests/, produces packages |
| Dynamic workspace resolver | Built | Reads active agent from DB |
| Request form template | Built | In agents/joe/curator/requests/ |
| Chrome Extraction Agent | Not built | New - reads NotebookLM via browser |
| Transformation pipeline | Not built | New - converts extraction to curator format |
| NotebookLM request form fields | Not built | Need Source, Notebook URL, Output Type fields |
| Visual handling logic | Not built | Tier 1/2/3 processing for images/charts |

---

## The Extended Request Form (NotebookLM Mode)

The existing Curator Knowledge Request form needs two new optional fields to support
NotebookLM as a source:

```markdown
## Source
[web research / NotebookLM / manual]

## NotebookLM Notebook URL
[Paste the full notebook URL here if Source is NotebookLM]

## NotebookLM Output Type
[Briefing Doc / FAQ / Study Guide / Timeline / Mind Map / Chat Q&A / All]
```

When the agent sees `Source: NotebookLM`, it switches from web research mode to
Chrome extraction mode.

---

## Key Risks and Honest Limitations

**NotebookLM UI changes:** Google can update NotebookLM's interface at any time.
The Chrome agent navigates by what it sees on screen. A UI change may require
updating how the agent finds and clicks elements.

**Authentication:** The browser must already be logged into the correct Google account
with access to the notebook. The agent cannot log in for you.

**Mind map extraction accuracy:** The text labels on a NotebookLM mind map are
extractable, but the spatial relationships (which nodes connect to which) depend
on what the agent can read from the DOM vs. from a screenshot. This may be
imperfect for complex maps.

**Quantitative data reconstruction:** If a source document contains charts and
NotebookLM summarized them in prose, the agent can extract the numbers from the
prose and rebuild the chart. But if the original chart image was never described
numerically, the data cannot be reconstructed - only screenshotted.

---

## Recommended Build Order

1. Extend the request form with Source / Notebook URL / Output Type fields
2. Update the Curator Agent to detect NotebookLM mode vs. web research mode
3. Build and test the Chrome Extraction Agent on a real notebook
4. Build the Transformation layer for text content first
5. Add Tier 1 visual handling (mind map -> Markdown outline)
6. Add Tier 2 visual handling (data -> generated chart) if needed
7. End-to-end test: notebook -> staged package -> Curator Inbox approval
