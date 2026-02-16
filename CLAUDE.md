# Conventions

## Self-Improvement

After every task, check what was learned and route it:

- Repeated instruction (corrected twice): add it to CLAUDE.md.
- Design decision: write or update a spec.
- Repeatable workflow: propose a skill.
- Must always happen: propose a hook.
- Learned pattern: save to memory.

Before implementing anything non-trivial, write or update a spec and get
approval first.

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
