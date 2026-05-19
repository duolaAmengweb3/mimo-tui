# 04 · Rust 主项目（v1.0 完整功能）

> 目标：实现 PRD 第六节全部 15 个模块，三平台跑通
> 估时：**4–5 周**（关键路径，整个项目最长）
> 依赖：03 代码仓库

## 4.1 项目骨架

- [ ] `Cargo.toml`（workspace 配置）
- [ ] `crates/cli/`（主二进制入口）
- [ ] `crates/core/`（agent loop / 协议层 / 配置）
- [ ] `crates/tools/`（12 个工具实现）
- [ ] `crates/tui/`（ratatui 界面）
- [ ] `crates/mcp/`（MCP 客户端）
- [ ] `crates/skills/`（Skills 系统）
- [ ] `crates/sandbox/`（三平台沙箱）
- [ ] `crates/anthropic-client/`（轻量 Anthropic protocol client）
- [ ] `npm/`（npm 包装器）
- [ ] `docs/`（mdBook 内部 dev docs）
- [ ] 基础 `cargo build` / `cargo test` 跑通

## 4.2 协议层（最优先）

- [ ] Anthropic Messages API 完整实现
- [ ] streaming SSE 解析
- [ ] tool_use / tool_result 支持
- [ ] thinking 字段解析
- [ ] 缓存 usage 字段解析
- [ ] OpenAI Chat Completions fallback
- [ ] 三地区集群路由
- [ ] 自动重试 + 退避
- [ ] 429 限流自动降级

## 4.3 基础 TUI

- [ ] ratatui 主界面布局（输入框 + 输出区 + 状态栏 + 侧边栏）
- [ ] 鼠标 + 键盘双交互
- [ ] streaming 输出逐 token 渲染
- [ ] thinking 颜色区分 + 折叠展开
- [ ] 状态栏：模型 + 模式 + 集群 + token 数 + 套餐剩余
- [ ] 滚动 / 复制 / 翻页
- [ ] 暗色模式
- [ ] 冷启动 < 200ms 验证

## 4.4 Slash 命令系统

- [ ] `/help` / `/exit` / `/clear`
- [ ] `/model <name>`
- [ ] `/region <cn|sgp|ams|auto>`
- [ ] `/mode <plan|agent|auto>`
- [ ] `/usage`（打开详细看板）
- [ ] `/mcp` 子命令
- [ ] `/skill` 子命令
- [ ] `/sessions` 子命令
- [ ] `/sandbox <on|off>`
- [ ] `/config`

## 4.5 首次启动鉴权向导

- [ ] 检测 `~/.mimo/auth.json` 不存在 → 进 wizard
- [ ] 引导提示
- [ ] 输入 API key（隐藏显示）
- [ ] 自动测三集群延迟选最快
- [ ] 选默认模型
- [ ] 调一次 `/v1/messages` 验证有效
- [ ] 写入 `~/.mimo/auth.json`（权限 0600）
- [ ] 环境变量覆盖：`MIMO_API_KEY` / `MIMO_REGION` / `MIMO_MODEL`
- [ ] `mimo auth login` / `mimo auth logout` 子命令

## 4.6 12 个工具

- [ ] `read_file`（支持行号范围）
- [ ] `write_file`
- [ ] `edit_file`（基于 diff）
- [ ] `apply_patch`（兼容 Claude Code 格式）
- [ ] `shell`（带审批 / 输出截断）
- [ ] `glob`
- [ ] `grep`（用 ripgrep）
- [ ] `git`（封装常用操作）
- [ ] `web_fetch`
- [ ] `web_search`（调 MiMo 原生）
- [ ] `task`（派子 agent）
- [ ] `todo`（任务清单）

## 4.7 三种工作模式

- [ ] Plan 模式（只读 + 输出方案不动文件）
- [ ] Agent 模式（写操作前人工审批）
- [ ] Auto 模式（YOLO）
- [ ] 模式中途切换

## 4.8 子 agent 系统

- [ ] `task` 工具实现
- [ ] 最多 10 个并发限制
- [ ] 进度回传主 agent
- [ ] 子任务自动用 mimo-v2-flash 降本

## 4.9 MCP 客户端

- [ ] stdio transport
- [ ] SSE transport
- [ ] streamable HTTP transport
- [ ] 服务端 capabilities 协商
- [ ] tools / resources / prompts 三类调用
- [ ] `/mcp install <repo>` 一键装
- [ ] `/mcp list` / `/mcp toggle`
- [ ] 预置 5 个 MCP：filesystem / git / sqlite / fetch / sequential-thinking
- [ ] 兼容 Claude Code 已有 MCP server

## 4.10 Skills 系统（完全 Anthropic 兼容）

- [ ] 解析 `.claude/skills/<name>.md` + frontmatter
- [ ] 加载 / 启用 / 禁用
- [ ] `/skill install <github-repo>` 一键装
- [ ] `/skill list` / enable / disable
- [ ] 内置 skills（5 个起）：
  - [ ] `mimo-cache-optimizer`
  - [ ] `china-dev-essentials`
  - [ ] `bug-hunter`
  - [ ] `refactor-pro`
  - [ ] `doc-writer`

## 4.11 会话管理

- [ ] 自动保存到 `~/.mimo/sessions/<uuid>.json`
- [ ] `/sessions list` / resume / fork / delete
- [ ] 1M 上下文自动压缩（80% 触发）
- [ ] 关键信息保留策略

## 4.12 三平台沙箱

- [ ] macOS：Seatbelt（sandbox-exec）
- [ ] Linux：Landlock + seccomp
- [ ] Windows：Job Objects + Restricted Token
- [ ] 默认策略：工作目录可写，外部只读，shell 白名单
- [ ] `/sandbox off` 可关
- [ ] 三平台都测过

## 4.13 用量与成本看板

- [ ] 状态栏实时：本次 / 本日 / 套餐剩余
- [ ] `/usage` 详细看板：
  - [ ] 月度日度趋势图（terminal 字符图表）
  - [ ] 模型分布饼图
  - [ ] 缓存命中率
  - [ ] 节省金额估算
- [ ] 接小米套餐用量 API（如开放）
- [ ] 本地计数 fallback（如未开放）
- [ ] 用量数据存 `~/.mimo/usage.db`（sqlite）

## 4.14 多模型 fallback

- [ ] 套餐耗尽 / 限流 / 异常时自动切
- [ ] 用户配置任何 OpenAI 兼容 endpoint
- [ ] `/model fallback on|off`
- [ ] fallback 链可配多级

## 4.15 配置系统

- [ ] `~/.mimo/config.toml` 加载
- [ ] 默认值合并
- [ ] 环境变量覆盖
- [ ] `/config edit` 打开 $EDITOR
- [ ] schema 校验

## 4.16 npm 包装器

- [ ] `npm/package.json`
- [ ] postinstall 脚本下载对应平台二进制
- [ ] bin 入口指向二进制
- [ ] `npm install -g mimo-tui` 测试通过

## 4.17 错误处理与体验

- [ ] 友好的错误信息（中英双语）
- [ ] 网络错误自动重试
- [ ] panic hook 友好提示 + 收集崩溃信息
- [ ] `--debug` 打开 verbose 日志
- [ ] 日志写到 `~/.mimo/logs/`

## 验收

- [ ] `cargo build --release` 在三平台全过
- [ ] `cargo test` 全过
- [ ] 单元测试覆盖率 > 70%
- [ ] 冷启动 < 200ms
- [ ] 50 个真实 use case 验证（详见 09）
- [ ] 内存占用 < 100MB（空闲）
- [ ] 无 P0/P1 bug
