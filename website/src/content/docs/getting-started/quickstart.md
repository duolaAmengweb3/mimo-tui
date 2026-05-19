---
title: 5 分钟快速开始
description: 从装好到跑通第一个任务
---

import { Aside, Steps } from '@astrojs/starlight/components';

假设你已经[装好了 mimo-tui](/getting-started/installation/)。

<Steps>

1. **拿 MiMo API key**

   去 [platform.xiaomimimo.com](https://platform.xiaomimimo.com) 注册账号 → 控制台 → API Keys → 创建一个。复制 `tp-` 开头的 token。

   首次注册有 16 亿 token 的免费额度，足够试用。

2. **首次启动 → 自动进配置向导**

   ```bash
   mimo
   ```

   屏幕会引导你：
   - 粘贴刚才拿的 API key
   - 自动测试三个集群（CN / SGP / AMS）延迟，推荐最快的
   - 选默认模型（推荐 `mimo-v2.5-pro`）
   - 测试一次调用验证 key 有效

3. **进入任何项目**

   ```bash
   cd ~/code/my-project
   mimo
   ```

4. **开始跟它对话**

   ```
   > 帮我把 Header.tsx 改成函数式组件，保留原 props
   ```

   它会：
   - 找到文件
   - 读内容
   - 提出修改方案
   - 改完给你看 diff
   - 你按 `y` 同意 / `n` 拒绝

5. **常用的 5 个任务**

   ```
   > 跑 npm test 修所有报错
   > 给项目写 README
   > 给所有 .py 文件加上 type hints
   > 解释一下 src/utils/cache.ts 是干啥的
   > 把这个 bug 修了：报错信息 [粘贴]
   ```

</Steps>

## 常用命令

启动后输入以下 slash 命令：

| 命令 | 干啥 |
|---|---|
| `/help` | 看所有命令 |
| `/model` | 切换模型 |
| `/mode plan` | 切到 Plan 模式（只读思考，不动文件） |
| `/mode auto` | 切到 Auto 模式（不需要每步审批） |
| `/usage` | 看套餐用量看板 |
| `/clear` | 清屏 |
| `/exit` | 退出 |

## 退出

按 `Ctrl+C` 或输入 `/exit`。会话会自动保存到 `~/.mimo/sessions/`，下次可以用 `/sessions resume` 恢复。

<Aside type="tip">
完整命令参考见 [Slash Commands](/reference/slash-commands/)。
</Aside>

## 下一步

- [首次启动配置详解](/getting-started/first-run/)
- [Plan / Agent / Auto 三种模式](/concepts/modes/)
- [所有工具列表](/tools/overview/)
- [FAQ](/faq/)
