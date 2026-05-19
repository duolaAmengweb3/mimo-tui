---
title: 安装 MCP Server
description: "把 MCP server 装到 mimo-tui 里，5 分钟"
---

import { Steps, Aside } from '@astrojs/starlight/components';

<Steps>

1. **挑一个 MCP server**

   官方列表：https://github.com/modelcontextprotocol/servers

   常用的：
   - `@modelcontextprotocol/server-filesystem` — 文件系统访问
   - `mcp-server-git` — git 高级操作
   - `mcp-server-fetch` — 抓网页
   - `mcp-server-sqlite` — SQL 查询

2. **拿到启动命令**

   MCP server 通常通过 `npx` / `uvx` 启动：

   ```bash
   # filesystem server
   npx -y @modelcontextprotocol/server-filesystem /path/to/dir

   # git server（需要 uv）
   uvx mcp-server-git --repository /path/to/repo
   ```

3. **改 `~/.mimo/mcp.json`**

   ```json
   {
     "servers": {
       "fs": {
         "command": "npx",
         "args": ["-y", "@modelcontextprotocol/server-filesystem", "/Users/you/code"]
       },
       "git": {
         "command": "uvx",
         "args": ["mcp-server-git", "--repository", "/Users/you/code/my-project"]
       }
     }
   }
   ```

4. **重启 mimo**

   ```bash
   mimo
   # 启动时会显示：mcp servers: 2 · skills: ...
   ```

   工具会以 `mcp__fs__*` / `mcp__git__*` 命名空间出现。

</Steps>

## 验证

启动后在 REPL 里：

```
> list available MCP tools and pick one to read /path/to/file
```

mimo 会枚举它看到的 MCP 工具，挑一个用。

## 报错排查

| 现象 | 原因 |
|---|---|
| 启动时 `mcp servers: 0` 但配置写了 | mcp.json 语法错 / command 找不到 |
| `mcp init warning: ...` | spawn 失败，看 `~/.mimo/logs/` |
| MCP 工具调用 `[mcp error]` | server 内部错误，看 stderr |
| `expected string, received undefined` | mimo 版本太老，升级到 ≥ v0.2.0-alpha.2 |

## 完整示例

仓库里 [examples/mcp/](https://github.com/duolaAmengweb3/mimo-tui/tree/main/examples/mcp) 有 4 个常用 server 的现成配置。
