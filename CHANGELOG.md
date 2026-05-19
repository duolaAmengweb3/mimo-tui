# Changelog

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [SemVer](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### Planned for 1.0.0
- 完整 ratatui TUI 渲染（替换当前 rustyline REPL）
- MCP 客户端 + 内置 5 个 MCP server
- Skills 系统（兼容 Anthropic Skills 格式）
- 子 agent（并行 task 派发）
- 三平台沙箱（Seatbelt / Landlock / Job Objects）
- 多模型 fallback
- npm 包一键安装（含三平台二进制）

## [0.1.0-alpha] - 2026-05-20

**首个可跑 agent**。已端到端用真实 MiMo Token Plan 验证：写文件、跑 shell、工具循环、prompt 缓存命中、用量统计全部工作。

### Added
- **`anthropic-client` crate**：完整 Anthropic Messages API client（types / streaming SSE / tool_use / thinking / prompt 缓存）+ 10 个测试全过
- **`tools` crate**：8 个内置工具
  - `read_file` `write_file` `edit_file` `shell` `glob` `grep` `web_fetch` `todo`
  - ApprovalMode（Plan/Agent/Auto）+ Approver trait
- **`core` crate**：
  - `~/.mimo/` 目录布局（auth.json / config.toml / sessions / usage.db）
  - 三集群（CN/SGP/AMS）延迟测量 + 自动选最快
  - Agent 主循环（system prompt 中英 / tool roundtrip / 32 步上限）
  - 会话持久化、用量 SQLite 记录
- **`mimo` CLI**：
  - 首次启动配置向导
  - REPL（rustyline，含历史、Ctrl+D 退出）
  - 7 个 slash 命令（`/help` `/model` `/mode` `/region` `/usage` `/clear` `/exit`）
  - One-shot 模式（`mimo -p "..."`）
- macOS x64 二进制（7.1 MB，stripped）

### Known limitations
- 当前仅 macOS x64 二进制（其他平台需 CI 编译）
- TUI 是简单 rustyline，不是完整 ratatui（v1.0 升级）
- MCP / Skills / 沙箱 / 子 agent 是占位（v1.0 实现）

## [1.0.0] - TBD

完整生产版本。完整功能见 [PRD.md](./PRD.md)。

[Unreleased]: https://github.com/duolaAmengweb3/mimo-tui/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/duolaAmengweb3/mimo-tui/releases/tag/v0.1.0-alpha
