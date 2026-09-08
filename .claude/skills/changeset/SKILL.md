---
name: changeset
description: Use this skill when a Biome change may affect users and you must decide whether it needs a changeset, choose the release level, or create and edit `.changeset/*.md` release-note text. Do not use for implementation details or PR descriptions.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Changesets

Changesets describe user-visible behavior for release notes. Root `AGENTS.md` defines when automated contributions require one; `CONTRIBUTING.md` is canonical for package selection, versioning, branch targets, and prose format.

## Decide Whether One Is Required

A changeset is required for behavior visible to users of the CLI, libraries, published crates, diagnostics, parser, formatter, linter, or assists.

No changeset is needed for an internal refactor with identical behavior, tests only, CI/build maintenance, or documentation-only edits.

Before opening a PR, explicitly confirm the user-facing classification as required by `AGENTS.md`. Do not infer user visibility only from the files changed.

## Choose the Release Level

| Change | Level | Target |
| --- | --- | --- |
| Bug fix or non-breaking behavior correction | `patch` | `main` |
| New nursery lint rule | `patch` | `main` |
| New user-facing feature or nursery promotion | `minor` | `next` |
| Breaking user API change | `major` | `next` |

Nursery rules are the exception to the usual new-feature mapping because they do not follow semantic versioning. Verify unusual cases against the current versioning policy.

## Create the File

Run `just new-changeset-empty` to generate an empty file.

```shell
just new-changeset-empty
```
An empty generated entry starts with frontmatter delimiters. Replace that frontmatter with the package and release level rather than appending a second block:

```markdown
---
"@biomejs/biome": patch
---

Description.
```

Use `####` or `#####` for headings inside a longer entry. Other heading levels interfere with changelog generation.

## Write the Entry

- Describe user-visible behavior, not implementation.
- Use one to three sentences unless the impact genuinely needs an example.
- Use past tense for the contribution and present tense for resulting Biome behavior.
- Start bug fixes with the linked issue when one exists.
- Link rule and assist names to their Biome website pages.
- End every sentence with a full stop.

Include only the example needed to understand impact:

- new rule: an invalid example;
- existing rule: what became valid or invalid;
- formatter: a `diff` block;
- parser: source that now parses or is now rejected.

Do not copy a PR summary into the changeset. A changeset is release-note text and should omit test plans, internal design, and reviewer guidance.

## Review Checklist

- The behavior is user-visible.
- The package is correct.
- The release level agrees with branch policy.
- The description matches the implemented behavior.
- Issue, rule, and assist links are accurate.
- Examples show observable impact.
- The file has one valid frontmatter block.

## References

- Changeset policy: `CONTRIBUTING.md#changelog`
- Branch targeting: `CONTRIBUTING.md#creating-pull-requests`
- Versioning: https://biomejs.dev/internals/versioning/
- Generator recipes: `justfile`
