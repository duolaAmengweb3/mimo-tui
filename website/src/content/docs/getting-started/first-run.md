---
title: 首次启动配置
description: "第一次跑 mimo 会发生什么 · 配置文件存哪儿"
---

import { Aside } from '@astrojs/starlight/components';

第一次跑 `mimo`，检测到没有 `~/.mimo/auth.json`，会自动进入配置向导。

## 配置向导步骤

```
┌────────────────────────────────────────────────┐
│  ∞ mimo-tui · 首次配置                         │
│                                                │
│  Step 1/4 · MiMo API Key                       │
│                                                │
│  1. 去 https://platform.xiaomimimo.com         │
│  2. 注册账号 → 控制台 → API Keys → Create      │
│  3. 复制 tp- 开头的 key 粘贴下面：              │
│                                                │
│  API Key: ▊                                    │
└────────────────────────────────────────────────┘
```

### Step 1: API Key

粘贴 `tp-xxxxx` 格式的 token。回车确认。

工具会调用一次 `/v1/messages` 验证 key 有效，无效会重新让你输。

### Step 2: 集群选择

```
正在测三个集群延迟...
  CN  (中国) ... 142 ms
  SGP (新加坡) ...  43 ms  ← 推荐
  AMS (欧洲) ... 287 ms

按推荐用 SGP？ [Y/n]
```

按推荐就直接 Enter。后续随时可以用 `/region cn|sgp|ams` 切换。

### Step 3: 默认模型

```
选默认模型：
  1. mimo-v2.5-pro  · 万亿 MoE 编程主力（推荐）
  2. mimo-v2.5      · 轻量
  3. mimo-v2-flash  · 快速档
  4. mimo-v2-omni   · 多模态（看图）

选 [1-4]: 1
```

### Step 4: 完成

```
✅ 配置完成

  Key 已存：~/.mimo/auth.json (权限 0600)
  配置已存：~/.mimo/config.toml
  默认集群：SGP
  默认模型：mimo-v2.5-pro

按任意键开始 →
```

## 文件存哪儿

| 文件 | 内容 |
|---|---|
| `~/.mimo/auth.json` | API key（权限 0600，只有你能读） |
| `~/.mimo/config.toml` | 集群 / 模型 / 沙箱 / 工具配置 |
| `~/.mimo/sessions/` | 历史会话 |
| `~/.mimo/usage.db` | 用量统计（sqlite） |
| `~/.mimo/skills/` | 安装的 skills |
| `~/.mimo/mcp/` | 安装的 MCP servers |
| `~/.mimo/logs/` | 运行日志 |

整个 `~/.mimo/` 目录跟 Claude Code 的 `~/.claude/` 完全独立，互不影响。

## 用环境变量代替（CI / 远程开发场景）

不想用配置文件？设环境变量就行：

```bash
export MIMO_API_KEY=tp-xxxxx
export MIMO_REGION=sgp          # 可选
export MIMO_MODEL=mimo-v2.5-pro # 可选
mimo
```

环境变量优先于配置文件。

## 多账号怎么办

如果有多个 MiMo Token Plan key（公司 + 个人）：

```bash
# 用环境变量临时切
MIMO_API_KEY=tp-personal mimo

# 或者
mimo auth login    # 重新跑向导覆盖配置
mimo auth logout   # 清掉
mimo auth status   # 看当前用哪个
```

## 重置配置

```bash
rm -rf ~/.mimo/
mimo  # 重新进向导
```

<Aside type="caution">
删除 `~/.mimo/sessions/` 会丢所有历史会话。删 `~/.mimo/usage.db` 会丢用量统计。
</Aside>

## 数据安全承诺

- API key **永远只存在你电脑本地**（`~/.mimo/auth.json` 0600 权限）
- 每次请求只发给 `token-plan-*.xiaomimimo.com`，不发给我们或任何第三方
- 不收 telemetry，不上传代码 / 对话 / 文件
- 详细见 [SECURITY.md](https://github.com/duolaAmengweb3/mimo-tui/blob/main/SECURITY.md)
