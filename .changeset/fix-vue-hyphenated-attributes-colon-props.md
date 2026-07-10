---
"@biomejs/biome": patch
---

Fixed [#10776](https://github.com/biomejs/biome/issues/10776): [`useVueHyphenatedAttributes`](https://biomejs.dev/linter/rules/use-vue-hyphenated-attributes/) no longer flags colon-separated attributes such as PrimeVue pass-through props (e.g. `pt:header:data-test-id`). Each colon-separated segment is now checked individually, so an attribute is valid when every segment is already hyphenated.
