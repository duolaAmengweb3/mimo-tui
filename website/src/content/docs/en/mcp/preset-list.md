---
title: Preset MCP server list
description: "5 recommended MCP servers · all stdio transport · verified with mimo-tui"
---

These 5 are popular, stable, and 100% compatible with mimo-tui. Copy-paste configs.

## 1. filesystem

Lets the agent read/write files **outside** your workspace (mimo's built-in tools are scoped to the workspace by default).

```json
{
  "servers": {
    "fs": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/you/Documents"]
    }
  }
}
```

Provides 13 tools: `read_text_file` / `write_file` / `edit_file` / `move_file` / `directory_tree` / etc.

## 2. git

Lets the agent run `git diff` / `git log` / `git blame` and other operations that are awkward via shell.

```json
{
  "servers": {
    "git": {
      "command": "uvx",
      "args": ["mcp-server-git", "--repository", "/Users/you/code/my-project"]
    }
  }
}
```

> Needs [`uv`](https://github.com/astral-sh/uv): `pip install uv` or `brew install uv`

## 3. fetch

Similar to the built-in `web_fetch` but handles cookies / redirects better.

```json
{
  "servers": {
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    }
  }
}
```

## 4. sqlite

Let the agent run SQL against a SQLite database.

```json
{
  "servers": {
    "sqlite": {
      "command": "uvx",
      "args": ["mcp-server-sqlite", "--db-path", "/Users/you/data/app.db"]
    }
  }
}
```

## 5. sequential-thinking

A structured "plan-then-execute" framework for complex tasks.

```json
{
  "servers": {
    "thinking": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"]
    }
  }
}
```

## All five at once

```json
{
  "servers": {
    "fs":       { "command": "npx",  "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/you/Documents"] },
    "git":      { "command": "uvx",  "args": ["mcp-server-git", "--repository", "/Users/you/code"] },
    "fetch":    { "command": "uvx",  "args": ["mcp-server-fetch"] },
    "sqlite":   { "command": "uvx",  "args": ["mcp-server-sqlite", "--db-path", "/Users/you/data/app.db"] },
    "thinking": { "command": "npx",  "args": ["-y", "@modelcontextprotocol/server-sequential-thinking"] }
  }
}
```

Start mimo and you'll see:

```
  mcp servers: 5 · skills: ...
```

The agent immediately has ~30 MCP tools + the 8 built-in = 38 total.

## More servers

- Official: https://github.com/modelcontextprotocol/servers
- Community indexes: https://mcp.so · https://glama.ai/mcp

## Troubleshooting

| Symptom | Fix |
|---|---|
| `mcp servers: 0` despite config | `cat ~/.mimo/mcp.json | jq` to validate JSON |
| `expected string, received undefined` | Upgrade mimo to ≥ v0.2.0-alpha.3 (camelCase fix) |
| `mcp tool error` opaque | `mimo --debug` to see stderr, or check `~/.mimo/logs/` |
| mimo hangs on startup | npx/uvx might be downloading packages first time, wait |
