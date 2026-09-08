---
"@biomejs/biome": patch
---

[`noTemplateCurlyInString`](https://biomejs.dev/linter/rules/no-template-curly-in-string/) no longer reports GitHub Actions expressions like `"${{ inputs.abc }}"` as errors. These use double curly braces (`${{ }}`) which are distinct from JavaScript template literal placeholders (`${}`).
