---
title: 拿 MiMo API Key
description: "去 platform.xiaomimimo.com 注册账号 · 拿 Token Plan 套餐"
---

import { Aside, Steps } from '@astrojs/starlight/components';

mimo-tui 不内置任何 API key，每个用户用自己的。这是标准做法（Claude Code 也是要你自己的 `ANTHROPIC_API_KEY`）。

<Steps>

1. **注册账号**

   打开 [platform.xiaomimimo.com](https://platform.xiaomimimo.com)，注册一个账号。

2. **领免费额度（可选）**

   小米现在有 "百万亿 Token 创造者激励计划"，注册后能免费拿 16 亿 token。够你重度用半个月。

3. **进控制台拿 key**

   登录后 → 控制台 → API Keys → **Create**

   会生成一个 `tp-` 开头的 token。复制下来。

4. **粘到 mimo-tui**

   ```bash
   mimo  # 首次启动会自动让你粘
   ```

   或者直接设环境变量：

   ```bash
   export MIMO_API_KEY=tp-xxxxx
   ```

</Steps>

## 套餐说明

MiMo Token Plan 有几档（具体看官方页面，可能会调整）：

| 档 | 价格 | Token 量 |
|---|---|---|
| 创作者激励 | 免费 | 16 亿 |
| Pro | 月度套餐 | 待查 |
| Max | ~¥583 / 月 | 待查 |

`mimo-v2.5-pro` 大约比 Claude Opus 省 40-60% token，等套餐能用得很久。

## 验证 key 有效

不想跑 mimo 直接测：

```bash
curl https://token-plan-sgp.xiaomimimo.com/anthropic/v1/messages \
  -H "x-api-key: tp-xxxxx" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "mimo-v2.5-pro",
    "max_tokens": 50,
    "messages": [{"role":"user","content":"hi"}]
  }'
```

返回 JSON 就是有效。返回 401 就要重新生成。

## 安全建议

- API key 等于"钱"，**不要 commit 到 git 仓库**
- 不要贴到聊天群 / 论坛 / issue
- 怀疑泄露立刻去控制台 revoke + 重新生成
- mimo-tui 把 key 存在 `~/.mimo/auth.json` 0600 权限，只有你能读

## 跟 Claude API key 的区别

| 项 | Claude (ANTHROPIC_API_KEY) | MiMo (MIMO_API_KEY) |
|---|---|---|
| 前缀 | `sk-ant-` | `tp-` |
| 域名 | `api.anthropic.com` | `token-plan-*.xiaomimimo.com` |
| 协议 | Anthropic Messages API | Anthropic Messages API ✓ |
| 控制台 | console.anthropic.com | platform.xiaomimimo.com |

协议完全一致，所以 mimo-tui 用的就是标准 Anthropic SDK 逻辑，只是 base URL 换一下。

## 下一步

- [三个集群（CN/SGP/AMS）选哪个](/auth/regions/)
- [环境变量完整清单](/auth/env-vars/)
- [config.toml 完整参考](/config/config-toml/)
