# mimo-tui

> MiMo 模型的原创开源终端 AI 编程 agent · 像 Claude Code / Codex CLI 一样在终端跑
>
> Native open-source terminal AI coding agent for Xiaomi MiMo

## 安装 / Install

```bash
npm install -g mimo-tui
```

`postinstall` 会从 GitHub Releases 自动下载对应平台的二进制：

| 平台 | 二进制 |
|---|---|
| macOS arm64 (M1+) | mimo-aarch64-apple-darwin |
| macOS x64 (Intel) | mimo-x86_64-apple-darwin |
| Linux x64 (glibc) | mimo-x86_64-unknown-linux-gnu |
| Linux arm64 | mimo-aarch64-unknown-linux-gnu |
| Windows x64 | mimo-x86_64-pc-windows-msvc |

## 用法 / Usage

```bash
mimo init     # 首次配置：API key + 集群 + 模型
mimo          # 进入交互式 REPL
mimo -p "..."  # 单次执行后退出
mimo --help    # 看所有命令
```

## 文档 / Docs

https://mimo-tui.pages.dev

## License

MIT
