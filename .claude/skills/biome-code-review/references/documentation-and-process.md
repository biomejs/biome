# Documentation and Process Review

Load this reference only when the diff changes comments, rustdoc, user documentation, changesets, branch targeting, or PR metadata.

## Internal Documentation

Load `doc-comments` for `//`, `///`, and `//!` changes. Review only:

- comments added or changed by the diff;
- existing comments made inaccurate by the new behavior;
- documentation removed when its invariant still exists in replacement code;
- new vocabulary or extension points whose contract is not recoverable from names and types.

Do not request documentation merely because an item is public. Report missing prose when a caller or implementor must preserve an unstated invariant, ordering, fallback, or safety contract.

## Rule and Assist Documentation

Rustdoc inside `declare_lint_rule!` and `declare_assist_rule!` is end-user documentation. Load `lint-rule-development` rather than `doc-comments`.

Check what automated validation cannot establish:

- the first paragraph accurately summarizes behavior;
- invalid examples precede valid examples and demonstrate the actual rule;
- each option states its default and shows configuration plus an applied example;
- `ignore` is not used to bypass a snippet that should be validated;
- prose explains why the pattern is a problem.

## Feature Documentation

Non-rule user-facing features may require a website PR. Verify current policy in `CONTRIBUTING.md` and the live PR template. Do not infer documentation requirements from an old skill copy.

## Changesets

Load `changeset` for format and release-level policy. The reviewer additionally checks:

- a user-facing change has a real `.changeset/*.md` entry, including untracked files;
- the entry describes the behavior in the diff rather than an earlier design;
- an issue link identifies the issue actually addressed by the test and implementation;
- the release level agrees with the target branch and the special policy for nursery rules;
- internal-only or documentation-only changes do not add release noise.

Do not create or edit a changeset during static review.

## Pull Request Process

Use `CONTRIBUTING.md` and `.github/PULL_REQUEST_TEMPLATE.md` as the current sources of truth.

- The title follows the supported conventional-commit format.
- The base branch matches the change type.
- The template remains intact.
- The contributor, not the reviewing agent, authored the PR description and communication.
- AI assistance is disclosed to the extent required by `CONTRIBUTING.md`.

Do not generate replacement PR prose as part of a finding. State the missing or contradictory requirement.
