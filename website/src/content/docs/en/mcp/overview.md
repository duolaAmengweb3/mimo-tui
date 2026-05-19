---
title: What is MCP
description: "Model Context Protocol lets the mimo agent use any external tool"
---

**Model Context Protocol (MCP)** is an open protocol from Anthropic that gives AI agents a uniform interface to **any** external data source or tool — filesystems, git repos, databases, APIs, design files, Slack messages...

mimo-tui v0.2 ships a complete MCP client, so you can **reuse any MCP server already written for Claude Code / Codex / Cursor**.

## Why it matters

Without MCP, every new integration means writing agent code with bespoke schemas. With MCP:

1. Install an MCP server (npm / pip / brew)
2. Add one entry to `~/.mimo/mcp.json`
3. Restart `mimo` — its tools appear, the agent can call them

## Current support

| Transport | Status |
|---|---|
| **stdio** (subprocess) | ✅ v0.2 |
| streamable HTTP | ⏳ v0.3 |
| SSE | ⏳ v0.3 |

The vast majority of official MCP servers use stdio, so stdio is enough.

## Native tools vs MCP tools

mimo ships 8 native tools (`read_file` / `shell` / `glob` / etc.). MCP tools are **additional**, namespaced as `mcp__<server>__<tool>` to avoid clashes:

```
read_file              ← built-in
mcp__fs__read_file     ← from MCP filesystem server
```

The model picks. Rule of thumb:

- **Built-ins**: common operations (read files, run shell)
- **MCP**: external systems (databases, advanced git, network services)

## Next

- [Install an MCP server](/en/mcp/installing/)
- [Configure mcp.json](/en/mcp/configuring/)
- [Preset server list](/en/mcp/preset-list/)
- [Write your own](/en/mcp/writing-server/)
