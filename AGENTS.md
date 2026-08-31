# Agent Guidelines for Contributing to Biome

This file contains rules that apply to every automated contribution. Detailed workflows live in `.claude/skills/` or the contributing guides they reference.

> [!NOTE]
> Automated agents may add 🤖🤖🤖 to the end of a PR title to opt into the streamlined merge process.

## Communication

- Keep responses short, factual, and technical.
- Do not use emojis in source, comments, rustdoc, diagnostics, tests, snapshots, commits, issues, PR communication, or agent output. The PR-title marker above is the only exception.
- Do not claim a Biome function, module, behavior, or convention exists without an exact file and line, a source excerpt, or a reproducible command.
- If evidence is unavailable, state that rather than presenting the claim as fact.

## Before Editing

- Read files in full before wide-ranging changes, before editing an unfamiliar file, and whenever the user requests an investigation or audit.
- Inspect the surrounding implementation and current contributing guide instead of copying remembered APIs or commands.
- Preserve unrelated worktree changes. Never revert or rewrite changes you did not make.

## Implementation Gate

- Make the smallest change that satisfies the requested behavior.
- Add tests for code changes. Bug fixes require a case that fails without the fix.
- Run the narrowest relevant tests first, then broader checks when justified.
- Review generated snapshots as behavior, not disposable output.
- Run `just f` and `just l` before committing.

Load [testing-codegen](./.claude/skills/testing-codegen/SKILL.md) for test fixtures, snapshots, and generator selection.

Required generated artifacts:

| Changed source | Required command |
| --- | --- |
| Grammar `.ungram` | `just gen-grammar <lang>` |
| Formatter in `*_formatter` | `just gen-formatter <lang>` |
| Lint rule in `*_analyze` | `just gen-rules` and `just gen-configuration` |

Bindings and other full analyzer outputs may be left to the CI Autofix job unless they are needed for local verification.

## Final Review

After implementation, code generation, formatting, linting, and tests, review the complete change with [biome-code-review](./.claude/skills/biome-code-review/SKILL.md).

- Use a fresh subagent when one is available.
- Provide only the review scope and intended business requirements. Do not include suspected defects, implementation hints, prior findings, or expected outcomes.
- Resolve actionable findings and rerun affected verification.
- Repeat the review when fixes materially change the implementation.

## User-Facing Changes

Before opening a PR, explicitly confirm whether the change is user-facing. User-facing behavior requires a changeset; internal refactors, tests, CI, and documentation-only changes do not.

For a user-facing change, load [changeset](./.claude/skills/changeset/SKILL.md) to choose the release level and create or edit the entry. Branch targeting and changeset policy are canonical in [`CONTRIBUTING.md`](./CONTRIBUTING.md#creating-pull-requests).

## Pull Requests

- Do not write PR descriptions or contributor communication. `CONTRIBUTING.md` requires the contributor to author that prose.
- Preserve `.github/PULL_REQUEST_TEMPLATE.md`; never replace or bypass it.
- If opening a PR from user-provided text, ensure it discloses the extent of AI assistance.
- Use a conventional-commit title and the target branch defined in `CONTRIBUTING.md`.

## Comments and Rustdoc

Load [doc-comments](./.claude/skills/doc-comments/SKILL.md) whenever editing `//`, `///`, or `//!` comments, including comments added incidentally.

- Write for a contributor reading the code at HEAD without access to the conversation, issue, PR, or diff.
- State contracts in `///`, module rationale and terminology in `//!`, and non-obvious rationale in `//`.
- Do not narrate change history or address reviewers.
- Delete comments that only restate names, types, or control flow.
- Rustdoc inside `declare_lint_rule!` and `declare_assist_rule!` is end-user documentation; also load the lint-rule guide for its content requirements.

## Cargo Dependencies

Internal `biome_*` crates under `[dev-dependencies]` use `path = "../biome_*"`, not `workspace = true`. Regular `[dependencies]` continue to use `workspace = true`.

## Skills

The skill catalog and trigger boundaries are maintained in [`.claude/skills/README.md`](./.claude/skills/README.md). Load only the skills relevant to the current task.
