#!/usr/bin/env bash
# 把所有 SVG 转成 PNG / ICO
# 依赖（任选其一）：
#   brew install librsvg     # rsvg-convert（推荐，渲染清晰快）
#   brew install imagemagick # convert
#
# 用法：
#   cd 品牌/
#   bash generate-pngs.sh

set -euo pipefail

cd "$(dirname "$0")"
mkdir -p png

# 检测可用工具
if command -v rsvg-convert >/dev/null 2>&1; then
  CONV() { rsvg-convert -w "$2" -h "$3" "$1" -o "$4"; }
  echo "使用 rsvg-convert"
elif command -v convert >/dev/null 2>&1; then
  CONV() { convert -background none -resize "${2}x${3}" "$1" "$4"; }
  echo "使用 ImageMagick convert"
else
  echo "ERROR：请先安装 librsvg 或 imagemagick："
  echo "  brew install librsvg"
  echo "  # 或"
  echo "  brew install imagemagick"
  exit 1
fi

echo "→ 主 logo（浅色背景版）..."
CONV logo.svg 760 200 png/logo@2x.png
CONV logo.svg 380 100 png/logo.png

echo "→ 主 logo（深色背景版）..."
CONV logo-dark.svg 760 200 png/logo-dark@2x.png
CONV logo-dark.svg 380 100 png/logo-dark.png

echo "→ favicon 各尺寸..."
CONV mark.svg 16 16   png/favicon-16x16.png
CONV mark.svg 32 32   png/favicon-32x32.png
CONV mark.svg 48 48   png/favicon-48x48.png
CONV mark.svg 64 64   png/favicon-64x64.png
CONV mark.svg 128 128 png/favicon-128x128.png
CONV mark.svg 180 180 png/apple-touch-icon.png
CONV mark.svg 192 192 png/android-chrome-192x192.png
CONV mark.svg 512 512 png/android-chrome-512x512.png

echo "→ OG / 社交卡片..."
CONV og-image.svg      1200 630 png/og-image.png
CONV og-image-docs.svg 1200 630 png/og-image-docs.png
CONV twitter-card.svg  1200 675 png/twitter-card.png
CONV github-social.svg 1280 640 png/github-social.png
CONV npm-icon.svg      128 128  png/npm-icon.png

echo "→ wordmark + 单色..."
CONV wordmark.svg          560 160 png/wordmark.png
CONV logo-mono.svg         760 200 png/logo-mono.png
CONV logo-mono-inverse.svg 760 200 png/logo-mono-inverse.png

# 生成 favicon.ico（多尺寸合并）
if command -v convert >/dev/null 2>&1; then
  echo "→ favicon.ico（多尺寸合一）..."
  convert png/favicon-16x16.png png/favicon-32x32.png png/favicon-48x48.png png/favicon.ico
elif command -v magick >/dev/null 2>&1; then
  magick png/favicon-16x16.png png/favicon-32x32.png png/favicon-48x48.png png/favicon.ico
else
  echo "WARN：缺 ImageMagick，跳过 favicon.ico 合成（用 realfavicongenerator.net 也行）"
fi

# 生成 web manifest
cat > png/site.webmanifest <<'EOF'
{
  "name": "mimo-tui",
  "short_name": "mimo-tui",
  "icons": [
    { "src": "/android-chrome-192x192.png", "sizes": "192x192", "type": "image/png" },
    { "src": "/android-chrome-512x512.png", "sizes": "512x512", "type": "image/png" }
  ],
  "theme_color": "#6366F1",
  "background_color": "#0a0a0a",
  "display": "standalone"
}
EOF

echo ""
echo "✅ 全部生成完成 → 看 png/ 目录"
ls -lh png/
