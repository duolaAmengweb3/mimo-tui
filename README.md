<div align="center">

<img src="./品牌/logo.svg#gh-light-mode-only" alt="mimo-tui" width="380">
<img src="./品牌/logo-dark.svg#gh-dark-mode-only" alt="mimo-tui" width="380">

**MiMo 模型的原创开源终端 AI 编程 agent**

像 [Claude Code](https://docs.claude.com/en/docs/claude-code) · [Codex CLI](https://github.com/openai/codex) 一样在终端跑 —— 但是给 MiMo 用

[![npm](https://img.shields.io/npm/v/mimo-tui?color=6366F1)](https://www.npmjs.com/package/mimo-tui)
[![License](https://img.shields.io/badge/license-MIT-EC4899)](./LICENSE)
[![Stars](https://img.shields.io/github/stars/mimo-tui/mimo-tui?style=flat&color=6366F1)](https://github.com/mimo-tui/mimo-tui)
[![Docs](https://img.shields.io/badge/docs-mimo--tui.pages.dev-EC4899)](https://mimo-tui.pages.dev)

[English](#english) · [中文](#中文)

</div>

---

## 中文

### 是什么

跟 Claude Code、Codex CLI 一样的终端 AI 编程工具，区别是底层用小米 MiMo 模型。

```bash
npm install -g mimo-tui     # 装
mimo                        # 首次启动 → 引导你配置自己的 API key
cd ~/code/my-project        # 进项目
mimo                        # 开始用

> 帮我把 Header.tsx 改成函数式组件
> 跑 npm test 修所有报错
> 给项目写 README
```

### 为什么用 mimo-tui 不用 Claude Code

- **省钱**：MiMo Token Plan 比 Claude Opus 便宜得多
- **原生体验**：不用配 Claude Code 然后改环境变量套壳
- **看得见用量**：实时知道套餐还剩多少 token、本次缓存命中省了多少
- **看得见 thinking**：MiMo 的推理过程实时渲染
- **配置独立**：跟 Claude Code 并存（`~/.mimo/` vs `~/.claude/`），互不污染

### 安装

```bash
# npm
npm install -g mimo-tui

# Homebrew
brew install mimo-tui/mimo-tui/mimo-tui

# Cargo
cargo install mimo-tui

# 一键脚本
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh

# Docker
docker run -it ghcr.io/mimo-tui/mimo-tui:latest
```

### 快速开始

```bash
# 1. 启动 → 自动进入首次配置向导
mimo
#  → 提示去 platform.xiaomimimo.com 创建你自己的 API key
#  → 粘贴 key
#  → 自动测延迟选最快集群（CN / SGP / AMS）
#  → 选默认模型（推荐 mimo-v2.5-pro）

# 2. 在任何项目里开始用
cd ~/code/my-project
mimo
```

详细文档：[mimo-tui.pages.dev/docs](https://mimo-tui.pages.dev/docs)

### 跟同类产品的对应关系

| 产品 | 谁做的 | 底层模型 |
|---|---|---|
| Claude Code | Anthropic 官方 | Claude Opus/Sonnet |
| Codex CLI | OpenAI 官方 | GPT-5 系列 |
| Gemini CLI | Google 官方 | Gemini |
| DeepSeek-TUI | 第三方个人 | DeepSeek V4 |
| **mimo-tui** | **第三方开源** | **MiMo V2.5-Pro** |

### 完全免费 + MIT 开源

- 不做 Pro / 团队 / 企业版
- 不收费、不卖数据、不投广告
- 每个用户用自己的 MiMo Token Plan API key

### 贡献

PRs welcome！见 [CONTRIBUTING.md](./CONTRIBUTING.md)。

### License

[MIT](./LICENSE) · "MiMo" 是小米的注册商标，本项目与小米无官方关联。

---

## English

### What is it

A native open-source terminal AI coding agent for Xiaomi's MiMo model, in the same vein as Claude Code, Codex CLI, and Gemini CLI.

```bash
npm install -g mimo-tui
mimo                    # First run → guided setup with your own API key
cd ~/code/my-project
mimo

> Refactor Header.tsx to a function component
> Run npm test and fix all errors
> Write a README for this project
```

### Why use mimo-tui instead of Claude Code

- **Cheaper**: MiMo Token Plan is significantly cheaper than Claude Opus
- **Native UX**: No need to hack Claude Code with env vars
- **Visible usage**: Real-time view of token consumption + cache hits
- **Visible thinking**: MiMo's reasoning rendered in real time
- **Isolated config**: Lives in `~/.mimo/`, coexists with Claude Code

### Install

```bash
npm install -g mimo-tui
brew install mimo-tui/mimo-tui/mimo-tui
cargo install mimo-tui
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh
```

### Free & Open Source Forever

MIT licensed. No Pro / Team / Enterprise tier. No data collection. No ads.
Each user uses their own MiMo Token Plan API key.

### Docs

[mimo-tui.pages.dev](https://mimo-tui.pages.dev)

### License

[MIT](./LICENSE) · "MiMo" is a registered trademark of Xiaomi Corporation. This project is not affiliated with Xiaomi.
