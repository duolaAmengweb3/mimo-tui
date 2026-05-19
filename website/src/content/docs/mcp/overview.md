---
title: 什么是 MCP
description: "Model Context Protocol · 让 mimo agent 用任意外部工具"
---

import { Aside } from '@astrojs/starlight/components';

**Model Context Protocol（MCP）** 是 Anthropic 推出的开放标准协议，让 AI agent 通过统一接口接入任何外部数据源或工具——文件系统、git 仓库、数据库、API、设计稿、Slack 消息……

mimo-tui v0.2 内置完整 MCP 客户端，**复用 Claude Code / Codex / Cursor 已有的 MCP server 生态**。

## 它解决什么

不用 MCP，每个新工具都得改 agent 代码加 schema。用 MCP，你只要：

1. 装一个 MCP server（npm / pip / brew 几条命令）
2. 在 `~/.mimo/mcp.json` 加一条配置
3. 重启 `mimo` —— 工具自动出现，agent 可调用

## 当前支持

| Transport | 状态 |
|---|---|
| **stdio**（子进程通信） | ✅ v0.2 完整支持 |
| streamable HTTP | ⏳ v0.3 |
| SSE | ⏳ v0.3 |

绝大多数官方 MCP servers 都用 stdio，所以 stdio 就够。

## 跟内置工具的区别

mimo 已经自带 8 个内置工具（`read_file` / `shell` / `glob` 等）。MCP 工具是**额外**的，命名空间用 `mcp__<server>__<tool>` 避免冲突：

```
read_file              ← 内置
mcp__fs__read_file     ← MCP filesystem server 提供
```

模型自己决定用哪个。一般规律：

- **内置工具**：最常见操作（读文件、跑 shell）
- **MCP 工具**：外部系统（数据库、git 高级操作、网络服务）

## 下一步

- [安装 MCP server](/mcp/installing/)
- [配置 mcp.json](/mcp/configuring/)
- [预置 server 列表](/mcp/preset-list/)
- [写自己的 MCP server](/mcp/writing-server/)
