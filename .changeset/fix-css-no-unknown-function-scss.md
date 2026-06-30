---
"@biomejs/biome": patch
---

Fixed [#10800](https://github.com/biomejs/biome/issues/10800): [`noUnknownFunction`](https://biomejs.dev/linter/rules/no-unknown-function/) no longer reports false positives in SCSS. SCSS allows user-defined `@function`s and provides built-in module functions (such as `nth`, `length`, and `map.get`) that are not CSS value functions, so the rule now skips SCSS files.

```scss
@function fibonacci($n) {
	$sequence: 0 1;
	@for $_ from 1 through $n {
		$sequence: append($sequence, nth($sequence, length($sequence)));
	}
	@return nth($sequence, length($sequence));
}

.sidebar {
	margin-left: fibonacci(4) * 1px;
}
```
