---
title: What is mimo-tui
description: "A native open-source terminal AI coding agent powered by Xiaomi's MiMo model"
---

**mimo-tui** is an AI coding assistant that runs in your terminal, powered by Xiaomi's MiMo model. You chat with it, and it reads your code, edits files, runs commands, fetches docs, manages git — the same workflow as Claude Code, Codex CLI, and Gemini CLI, just with a different underlying model.

## Why this project exists

Xiaomi's MiMo (especially MiMo-V2.5-Pro — a trillion-parameter MoE with 1M context) is one of the most cost-effective Chinese LLMs in 2026. It supports the Anthropic Messages protocol, tool use, MCP, prompt caching, and thinking — everything Claude Code uses.

But Xiaomi never shipped a corresponding terminal tool:

- The `XiaomiMiMo` GitHub org has 11 repositories — **zero CLI / TUI / SDK**
- The "official" path is "configure Claude Code, then override env vars" — broken UX, config conflicts, and a model that thinks it's Claude
- The only existing "MiMo TUI" project is built on reverse-engineered Claude Code — legal grey area

So we built one. **Original, compliant, open source, free forever.**

## Relationship to Claude Code

| Aspect | Claude Code | mimo-tui |
|---|---|---|
| Built by | Anthropic | Community OSS |
| Model | Claude Opus / Sonnet | MiMo-V2.5-Pro |
| Protocol | Anthropic Messages API | Anthropic Messages API ✓ |
| Tool Use | ✓ | ✓ |
| MCP | ✓ | ✓ |
| Skills | ✓ Anthropic format | ✓ **Anthropic format compatible** |
| Sub-agents | ✓ | ✓ |
| Sandbox | ✓ | ✓ |
| Config dir | `~/.claude/` | `~/.mimo/` |
| Pricing | Claude plan | MiMo Token Plan |
| Paid? | Yes | **Always free** |

**Skills are fully compatible** — you can drop your existing Claude Code skill folder into mimo-tui and use it as-is.

## Design principles

1. **Native UX, no skin-on-top.** Model identifies itself correctly ("I'm MiMo"). All UI, commands, and docs are MiMo-native.
2. **Deep MiMo optimization.** Live thinking rendering. Cache hit visualization. Plan usage bar. 1M context management.
3. **Stick to standards.** Anthropic protocol, MCP protocol, Skills format — no NIH.
4. **Free and open forever.** MIT licensed. No Pro / Enterprise tier.

## Who's this for

- Existing MiMo Token Plan subscribers who want a real CLI
- Devs who want a Chinese-model alternative but won't tolerate the env-var hack
- Compliance-sensitive users who don't trust reverse-engineered tools
- Chinese devs / international devs wanting to try Chinese LLMs

## Next

- [Installation](/en/getting-started/installation/)
- [5-minute quickstart](/en/getting-started/quickstart/)
- [First-run setup](/en/getting-started/first-run/)
