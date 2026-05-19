---
title: 5-minute Quickstart
description: "From install to your first task"
---

import { Aside, Steps } from '@astrojs/starlight/components';

This assumes you've already [installed mimo-tui](/en/getting-started/installation/).

<Steps>

1. **Get your MiMo API key**

   Sign up at [platform.xiaomimimo.com](https://platform.xiaomimimo.com) → console → API Keys → Create. Copy the `tp-` prefixed token.

   New accounts get 1.6 billion tokens free under the creator incentive program — enough to try seriously.

2. **First run → auto-launches setup wizard**

   ```bash
   mimo
   ```

   The wizard will:
   - Ask you to paste your API key
   - Ping the three regions (CN / SGP / AMS) and recommend the fastest
   - Pick a default model (recommended: `mimo-v2.5-pro`)
   - Validate the key with one test call

3. **Enter any project**

   ```bash
   cd ~/code/my-project
   mimo
   ```

4. **Talk to it**

   ```
   > Refactor Header.tsx to a function component, preserve all props
   ```

   It will:
   - Find the file
   - Read it
   - Propose changes
   - Show you the diff
   - You press `y` to accept or `n` to reject

5. **5 common tasks**

   ```
   > Run npm test and fix all failures
   > Write a README for this project
   > Add type hints to every .py file
   > Explain what src/utils/cache.ts does
   > Fix this bug: <paste error>
   ```

</Steps>

## Common commands

Once running, type these slash commands:

| Command | What it does |
|---|---|
| `/help` | List all commands |
| `/model` | Switch model |
| `/mode plan` | Plan mode (read-only thinking, no file edits) |
| `/mode auto` | Auto mode (no per-step approval) |
| `/usage` | Open plan-usage dashboard |
| `/clear` | Clear screen |
| `/exit` | Quit |

## Exit

`Ctrl+C` or `/exit`. Sessions auto-save to `~/.mimo/sessions/` — resume any time with `/sessions resume`.

<Aside type="tip">
Full reference: [Slash Commands](/en/reference/slash-commands/).
</Aside>

## Next

- [First-run setup details](/en/getting-started/first-run/)
- [Plan / Agent / Auto modes](/en/concepts/modes/)
- [All tools](/en/tools/overview/)
- [FAQ](/en/faq/)
