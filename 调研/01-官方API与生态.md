# MiMo 官方 API 与生态调研

> 时间：2026-05-19
> 来源：platform.xiaomimimo.com 官方文档 + GitHub XiaomiMiMo 组织 + 实测

---

## 一、官方平台

- **平台名**：Xiaomi MiMo 开放平台
- **官网**：https://platform.xiaomimimo.com
- **GitHub**：https://github.com/XiaomiMiMo（1.1k followers）

## 二、两套 API 协议（重要）

MiMo 同时支持两套协议，**Token Plan 用户必须用 Anthropic 协议**：

### OpenAI 兼容协议
- Base URL：`https://api.xiaomimimo.com/v1`
- 鉴权：`Authorization: Bearer <key>`
- 协议：标准 OpenAI Chat Completions
- 适合：iFlowCLI、Cursor、VS Code 等 OpenAI 协议工具

### Anthropic 协议（Token Plan 专用）
- **新加坡（默认）**：`https://token-plan-sgp.xiaomimimo.com/anthropic`
- **中国**：`https://token-plan-cn.xiaomimimo.com/anthropic`
- **欧洲**：`https://token-plan-ams.xiaomimimo.com/anthropic`
- 鉴权：`x-api-key: <key>` + `anthropic-version: 2023-06-01`
- 协议：Anthropic Messages API
- 适合：Claude Code 套壳

> Token Plan 用户用 OpenAI 协议会 401（实测）。

## 三、模型清单

| 模型 | 类型 | 上下文 | 特点 |
|---|---|---|---|
| **mimo-v2.5-pro** | 主力 | 1M | 万亿 MoE，token 效率比 Claude Opus 省 40-60% |
| mimo-v2-pro | 旧版 | - | V2.5 出来前的主力 |
| mimo-v2-flash | 快速档 | - | 低延迟低成本 |
| mimo-omni | 多模态 | - | 图像/音频/视频理解 |
| mimo-v2.5-tts | TTS | - | 语音合成 |
| mimo-v2.5-asr | ASR | - | 语音识别 |

## 四、已确认支持的能力

实测 `/v1/messages` 响应里直接包含：

```json
{
  "content": [
    {"type": "text", "text": "..."},
    {"type": "thinking", "thinking": "...", "signature": ""}
  ],
  "usage": {
    "input_tokens": 69,
    "output_tokens": 47,
    "cache_read_input_tokens": 192  // 支持 prompt 缓存
  }
}
```

- ✅ Thinking / 推理可视化
- ✅ Prompt 缓存（已经默认开了）
- ✅ Tool calling
- ✅ MCP
- ✅ Streaming（SSE）
- ✅ 多模态（mimo-omni / mimo-v2.5 部分）
- ✅ Web Search 工具

## 五、Token Plan 套餐

根据公开博客信息：

| 套餐 | 价格 | Token 量 |
|---|---|---|
| 免费创作者激励 | ¥0 | 16 亿 token |
| Pro 月度 | 待查 | 待查 |
| Max 月度 | ~¥583/月（折合 ¥7000/年） | 16 亿 token / 年首发 |

> 注：套餐细节后续需要直接查 platform.xiaomimimo.com 控制台确认。

## 六、官方 GitHub 仓库（11 个）

| 仓库 | Stars | 类别 |
|---|---|---|
| MiMo | 2.1k | 论文/权重 |
| MiMo-V2-Flash | 1.3k | 模型权重 |
| MiMo-Audio | 1k | 音频模型 |
| MiMo-VL | 641 | 视觉模型 |
| MiMo-Embodied | 386 | 具身智能 |
| MiMo-V2.5-ASR | 216 | 语音识别 |
| MiMo-Audio-Tokenizer | 143 | 工具 |
| MiMo-Audio-Training | 106 | 训练工具 |
| MiMo-Audio-Eval | 87 | 评测 |
| **MiMo-Skills** | 49 | Agent skills |
| MiMo-Audio-Demo | 11 | 演示 |

**关键结论**：
- ✅ **没有任何 CLI / TUI / Desktop / Mobile / SDK 仓库**
- 所有仓库都是模型权重、训练工具或 Agent skills
- MiMo-Skills 是唯一可能跟"工具生态"沾边的，但才 49 stars 且偏小规模

## 七、官方对开发者工具的态度

公开的策略是**对接已有生态**而不是自建：

- 主推 Claude Code（Anthropic 协议）
- 主推 Cursor、VS Code 插件（OpenAI 协议）
- 主推 iFlow-CLI、OpenClaw 等社区工具
- 没有自建任何 IDE / CLI / 桌面 App 的迹象

> 这意味着"做 MiMo 专属 TUI"是**结构性红利**，至少 6-12 个月窗口期。

## 八、参考链接

- [Xiaomi MiMo 开放平台](https://platform.xiaomimimo.com)
- [llms.txt 元文档](https://platform.xiaomimimo.com/llms.txt)
- [XiaomiMiMo GitHub](https://github.com/XiaomiMiMo)
- [小米 16 亿 Token + Claude Code 实战测评（鱼皮）](https://www.cnblogs.com/yupi/p/19961989)
- [白嫖小米 MiMo 百万亿 Token + Claude Code 配置（追逐时光者）](https://www.cnblogs.com/Can-daydayup/p/19972541)
