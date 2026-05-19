# 字体规范

## 字体栈

### 主字体（Sans）
用于 wordmark、标题、正文。

```css
font-family: Inter, 'SF Pro Display', 'PingFang SC', system-ui, -apple-system, sans-serif;
```

- **Inter**：首选英文字体（开源、字重齐全、屏幕渲染优）
- **SF Pro Display**：macOS 备选
- **PingFang SC**：中文兜底
- **system-ui**：最终兜底

### 等宽字体（Mono）
用于代码、命令、终端 UI、用量数字。

```css
font-family: 'JetBrains Mono', 'SF Mono', 'Cascadia Code', Menlo, Consolas, monospace;
```

- **JetBrains Mono**：首选（连字、字重齐全）
- **SF Mono**：macOS 备选
- **Cascadia Code**：Windows 备选

## 字号阶梯

| token | 字号 | 行高 | 用途 |
|---|---|---|---|
| display-1 | 80–120px | 1.0 | OG 图 ∞ 符号 |
| display-2 | 56–80px | 1.1 | 官网 hero 主标题 |
| h1 | 40–48px | 1.2 | 章节标题 |
| h2 | 28–36px | 1.3 | 子章节 |
| h3 | 22–24px | 1.4 | 小节 |
| body-large | 18–20px | 1.6 | 文档正文 |
| body | 16px | 1.6 | 默认正文 |
| body-small | 14px | 1.5 | 辅助文本 |
| caption | 12–13px | 1.5 | 标签 / meta |
| micro | 11px | 1.4 | tag / badge |

## 字重

| 用途 | 字重 |
|---|---|
| ∞ 符号 | 900 (Black) |
| wordmark `mimo-tui` | 700 (Bold) |
| H1 / H2 | 700 |
| H3 | 600 |
| body | 400 (Regular) |
| body emphasis | 600 (SemiBold) |
| caption | 500 (Medium) |

## 字间距（letter-spacing）

| 用途 | 值 |
|---|---|
| display 标题 | -2 ~ -3px |
| wordmark | -1 ~ -1.5px |
| body | 0 |
| caption / label | +0.5 ~ +1px |
| micro / tag | +0.8 ~ +1px（大写时） |

## 中英混排规则

- 中英文之间留 1 个半角空格：`像 Claude Code 一样`，不是 `像Claude Code一样`
- 中文标点用全角，英文标点用半角
- 数字与单位之间留空格：`16 亿 token`，不是 `16亿token`
- 路径、命令、URL 用等宽字体

## Web Font 加载策略

```html
<!-- 官网 head 里 -->
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;600;700&display=swap" rel="stylesheet">
```

子集化：仅加载需要的字符集（latin + 数字 + 常用符号），中文兜底 system PingFang SC（不远程加载）。
