---
"@biomejs/biome": patch
---

[useRegexLiterals](https://biomejs.dev/linter/rules/use-regex-literals) now suggests a correct fix when the pattern contains an escaped anti-slash `\/`.

Previously the rule suggested the following fix that led to a syntax error:

```diff
- new RegExp("\/");
+ /\\//
```

The rule now suggests a correct fix:

```diff
- new RegExp("\/");
+ /\//
```

Fixed [#5487](https://github.com/biomejs/biome/issues/5487).
