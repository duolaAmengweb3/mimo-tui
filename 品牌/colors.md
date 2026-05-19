# 色彩规范

## 品牌主色（渐变）

用于 ∞ 符号、CTA 按钮、高亮元素。

| 名称 | 浅色背景用 | 深色背景用 | 说明 |
|---|---|---|---|
| Indigo（左端） | `#6366F1` | `#818CF8` | 起始色 |
| Pink（右端） | `#EC4899` | `#F472B6` | 终止色 |

**CSS 渐变定义**：

```css
/* 浅色背景 */
background: linear-gradient(90deg, #6366F1 0%, #EC4899 100%);

/* 深色背景（提亮 100 阶到 400 阶） */
background: linear-gradient(90deg, #818CF8 0%, #F472B6 100%);
```

**SVG 渐变定义**：

```svg
<linearGradient id="grad" x1="0%" y1="0%" x2="100%" y2="0%">
  <stop offset="0%" stop-color="#6366F1"/>
  <stop offset="100%" stop-color="#EC4899"/>
</linearGradient>
```

## 文字色

| 用途 | 浅色背景 | 深色背景 |
|---|---|---|
| 主标题 / wordmark | `#1a1a1a` | `#f5f5f5` |
| 正文 | `#2a2a2a` | `#e5e5e5` |
| 次要正文 | `#525252` | `#a3a3a3` |
| 弱化文本 | `#737373` | `#737373` |
| 占位符 | `#a3a3a3` | `#525252` |

## 背景色

| 用途 | HEX |
|---|---|
| 浅色主背景 | `#ffffff` |
| 浅色副背景 | `#fafafa` |
| 浅色卡片背景 | `#ffffff`（带 1px `#eeeeee` 边框） |
| 深色主背景 | `#0a0a0a` |
| 深色副背景 | `#1a1a1a` |
| 深色卡片背景 | `#1a1a1a`（带 1px `#2a2a2a` 边框） |

## 状态色

| 状态 | HEX |
|---|---|
| Success | `#00D26A` |
| Warning | `#FFB800` |
| Danger | `#EF4444` |
| Info | `#3B82F6` |

## 终端 ANSI 映射（TUI 内用）

mimo-tui 在终端输出时，按以下映射用 256/24-bit color：

| 元素 | 色彩 | ANSI 24-bit |
|---|---|---|
| ∞ 符号 / 品牌强调 | indigo→pink 渐变 | RGB(99,102,241) → RGB(236,72,153) |
| 用户输入 | white | RGB(245,245,245) |
| AI 回复 | light gray | RGB(229,229,229) |
| thinking | dim indigo | RGB(165,180,252) |
| 工具调用 | pink-400 | RGB(244,114,182) |
| 错误 | red-400 | RGB(248,113,113) |
| 成功 | green-400 | RGB(74,222,128) |
| 警告 | yellow-400 | RGB(250,204,21) |

## 灰阶（9 档，对齐 Tailwind）

| 名称 | HEX |
|---|---|
| neutral-50  | `#fafafa` |
| neutral-100 | `#f5f5f5` |
| neutral-200 | `#e5e5e5` |
| neutral-300 | `#d4d4d4` |
| neutral-400 | `#a3a3a3` |
| neutral-500 | `#737373` |
| neutral-600 | `#525252` |
| neutral-700 | `#404040` |
| neutral-800 | `#262626` |
| neutral-900 | `#171717` |
