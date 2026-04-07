---
name: pull-request
description: Guide for creating proper pull requests for the Biome project. Use when opening a PR, writing a PR description, choosing the correct target branch, or filling out the PR template. Trigger when creating PRs, writing PR summaries, or preparing code for review.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Use this skill when creating a pull request for the Biome repository. It covers branch targeting, title formatting, the PR template, and AI disclosure requirements.

## AI Assistance Disclosure

If AI was used in any capacity to produce the PR, **you must disclose it** in the PR description. Examples:

> This PR was written primarily by Claude Code.
>
> I consulted ChatGPT to understand the codebase but the solution was fully authored manually by myself.

This is mandatory. It helps reviewers apply appropriate scrutiny.

## Choose the Target Branch

| Change type | Target branch |
| --- | --- |
| Bug fix (code or docs) | `main` |
| New **nursery** rule | `main` |
| Rule promotion from nursery | `next` |
| New feature (user-facing) | `next` |
| New feature (internal only) | `main` |

## PR Title

Use **conventional commit format**. The title becomes the squash-merge commit message.

```
<type>(<scope>): <short description>
```

Supported prefixes:

- `feat:` — new feature
- `fix:` — bugfix
- `docs:` — documentation update
- `refactor:` — code refactor (no behavior change)
- `test:` — test update
- `chore:` — housekeeping
- `perf:` — performance improvement
- `ci:` — CI change
- `build:` — build system or dependency change
- `revert:` — revert a previous change

Examples:

```
feat(css): add noDuplicateSelectors rule
fix(linter): handle edge case in useOptionalChain
docs: update contributing guide
refactor(parser): simplify HTML attribute parsing
```

The CI runs [action-semantic-pull-request](https://github.com/amannn/action-semantic-pull-request) to validate the title. Fix it if the workflow fails.

## PR Template

The repository has a PR template at `.github/PULL_REQUEST_TEMPLATE.md`. Every PR description must follow this structure:

```markdown
## Summary

<!-- Explain the motivation for this change. What problem does it solve? -->
<!-- Link relevant issues or Discord discussions. -->
<!-- If user-facing, mention the changeset. -->

## Test Plan

<!-- What demonstrates correctness? Mention tests added/updated. -->

## Docs

<!-- For new rules/actions/options: docs are inline in the code (rustdoc). -->
<!-- For other features: link a docs PR to the `next` branch of biomejs/website. -->
```

### Writing a Good Summary

- **Bug fixes**: Explain the fix concisely. If the fix introduces exceptions or unusual code paths, call those out so reviewers know what to watch for.
- **New concepts**: If the PR adds new abstractions, types, or patterns to the codebase, explain the technical design so reviewers can evaluate it.
- **General rule**: Provide enough context for reviewers to understand how to review the PR. The summary serves the reviewer.
- Link related issues: `Fixes #1234` or `Related to #5678`.
- A changeset description is a good starting point, but the summary should add context the changelog alone would not convey (design decisions, trade-offs, scope limitations).

### Test Plan

Keep it brief. Examples:

- "Added new tests from the bug report."
- "Extended existing snapshot tests to cover the new edge case."

Do not list individual test files — the diff speaks for itself. If automated tests were not possible, state that manual testing is required.

### Docs

- New features require documentation. This section is for linking the PR against the `next` branch of [biomejs/website](https://github.com/biomejs/website/).
- Lint rules and helps to carry their own docs as rustdoc in the source code — no separate website PR needed.
- If the PR doesn't need for documentation changes, add `N/A` under the section.

## Pre-PR Checklist

Before opening, ensure:

1. Code compiles: `cargo check`
2. Tests pass: `cargo test` (or `just test-crate <crate>` for scoped runs)
3. Code is formatted: `just f`
4. Lints pass: `just l`
5. Code generation is up to date (CI autofix handles this, but check if unsure):
   - Lint rules: `just gen-rules && just gen-configuration`
   - Grammar: `just gen-grammar <lang>`
   - Bindings: `just gen-bindings`
6. Changeset created (if user-facing change): `just new-changeset-empty`
7. Snapshot tests reviewed: `cargo insta review`

## References

- Contribution guide: `CONTRIBUTING.md` sections "Commit messages" and "Creating pull requests"
- PR template: `.github/PULL_REQUEST_TEMPLATE.md`
- Conventional commits: https://www.conventionalcommits.org/en/v1.0.0-beta.2/
- Versioning policy: https://biomejs.dev/internals/versioning/
