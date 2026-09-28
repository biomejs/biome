---
"@biomejs/biome": patch
---

Fixed [#11939](https://github.com/biomejs/biome/issues/11939): the fix of [`useRegexLiterals`](https://biomejs.dev/linter/rules/use-regex-literals/) no longer escapes a slash that is already escaped. `new RegExp("\\/")` is now fixed to `/\//` instead of the invalid `/\\//`.
