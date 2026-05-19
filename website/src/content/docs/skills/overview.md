---
title: 什么是 Skills
description: "可组合的能力包，完全兼容 Anthropic Skills 格式"
---

import { Aside } from '@astrojs/starlight/components';

**Skill** 是一个 markdown 文件，告诉 mimo agent 在特定场景下应该遵循什么规则。模型每次对话时，匹配到的 skill 内容会自动注入 system prompt。

## 一个 skill 长啥样

```markdown
---
name: python-style
description: When writing Python code, always include type hints and a docstring.
triggers:
  - python
  - .py
---

Whenever you write or edit Python files:
1. Add type hints to all function signatures
2. Include a one-line docstring for every function
3. Prefer `pathlib.Path` over `os.path`
```

把这个文件放进 `~/.mimo/skills/`，下次跟 mimo 说"写一个 Python 脚本"，模型自动遵循这些规则。

## 完全兼容 Anthropic Skills

mimo-tui 用的就是 Claude Code 的 Skills 格式——**你的 `.claude/skills/*.md` 可以直接拷过来用**，零迁移成本。

## 加载位置

按顺序加载，重名后面覆盖前面：

| 路径 | 作用域 |
|---|---|
| `~/.mimo/skills/*.md` | 全局所有项目 |
| `./.claude/skills/*.md` | 当前 workspace（Claude Code 兼容） |
| `./.mimo/skills/*.md` | 当前 workspace（mimo 专用） |

## 触发匹配

mimo 在每次对话的**第一轮**：

1. 看你输入的最新一条消息
2. 遍历所有已加载 skill：
   - 如果 skill 有 `triggers`：里面任何一个关键字出现就匹配
   - 如果没有 `triggers`：用 `description` 里的 4+ 字符词做模糊匹配
3. 命中的 skill 内容拼到 system prompt 末尾

可以同时命中多个 skill —— 全部注入。

## 下一步

- [安装 skill](/skills/installing/)
- [编写自己的 skill](/skills/writing/)
- [内置 skills](/skills/built-in/)
- [分享到社区](/skills/sharing/)
