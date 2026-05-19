---
title: First-run setup
description: "What happens the first time you run mimo · where configs live"
---

import { Aside } from '@astrojs/starlight/components';

The first time you run `mimo` with no `~/.mimo/auth.json` present, the setup wizard kicks in automatically.

## Wizard steps

```
┌────────────────────────────────────────────────┐
│  ∞ mimo-tui · First-run setup                  │
│                                                │
│  Step 1/4 · MiMo API Key                       │
│                                                │
│  1. Go to https://platform.xiaomimimo.com      │
│  2. Sign up → Console → API Keys → Create      │
│  3. Paste the tp- key below:                   │
│                                                │
│  API Key: ▊                                    │
└────────────────────────────────────────────────┘
```

### Step 1: API Key

Paste the `tp-xxxxx` token. Press Enter.

The tool calls `/v1/messages` once to validate the key. Invalid keys prompt for retry.

### Step 2: Region selection

```
Pinging regions...
  CN  (China)        ... 142 ms
  SGP (Singapore)    ...  43 ms  ← recommended
  AMS (Europe)       ... 287 ms

Use SGP? [Y/n]
```

Hit Enter to accept. You can switch later with `/region cn|sgp|ams`.

### Step 3: Default model

```
Choose default model:
  1. mimo-v2.5-pro   · Trillion-param MoE, primary coding (recommended)
  2. mimo-v2.5       · Lightweight
  3. mimo-v2-flash   · Fast tier
  4. mimo-v2-omni    · Multimodal (vision)

Select [1-4]: 1
```

### Step 4: Done

```
✅ Setup complete

  Key saved:    ~/.mimo/auth.json (mode 0600)
  Config saved: ~/.mimo/config.toml
  Default region: SGP
  Default model:  mimo-v2.5-pro

Press any key to start →
```

## Where files live

| File | Contents |
|---|---|
| `~/.mimo/auth.json` | API key (mode 0600, owner-only) |
| `~/.mimo/config.toml` | Region / model / sandbox / tool settings |
| `~/.mimo/sessions/` | Historical sessions |
| `~/.mimo/usage.db` | Usage stats (sqlite) |
| `~/.mimo/skills/` | Installed skills |
| `~/.mimo/mcp/` | Installed MCP servers |
| `~/.mimo/logs/` | Runtime logs |

The entire `~/.mimo/` tree is fully isolated from Claude Code's `~/.claude/`.

## Or just use env vars (CI / remote dev)

Don't want a config file? Just export:

```bash
export MIMO_API_KEY=tp-xxxxx
export MIMO_REGION=sgp          # optional
export MIMO_MODEL=mimo-v2.5-pro # optional
mimo
```

Env vars override the config file.

## Multiple accounts

Got multiple MiMo Token Plan keys (work + personal)?

```bash
# Temporary switch via env
MIMO_API_KEY=tp-personal mimo

# Or via auth subcommand
mimo auth login    # re-run wizard, overwrites config
mimo auth logout   # wipe credentials
mimo auth status   # show current
```

## Reset config

```bash
rm -rf ~/.mimo/
mimo  # wizard runs again
```

<Aside type="caution">
Removing `~/.mimo/sessions/` wipes all your historical chats. Removing `~/.mimo/usage.db` wipes usage stats.
</Aside>

## Data safety guarantees

- API key **only lives on your machine** (`~/.mimo/auth.json` mode 0600)
- Each request goes only to `token-plan-*.xiaomimimo.com`, never to us or any third party
- No telemetry. No code, conversation, or file uploads.
- See [SECURITY.md](https://github.com/duolaAmengweb3/mimo-tui/blob/main/SECURITY.md) for details
