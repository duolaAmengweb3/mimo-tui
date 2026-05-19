// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  site: 'https://mimo-tui.pages.dev',
  integrations: [
    starlight({
      title: 'mimo-tui',
      description: 'MiMo 模型的原创开源终端 AI 编程 agent',
      logo: {
        light: './src/assets/logo.svg',
        dark: './src/assets/logo-dark.svg',
        replacesTitle: true,
      },
      favicon: '/favicon.svg',
      head: [
        {
          tag: 'meta',
          attrs: { property: 'og:image', content: 'https://mimo-tui.pages.dev/og-image.svg' },
        },
        {
          tag: 'meta',
          attrs: { name: 'twitter:card', content: 'summary_large_image' },
        },
        {
          tag: 'meta',
          attrs: { name: 'twitter:image', content: 'https://mimo-tui.pages.dev/twitter-card.svg' },
        },
      ],
      social: {
        github: 'https://github.com/duolaAmengweb3/mimo-tui',
        'x.com': 'https://x.com/hunterweb303',
      },
      defaultLocale: 'root',
      locales: {
        root: { label: '中文', lang: 'zh-CN' },
        en: { label: 'English', lang: 'en' },
      },
      customCss: ['./src/styles/brand.css'],
      editLink: {
        baseUrl: 'https://github.com/duolaAmengweb3/mimo-tui/edit/main/website/',
      },
      lastUpdated: true,
      sidebar: [
        {
          label: '开始',
          translations: { en: 'Get Started' },
          items: [
            { label: '什么是 mimo-tui', translations: { en: 'What is mimo-tui' }, slug: 'getting-started/what-is-mimo-tui' },
            { label: '安装', translations: { en: 'Installation' }, slug: 'getting-started/installation' },
            { label: '快速开始', translations: { en: 'Quickstart' }, slug: 'getting-started/quickstart' },
            { label: '首次启动配置', translations: { en: 'First-run setup' }, slug: 'getting-started/first-run' },
          ],
        },
        {
          label: '核心概念',
          translations: { en: 'Concepts' },
          items: [
            { label: 'Agent Loop', slug: 'concepts/agent-loop' },
            { label: '三种模式（Plan/Agent/Auto）', translations: { en: 'Three Modes' }, slug: 'concepts/modes' },
            { label: '子 Agent', translations: { en: 'Sub-agents' }, slug: 'concepts/sub-agents' },
            { label: '1M 上下文', translations: { en: '1M Context' }, slug: 'concepts/context-window' },
          ],
        },
        {
          label: '鉴权与配置',
          translations: { en: 'Auth & Config' },
          items: [
            { label: '拿 API Key', translations: { en: 'Getting API Key' }, slug: 'auth/getting-api-key' },
            { label: '三个集群', translations: { en: 'Regions' }, slug: 'auth/regions' },
            { label: '环境变量', translations: { en: 'Env Vars' }, slug: 'auth/env-vars' },
            { label: 'config.toml 完整参考', translations: { en: 'config.toml reference' }, slug: 'config/config-toml' },
          ],
        },
        {
          label: '工具',
          translations: { en: 'Tools' },
          items: [
            { label: '总览', translations: { en: 'Overview' }, slug: 'tools/overview' },
            { label: '文件工具', translations: { en: 'File Tools' }, slug: 'tools/file-tools' },
            { label: 'Shell 执行', translations: { en: 'Shell' }, slug: 'tools/shell' },
            { label: '搜索（glob/grep）', translations: { en: 'Search' }, slug: 'tools/search' },
            { label: 'Git', slug: 'tools/git' },
            { label: 'Web 工具', translations: { en: 'Web Tools' }, slug: 'tools/web' },
            { label: 'Task / Todo', slug: 'tools/task-todo' },
          ],
        },
        {
          label: 'MCP',
          items: [
            { label: '什么是 MCP', translations: { en: 'What is MCP' }, slug: 'mcp/overview' },
            { label: '安装 MCP Server', translations: { en: 'Installing' }, slug: 'mcp/installing' },
            { label: '配置', translations: { en: 'Configuring' }, slug: 'mcp/configuring' },
            { label: '预置 MCP 列表', translations: { en: 'Preset MCPs' }, slug: 'mcp/preset-list' },
            { label: '写一个自己的 MCP', translations: { en: 'Writing your own' }, slug: 'mcp/writing-server' },
          ],
        },
        {
          label: 'Skills',
          items: [
            { label: '什么是 Skills', translations: { en: 'What are Skills' }, slug: 'skills/overview' },
            { label: '安装 Skill', translations: { en: 'Installing' }, slug: 'skills/installing' },
            { label: '编写 Skill', translations: { en: 'Writing' }, slug: 'skills/writing' },
            { label: '内置 Skills', translations: { en: 'Built-in' }, slug: 'skills/built-in' },
            { label: '分享到社区', translations: { en: 'Sharing' }, slug: 'skills/sharing' },
          ],
        },
        {
          label: '会话 / 沙箱 / 用量',
          translations: { en: 'Sessions / Sandbox / Usage' },
          items: [
            { label: '会话管理', translations: { en: 'Sessions' }, slug: 'sessions/managing' },
            { label: '沙箱总览', translations: { en: 'Sandbox overview' }, slug: 'sandbox/overview' },
            { label: '三平台差异', translations: { en: 'Platforms' }, slug: 'sandbox/platforms' },
            { label: '用量看板', translations: { en: 'Usage dashboard' }, slug: 'usage/dashboard' },
            { label: '缓存优化', translations: { en: 'Cache optimization' }, slug: 'usage/cache-optimization' },
            { label: '多模型 fallback', translations: { en: 'Fallback' }, slug: 'fallback/overview' },
          ],
        },
        {
          label: '高级',
          translations: { en: 'Advanced' },
          items: [
            { label: '脚本化（CI/CD）', translations: { en: 'Scripting' }, slug: 'advanced/scripting' },
            { label: '非交互模式', translations: { en: 'Headless mode' }, slug: 'advanced/headless-mode' },
            { label: '自定义工具', translations: { en: 'Extending' }, slug: 'advanced/extending' },
          ],
        },
        {
          label: '参考',
          translations: { en: 'Reference' },
          items: [
            { label: 'CLI 命令', translations: { en: 'CLI Commands' }, slug: 'reference/commands' },
            { label: 'Slash 命令', translations: { en: 'Slash Commands' }, slug: 'reference/slash-commands' },
            { label: '键盘快捷键', translations: { en: 'Keyboard Shortcuts' }, slug: 'reference/keyboard-shortcuts' },
            { label: 'config schema', slug: 'reference/config-schema' },
          ],
        },
        {
          label: '迁移与对比',
          translations: { en: 'Migration & Compare' },
          items: [
            { label: '从 Claude Code 迁移', translations: { en: 'From Claude Code' }, slug: 'migration/from-claude-code' },
            { label: 'vs Claude Code', slug: 'comparison/vs-claude-code' },
            { label: 'vs XIAOMI-MiMo-code', slug: 'comparison/vs-xiaomi-mimo-code' },
          ],
        },
        {
          label: '故障排查',
          translations: { en: 'Troubleshooting' },
          items: [
            { label: '连接问题', translations: { en: 'Connection' }, slug: 'troubleshooting/connection' },
            { label: 'API 错误', translations: { en: 'API Errors' }, slug: 'troubleshooting/api-errors' },
            { label: '沙箱问题', translations: { en: 'Sandbox' }, slug: 'troubleshooting/sandbox' },
            { label: '性能问题', translations: { en: 'Performance' }, slug: 'troubleshooting/performance' },
          ],
        },
        {
          label: 'FAQ',
          slug: 'faq',
        },
      ],
    }),
  ],
});
