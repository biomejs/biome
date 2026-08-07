---
"@biomejs/biome": patch
---

Added a new nursery rule [`useFencedCodeLanguage`](https://biomejs.dev/linter/rules/use-fenced-code-language/), which requires fenced code blocks in Markdown to declare a language. It is a port of markdownlint's `MD040`.

For example, the following snippet triggers the rule:

````md
```
console.log(1)
```
````

The rule supports the `allowedLanguages` option to restrict which languages are accepted, and the `languageOnly` option to forbid extra content in the info string beyond the language.
