---
"@biomejs/biome": patch
---

Fixed [#13280](https://github.com/biomejs/biome/issues/13280): the suppression parser no longer panics on non-ASCII whitespace used as a separator between category specs. Such whitespace is now a parsing error, including `lint ␣format`, `lint(a)␣format` and `lint(a)␣:` (where `␣` is U+00A0), which were previously accepted. Leading (including the comment prefix and the directive boundary), trailing, value and reason whitespace remains Unicode-aware.
