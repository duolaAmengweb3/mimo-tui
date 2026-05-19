# MiMo-TUI · 产品需求文档（PRD）

> 版本：v1.0 定稿
> 时间：2026-05-19
> 立项依据：知识库 04-产品机会推荐清单 · S 级 #1（模型专属 TUI 系列）

## 已锁定决策

| 项 | 决策 |
|---|---|
| **产品名** | `mimo-tui` |
| **域名** | `mimo-tui.dev` |
| **许可证** | MIT（完全开源） |
| **商业模式** | **不收费、纯开源**，不做 Pro / 团队 / 企业版 |
| **交付模式** | **一次性产品级 v1.0 完工**，不分 MVP / Beta 阶段 |
| **Skills** | **完全兼容 Anthropic Skills 格式**，可直接复用 Claude Code skill |
| **商标** | 不考虑（开源项目，用 MIT 许可证规避风险） |
| **API key** | 用户自有 Token Plan key，已可实测 |

---

## 一、一句话定义

**MiMo-TUI** 是给小米 MiMo 模型用户做的、跑在终端里的、原创开源的 AI 编程 agent —— 让 MiMo Token Plan 用户不用再套壳 Claude Code，也能拿到原生级体验。

---

## 二、立项依据

### 为什么是 MiMo

1. **模型流量真实**：MiMo-V2-Pro 在 OpenRouter 2026-04 拿月度第一，22.3% 份额、4.65T tokens
2. **价格屠夫**：mimo-v2.5-pro 号称比 Claude Opus 省 40-60% token，万亿 MoE + 1M 上下文
3. **官方零工具**：XiaomiMiMo GitHub 11 个仓库**没有任何 CLI/TUI/SDK**（已确认）
4. **用户付费基础已经在**：Token Plan 是订阅制，用户对工具体验敏感（套餐用量、ROI）
5. **API 协议友好**：Anthropic 协议接入意味着可以复用 Anthropic SDK 生态

### 为什么是现在

- 直接竞品 [XIAOMI-MiMo-code](https://github.com/Gurabit77/XIAOMI-MiMo-code) 只有 25 stars 且**基于逆向 CCB**，法律灰色 + 推广乏力 → 心智位未真正被占
- DeepSeek-TUI（32K stars）已经验证了"模型专属 TUI"这个范式
- 小米官方下场做工具概率低（其历史更重 ToC、ToD 弱）
- 窗口期 **6–12 个月**

### 为什么是我们

- 走**原创架构 + Rust 单二进制**（区别于竞品的 TS + 逆向）
- 走 **MiMo 深度优化**（thinking 可视化 / 缓存命中 / 套餐用量 / 1M 上下文）
- 走 **矩阵化思维**：同代码库未来复刻给 Hy3-TUI / GLM-TUI / Kimi-TUI

---

## 三、产品定位

### 三个差异化锚点

**1. 原生体验 vs 套壳**
- 不是把 Claude Code 改个 prompt 就叫 MiMo 版
- 模型自我认知正确（"我是 MiMo"）
- 命令、文档、UI 全部 MiMo 化
- 配置独立（`~/.mimo/`），不污染 Claude Code

**2. MiMo 深度优化 vs 通用适配**
- **Thinking 实时流**：MiMo 返回的 `thinking` 字段直接渲染（不是 streaming 黑盒）
- **缓存命中可视化**：`cache_read_input_tokens` 实时显示"省了多少"
- **套餐用量栏**：剩余 token 实时显示，月度/日度趋势图
- **1M 上下文管理器**：可视化已用 / 剩余，超出自动压缩
- **地区集群切换**：CN / SGP / AMS 一键切，自动按延迟选择

**3. 合规可信 vs 逆向风险**
- 完全原创，不基于任何 Claude Code 逆向
- 用 Anthropic 公开协议 + 公开 SDK
- 用 MIT / Apache-2.0 许可证开源

### 不做什么（边界明确）

- 不做 IDE 集成（让位给 Cursor / 通义灵码这类专业工具）
- 不做 Web 桌面客户端（让位给 Cherry Studio / LobeChat）
- 不做模型聚合（不抢 OpenCode / CC Switch 的位）
- 不做 MiMo 之外的模型为主（但留 fallback 入口）

---

## 四、目标用户

### 核心用户：MiMo Token Plan 重度用户

特征：
- 已经买了 MiMo 套餐（¥583+ /月）或在用免费 16 亿 token
- 当前用 Claude Code + 改 env vars 的方式套壳，体验割裂
- 对**套餐用量、ROI、缓存命中**敏感
- 重度使用 coding agent（每天 10 次以上）

### 次要用户：

1. **国产 AI 工具关注者**：想用国产模型但不想配 Claude Code 的开发者
2. **合规敏感用户**：不放心逆向 Claude Code 类项目的企业开发者
3. **DeepSeek-TUI 用户的延伸**：已经接受"模型专属 TUI"理念的开发者

### 用户规模估算

- MiMo Token Plan 订阅用户：小米未公开，按 DeepSeek-TUI 早期类比，**保守 1-5 万人**
- 第三方 MiMo 工具使用者（含套壳 Claude Code）：**5-20 万**

---

## 五、核心价值

### 用户痛点（按强度排序）

1. **套壳 Claude Code 体验割裂**：模型自我认知错乱、错误信息引用 Claude、命令文档全是 Anthropic 术语
2. **配置冲突**：`~/.claude/settings.json` 改成 MiMo 后再切回 Claude 就麻烦
3. **看不到 thinking 和缓存**：Claude Code UI 没有 MiMo thinking 字段的展示
4. **套餐用量不透明**：不知道这次回复花了多少 token、套餐还剩多少
5. **逆向项目不放心**：XIAOMI-MiMo-code 法律灰色，怕 Anthropic 发 DMCA

### 我们的解决方案

| 痛点 | 解决方案 |
|---|---|
| 套壳体验割裂 | 原生 MiMo TUI，所有 UI/命令/文档 MiMo 化 |
| 配置冲突 | 完全独立配置目录 `~/.mimo/`、缓存独立、socket 独立 |
| Thinking / 缓存不可见 | 推理流实时渲染 + 缓存命中浮窗 |
| 套餐用量不透明 | 实时用量栏 + 月度日度趋势 + 警戒线告警 |
| 逆向项目不放心 | 原创架构 + 公开 SDK + MIT 许可 |

---

## 六、功能设计（v1.0 完整产品 · 一次性交付）

> 全部功能在 v1.0 同时发布。不分阶段。

### 6.1 启动与基础交互
- 命令：`mimo`（启动）/ `mimo --version` / `mimo --help` / `mimo init`（首次配置向导）/ `mimo auth login`（重新配置 key）/ `mimo auth logout`（清除 key）
- TUI 主界面（ratatui）：输入框 + 历史输出 + 状态栏 + 侧边栏
- Slash 命令：`/help` `/exit` `/clear` `/model` `/region` `/mode` `/usage` `/mcp` `/skill` `/sessions` `/sandbox` `/config`
- 鼠标 + 键盘双交互
- 冷启动 < 200ms

#### 首次启动鉴权流程（关键 UX）

**每个用户使用自己的 MiMo Token Plan key**（跟 Claude Code 用各自 `ANTHROPIC_API_KEY` 一致）。

首次运行 `mimo` 自动触发引导：

1. 检测 `~/.mimo/auth.json` 不存在 → 进入 `mimo init` 向导
2. 屏幕引导用户去 `https://platform.xiaomimimo.com` 注册账号 → 控制台 → API Keys → 创建一个
3. 用户粘贴 `tp-xxx` 格式的 key
4. 工具自动 ping 三个集群（CN / SGP / AMS）测延迟，推荐最快的
5. 用户选默认模型（默认 `mimo-v2.5-pro`）
6. 测试调用一次 `/v1/messages` 验证 key 有效
7. 写入 `~/.mimo/auth.json`（权限 0600，仅当前用户可读）
8. 提示"配置完成，开始使用"

**也支持环境变量**（CI / 远程开发场景）：
```bash
export MIMO_API_KEY=tp-xxx
export MIMO_REGION=sgp    # 可选
export MIMO_MODEL=mimo-v2.5-pro  # 可选
mimo
```

**永不上传**：key 仅本地存储 + 每次请求时发给 `token-plan-*.xiaomimimo.com`，不发给我们或任何第三方。

### 6.2 模型与协议层
- **主协议**：Anthropic Messages API（接 Token Plan）
- **Fallback 协议**：OpenAI Chat Completions（接备用 endpoint）
- **支持模型**：
  - `mimo-v2.5-pro`（默认编程主力）
  - `mimo-v2.5`（轻量）
  - `mimo-v2-flash`（快速档）
  - `mimo-v2-omni`（多模态，处理图像输入）
- **Streaming**：SSE 流式输出，逐 token 渲染
- **Thinking 实时渲染**：推理流单独颜色 + 可折叠展开
- **Prompt 缓存可视化**：状态栏实时显示 `cache_read / 节省 X tokens`
- **错误重试**：网络抖动自动重试 3 次，429 限流自动降级模型

### 6.3 工具系统（agent 核心能力）

完整工具集，与 Claude Code 持平：

| 工具 | 功能 |
|---|---|
| `read_file` | 读文件 + 行号范围 |
| `write_file` | 写新文件 |
| `edit_file` | 修改已有文件（diff-based） |
| `apply_patch` | 应用 patch（兼容 Claude Code 格式） |
| `shell` | 执行 shell 命令 |
| `glob` | 文件匹配 |
| `grep` | 内容搜索 |
| `git` | git 操作封装 |
| `web_fetch` | 抓网页内容 |
| `web_search` | 调用 MiMo 原生 Web Search |
| `task` | 派发子 agent 任务 |
| `todo` | 任务清单管理 |

工具调用走 Anthropic 标准 `tool_use` / `tool_result` 格式。

### 6.4 三种工作模式
- **Plan**：只读 + 思考，输出方案不动文件
- **Agent**：完整工具集 + 写操作前要人工审批
- **Auto**：完全自动执行（YOLO 模式）

`/mode <plan|agent|auto>` 随时切换。

### 6.5 子 agent 系统
- 最多 10 个并发
- 主 agent 实时看子 agent 进度
- 子任务自动用 `mimo-v2-flash` 降本，主任务保持 `mimo-v2.5-pro`

### 6.6 MCP 客户端
- 完整 MCP 协议（stdio / SSE / streamable HTTP）
- `/mcp install <github-repo>` 一键装
- `/mcp list` / `/mcp toggle <name>`
- 预置：filesystem / git / sqlite / fetch / sequential-thinking
- 兼容 Claude Code / Codex / Cursor 已有的 MCP server 生态

### 6.7 Skills 系统（完全兼容 Anthropic）
- **直接复用 Anthropic Skills 格式**（`.claude/skills/<name>.md` + frontmatter）
- 用户可把 Claude Code 用的 skill 文件夹直接拷过来
- `/skill install <github-repo>` 一键装
- `/skill list` / `/skill enable` / `/skill disable`
- 内置精选 skills：
  - `mimo-cache-optimizer`（自动优化 prompt 命中缓存）
  - `china-dev-essentials`（中文项目常用模板）
  - `bug-hunter` / `refactor-pro` / `doc-writer` 等

### 6.8 会话管理
- 自动保存所有对话到 `~/.mimo/sessions/`
- `/sessions list` / `/sessions resume <id>` / `/sessions fork <id>`
- 1M 上下文自动压缩（超过 80% 触发，关键信息保留）

### 6.9 沙箱与安全
- **macOS**：Seatbelt（sandbox-exec）
- **Linux**：Landlock + seccomp
- **Windows**：Job Objects + Restricted Token
- 默认沙箱：工作目录可读写，外部仅读，shell 白名单
- `/sandbox off` 可关（不推荐）

### 6.10 用量与成本看板
- 状态栏实时：本次 token / 本日累计 / 套餐剩余
- `/usage` 详细看板：
  - 月度日度趋势图（terminal 字符图表）
  - 模型分布
  - 缓存命中率
  - 节省金额估算
- 接小米套餐用量 API（如未开放，本地计数 fallback）

### 6.11 地区集群
- 三集群：CN / SGP / AMS
- `/region auto` 自动按延迟选最快
- `/region cn|sgp|ams` 手动切
- 启动时显示当前集群延迟

### 6.12 多模型 fallback
- 套餐耗尽 / 限流 / 服务异常时自动切到备用 endpoint
- 用户配置任何 OpenAI 兼容 endpoint（DeepSeek / Qwen / Kimi 等）
- `/model fallback on|off`

### 6.13 配置与扩展
- 配置文件：`~/.mimo/config.toml`
- 鉴权文件：`~/.mimo/auth.json`（权限 0600）
- 目录跟 Claude Code 完全隔离
- 环境变量覆盖：`MIMO_API_KEY` / `MIMO_REGION` / `MIMO_MODEL` 等

### 6.14 分发渠道（v1.0 同时上线）

| 渠道 | 安装命令 |
|---|---|
| npm | `npm install -g mimo-tui` |
| Homebrew | `brew install mimo-tui` |
| Cargo | `cargo install mimo-tui` |
| Scoop | `scoop install mimo-tui` |
| Docker | `docker run ghcr.io/<org>/mimo-tui` |
| 一键脚本 | `curl -fsSL https://mimo-tui.dev/install.sh \| sh` |
| GitHub Releases | 预编译二进制（macOS arm64/x64 · Linux x64/arm64 · Windows x64） |

### 6.15 配套官网（mimo-tui.dev）
- 首页：定位 + 一键安装 + Demo 录屏
- 文档：完整使用手册 + API 参考 + Skills 编写指南
- Skills 市场：社区共享 skill 列表（github 仓库聚合）
- 中英双语

---

## 七、技术架构

### 技术栈选型

| 层 | 选型 | 理由 |
|---|---|---|
| 主语言 | **Rust** | 学 DeepSeek-TUI 路线，单二进制易分发，跟 TS 竞品差异化 |
| TUI 框架 | **ratatui** | Rust 生态最成熟的 TUI 框架 |
| HTTP | reqwest + async | 主流 Rust HTTP |
| 异步运行时 | tokio | 标准选择 |
| Anthropic SDK | 自实现轻量 client | 官方无 Rust SDK，但协议简单 |
| 配置 | toml + serde | Rust 生态标准 |
| 分发 | cargo + npm 包装器 + GitHub Releases | 多渠道覆盖 |

### 关键模块

```
mimo-tui/
├── crates/
│   ├── core/           # 模型客户端 / 协议 / agent loop
│   ├── tools/          # 文件 / shell / git / web 工具
│   ├── tui/            # ratatui 渲染
│   ├── cli/            # 命令行入口
│   ├── mcp/            # MCP 客户端
│   ├── skills/         # Skills 系统
│   └── sandbox/        # 平台沙箱
├── npm/                # npm 包装器
├── deploy/             # CI / release
└── docs/               # 用户文档
```

### 关键技术决策

1. **不 fork 任何现有项目**——从零写，避免法律风险，但参考 DeepSeek-TUI 和 OpenCode 的架构
2. **Anthropic 协议优先**——MiMo Token Plan 用户最多
3. **OpenAI 协议作为 fallback** ——未来扩展到其他模型
4. **Skills 兼容 Anthropic 格式**——降低生态迁移成本
5. **MCP 走主流**——直接复用社区已有 MCP servers

---

## 八、分发与定位（无商业化）

> 已决定：**完全不收费、纯开源**。不做 Pro / 团队 / 企业版。不接受赞助以外的商业化路径。

### 8.1 定位
- 完全免费 + MIT 开源
- 给 MiMo 模型用户一个"原生级"的终端 agent
- 长期愿景：成为 MiMo 模型生态里**事实标准**的命令行工具

### 8.2 流量目标（非收入指标，仅用于判断产品成功）
- 上线 3 个月：> 1000 GitHub stars / > 500 npm 周下载
- 上线 6 个月：> 5000 stars / > 2000 npm 周下载
- 上线 12 个月：> 10000 stars（DeepSeek-TUI 32K 的 1/3 体量）

### 8.3 长期价值
即使不收费，长期价值落在：
- **个人/团队的技术品牌**（被 MiMo 用户认知为"做了 MiMo 唯一像样 TUI 的人/团队"）
- **小米官方推荐 / 收编可能性**（参考字节收购 Trae）
- **矩阵复利**：同架构复制到 Hy3-TUI / GLM-TUI / Kimi-TUI，单个产品免费但矩阵起势

---

## 九、开发计划（一次性产品级交付）

### 总目标
**6–8 周内完成 v1.0 全部功能并发布**，不分 MVP / Beta 阶段。

### 模块开发顺序（建议）

| 阶段 | 内容 | 估时 |
|---|---|---|
| Week 1 | Rust 项目骨架 + Anthropic protocol client + 3 集群 + 鉴权 | 1 周 |
| Week 2 | ratatui 基础 UI + streaming 渲染 + thinking 可视化 + 缓存显示 | 1 周 |
| Week 3 | 完整工具集（read/write/edit/shell/glob/grep/git/web）| 1 周 |
| Week 4 | Plan/Agent/Auto 三模式 + 沙箱（mac/linux/win）| 1 周 |
| Week 5 | MCP 客户端 + Skills 系统（Anthropic 兼容） | 1 周 |
| Week 6 | 子 agent + 会话管理 + 1M 上下文压缩 | 1 周 |
| Week 7 | 用量看板 + 多模型 fallback + 配置完善 | 1 周 |
| Week 8 | 全渠道打包 + 官网 mimo-tui.dev + 文档 + 中英双语 + 发布 | 1 周 |

### 验收标准（v1.0 必须全部满足才发布）
- [ ] 6.1–6.15 所有功能在 macOS / Linux / Windows 三平台跑通
- [ ] 至少 50 个真实 use case 测试通过
- [ ] 中英双语文档完整
- [ ] 官网上线 + 安装脚本可用
- [ ] 全部分发渠道上架
- [ ] 没有已知 P0/P1 bug
- [ ] Demo 视频录制完成（含核心场景 5 个）

---

## 十、风险与对策

| 风险 | 概率 | 影响 | 对策 |
|---|---|---|---|
| 小米官方下场做 TUI | 中 | 高 | 抢窗口期 + 做厚社区 + 主动接触小米求合作 |
| Anthropic 协议变更 | 低 | 中 | 协议层抽象，便于切换 |
| MiMo 模型质量不稳定 | 中 | 中 | 多模型 fallback + 工具友好度优先 |
| XIAOMI-MiMo-code 突然爆发 | 低 | 中 | 差异化清晰（合规 + 原生 + Rust）|
| 套餐用量 API 不公开 | 中 | 低 | 本地计数 + 后续推动小米开放 |
| Token Plan 价格变动 | 低 | 低 | 不收费，跟模型价格脱钩 |

---

## 十一、关键指标（成功定义）

> 不收费，所以指标全是流量与口碑型，没有营收指标。

### 上线后 1 个月
- GitHub Stars > 200
- npm 周下载 > 500
- 至少 1 篇知乎 / V2EX / X 评测引发讨论
- GitHub Issues 响应中位数 < 24h

### 上线后 3 个月
- Stars > 3000
- npm 月下载 > 5000
- 100+ 人的 Discord 或微信社群
- 已有第三方贡献 PR / Skills

### 上线后 12 个月
- Stars > 10000（DeepSeek-TUI 32K 的 1/3 体量）
- 月活 > 10000
- 进入 awesome-cli-coding-agents 等头部清单
- 与小米官方建立 联络（推荐 / 合作 / 收编）

---

## 十二、附录

### 调研文档

- [调研/01-官方API与生态.md](./调研/01-官方API与生态.md) —— MiMo 平台、模型、协议、官方仓库
- [调研/02-竞品架构对比.md](./调研/02-竞品架构对比.md) —— XIAOMI-MiMo-code / mimo2codex / CC Switch / DeepSeek-TUI / OpenCode 对比
- [调研/03-API实测结果.md](./调研/03-API实测结果.md) —— 实测响应 + thinking / 缓存 / 集群

### 知识库参考

- `知识库/04-产品机会推荐清单.md` —— 立项依据，S 级 #1
- `知识库/05-AI产品热门案例参考.md` —— DeepSeek-TUI 等范式参考
- `知识库/02-国产模型工具空位调研.md` —— MiMo 在国产模型生态中的位置

### 已定决策（2026-05-19 用户确认）

- ✅ 项目名：`mimo-tui`
- ✅ 域名：`mimo-tui.dev`
- ✅ 不考虑商标
- ✅ Skills 格式完全兼容 Anthropic（用户可拷贝 Claude Code 已有 skill 直接用）
- ✅ 完全不收费 + MIT 开源
- ✅ 一次性产品级 v1.0 交付，不做 MVP / Beta 分阶段
- ✅ API key 已配置在 `.env`（用户自有，可直接测试）

### 待 v1.0 开发期间确认的小事

- 小米套餐用量 API 是否开放第三方调用（不影响发布，本地计数 fallback）
- 是否在 v1.0 发布前主动联系小米官方做品牌沟通（不影响开发，发布前再定）
