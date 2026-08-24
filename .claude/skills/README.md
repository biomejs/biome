# Biome Skills

Procedural knowledge for AI coding assistants working on Biome. Each skill is a `SKILL.md` file under its own directory.

## How skills load

Only each skill's `name` and `description` are always in context. The full `SKILL.md` loads when its description matches the task; referenced files load only when needed. Keep descriptions specific and avoid accidental overlap because they decide which skill fires. When a task requires multiple skills, state that co-loading explicitly.

## Universal rules live in AGENTS.md

Project-wide standards — no emojis, the evidence rule, dev-dependency rules, the comment-style summary — are stated once in [`AGENTS.md`](../../AGENTS.md). Skills do not repeat them.

## Catalog

| Skill | Use for |
| --- | --- |
| biome-code-review | Reviewing completed changes before committing or opening a PR |
| lint-rule-development | Creating and implementing lint rules and assists, code actions, rule options |
| formatter-development | Implementing formatters, IR primitives, comment handling, Prettier comparison |
| parser-development | Grammars (`.ungram`), lexers, parse rules, error recovery |
| type-inference | JavaScript/TypeScript inference and module-graph type queries |
| diagnostics-development | Diagnostic messages, advice, code frames — the canonical diagnostics guide |
| eslint-migrate-options | ESLint-to-Biome rule option migrators |
| testing-codegen | Tests, `insta` snapshots, code generation commands |
| changeset | Writing changesets for the CHANGELOG |
| doc-comments | Rust comment hygiene; lint/assist rustdoc also loads lint-rule-development |
| syntax-text-handling | Syntax tokens, `TokenText`, ranges, string extraction, and embedded-language value shapes |

## Adding a skill

1. Create `.claude/skills/<name>/SKILL.md` with `name` and `description` frontmatter.
2. Write a specific description that states when to use the skill, avoids accidental trigger overlap, and names any intentional co-loading.
3. Put only trigger conditions and exclusions in the description; leave workflow details in the body.
4. Keep the body focused and under 500 lines. Prefer a short workflow that links exact canonical sections over copying them.
5. Move conditional or domain-specific detail into `references/`, and state when each reference should be read.
6. Do not restate universal rules from `AGENTS.md` or procedures already maintained in another skill.
7. Add a row to the catalog above.

## Resources

- Agent guidelines: [`AGENTS.md`](../../AGENTS.md)
- Contributing guide: [`CONTRIBUTING.md`](../../CONTRIBUTING.md)
- Biome internals: https://biomejs.dev/internals
