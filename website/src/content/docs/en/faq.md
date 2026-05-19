---
title: FAQ
description: "20+ most-asked questions"
---

## Basics

### How is this related to Claude Code?

**It's a fully independent project** — not based on Claude Code's source code, no reverse engineering. Written from scratch in Rust. We just use the same protocol (Anthropic Messages API) and UX patterns.

You can install both mimo-tui and Claude Code at the same time. Configs are fully isolated (`~/.mimo/` vs `~/.claude/`) and don't interfere.

### How is this related to XIAOMI-MiMo-code?

That's a separate independent project, built on reverse-engineered Claude Code (CCB) — legal grey area.

mimo-tui is **original architecture**, using the public Anthropic protocol + public SDK shape. None of Claude Code's code is used.

### What's the relationship to Xiaomi (official)?

**Zero affiliation**. mimo-tui is a community OSS project. "MiMo" is a Xiaomi registered trademark — this project just uses Xiaomi's open API.

### Is it free?

**Free forever**. MIT licensed. No Pro / Team / Enterprise tier. No data collection, no ads.

## Auth / API

### Whose API key does it use?

**Yours.** Sign up at [platform.xiaomimimo.com](https://platform.xiaomimimo.com) and get your own `tp-` token.

See [Getting API Key](/en/auth/getting-api-key/).

### Is my API key safe?

API key only lives on your machine (`~/.mimo/auth.json` mode 0600). Each request goes only to Xiaomi's `token-plan-*.xiaomimimo.com` — never to us or any third party.

### What if my plan runs out?

Two options:
1. Top up at platform.xiaomimimo.com
2. Configure [multi-model fallback](/en/fallback/overview/) — auto-switch to DeepSeek / Qwen / any OpenAI-compatible endpoint

### Does it support OpenAI protocol?

Primary protocol is Anthropic (for Token Plan). OpenAI protocol is supported as fallback — any OpenAI-compatible endpoint can serve as backup.

## Features

### Which models are supported?

- `mimo-v2.5-pro` (recommended, trillion-param MoE, primary coding model)
- `mimo-v2.5`
- `mimo-v2-flash` (fast tier)
- `mimo-v2-omni` (multimodal / vision)

### Is MCP supported?

**Fully.** stdio / SSE / HTTP transports all work. See [MCP docs](/en/mcp/overview/).

You can reuse any MCP server already written for Claude Code / Codex / Cursor.

### How do I use Skills?

**Anthropic Skills format is fully compatible.** Drop your existing Claude Code skill folder (`.claude/skills/<name>.md`) into mimo-tui as-is.

See [Skills docs](/en/skills/overview/).

### Is there sandbox protection?

Yes. Native per-platform:
- macOS: Seatbelt (sandbox-exec)
- Linux: Landlock + seccomp
- Windows: Job Objects + Restricted Token

Default: working dir writable, outside read-only, shell commands allowlisted. `/sandbox off` to disable (not recommended).

### Which OSes?

macOS (arm64 + x64), Linux (x64 + arm64), Windows (x64).

## vs Claude Code

### Why not just use Claude Code with env-var hack?

You can, but the UX breaks:
- Model thinks it's Claude ("I'm Claude")
- Error messages full of Anthropic terminology
- Config conflicts when switching back and forth
- MiMo's thinking and cache data aren't shown

mimo-tui is **MiMo-native**: all UI, commands, and logs reflect MiMo properly.

### Can I use my Claude Code skills / MCPs?

Yes. Skills and MCP protocol are fully compatible. Just copy them over.

### Performance?

mimo-tui is Rust. Single binary (5–10MB). Cold start < 200ms. Much lighter than TypeScript / Node clients.

## Troubleshooting

### It hangs / no response

```bash
# Network check
ping token-plan-sgp.xiaomimimo.com

# Try another region
mimo
> /region cn  # or ams
```

See [Connection troubleshooting](/en/troubleshooting/connection/).

### How do I report a bug?

[GitHub Issues](https://github.com/duolaAmengweb3/mimo-tui/issues/new/choose). **Make sure to scrub API keys from logs first.**

### How do I contribute?

See [CONTRIBUTING.md](https://github.com/duolaAmengweb3/mimo-tui/blob/main/CONTRIBUTING.md). All contributions welcome.

## About the project

### Who maintains it?

Community maintained. All commits in the [GitHub repo](https://github.com/duolaAmengweb3/mimo-tui).

### Will Xiaomi adopt it officially?

We hope so (cf. ByteDance acquiring Trae). But it stays open and maintained either way.

### Will there be a mimo-tui Pro?

**No.** This is an individual OSS project. Not chasing revenue.

### What's the license?

MIT.
