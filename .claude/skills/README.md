# Biome Skills

Procedural knowledge for AI coding assistants working on Biome. Each skill is a `SKILL.md` file under its own directory.

## How skills load

Only each skill's `name` and `description` are always in context. Claude reads the full `SKILL.md` (and any `references/` files) on demand, when the description matches the task. Keep descriptions specific and non-overlapping — the description is what decides which skill fires, and two skills competing for the same trigger phrase make the choice unreliable.

## Universal rules live in AGENTS.md

Project-wide standards — no emojis, the evidence rule, dev-dependency rules, the comment-style summary — are stated once in [`AGENTS.md`](../../AGENTS.md). Skills do not repeat them.

## Catalog

| Skill | Use for |
| --- | --- |
| lint-rule-development | Creating and implementing lint rules and assists, code actions, rule options |
| formatter-development | Implementing formatters, IR primitives, comment handling, Prettier comparison |
| parser-development | Grammars (`.ungram`), lexers, parse rules, error recovery |
| type-inference | Module graph and type system, for type-aware rules |
| diagnostics-development | Diagnostic messages, advice, code frames — the canonical diagnostics guide |
| eslint-migrate-options | ESLint-to-Biome rule option migrators |
| testing-codegen | Tests, `insta` snapshots, code generation commands |
| changeset | Writing changesets for the CHANGELOG |
| pull-request | PR titles, descriptions, branch targeting |
| doc-comments | `//`, `///`, `//!` style for readers of the source |
| biome-developer | Cross-cutting gotchas: syntax-node APIs, string extraction, embedded languages |

## Adding a skill

1. Create `.claude/skills/<name>/SKILL.md` with `name` and `description` frontmatter.
2. Write a specific description (what it does and when to use it) that does not overlap an existing skill's triggers.
3. Keep the body focused and under 500 lines; move deep reference material into a `references/` subdirectory, linked one level deep from `SKILL.md`.
4. Do not restate the universal rules from `AGENTS.md`.
5. Add a row to the catalog above.

## Resources

- Agent guidelines: [`AGENTS.md`](../../AGENTS.md)
- Contributing guide: [`CONTRIBUTING.md`](../../CONTRIBUTING.md)
- Biome internals: https://biomejs.dev/internals
