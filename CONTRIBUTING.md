# 贡献指南

感谢你想为 mimo-tui 做贡献！

## 开发环境

### 必备
- Rust **1.78+**（推荐用 [rustup](https://rustup.rs/)）
- Node.js **20+**（用于 npm 包装器构建）
- Git

### 平台依赖
- **macOS**：Xcode CLI Tools（`xcode-select --install`）
- **Linux**：`build-essential` / `pkg-config` / `libssl-dev`
- **Windows**：Visual Studio Build Tools

## 本地构建

```bash
# 克隆
git clone https://github.com/mimo-tui/mimo-tui.git
cd mimo-tui

# 构建
cargo build --release

# 跑测试
cargo test --workspace

# 本地运行
cargo run --bin mimo
```

## 代码规范

- **格式**：`cargo fmt`（提交前必跑）
- **静态检查**：`cargo clippy -- -D warnings`
- **测试**：新功能必须带测试
- **覆盖率**：核心模块 > 70%

## 提交规范

用 [Conventional Commits](https://www.conventionalcommits.org/zh-hans/v1.0.0/)：

```
feat(tui): 添加 thinking 折叠展开支持
fix(mcp): 修复 SSE 断连重试
docs: 更新 quickstart
refactor(core): 抽取协议层
test: 新增 sandbox 测试
chore: 升级依赖
```

类型：`feat` / `fix` / `docs` / `refactor` / `test` / `chore` / `perf` / `ci`

## PR 流程

1. Fork 仓库
2. 新建 feature branch：`git checkout -b feat/your-feature`
3. 提交代码，commit 信息按上面规范
4. 推到自己 fork
5. 提 PR，填好模板
6. 等 CI 通过 + maintainer review

## 报 bug / 提需求

- **bug**：用 [bug report 模板](.github/ISSUE_TEMPLATE/bug_report.yml)
- **新功能**：用 [feature request 模板](.github/ISSUE_TEMPLATE/feature_request.yml)
- **问问题**：用 [discussion](https://github.com/mimo-tui/mimo-tui/discussions)，不要开 issue

## 文档贡献

文档源文件在 `website/src/content/docs/`，PR 直接改 markdown 即可。

中英双语同步原则：英文先更，中文跟上（中英 commit 可以分两个 PR）。

## Skill 贡献

写一个 MiMo 优化 skill 想分享？

1. 按 [Skills 编写指南](https://mimo-tui.pages.dev/docs/skills/writing) 写
2. 推到自己的 GitHub 仓库
3. 给仓库加 topic `mimo-tui-skill`
4. 自动出现在 [Skills 市场](https://mimo-tui.pages.dev/skills)

## 行为准则

请遵守 [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md)。
