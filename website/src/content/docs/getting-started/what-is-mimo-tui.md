---
title: 什么是 mimo-tui
description: "mimo-tui 是 MiMo 模型的原创开源终端 AI 编程 agent"
---

**mimo-tui** 是跑在终端里、用 MiMo 模型的 AI 编程助手。你跟它对话，它帮你读代码、改代码、跑命令、查文档、管 git——跟 Claude Code、Codex CLI、Gemini CLI 一样的工作方式，区别只是底层模型换成了小米的 MiMo。

## 为什么有这个项目

小米 MiMo（特别是 MiMo-V2.5-Pro，万亿参数 MoE，1M 上下文）是 2026 年最具性价比的国产大模型之一。它支持 Anthropic 协议、Tool Use、MCP、prompt 缓存、thinking 字段——所有 Claude Code 用的能力它都有。

但是小米官方没出对应的终端工具：

- GitHub 上 `XiaomiMiMo` 组织有 11 个仓库，**没有任何 CLI / TUI / SDK**
- 官方推荐的接入方式是"配置 Claude Code 后改环境变量套壳"——体验割裂、配置冲突、模型自我认知错乱
- 唯一现存的"MiMo 专属 TUI"项目基于逆向 Claude Code，有法律灰色风险

所以我们自己做一个。**原创、合规、开源、永远免费**。

## 跟 Claude Code 是什么关系

| 项 | Claude Code | mimo-tui |
|---|---|---|
| 谁做的 | Anthropic 官方 | 第三方独立开源 |
| 底层模型 | Claude Opus / Sonnet | MiMo-V2.5-Pro |
| 协议 | Anthropic Messages API | Anthropic Messages API ✓ |
| Tool Use | ✓ | ✓ |
| MCP | ✓ | ✓ |
| Skills | ✓ Anthropic 格式 | ✓ **完全兼容 Anthropic 格式** |
| 子 agent | ✓ | ✓ |
| 沙箱 | ✓ | ✓ |
| 配置目录 | `~/.claude/` | `~/.mimo/` |
| 价格 | Claude 套餐 | MiMo Token Plan |
| 收费 | 是 | **完全免费** |

**Skills 完全兼容**意味着你 Claude Code 里写好的 skill，可以直接拷贝到 mimo-tui 用。

## 设计原则

1. **原生体验，不套壳**——模型自我认知正确（"我是 MiMo"），所有 UI / 命令 / 文档都是 MiMo 化的
2. **MiMo 深度优化**——thinking 实时渲染、缓存命中可视化、套餐用量栏、1M 上下文管理
3. **跟主流标准并轨**——Anthropic 协议、MCP 协议、Skills 格式——不发明新格式
4. **完全开源不收费**——MIT 许可，没有 Pro / 企业版

## 适合谁用

- 已经在用 MiMo Token Plan 套餐的开发者
- 想用国产模型但不愿意配套壳的开发者
- 不放心逆向 Claude Code 项目的合规敏感用户
- 中文开发者 / 海外想试国产模型的开发者

## 下一步

- [安装](/getting-started/installation/)
- [5 分钟快速开始](/getting-started/quickstart/)
- [首次启动配置](/getting-started/first-run/)
