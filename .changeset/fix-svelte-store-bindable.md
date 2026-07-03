---
"@biomejs/biome": patch
---

Fixed [`noUnusedVariables`](https://biomejs.dev/linter/rules/no-unused-variables/) false positives in Svelte files: Svelte store subscriptions (`$store` references in templates now keep the underlying `store` binding from being flagged), and `$bindable()` props that are only written to in the script block (write-only is intentional for bindable props) are no longer reported as unused.
