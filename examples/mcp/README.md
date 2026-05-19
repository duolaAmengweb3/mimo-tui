# MCP 配置示例 · MCP config examples

> mimo-tui 读 `~/.mimo/mcp.json` 启动你声明的 MCP servers，并把它们的工具以 `mcp__<server>__<tool>` 命名空间注册到 agent。

## 安装方法 / Install

把这个目录里的 `mcp.json` 复制到 `~/.mimo/mcp.json`，按需修改路径：

```bash
cp examples/mcp/mcp.json ~/.mimo/mcp.json
$EDITOR ~/.mimo/mcp.json   # 改路径
mimo                       # 启动后会显示 "mcp servers: 4 · skills: ..."
```

## 4 个示例 server

| Server | 装法 / Install | 用途 |
|---|---|---|
| `fs` | `npx -y @modelcontextprotocol/server-filesystem` | 允许 agent 读/写指定目录外的文件 |
| `git` | `uvx mcp-server-git` | git 高级操作（diff / blame / log） |
| `fetch` | `uvx mcp-server-fetch` | 抓网页（替代/补充内置 `web_fetch`） |
| `sqlite` | `uvx mcp-server-sqlite` | 让 agent 跑 SQL 查询 |

> `uvx` 来自 [`uv`](https://github.com/astral-sh/uv) — `pip install uv` 装。
> `npx` 自带 Node.js 18+。

## 写自己的

JSON schema：

```json
{
  "servers": {
    "<your-server-name>": {
      "command": "...",
      "args": ["..."],
      "env": { "KEY": "value" }
    }
  }
}
```

- `command`：可执行文件（PATH 里能找到，或绝对路径）
- `args`：命令行参数数组
- `env`：传给子进程的额外环境变量（可选）

只支持 **stdio transport**。`streamable-http` / `sse` 等 v0.3 加。

## 完整 server 列表

社区维护的 MCP servers 索引：
- 官方：https://github.com/modelcontextprotocol/servers
- 中文聚合：https://mcp.so
