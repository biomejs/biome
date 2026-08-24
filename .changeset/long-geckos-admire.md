---
"@biomejs/biome": patch
---

Fixed [#10776](https://github.com/biomejs/biome/issues/10776): [`useVueHyphenatedAttributes`](https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes/) no longer reports lowercase attribute names containing punctuation, such as `pt:header:data-test-id` and `some_attr`.
