---
name: biome-code-review
description: Use only for reviewing completed Biome PRs, branches, commit ranges, diffs, or working trees against business logic and requirements. Excludes broad code-quality and process audits, triage, reproduction, and implementation.
compatibility: Designed for read-only review of the Biome codebase (github.com/biomejs/biome).
metadata:
  repository: biomejs/biome
  mode: read-only
---

# Biome Code Review

Review changes read-only against business requirements and applicable behavioral, architectural, and subsystem constraints.

## Invocation

Delegate completed reviews to a fresh subagent when available. Supply only:

- the review scope;
- the intended business requirements, including constraints the repository cannot establish.

To avoid bias, omit implementation details, suspected defects, priority files, prior findings, and expected review outcomes.

Review subagents must not delegate again. Without subagents, review the complete scope directly.

Treat delegated findings as candidates; the parent must [validate them](#parent-validation) before confirming or acting.

## Safety Boundary

Reviewers and validating parents must preserve the worktree. Authorized implementation follows validation, outside this skill.

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

Use the supplied PR, range, diff, files, or base; exclude unrelated worktree changes.

For a PR number, read its title, body, base, and files with `gh pr view`, then read `gh pr diff`. Do not check it out.

When no explicit PR, range, diff, or file scope is supplied, review the current branch and working tree:

1. Read branch, upstream, and every untracked path with `git status --short --branch --untracked-files=all`.
2. Use the supplied base when present. Otherwise, use the tracking branch when it is `origin/main` or `origin/next`, or compare merge bases against both branches and choose the actual ancestor. Ask only when the result is genuinely ambiguous.
3. Fetch the selected base once.
4. Diff the merge base through the working tree so committed, staged, and unstaged changes are included.
5. Read every reported untracked file; untracked tests and changesets are part of the review.

Never fall back to `HEAD` as the base without saying so. That would omit committed branch changes.

Infer intent from the brief, PR/commit text, linked issue, tests, and code; explicit requirements take precedence. A steered brief still requires reviewing the entire supplied scope; disclose the steer.

## Behavioral Boundary

Before tracing beyond the diff, record intended behavior, requirements, and applicable behavioral, architectural, and subsystem constraints with sources. Report violations or concrete avoidable costs/failures introduced, worsened, or newly exposed by the change.

Read surrounding code only to verify changed behavior and constraints; full-file reads do not expand scope. Stop tracing once the question is settled.

Apply guidance only to changed or directly affected code; preferences are not violations, and requirements must not be invented. Exclude unrelated cleanup, defects, process checks, and speculative optimization, even from optional suggestions or questions.

## Gather Context

- Read every changed file in full.
- Inspect affected callers, registrations, generated counterparts, neighbors, and tests only to verify scoped behavior and constraints.
- Read root `AGENTS.md` and only the relevant sections of `CONTRIBUTING.md` or crate guides.
- Load relevant skills and references for contracts, not additional objectives or permission to execute workflows.
- Prefer checked-out source over documentation or memory.

Use this routing table instead of loading every reference:

| In-scope question concerns | Load |
| --- | --- |
| Grammar, lint, parser, formatter, diagnostics, types, tests, generated files | [repository-and-subsystems.md](references/repository-and-subsystems.md) and the matching implementation skill |
| `biome_service`, workspace DB, CLI/LSP execution, cancellation | [workspace-access.md](references/workspace-access.md) |
| Rust production totality, failure paths, recursion, syntax text, ranges, allocation, or API shape | [rust-safety-and-syntax.md](references/rust-safety-and-syntax.md) |
| Documentation describing required behavior or affected contracts | [documentation-and-process.md](references/documentation-and-process.md) |

## Review Method

Perform two passes:

1. **Behavior:** trace requirements, control and data flow, and relevant ownership and execution contracts.
2. **Implementation:** inspect every human-written changed line and relevant test against those requirements and affected behavior.

Try to falsify claimed requirements such as zero-copy, unchanged behavior, faster execution, or no new dependencies. Rate counterexamples by impact.

Check required paths, callers, variants, and failure behavior before supporting artifacts. Behavioral failures require reachability; constraint violations, including [production totality](#production-totality), require evidence of an unmet constraint, not a runtime counterexample.

## Cross-Cutting Checks

- Verify a bug fix's regression test reaches the changed behavior and fails without the fix.
- Read snapshot changes as expected behavior. A snapshot can faithfully record an incorrect range, message, or output.
- A safe fix must preserve semantics for every reachable case and stop the rule from reporting after application.
- Check required registration and generated artifacts against affected sources and `AGENTS.md`; honor CI Autofix exceptions.
- Consolidate repeated symptoms under their root cause.

## Finding Threshold

Report only actionable, in-scope issues supported by inspected code.

- Cite the unmet requirement, violated contract, or concrete avoidable cost and its connection to the diff.
- For behavioral failures, give the trigger, expected versus actual behavior, and impact. For test gaps, name the required scenario and defect to catch.
- Check guards, types, caller invariants, and tests for counter-evidence.
- Cite the smallest relevant changed range.
- Give minimal remediation, not a patch.

Put unresolved in-scope requirements or correctness assumptions under questions, not findings.

### Production Totality

Report production `unwrap`, `expect`, indexing, slicing, panic macros, integer division/remainder, and other partial operations unless release-mode control flow, types, or API contracts establish totality. No concrete failing input is required.

Limit this to added or modified operations, or existing operations whose preconditions or reachability the diff affects. Check relevant guards and callers; proofs need not be local.

Cite the unmet precondition and inspected evidence without claiming a demonstrated panic. See [operation-specific checks](references/rust-safety-and-syntax.md#partial-operations).

## Parent Validation

Validate every candidate independently; confidence is not evidence.

1. Read the cited diff, source, callers or tests, and claimed requirement, constraint, or cost; the summary is not evidence.
2. Try to disprove claims using guards, types, call order, tests, and pre-change behavior. Behavioral failures require reachability; totality findings require checking release-mode control flow, types, and API contracts, not a failing input.
3. Apply the same scope and finding threshold; verify applicable constraints, avoidable costs, and proportional remediation. Reject out-of-scope claims even if correct; never invent requirements.
4. Mark each **validated**, **rejected**, or **unresolved**, citing supporting or specific missing evidence. Seek targeted clarification when needed.
5. Only validated findings qualify for confirmation or authorized remediation outside review. Note rejected/unresolved candidates in validation status; unresolved requirements belong under questions and never justify fixes.

## Report Format

Return only raw Markdown in one fenced block, findings first by severity.

Every finding starts with exactly one `<severity>/<area>` token.

| Severity | Meaning |
| --- | --- |
| `high` | Material regression, corruption or data loss, exploitable security/privacy failure, availability failure, broad false positive, incorrect safe fix, user-reachable panic, or a change that defeats its core requirement |
| `medium` | Credible edge-case failure, missing required variant or registration, demonstrated performance regression, material test gap for required behavior, or unjustified in-scope production partial operation |
| `low` | Localized correctness, maintainability, documentation, or implementation-constraint issue that meets the finding threshold |

Areas: `design`, `correctness`, `security`, `privacy`, `availability`, `performance`, `completeness`, `error-handling`, `tests`, `maintainability`, `documentation`, `changeset`, `process`.

Areas classify eligible findings; they do not expand scope.

Use exactly this format:

````md
```
## Findings

- `high/correctness` `path/to/file.rs:42` - Short title. Cite the requirement, trigger, expected/actual behavior, impact, and minimal remediation.
- `medium/error-handling` `path/to/file.rs:57` - Missing totality guarantee. Cite the affected operation, unmet precondition, inspected evidence, and minimal remediation; do not claim a demonstrated panic.

## Questions

- Include only unresolved assumptions that affect correctness. Omit this section when there are none.

## Review Status

Scope: `<supplied diff or base-sha through head or working tree>`, `<n>` files, plus listed in-scope untracked files.
Requirements: `<intended behavior, applicable constraints, and sources>`.
Brief: independent | steered toward `<area>`; full supplied scope reviewed.
Validation: Static review only; no project code was run.
Parent validation: not delegated | pending: parent must independently check source and requirements before confirming or acting | completed: `<evidence-backed candidate dispositions>`.
Fetch: updated `origin/<base>` | failed, local `origin/<base>` used | not needed.
```
````

Use `No findings.` under `## Findings` when empty. Severity reflects impact, not confidence. Subagents mark parent validation pending; only the parent may mark it completed after validation.
