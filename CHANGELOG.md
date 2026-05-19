# Changelog

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [SemVer](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### Planned for 1.0.0
- 完整 ratatui TUI 渲染（替换当前 rustyline REPL）
- 子 agent（并行 task 派发）
- 三平台沙箱（Seatbelt / Landlock / Job Objects）
- 多模型 fallback
- streamable-http / SSE MCP transports（v0.2 只有 stdio）

## [0.2.0-alpha.1] - 2026-05-20

**MCP + Skills + 多平台 CI + npm 跨平台安装**。

### Added
- **`mcp` crate**：完整 MCP 客户端
  - stdio transport（spawn subprocess + JSON-RPC line-delimited）
  - `initialize` 握手 + `tools/list` + `tools/call`
  - 配置文件 `~/.mimo/mcp.json` (`{ "servers": { "fs": { "command": "...", "args": [...] } } }`)
  - `core::McpHub` 自动 spawn + 把 MCP 工具注册为 `mcp__<server>__<tool>` 命名空间
- **`skills` crate**：兼容 Anthropic Skills 格式
  - YAML frontmatter + markdown body 解析
  - 关键字 / 显式 trigger 匹配
  - 从 `~/.mimo/skills/`、`./.claude/skills/`、`./.mimo/skills/` 三处加载
  - 4 个单元测试全过
- **`core::skills_bridge::load_default_skills()`**：默认 skill 加载入口
- **`core::McpHub::init()`**：spawn 所有配置好的 MCP servers + 注册其工具
- **agent loop**：每轮自动按用户输入选 skill 注入 system prompt
- **CLI 启动**：显示 `mcp servers: N · skills: M`
- **GitHub Actions**：
  - `test.yml`：三平台 cargo test + clippy + fmt
  - `release.yml`：tag → 6 平台 cross-build → release 自动上传 + sha256
  - `website.yml`：website/ 变更自动 Cloudflare Pages 部署
- **npm 包装器**：
  - `install.js`：postinstall 检测平台 → 下载二进制 → sha256 校验 → 解压
  - `bin/mimo.js`：透传 stdin/stdout 给原生二进制
  - 支持 5 个平台：darwin-arm64/x64 · linux-x64/arm64 · win32-x64

### Verified end-to-end
- Skill auto-injection works: 一个写 "always add type hints + docstrings" 的 skill
  让 mimo 写出符合规范的 Python 代码

## [0.1.0-alpha] - 2026-05-20

**首个可跑 agent**。已端到端用真实 MiMo Token Plan 验证：写文件、跑 shell、工具循环、prompt 缓存命中、用量统计全部工作。

### Added
- `anthropic-client` crate：完整 Anthropic Messages API client
- `tools` crate：8 个内置工具（read_file / write_file / edit_file / shell / glob / grep / web_fetch / todo）
- `core` crate：paths/auth/config/region/session/usage/agent
- `mimo` CLI：init wizard / REPL / 7 个 slash 命令 / one-shot 模式
- macOS x64 二进制（7.1 MB）

### Known limitations
- 当前仅 macOS x64 二进制（其他平台需 CI 编译）
- 简单 rustyline REPL（非 ratatui）

## [1.0.0] - TBD

完整生产版本。完整功能见 [PRD.md](./PRD.md)。

[Unreleased]: https://github.com/duolaAmengweb3/mimo-tui/compare/v0.2.0-alpha.1...HEAD
[0.2.0-alpha.1]: https://github.com/duolaAmengweb3/mimo-tui/releases/tag/v0.2.0-alpha.1
[0.1.0-alpha]: https://github.com/duolaAmengweb3/mimo-tui/releases/tag/v0.1.0-alpha
