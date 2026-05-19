---
title: 常见问题（FAQ）
description: 20+ 个最常被问的问题
---

## 基础

### 这跟 Claude Code 是什么关系？

**完全独立的项目**，不基于 Claude Code 源码或任何逆向，从零用 Rust 写。只是用了同样的协议（Anthropic Messages API）和 UX 范式。

可以同时装 mimo-tui 和 Claude Code，配置完全隔离（`~/.mimo/` vs `~/.claude/`），互不影响。

### 这跟 XIAOMI-MiMo-code 是什么关系？

那是另一个独立项目，基于"逆向 Claude Code"做的（CCB），有法律灰色风险。

mimo-tui 是**原创架构**，用公开的 Anthropic 协议 + 公开的 SDK，没用任何 Claude Code 的代码。

### 这跟小米官方是什么关系？

**无任何关联**。mimo-tui 是社区开源项目。"MiMo" 是小米的注册商标，本项目只是用了小米开放的 API。

### 收费吗？

**永远免费**。MIT 开源，没有 Pro / 团队 / 企业版。不收费也不卖数据。

## 鉴权 / API

### 用谁的 API key？

**用你自己的**。去 [platform.xiaomimimo.com](https://platform.xiaomimimo.com) 注册账号，拿自己的 `tp-` token。

详见 [拿 API Key](/auth/getting-api-key/)。

### 我的 API key 安全吗？

API key 只存你电脑本地（`~/.mimo/auth.json` 0600 权限），每次请求只发给小米的 `token-plan-*.xiaomimimo.com`，不发给我们或任何第三方。

### 套餐用完了怎么办？

两个选项：
1. 去 platform.xiaomimimo.com 充值续费
2. 配置 [多模型 fallback](/fallback/overview/)，自动切到 DeepSeek / Qwen 等其他兼容 endpoint

### 支持 OpenAI 协议吗？

主力协议是 Anthropic（接 Token Plan），OpenAI 协议作为 fallback——可以用任何 OpenAI 兼容 endpoint 当备用。

## 功能

### 支持哪些模型？

- `mimo-v2.5-pro`（推荐，万亿 MoE 编程主力）
- `mimo-v2.5`
- `mimo-v2-flash`（快速档）
- `mimo-v2-omni`（多模态，看图）

### 支持 MCP 吗？

**完整支持**。stdio / SSE / HTTP 三种 transport 都有。详见 [MCP 文档](/mcp/overview/)。

可以直接用 Claude Code / Codex / Cursor 已有的 MCP server。

### Skills 怎么用？

**完全兼容 Anthropic Skills 格式**。你 Claude Code 用的 skill 文件夹（`.claude/skills/<name>.md`）可以直接拷到 mimo-tui 用。

详见 [Skills 文档](/skills/overview/)。

### 有沙箱保护吗？

有。三平台原生方案：
- macOS：Seatbelt（sandbox-exec）
- Linux：Landlock + seccomp
- Windows：Job Objects + Restricted Token

默认工作目录可读写、外部只读、shell 命令白名单。`/sandbox off` 可关闭（不推荐）。

### 支持哪些操作系统？

macOS（arm64 + x64）、Linux（x64 + arm64）、Windows（x64）。

## 跟 Claude Code 对比

### 为啥不直接用 Claude Code 改 env 套壳？

可以这样用，但体验割裂：
- 模型自我认知错乱（"我是 Claude"）
- 错误信息全是 Anthropic 术语
- 配置目录冲突（一会儿想用 Claude 一会儿用 MiMo）
- 看不到 MiMo 的 thinking 和缓存数据

mimo-tui 是**原生 MiMo 体验**：所有 UI / 命令 / 日志都是 MiMo 化的。

### Claude Code 的 skill / MCP 能用吗？

能。Skills 和 MCP 协议完全兼容，直接拷过来就用。

### 性能怎么样？

mimo-tui 用 Rust 写，单二进制（5-10MB），冷启动 < 200ms。比 TypeScript / Node 写的客户端轻得多。

## 故障 / 维护

### 卡住 / 没响应怎么办

```bash
# 看是不是网络问题
ping token-plan-sgp.xiaomimimo.com

# 切个集群试试
mimo
> /region cn  # 或 ams
```

详见 [故障排查](/troubleshooting/connection/)。

### 怎么报 bug

去 [GitHub Issues](https://github.com/duolaAmengweb3/mimo-tui/issues/new/choose)，**注意删掉日志里的 API key**。

### 想贡献代码怎么搞

看 [CONTRIBUTING.md](https://github.com/duolaAmengweb3/mimo-tui/blob/main/CONTRIBUTING.md)。欢迎一切贡献。

## 关于项目

### 维护者是谁

社区维护。主要 commit 在 [GitHub 仓库](https://github.com/duolaAmengweb3/mimo-tui)。

### 会不会被小米官方收编

希望会（参考字节收购 Trae 模式）。但即使没有也会作为开源项目持续维护。

### 会做 mimo-tui Pro 收费版吗

**不会**。这是个人开源项目，目标不是赚钱。

### 用什么协议

MIT 开源。
