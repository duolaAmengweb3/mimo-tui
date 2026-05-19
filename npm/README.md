# mimo-tui

> MiMo 模型的原创开源终端 AI 编程 agent · 像 [Claude Code](https://docs.claude.com/en/docs/claude-code) / [Codex CLI](https://github.com/openai/codex) 一样在终端跑
>
> Native open-source terminal AI coding agent for Xiaomi MiMo · MIT, no paid tier

## 安装 / Install

```bash
npm install -g mimo-tui
```

postinstall 自动从 GitHub Releases 下载对应平台二进制：

| Platform | Binary |
|---|---|
| macOS arm64 (M1+) | `mimo-aarch64-apple-darwin` |
| Linux x64 (glibc) | `mimo-x86_64-unknown-linux-gnu` |
| Linux arm64 | `mimo-aarch64-unknown-linux-gnu` |
| Windows x64 | `mimo-x86_64-pc-windows-msvc` |

## 快速开始 / Quickstart

```bash
mimo init      # 首次配置：API key + 集群 + 默认模型
mimo           # 进入交互式 REPL
mimo -p "..."  # 单次执行后退出
mimo --help
```

首次启动会引导你去 [platform.xiaomimimo.com](https://platform.xiaomimimo.com) 拿你自己的 API key。

## v0.2 能力 / Features

- ✅ 完整 agent loop：thinking 实时渲染 + 工具调用 + prompt 缓存
- ✅ **streaming SSE**：模型回复逐 token 显示，不阻塞
- ✅ 8 个内置工具：`read_file` `write_file` `edit_file` `shell` `glob` `grep` `web_fetch` `todo`
- ✅ **MCP 客户端**：装任意标准 MCP server，工具自动注册到 agent
- ✅ **Skills 系统**：完全兼容 Anthropic Skills 格式，可直接复用 `.claude/skills/`
- ✅ 三集群（CN/SGP/AMS）自动选最快
- ✅ Plan / Agent / Auto 三种审批模式
- ✅ 本地用量看板（SQLite）

## 完整文档 / Docs

https://mimo-tui.pages.dev

## 反馈 / Issues

https://github.com/duolaAmengweb3/mimo-tui/issues

## License

MIT · "MiMo" 是小米注册商标，本项目与小米无官方关联。
