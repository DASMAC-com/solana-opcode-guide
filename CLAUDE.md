# Conventions

## Self-Improvement Loop

After every task, ask: "What did I get wrong, get told twice, or discover?
Should it go in CLAUDE.md, a spec, a hook, a skill, or memory?"

## Rules

- **Before implementing anything non-trivial**: write or update a spec and get
  approval first.

- **After every change**: ask the user if CLAUDE.md, specs, or skills should be
  updated to reflect what was learned.

- **Repeated instruction** (user corrects the same thing twice): add it to
  CLAUDE.md.

- **Design decision**: write or update a spec.

- **Repeatable workflow**: propose a skill.

- **Must always happen**: propose a hook.

- **Learned pattern**: save to memory.

## Markdown

- Use `-` for list markers.
- End list items with periods.
- Wrap lines to maximize use of the 80-column limit. Break at word boundaries,
  not early.

Every markdown change must pass (run both):

```sh
cfg=cfg/pre-commit/quick-lint.yml
pre-commit run -c $cfg markdownlint-fix --files <file>
pre-commit run -c $cfg mdformat --files <file>
```

## Specs

- `specs/` for cross-cutting concerns (build, CI, conventions).
- `examples/tree/specs/` for tree-specific design decisions.
