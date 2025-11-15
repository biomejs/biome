---
"@biomejs/biome": patch
---

Fixed [`noInvalidUseBeforeDeclaration`](https://biomejs.dev/linter/rules/no-invalid-use-before-declaration/).
The rule no longer reports a use of an ambient variable before its declarations.
The rule also completely ignores TypeScript declaration files.
The following code is no longer reported as invalid:

```ts
CONSTANT;
declare const CONSTANT: number;
```
