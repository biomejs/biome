---
"@biomejs/biome": major
---

Reworked some recommended rules recommended to be less pedantic and blocking. This is a **breaking change** if your project relied on those rules to block the CI in case of violations; if that's the case, you should raise their severity level to **error**.

Some rules aren't recommended anymore, and some others return a different severity.

The following rules return a **warning** diagnostic:
- `noDelete`
- `noForEach`
- `noSuspiciousSemicolonInJsx`
- `noThisInStatic`
- `noUnusedLabels`

The following rules return an **information** diagnostic:
- `noUselessCatch`
- `noUselessConstructor`
- `noUselessEmptyExport`
- `noUselessFragments`
- `noUselessLabel`
- `noUselessLoneBlockStatements`
- `noUselessSwitchCase`
- `noUselessTernary`
- `noUselessThisAlias`
- `noUselessTypeConstraint`
- `noFlatMapIdentity`

The following rules aren't recommended anymore:
- `noDelete`
- `noForEach`

The rule `noRenderReturnValue` is only recommended when the `react` domain is enabled.
