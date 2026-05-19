# 安全策略

## 支持的版本

| 版本 | 支持状态 |
|---|---|
| 1.x | ✅ 接收安全补丁 |
| < 1.0 | ❌ 不再支持 |

## 报告漏洞

**请不要在公开 issue 里报告安全漏洞**。

私密邮箱：**security@mimo-tui.pages.dev**

报告时请尽量包括：
- 漏洞类型（如 RCE / 信息泄露 / 沙箱逃逸 / 凭证泄露）
- 完整复现步骤
- 受影响的版本范围
- 建议的修复方案（如有）

## 响应承诺

| 阶段 | 时限 |
|---|---|
| 收到报告确认 | 48 小时内 |
| 初步评估 | 7 天内 |
| 严重漏洞修复 + 公告 | 14 天内 |
| 中低危漏洞修复 | 30 天内 |

修复后我们会在 release notes 中署名感谢报告者（如同意）。

## 数据与隐私

mimo-tui 是本地工具，**永远不会**：

- 上传你的代码、对话、文件到任何第三方
- 收集 telemetry / 使用统计（除非用户明确开启）
- 把 API key 发到 MiMo 以外的任何 endpoint

API key 仅存在用户本地（`~/.mimo/auth.json`，权限 0600），每次请求只发给用户配置的 MiMo Token Plan endpoint。

详见 [Data Flow 文档](https://mimo-tui.pages.dev/docs/security/data-flow)。
