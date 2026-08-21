---
name: biome-code-review
description: Use this skill only when asked to review completed Biome changes in a PR, branch, commit range, diff, or working tree. Perform a read-only static review and report findings without editing files or running project code. Do not use for triage, reproduction, or implementation.
compatibility: Designed for read-only review of the Biome codebase (github.com/biomejs/biome).
metadata:
  repository: biomejs/biome
  mode: read-only
---

# Biome Code Review

Review proposed changes for correctness and fit with the Biome codebase.

## Invocation

When an implementation agent is orchestrating the work and subagents are available, assign the completed review to a fresh subagent. Give it only:

- the review scope;
- the intended business requirements that the repository cannot establish.

Do not include implementation details, suspected defects, files to prioritize, prior findings, or expected outcomes. Those pointers bias the review toward confirming the orchestrator's assumptions.

An agent already invoked as the fresh reviewer performs the review directly and does not delegate it again. If subagents are unavailable, the orchestrator may load this skill and review the complete scope itself.

## Safety Boundary

Preserve the working tree exactly as found.

- Do not create, edit, move, or delete files.
- Do not run project code, builds, tests, formatters, linters, codegen, benchmarks, package managers, LSPs, or daemons.
- Do not run mutating Git or GitHub commands except the single base-branch fetch allowed below.
- Do not use shell pipelines, scripts, `sed`, or `awk` to inspect source. Use file reads, globs, and text search.

Shell is limited to these review commands:

```text
git fetch origin <main|next>
git status --short --branch --untracked-files=all
git branch --show-current
git rev-parse ...
git merge-base ...
git --no-pager diff --no-ext-diff --no-textconv ...
git --no-pager show --no-ext-diff --no-textconv ...
git --no-pager log ...
git ls-files ...
gh pr view <number> [--json ...]
gh pr diff <number>
gh issue view <number> [--json ...]
```

One fetch of the resolved base is allowed. If it fails, continue with the local remote-tracking branch and disclose that it may be stale. Documentation lookups are allowed only when checked-out source cannot settle an external language or API contract.

## Establish Scope

Prefer the scope supplied by the user: a PR number, commit range, diff, files, or base branch.

For a PR number, read its title, body, base, and files with `gh pr view`, then read `gh pr diff`. Do not check it out.

Otherwise review the current branch and working tree:

1. Read branch, upstream, and every untracked path with `git status --short --branch --untracked-files=all`.
2. Use the tracking branch when it is `origin/main` or `origin/next`. Otherwise, compare merge bases against both branches and choose the actual ancestor. Ask only when the result is genuinely ambiguous.
3. Fetch the selected base once.
4. Diff the merge base through the working tree so committed, staged, and unstaged changes are included.
5. Read every reported untracked file; untracked tests and changesets are part of the review.

Never fall back to `HEAD` as the base without saying so. That would omit committed branch changes.

Establish intended behavior from the user brief, PR or commit text, linked issue, tests, and surrounding code. If the brief steers toward a suspected defect, review the entire scope anyway and disclose the steer in the status.

## Gather Context

- Read every changed file in full.
- Inspect callers, registrations, generated counterparts, neighboring implementations, and tests affected by the change.
- Read root `AGENTS.md` and only the relevant sections of `CONTRIBUTING.md` or crate guides.
- Load only the implementation skills and review references matching the changed area.
- Prefer checked-out source over documentation or memory.
- Report pre-existing problems only when the change depends on, worsens, or newly exposes them.

Use this routing table instead of loading every reference:

| Diff touches | Load |
| --- | --- |
| Grammar, lint, parser, formatter, diagnostics, types, tests, generated files | [repository-and-subsystems.md](references/repository-and-subsystems.md) and the matching implementation skill |
| `biome_service`, workspace DB, CLI/LSP execution, cancellation | [workspace-access.md](references/workspace-access.md) |
| Production Rust partial operations, recursion, syntax text, ranges, allocation, API shape | [rust-safety-and-syntax.md](references/rust-safety-and-syntax.md) |
| Comments, rustdoc, user documentation, changesets, branch target, PR metadata | [documentation-and-process.md](references/documentation-and-process.md) |

## Review Method

Perform two passes:

1. **Design:** understand the goal, crate ownership, execution model, and data flow.
2. **Implementation:** inspect every human-written changed line and the relevant tests for correctness, failure behavior, allocation cost, and maintainability.

When the title, description, changeset, or source claims a property such as zero-copy, no behavior change, faster execution, or no new dependency, try to falsify that property first. A counterexample that defeats the stated purpose is a high-severity finding.

Review in this order:

1. Design, crate placement, workspace execution model, and claimed properties.
2. Functional correctness, false positives, fix safety, panics, and partial operations.
3. Text/range correctness, allocation, API shape, recursion, and error handling.
4. Registration, codegen, tests, snapshots, documentation, changeset, and process compliance.

Focus on concrete impact. Do not report stylistic preferences or theoretical edge cases without a reachable failure, except for unjustified production partial operations as defined in the Rust review reference.

## Cross-Cutting Checks

- A bug fix needs a test that fails without the fix.
- Read snapshot changes as expected behavior. A snapshot can faithfully record an incorrect range, message, or output.
- A safe fix must preserve semantics for every reachable case and stop the rule from reporting after application.
- Verify generated artifacts required by `AGENTS.md`; do not report outputs intentionally left to CI Autofix.
- Search for existing helpers before accepting a new abstraction.
- Consolidate repeated symptoms under their root cause.

## Finding Threshold

Report only actionable issues supported by inspected code.

- Explain the triggering input, call path, or maintenance condition and the resulting behavior or cost.
- Point to the smallest relevant changed line or range.
- Give a minimal remediation direction without writing the patch.
- Put uncertain requirements or design choices under questions, not findings.
- List optional improvements after required findings.

Production `unwrap`, `expect`, indexing, slicing, panic macros, and integer division are the exception: if release-mode control flow, a type, or an API contract does not establish totality, their presence is a finding even when a concrete user input was not found.

## Report Format

Return raw, unrendered Markdown inside one fenced code block, with no prose before or after it. Findings come first and are ordered by severity.

Every finding starts with exactly one `<severity>/<area>` token.

| Severity | Meaning |
| --- | --- |
| `high` | Regression, corruption or data loss, exploitable security/privacy failure, availability failure, broad false positive, incorrect safe fix, user-reachable panic, workspace execution violation, or a change that defeats its stated purpose |
| `medium` | Credible edge-case failure, missing variant or registration, hot-path allocation, material test gap, or unjustified production partial operation |
| `low` | Localized correctness, maintainability, documentation, or process issue |
| `optional` | Non-blocking improvement with a concrete benefit |

Areas: `design`, `correctness`, `security`, `privacy`, `availability`, `performance`, `completeness`, `error-handling`, `tests`, `maintainability`, `documentation`, `changeset`, `process`.

You **must** adhere to the following format:

````md
```
## Findings

- `high/correctness` `path/to/file.rs:42` - Short title. Explain the trigger, impact, and remediation direction.

## Questions

- Include only unresolved assumptions that affect correctness. Omit this section when there are none.

## Review Status

Scope: `<base-sha>` through `<head or working tree>`, `<n>` files, plus listed untracked files.
Branch target: `<base>` is correct | should target `<main|next>`.
Changeset: present and correct | present but `<problem>` | missing | not required.
Brief: independent | steered toward `<area>`; the full scope was reviewed.
Validation: Static review only; no project code was run.
Fetch: updated `origin/<base>` | failed, local `origin/<base>` used | not needed.
```
````

If there are no findings, write `No findings.` under `## Findings`. Severity reflects impact, not confidence.
