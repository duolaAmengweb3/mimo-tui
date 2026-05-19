---
title: 预置 MCP server 列表
description: "5 个推荐 MCP server · 都是 stdio transport · 已经在 mimo-tui 上验证过"
---

import { Aside } from '@astrojs/starlight/components';

下面 5 个是常用、稳定、跟 mimo-tui 100% 兼容的官方 / 准官方 MCP server。完整配置直接拷贝。

## 1. filesystem · 文件系统

让 agent 可以读写**指定目录之外**的文件（mimo 内置工具默认只能动 workspace 内）。

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

提供 13 个工具：`read_text_file` / `write_file` / `edit_file` / `move_file` / `directory_tree` / 等。

## 2. git · Git 高级操作

让 agent 能跑 `git diff` / `git log` / `git blame` 等内置 shell 工具不方便做的操作。

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

> 需要 [`uv`](https://github.com/astral-sh/uv)：`pip install uv` 或 `brew install uv`

## 3. fetch · 抓网页（含 JS 渲染）

跟内置 `web_fetch` 类似，但走更专业的 server（处理 cookie / redirects 更好）。

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

## 4. sqlite · SQL 查询

让 agent 能跑 SQL 查 / 改 SQLite 数据库。

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

## 5. sequential-thinking · 推理框架

让 agent 把复杂任务用结构化"先想后做"的方式拆解。

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

## 全部混在一起

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

启动 mimo 后会看到：

```
  mcp servers: 5 · skills: ...
```

Agent 立即有了 ~30 个 MCP 工具可用，加上 mimo 自带的 8 个，总共 38 个工具。

## 更多 servers

- 官方 servers：https://github.com/modelcontextprotocol/servers
- 社区聚合：https://mcp.so · https://glama.ai/mcp

## 排错

| 现象 | 处理 |
|---|---|
| `mcp servers: 0` 但你配置了 | `cat ~/.mimo/mcp.json | jq` 验证 JSON 没语法错 |
| `expected string, received undefined` 错误 | 升级 mimo 到 ≥ v0.2.0-alpha.3，camelCase 修复了 |
| `mcp tool error` 但不知道为啥 | `mimo --debug` 看 stderr 输出，或看 `~/.mimo/logs/` |
| 启动后 mimo 卡住 | 可能 npx/uvx 在下载包，第一次等几秒 |
