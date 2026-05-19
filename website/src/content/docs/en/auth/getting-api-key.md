---
title: Getting a MiMo API Key
description: "Sign up at platform.xiaomimimo.com and grab your Token Plan key"
---

import { Aside, Steps } from '@astrojs/starlight/components';

mimo-tui doesn't bundle any API key — every user uses their own. This is standard practice (Claude Code requires your own `ANTHROPIC_API_KEY` too).

<Steps>

1. **Sign up**

   Open [platform.xiaomimimo.com](https://platform.xiaomimimo.com) and create an account.

2. **Claim free tier (optional)**

   Xiaomi runs a "Creator Incentive Program" that grants new accounts **1.6 billion free tokens**. Enough to run mimo-tui heavily for about half a month.

3. **Create the key**

   After login → Console → API Keys → **Create**

   A `tp-` prefixed token is generated. Copy it.

4. **Paste into mimo-tui**

   ```bash
   mimo  # the first-run wizard will prompt you
   ```

   Or just export it:

   ```bash
   export MIMO_API_KEY=tp-xxxxx
   ```

</Steps>

## Plan tiers

MiMo Token Plan has several tiers (subject to change — check the official page):

| Tier | Price | Tokens |
|---|---|---|
| Creator Incentive | Free | 1.6 billion |
| Pro | Monthly | TBD |
| Max | ~¥583 / month | TBD |

`mimo-v2.5-pro` uses roughly 40–60% fewer tokens than Claude Opus on equivalent tasks, so the plan stretches further.

## Validate the key without mimo-tui

Want to test outside the CLI?

```bash
curl https://token-plan-sgp.xiaomimimo.com/anthropic/v1/messages \
  -H "x-api-key: tp-xxxxx" \
  -H "anthropic-version: 2023-06-01" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "mimo-v2.5-pro",
    "max_tokens": 50,
    "messages": [{"role":"user","content":"hi"}]
  }'
```

JSON response means valid. 401 means regenerate.

## Security tips

- The key is your money — **never commit it to git**
- Don't paste it into chat groups, forums, or GitHub issues
- If you suspect a leak, revoke and regenerate in the console immediately
- mimo-tui stores it in `~/.mimo/auth.json` with mode `0600` (owner-only)

## Compared to a Claude API key

| Aspect | Claude (ANTHROPIC_API_KEY) | MiMo (MIMO_API_KEY) |
|---|---|---|
| Prefix | `sk-ant-` | `tp-` |
| Domain | `api.anthropic.com` | `token-plan-*.xiaomimimo.com` |
| Protocol | Anthropic Messages API | Anthropic Messages API ✓ |
| Console | console.anthropic.com | platform.xiaomimimo.com |

Protocols are identical — mimo-tui uses the same standard Anthropic SDK shape, just a different base URL.

## Next

- [Which region (CN/SGP/AMS) to pick](/en/auth/regions/)
- [Environment variables reference](/en/auth/env-vars/)
- [config.toml full reference](/en/config/config-toml/)
