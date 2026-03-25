---
name: changeset
description: Guide for creating and writing proper changesets for Biome PRs. Use when a PR introduces user-visible changes (bug fixes, new features, rule changes, formatter changes, parser changes) that need a changeset entry for the CHANGELOG. Trigger when creating changesets, writing changeset descriptions, or choosing the correct change type.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Use this skill when a PR introduces user-facing changes that require a changeset. Changesets drive CHANGELOG generation and release automation. Internal-only changes (refactors with no user-visible effect) do not need changesets.

## Create a Changeset

**Do not create changeset files manually.** Use:

```shell
just new-changeset-empty
```

The command will create an empty file `.changeset/`. Edit it directly to add detail.

> Requires `pnpm` — run `pnpm i` from repo root first.

## Choose the Correct Change Type

- `patch` — Bug fixes.
- `minor` — New features. PR must target the `next` branch.
- `major` — Breaking user API changes. PR must target the `next` branch. These are rare and strictly controlled.

Refer to the [versioning page](https://biomejs.dev/internals/versioning/) when unsure.

## Changeset Format

```markdown
---
"@biomejs/biome": patch
---

Description here.
```

If you need headers inside the description, use `####` or `#####` only. Other header levels break the CHANGELOG tooling.

## Writing Guidelines

### General Rules

- Write about **user-facing changes only**. No changeset needed for pure refactoring. Describe how the change affects the user.
- Be **concise and clear** — 1 to 3 sentences. Longer entries signal to users that the change deserves attention.
- **Past tense** for what you did: "Added", "Fixed", "Changed".
- **Present tense** for Biome behavior: "Biome now supports...", "The rule now detects...".
- End every sentence with a **full stop** (`.`).

### Bug Fixes

Start with a link to the issue:

```markdown
Fixed [#4444](https://github.com/biomejs/biome/issues/4444): [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) now detects negated logical OR chains.
```

### New Lint Rules

Show an example of an invalid case. Use inline code for simple things, a code block for complex ones:

```markdown
Added a new nursery rule [`noDuplicateSelectors`](https://biomejs.dev/linter/rules/no-duplicate-selectors/), that disallows duplicate selector lists within the same at-rule context.

For example, the following snippet triggers the rule:

` ` `css
.foo {}
.foo {}
` ` `
```

### Changes to Existing Rules

Clearly show what is now invalid that was not before (or vice versa). Show both sides if helpful:

```markdown
Fixed [#7211](https://github.com/biomejs/biome/issues/7211): [`useOptionalChain`](https://biomejs.dev/linter/rules/use-optional-chain/) now detects negated logical OR chains. The following code is now considered invalid:

` ` `js
!foo || !foo.bar
` ` `
```

### Formatter Changes

Show the formatting diff:

```markdown
Changed formatting of arrow function parameters. Example:

` ` `diff
- const fn = (  a,  b  ) => {};
+ const fn = (a, b) => {};
` ` `
```

### Parser Changes

Brief inline example of what can now be parsed:

```markdown
Added support for parsing `using` declarations in JavaScript.
```

Use a code block if multiline clarity helps.

### Linking Rules and Assists

Always link to the website, even if the page does not exist yet (it will after merge):

- Rules: `` [`useConst`](https://biomejs.dev/linter/rules/use-const/) ``
- Assists: `` [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) ``

## Tips

- Create the changeset before opening the PR; you can edit it after.
- Look at existing files in `.changeset/` or recent `CHANGELOG.md` entries for reference.
- One changeset per PR is typical. Multiple changesets are allowed if the PR addresses multiple, distinct bugs.

## References

- Contribution guide: `CONTRIBUTING.md` section "Changelog"
- Versioning policy: https://biomejs.dev/internals/versioning/
- Changesets documentation: https://github.com/changesets/changesets
