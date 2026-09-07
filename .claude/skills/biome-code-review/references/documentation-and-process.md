# Behavioral Documentation Review

Use only for in-scope requirements and contracts, not writing-style, release-policy, or PR-process audits.

## Internal Documentation

Load `doc-comments` for `//`, `///`, and `//!` changes. Review only:

- comments added or changed by the diff;
- existing comments made inaccurate by the new behavior;
- documentation removed when an affected caller or implementor still needs its contract.

Require prose only for an explicit requirement or a caller/implementor contract needed for correct behavior. Name the contract and resulting misuse; public visibility alone is insufficient.

## Rule and Assist Documentation

For end-user rustdoc in `declare_lint_rule!` or `declare_assist_rule!`, load `lint-rule-development` and `doc-comments` for contracts, not style or process checks.

Check agreement with the required behavior:

- descriptions and examples reflect what the rule should report;
- documented option defaults and configuration examples match the intended behavior;
- advice and fix descriptions do not promise semantics the implementation violates.

## User-Facing Documentation and Changesets

Check changed documentation and changeset claims against requirements and implementation before reporting contradictions.

Do not audit changeset presence, release levels, branch targets, or PR metadata; `AGENTS.md` and `CONTRIBUTING.md` govern these outside this review.
