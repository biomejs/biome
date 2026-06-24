---
"@biomejs/biome": patch
---

[`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) now detects missing cases when the discriminant is a union typed through a generic type alias.

```ts
type Id<T> = T;
declare const s: Id<"a" | "b">;
switch (s) {
	case "a":
		break;
}
```
