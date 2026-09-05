---
"@biomejs/biome": patch
---

Fixed [#10531](https://github.com/biomejs/biome/issues/10531): the JavaScript formatter is no longer non-idempotent on member chains whose final call argument is an object literal that exceeds `lineWidth`.

Previously, `biome format --write` followed by `biome check` would report a formatting difference on code like:

```js
id: integer().primaryKey().generatedByDefaultAsIdentity({
	name: "example_id_seq",
	startWith: 1,
	increment: 1,
	minValue: 1,
	maxValue: 2147483647,
	cache: 1,
}),
```

The formatter now produces stable output on the first pass.
