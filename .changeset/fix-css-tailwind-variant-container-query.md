---
"@biomejs/biome": patch
---

Fixed [#11311](https://github.com/biomejs/biome/issues/11311): the CSS parser now accepts Tailwind container-query variant names in `@variant`, such as `@xl` and `@max-xl`. These previously produced a parse error and a [`noUnknownAtRules`](https://biomejs.dev/linter/rules/no-unknown-at-rules/) diagnostic.

```css
@variant @xl {
	div {
		background: red;
	}
}
```
