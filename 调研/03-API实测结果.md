# MiMo API 实测结果

> 时间：2026-05-19
> 用户提供的 Token Plan API key（已存 .env，gitignore）

---

## 一、关键发现

**Token Plan API key 必须用 Anthropic 协议**：

| 协议 | Base URL | 鉴权 | 测试结果 |
|---|---|---|---|
| OpenAI | `https://api.xiaomimimo.com/v1` | `Authorization: Bearer` | ❌ 401 Invalid API Key |
| Anthropic | `https://token-plan-sgp.xiaomimimo.com/anthropic` | `x-api-key` + `anthropic-version` | ✅ 通过 |

## 二、实测调用

```bash
curl -sS "https://token-plan-sgp.xiaomimimo.com/anthropic/v1/messages" \
  -H "x-api-key: ${MIMO_API_KEY}" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{
    "model":"mimo-v2.5-pro",
    "max_tokens":120,
    "messages":[{"role":"user","content":"你是谁？一句话回答并告诉我你的模型名"}]
  }'
```

## 三、响应结构

```json
{
  "id": "517aa5f2c2c04029add0a4a49ebb94b6",
  "type": "message",
  "role": "assistant",
  "model": "mimo-v2.5-pro",
  "stop_reason": "end_turn",
  "content": [
    {
      "type": "text",
      "text": "我是**MiMo-v2.5-pro**，由小米大模型Core团队开发的AI助手，很高兴认识你！"
    },
    {
      "type": "thinking",
      "thinking": "The user is asking me to identify myself in one sentence and tell them my model name.",
      "signature": ""
    }
  ],
  "usage": {
    "input_tokens": 69,
    "output_tokens": 47,
    "cache_read_input_tokens": 192
  }
}
```

## 四、关键能力确认

| 能力 | 确认状态 | 备注 |
|---|---|---|
| Anthropic Messages 协议 | ✅ | 完全兼容 |
| thinking 字段 | ✅ | content 数组里直接返回，可做推理可视化 |
| prompt 缓存 | ✅ | usage.cache_read_input_tokens 已经返回，默认开启 |
| 中文对话 | ✅ | 模型默认识别中文输入并输出中文 |
| 模型自我认知 | ✅ | 知道自己是"MiMo-v2.5-pro，由小米大模型 Core 团队开发"|

## 五、推论

1. **可以直接用 Anthropic SDK** 接 MiMo——这意味着代码量大幅减少（不用自己重新实现 messaging）
2. **thinking 是可见的字段**——我们可以做差异化展示（Claude Code 的 thinking 是流式 SSE，需要不同处理）
3. **缓存命中数据已经返回**——可以做"省了多少钱"的可视化
4. **签名（signature）字段空**——这是 Anthropic 协议的扩展签名验证字段，MiMo 暂未启用

## 六、待测项（V0.1 开发前要补测）

- [ ] `/v1/messages` 的 streaming（SSE）模式
- [ ] tool_use 工具调用响应格式
- [ ] 多轮对话 + 缓存预热行为
- [ ] 套餐用量查询端点（是否有 `/v1/usage` 等）
- [ ] CN / SGP / AMS 三个集群延迟对比
- [ ] mimo-v2-flash 和 mimo-omni 的协议兼容性
- [ ] tool_use 工具的 schema 是否标准 Anthropic 格式
- [ ] thinking 字段在 streaming 模式下的传输方式

## 七、安全注意

- ✅ API key 存储在 `.env`，已加入 `.gitignore`
- ✅ 实测请求未把 key 暴露在 git 历史 / 用户对话
- ⚠️ 后续开发要避免：把 key 写死在代码里、commit 包含 key 的日志
