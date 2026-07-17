---
"@biomejs/biome": patch
---

Fixed [`useExhaustiveSwitchCases`](https://biomejs.dev/linter/rules/use-exhaustive-switch-cases/) for unions of bigint literals. The rule now reports missing bigint cases. For example, this switch now reports the missing `2n` case:

```ts
declare const value: 1n | 2n;
switch (value) {
	case 1n:
		break;
}
```
