---
title: Installation
description: "7 ways to install mimo-tui · macOS / Linux / Windows"
---

import { Tabs, TabItem, Aside } from '@astrojs/starlight/components';

<Aside type="note" title="Alpha stage">
The latest release is `v0.3.0-alpha.2` and runs on all six supported platforms. Alpha is feature-complete; feedback welcome on [GitHub Issues](https://github.com/duolaAmengweb3/mimo-tui/issues).
</Aside>

## Recommended

### npm (most common)

```bash
npm install -g mimo-tui
```

Requires Node.js 18+.

### Homebrew (macOS / Linux)

```bash
brew install mimo-tui/mimo-tui/mimo-tui
```

### One-liner script

```bash
curl -fsSL https://mimo-tui.pages.dev/install.sh | sh
```

Auto-detects your platform and architecture, downloads the right binary from GitHub Releases.

## Alternatives

### Cargo (Rust users)

```bash
cargo install mimo-tui
```

Builds from source locally. Requires Rust 1.82+.

### Scoop (Windows)

```bash
scoop bucket add mimo-tui https://github.com/duolaAmengweb3/scoop-mimo-tui
scoop install mimo-tui
```

### Docker

```bash
docker run -it -v "$PWD":/workspace ghcr.io/duolaamengweb3/mimo-tui:latest
```

### GitHub Releases (manual)

Grab the right archive from [Releases](https://github.com/duolaAmengweb3/mimo-tui/releases) and put the binary in your `$PATH`:

| Platform | Filename |
|---|---|
| macOS arm64 (M1+) | `mimo-aarch64-apple-darwin.tar.gz` |
| macOS x64 (Intel) | `mimo-x86_64-apple-darwin.tar.gz` |
| Linux x64 (glibc) | `mimo-x86_64-unknown-linux-gnu.tar.gz` |
| Linux x64 (musl) | `mimo-x86_64-unknown-linux-musl.tar.gz` |
| Linux arm64 | `mimo-aarch64-unknown-linux-gnu.tar.gz` |
| Windows x64 | `mimo-x86_64-pc-windows-msvc.zip` |

## Verify

```bash
mimo --version
# → mimo-tui 1.0.0 (build ...)
```

## Upgrade

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
  <TabItem label="One-liner">
    Just run the install script again.
  </TabItem>
</Tabs>

## Uninstall

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
  <TabItem label="Manual">
    ```bash
    # Remove binary
    rm /usr/local/bin/mimo

    # Optional: remove config
    rm -rf ~/.mimo/
    ```
  </TabItem>
</Tabs>

`~/.mimo/` isn't auto-deleted on uninstall — clean it up manually if you want.

## Next

Installed? Continue to [Quickstart](/en/getting-started/quickstart/).
