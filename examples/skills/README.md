# Skills 示例 · Example skills

> mimo-tui 兼容 **Anthropic Skills 格式**——你 Claude Code 用的 skill 文件可以直接拷过来用。
>
> mimo-tui is fully compatible with the Anthropic Skills format — drop your Claude Code skill files in unchanged.

## 加载顺序 · Load order

mimo 启动时从以下位置（按顺序）加载 `.md` skill 文件：

| 路径 / Path | 作用域 |
|---|---|
| `~/.mimo/skills/*.md` | 全局（所有项目） |
| `./.claude/skills/*.md` | 当前 workspace（兼容 Claude Code） |
| `./.mimo/skills/*.md` | 当前 workspace（mimo 专用） |

## 安装示例 / Install

```bash
# 全局
mkdir -p ~/.mimo/skills
cp examples/skills/*.md ~/.mimo/skills/

# 或当前项目
mkdir -p .claude/skills
cp examples/skills/python-style.md .claude/skills/
```

## 提供的示例

| 文件 | 触发关键字 | 干什么 |
|---|---|---|
| `python-style.md` | `python` / `.py` / `django` / `fastapi` | 写 Python 时强制类型注解 + docstring |
| `rust-style.md` | `rust` / `.rs` / `cargo` | 写 Rust 时偏好 Result + thiserror |
| `git-commits.md` | `commit` / `git` | 让 git commit 走 Conventional Commits |

## 自己写一个

```markdown
---
name: my-skill
description: One-line description of when this skill applies.
triggers:
  - keyword1
  - keyword2
---

Body content here. The agent will see this verbatim in the system prompt
whenever any trigger keyword (or fallback: description keyword) appears
in the user's message.
```

字段：

- `name`：标识（必填）
- `description`：作为 fallback 触发关键字（4+ 字符的单词会被匹配）
- `triggers`：显式触发关键字数组（指定后只看这些，不看 description）
- `model`：建议的模型（可选，目前 mimo 不强制）

## 工作原理

mimo 在每次对话第一轮：
1. 看用户最新一条消息文本
2. 遍历所有已加载 skill，跑 `triggers` 或 `description` 关键字匹配
3. 命中的 skill 把 body 拼到 system prompt 末尾
4. 模型自然遵循 skill 内的规则
