---
name: promote-lint-rules
description: Use this skill when promoting one or more Biome lint rules from nursery to stable groups, including promotion plans from GitHub issues, metadata changes, rule renames, generated configuration, and promotion snapshots. Also load testing-codegen for snapshot and generator mechanics and changeset for the release entry.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Promote Lint Rules

Use the batch promotion task instead of invoking `just move-rule` repeatedly.

## Build the Manifest

1. Read the complete requirements. For a GitHub issue, read both its body and comments.
2. Verify every listed current rule exists under a `crates/biome_*_analyze/src/lint/nursery/` directory.
3. Resolve incomplete or malformed requirements before editing. Do not infer missing group, recommendation, severity, or rename decisions from neighboring rows.
4. Create a temporary JSON file outside the repository, such as `/tmp/biome-rule-promotions.json`.

The top-level object maps each current lower-camel-case rule name to its final stable metadata:

```json
{
  "noCurrentRule": {
    "group": "correctness",
    "recommended": true,
    "severity": "error",
    "newName": "noRenamedRule"
  },
  "useAnotherRule": {
    "group": "style",
    "recommended": false
  }
}
```

Fields:

- `group` is required and must be `a11y`, `complexity`, `correctness`, `performance`, `security`, `style`, or `suspicious`.
- `recommended` is optional. Include it when the requirements specify the final recommendation status; both `true` and `false` are meaningful.
- `severity` is optional. Accepted values are `info`, `warn`, and `error`. Include it when the requirements specify a new final severity.
- `newName` is optional. Omit it when the rule keeps its current name.

Use current names as object keys, even when `newName` is present. A rule implemented by multiple analyzers, such as matching JavaScript and HTML rules, needs only one entry.

## Apply the Plan

Run the task once:

```shell
just promote-rules /tmp/biome-rule-promotions.json
```

The task validates the complete manifest before moving files. It updates declarations, diagnostic categories, fixture option files, rule-option modules, and the migration rename table as applicable. It does not regenerate derived files or snapshots.

Inspect the move before generation:

```shell
git status --short
git diff --stat
```

Confirm that every requested source file and fixture directory moved from `nursery` to its intended group. If a rule was renamed, search for stale occurrences of its old name and classify each occurrence before changing it; historical migration entries are expected to retain old names.

## Generate and Test

Load `testing-codegen` and run the required generators:

```shell
just gen-rules
just gen-configuration
just gen-migrate
```

Run focused tests using each rule's final name. Review and accept pending snapshots individually according to `testing-codegen`; promotion snapshots must show the intended group and severity and must no longer contain the nursery notice.

Then run the analyzer crates affected by the manifest. If any rule was renamed, also run the `biome_migrate` tests and verify that configuration migration moves and renames the old key.

Run the repository-required checks:

```shell
just f
just l
```

Run `just lint-rules` when rule documentation or suppression examples changed.

## Release Entry and Cleanup

Rule promotion is user-facing and requires a minor changeset targeting `next`. Load `changeset` and describe the promoted groups, recommendation changes, severity changes, and renames that users need to know.

Delete the temporary manifest after the promotion is verified. Do not commit it unless the task explicitly requires a persistent promotion plan.
