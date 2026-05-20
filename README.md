<div align="center">

<img src="./品牌/logo.svg#gh-light-mode-only" alt="mimo-tui" width="380">
<img src="./品牌/logo-dark.svg#gh-dark-mode-only" alt="mimo-tui" width="380">

**MiMo 模型的原创开源终端 AI 编程 agent**

像 [Claude Code](https://docs.claude.com/en/docs/claude-code) · [Codex CLI](https://github.com/openai/codex) 一样在终端跑 —— 但是给 MiMo 用

[![npm](https://img.shields.io/npm/v/mimo-tui?color=6366F1)](https://www.npmjs.com/package/mimo-tui)
[![License](https://img.shields.io/badge/license-MIT-EC4899)](./LICENSE)
[![Stars](https://img.shields.io/github/stars/duolaAmengweb3/mimo-tui?style=flat&color=6366F1)](https://github.com/duolaAmengweb3/mimo-tui)
[![Docs](https://img.shields.io/badge/docs-mimo--tui.pages.dev-EC4899)](https://mimo-tui.pages.dev/docs/)
[![CI](https://github.com/duolaAmengweb3/mimo-tui/actions/workflows/test.yml/badge.svg)](https://github.com/duolaAmengweb3/mimo-tui/actions/workflows/test.yml)

[English](#english) · [中文](#中文)

</div>

---

## 中文

### 是什么

跟 Claude Code、Codex CLI 一样的终端 AI 编程工具，区别是底层用小米 MiMo 模型。

**v0.3.0-alpha.2 已交付** · MIT 开源永久免费：

- ✅ 完整 **agent loop**：streaming + thinking + 工具循环 + 最多 32 步迭代
- ✅ **8 个内置工具**：`read_file` `write_file` `edit_file` `shell` `glob` `grep` `web_fetch` `todo`
- ✅ **子 agent 并行**：`task` 工具最多并行 10 个子 agent，拆长任务
- ✅ **MCP 双协议**：stdio + Streamable HTTP，官方 MCP server 装上就用
- ✅ **Skills 兼容**：直接复用你的 `.claude/skills/*.md`，三个内置 skill 开箱即用
- ✅ **三平台沙箱**：macOS Seatbelt / Linux Landlock / Windows Job Objects，默认开启
- ✅ **智能区域路由**：CN / SGP / AMS 按延迟自动挑，region key 失效自动 fallback
- ✅ **Plan / Agent / Auto 三模式**：只读 / 需审批 / 全自动三种风险偏好
- ✅ **自动 context 压缩**：接近上限时用 flash 模型总结旧消息（同 Claude Code auto-compact 思路）
- ✅ **用量看板**：SQLite 本地存储 + 缓存命中可视化
- ✅ **6 平台 binary**：macOS arm/x64 · Linux x64/arm64 gnu+musl · Windows x64
- ✅ **5 种分发**：npm / brew / cargo / docker / curl

```bash
npm install -g mimo-tui        # 装
mimo init                      # 首次配置：粘贴 MiMo API key，自动测三集群延迟
mimo                           # 进 TUI，开始干活

> 帮我把 Header.tsx 改成函数式组件
> 跑 npm test 修所有报错
> 给项目写 README
```

### 为什么用 mimo-tui 不用 Claude Code 套壳跑 MiMo

| | 套壳 Claude Code | mimo-tui |
|---|---|---|
| 模型身份 | "I'm Claude" | 原生 MiMo |
| 配置目录 | `~/.claude/` 冲突 | `~/.mimo/` 独立 |
| 合规性 | 踩 Anthropic ToS | 原创 + MIT |
| thinking 块 | 部分隐藏 | 完整暴露 |
| 缓存命中 | 不可见 | 实时显示 |
| 扩展性 | 受上游约束 | 自主演进 |

### 安装

```bash
# npm（最常用）
npm install -g mimo-tui

# Homebrew
brew install duolaAmengweb3/mimo-tui/mimo-tui

# Cargo
cargo install --git https://github.com/duolaAmengweb3/mimo-tui mimo-tui

# 一键脚本
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh

# Docker
docker run -it -v "$PWD":/workspace ghcr.io/duolaamengweb3/mimo-tui:latest
```

### 快速开始

```bash
# 1. 启动 → 进入首次配置向导
mimo init
#  → 提示去 platform.xiaomimimo.com 创建你自己的 API key
#  → 粘贴 key（wizard 会自动测延迟选 CN/SGP/AMS 最快的）
#  → 选默认模型（推荐 mimo-v2.5-pro）

# 2. 在任何项目里开始用
cd ~/code/my-project
mimo
```

详细文档：[mimo-tui.pages.dev/docs](https://mimo-tui.pages.dev/docs/)

### 同类产品对比

| 产品 | 谁做的 | 底层模型 |
|---|---|---|
| Claude Code | Anthropic 官方 | Claude Sonnet 4.6 / Opus |
| Codex CLI | OpenAI 官方 | GPT-5 系列 |
| Gemini CLI | Google 官方 | Gemini |
| DeepSeek-TUI | 第三方开源 | DeepSeek V4 |
| **mimo-tui** | **第三方开源** | **MiMo V2.5-Pro** |

### 完全免费 + MIT 开源

- 不做 Pro / 团队 / 企业版
- 不收费、不卖数据、不投广告
- 每个用户用自己的 MiMo Token Plan API key

### 联系 / 反馈

- X · [@hunterweb303](https://x.com/hunterweb303)
- Telegram · [t.me/dsa885](https://t.me/dsa885)
- Issues · [github.com/duolaAmengweb3/mimo-tui/issues](https://github.com/duolaAmengweb3/mimo-tui/issues)

### 贡献

PRs welcome！见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

### License

[MIT](./LICENSE) · "MiMo" 是小米的注册商标，本项目与小米无官方关联。

---

## English

### What is it

A native open-source terminal AI coding agent for Xiaomi's MiMo model, in the same vein as Claude Code, Codex CLI, and Gemini CLI.

**v0.3.0-alpha.2 ships** · MIT, free forever:

- ✅ Complete **agent loop** — streaming + thinking + tool loop, up to 32 iterations
- ✅ **8 built-in tools** — `read_file` `write_file` `edit_file` `shell` `glob` `grep` `web_fetch` `todo`
- ✅ **Parallel sub-agents** — `task` tool dispatches up to 10 concurrent sub-agents
- ✅ **MCP dual-protocol** — stdio + Streamable HTTP, drop in any official MCP server
- ✅ **Skills compatible** — bring your `.claude/skills/*.md` over unchanged, 3 built-ins ship inside
- ✅ **Three-platform sandbox** — macOS Seatbelt / Linux Landlock / Windows Job Objects, on by default
- ✅ **Smart region routing** — CN / SGP / AMS auto-picked by latency, falls back if key isn't valid in chosen region
- ✅ **Plan / Agent / Auto modes** — read-only / approval required / YOLO, switch on the fly
- ✅ **Auto context compaction** — past the threshold, older turns summarized by a flash model (same idea as Claude Code's auto-compact)
- ✅ **Usage dashboard** — local SQLite + live cache-hit display
- ✅ **6 platform binaries** — macOS arm/x64 · Linux x64/arm64 gnu+musl · Windows x64
- ✅ **5 distribution channels** — npm / brew / cargo / docker / curl

```bash
npm install -g mimo-tui
mimo init                       # First run → guided setup with your own API key
mimo                            # Enter the full-screen TUI

> Refactor Header.tsx to a function component
> Run npm test and fix all errors
> Write a README for this project
```

### Why use mimo-tui instead of hacking Claude Code's env vars to run MiMo

| | Claude Code env-hack | mimo-tui |
|---|---|---|
| Model identity | "I'm Claude" | Native MiMo |
| Config directory | `~/.claude/` clash | `~/.mimo/` isolated |
| Compliance | Violates Anthropic ToS | Original + MIT |
| Thinking blocks | Partially hidden | Fully visible |
| Cache hits | Invisible | Live display |
| Roadmap | Bound to upstream | Independent |

### Install

```bash
npm install -g mimo-tui
brew install duolaAmengweb3/mimo-tui/mimo-tui
cargo install --git https://github.com/duolaAmengweb3/mimo-tui mimo-tui
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh
docker run -it -v "$PWD":/workspace ghcr.io/duolaamengweb3/mimo-tui:latest
```

### How it compares

| Product | By | Model |
|---|---|---|
| Claude Code | Anthropic (official) | Claude Sonnet 4.6 / Opus |
| Codex CLI | OpenAI (official) | GPT-5 family |
| Gemini CLI | Google (official) | Gemini |
| DeepSeek-TUI | Community | DeepSeek V4 |
| **mimo-tui** | **Community** | **MiMo V2.5-Pro** |

### Free & Open Source Forever

MIT licensed. No Pro / Team / Enterprise tier. No data collection. No ads.
Each user uses their own MiMo Token Plan API key.

### Contact / Feedback

- X · [@hunterweb303](https://x.com/hunterweb303)
- Telegram · [t.me/dsa885](https://t.me/dsa885)
- Issues · [github.com/duolaAmengweb3/mimo-tui/issues](https://github.com/duolaAmengweb3/mimo-tui/issues)

### Docs

[mimo-tui.pages.dev/docs](https://mimo-tui.pages.dev/docs/)

### License

[MIT](./LICENSE) · "MiMo" is a registered trademark of Xiaomi Corporation. This project is not affiliated with Xiaomi.
