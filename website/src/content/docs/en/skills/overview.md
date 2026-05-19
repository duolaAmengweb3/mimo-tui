---
title: What are Skills
description: "Composable capability packs · fully compatible with Anthropic Skills format"
---

A **skill** is a markdown file telling the mimo agent what rules to follow in specific contexts. On each conversation turn, matched skill content is injected into the system prompt automatically.

## What a skill looks like

```markdown
---
name: python-style
description: When writing Python code, always include type hints and a docstring.
triggers:
  - python
  - .py
---

Whenever you write or edit Python files:
1. Add type hints to all function signatures
2. Include a one-line docstring for every function
3. Prefer `pathlib.Path` over `os.path`
```

Drop this into `~/.mimo/skills/` and the next time you tell mimo "write a Python script", the model auto-applies these rules.

## Fully Anthropic-compatible

mimo-tui uses **exactly** the Claude Code Skills format. **Drop your `.claude/skills/*.md` in unchanged** — zero migration.

## Load order

Loaded in order, duplicates overwrite earlier ones:

| Path | Scope |
|---|---|
| `~/.mimo/skills/*.md` | Global (all projects) |
| `./.claude/skills/*.md` | Current workspace (Claude Code compat) |
| `./.mimo/skills/*.md` | Current workspace (mimo-specific) |

## Trigger matching

On the **first turn** of each conversation:

1. mimo reads your latest message
2. For each loaded skill:
   - If `triggers` is present: matches if any trigger keyword appears
   - Otherwise: fuzzy-matches words ≥4 chars from `description`
3. Matched skill bodies are appended to the system prompt

Multiple skills can match — all of them get injected.

## Next

- [Installing a skill](/en/skills/installing/)
- [Writing your own](/en/skills/writing/)
- [Built-in skills](/en/skills/built-in/)
- [Sharing with the community](/en/skills/sharing/)
