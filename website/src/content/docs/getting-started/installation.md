---
title: 安装
description: 7 种方式装 mimo-tui · macOS / Linux / Windows 三平台全支持
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';

<Aside type="caution" title="v1.0 准备中">
当前 npm 上只有 v0.0.1 占位版本，跑起来只会显示提示信息。v1.0 完整版准备中，关注 [GitHub 仓库](https://github.com/duolaAmengweb3/mimo-tui) 获取进度。
</Aside>

## 推荐方式

### npm（最常用）

```bash
npm install -g mimo-tui
```

要求 Node.js 18+。

### Homebrew（macOS / Linux）

```bash
brew install mimo-tui/mimo-tui/mimo-tui
```

### 一键脚本

```bash
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh
```

自动检测平台 + 架构，从 GitHub Releases 下对应二进制。

## 其他方式

### Cargo（Rust 用户）

```bash
cargo install mimo-tui
```

会从源码本地编译，需要 Rust 1.78+。

### Scoop（Windows）

```bash
scoop bucket add mimo-tui https://github.com/duolaAmengweb3/scoop-mimo-tui
scoop install mimo-tui
```

### Docker

```bash
docker run -it -v "$PWD":/workspace ghcr.io/duolaamengweb3/mimo-tui:latest
```

### GitHub Releases（手工下二进制）

到 [Releases 页](https://github.com/duolaAmengweb3/mimo-tui/releases) 下对应平台的 zip / tar.gz，解压放到 `$PATH`：

| 平台 | 文件名 |
|---|---|
| macOS arm64 (M1+) | `mimo-darwin-arm64.tar.gz` |
| macOS x64 (Intel) | `mimo-darwin-x64.tar.gz` |
| Linux x64 (glibc) | `mimo-linux-x64-gnu.tar.gz` |
| Linux x64 (musl) | `mimo-linux-x64-musl.tar.gz` |
| Linux arm64 | `mimo-linux-arm64.tar.gz` |
| Windows x64 | `mimo-windows-x64.zip` |

## 验证安装

```bash
mimo --version
# → mimo-tui 1.0.0 (build ...)
```

## 升级

<Tabs>
  <TabItem label="npm">
    ```bash
    npm update -g mimo-tui
    ```
  </TabItem>
  <TabItem label="Homebrew">
    ```bash
    brew upgrade mimo-tui
    ```
  </TabItem>
  <TabItem label="一键脚本">
    重新跑一次 install.sh 即可。
  </TabItem>
</Tabs>

## 卸载

<Tabs>
  <TabItem label="npm">
    ```bash
    npm uninstall -g mimo-tui
    ```
  </TabItem>
  <TabItem label="Homebrew">
    ```bash
    brew uninstall mimo-tui
    ```
  </TabItem>
  <TabItem label="手工">
    ```bash
    # 删二进制
    rm /usr/local/bin/mimo  # 或装在哪儿

    # 清配置（可选）
    rm -rf ~/.mimo/
    ```
  </TabItem>
</Tabs>

卸载后 `~/.mimo/` 不会自动删，需要手工清理。

## 下一步

装好了？看 [快速开始](/getting-started/quickstart/)。
