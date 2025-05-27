---
"@biomejs/biome": minor
---

An option called `allowNoReferrer` has been added to the
[`noBlankTarget`](https://biomejs.dev/linter/rules/no-blank-target/) rule.

By default, `noBlankTarget` accepts both `rel="noopener"` and `rel="noreferrer"`
with links that have `target="_blank"`. This is because the latter _implies_ the
former, so either one is sufficient to mitigate the security risk.

However, allowing `rel="noreferrer"` may still be undesirable, because it can
break tracking, which may be an undesirable side-effect. As such, you can set
`allowNoReferrer: false` to _only_ accept `rel="noopener"`.
