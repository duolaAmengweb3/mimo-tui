# 07 · CI / CD

> 目标：push 代码 → 测试 → 构建多平台二进制 → 发布到所有渠道，全自动化
> 估时：2–3 天
> 依赖：02 基础设施 / 03 仓库

## 7.1 测试 workflow

`.github/workflows/test.yml`：

- [ ] 触发：push to main / PR
- [ ] 矩阵：macOS（arm64 + x64）/ Ubuntu / Windows
- [ ] 步骤：
  - [ ] checkout
  - [ ] 安装 Rust toolchain
  - [ ] cache cargo / target
  - [ ] `cargo fmt --check`
  - [ ] `cargo clippy -- -D warnings`
  - [ ] `cargo test --workspace`
  - [ ] 覆盖率上报到 codecov

## 7.2 Release workflow

`.github/workflows/release.yml`：

- [ ] 触发：push tag `v*`
- [ ] 矩阵构建：
  - [ ] macOS arm64
  - [ ] macOS x64
  - [ ] Linux x64 (gnu)
  - [ ] Linux x64 (musl，更便携)
  - [ ] Linux arm64
  - [ ] Windows x64
- [ ] 每平台产出：
  - [ ] 二进制 `mimo`
  - [ ] tar.gz / zip 打包
  - [ ] sha256 校验和
- [ ] 创建 GitHub Release：
  - [ ] 自动从 CHANGELOG 提取 release notes
  - [ ] 上传所有平台包

## 7.3 npm 发布

`.github/workflows/npm-publish.yml`：

- [ ] release workflow 完成后触发
- [ ] 用 `npm publish`
- [ ] postinstall 会根据平台下载对应二进制

## 7.4 Cargo 发布

`.github/workflows/cargo-publish.yml`：

- [ ] release workflow 完成后触发
- [ ] `cargo publish` 各 crate

## 7.5 Docker 镜像

`.github/workflows/docker.yml`：

- [ ] 触发：tag + main
- [ ] 多架构：linux/amd64 + linux/arm64
- [ ] push 到 ghcr.io
- [ ] tag 策略：`latest` + `v1.0.0` + `v1` + `v1.0`

## 7.6 Homebrew tap 更新

`.github/workflows/homebrew.yml`：

- [ ] release 完成后触发
- [ ] 自动 PR 到 `homebrew-mimo-tui` 仓库更新 formula
- [ ] 用 [dawidd6/action-homebrew-bump-formula](https://github.com/dawidd6/action-homebrew-bump-formula) 或自写

## 7.7 官网部署

`.github/workflows/website.yml`：

- [ ] 触发：push to main（website 目录改变）
- [ ] Astro build
- [ ] `wrangler pages deploy`
- [ ] 部署到 `mimo-tui.pages.dev`

## 7.8 文档自动同步

- [ ] CHANGELOG 改变 → 官网 changelog 页自动更新
- [ ] 文档 markdown 改变 → 官网自动 rebuild

## 7.9 Issue / PR 自动化

- [ ] Issue 自动加 label（用 `actions/labeler`）
- [ ] PR welcome bot（第一次贡献者欢迎）
- [ ] 自动 close stale issues（30 天无回复）

## 7.10 监控

- [ ] CI 失败时发通知（Discord webhook）
- [ ] Release 完成后发通知

## 验收

- [ ] 提个测试 tag `v0.0.1-test` 全流程跑通
- [ ] 7 个分发渠道都收到新版本
- [ ] 官网自动重建
- [ ] CHANGELOG 反映到 release notes
- [ ] 全部 secrets 配齐
