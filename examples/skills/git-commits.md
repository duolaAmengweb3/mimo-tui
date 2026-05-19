---
name: git-commits
description: Use Conventional Commits when running git commit.
triggers:
  - commit
  - git
---

When you run `git commit`, follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat | fix | docs | refactor | test | chore | perf | ci | build | style | revert`

Rules:
- Subject ≤ 70 chars, imperative mood, no trailing period.
- Use scope when the change is localized to one area (`fix(parser): ...`).
- Wrap body at 80 chars.
- Reference issues in footer: `Closes #42` or `Fixes #99`.

Examples of good commits:
- `feat(auth): add token plan support for MiMo Token Plan`
- `fix(tui): handle empty input lines in REPL`
- `refactor(core): extract session module from agent`
