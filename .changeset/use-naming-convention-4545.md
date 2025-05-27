---
"@biomejs/biome": major
---

Fixed [#4545](https://github.com/biomejs/biome/issues/4545): [`useNamingConvention`](https://biomejs.dev/linter/rules/use-naming-convention/) now correctly ignores declarations inside TypeScript's external modules.

The following interface name is no longer reported by the rule:

```ts
declare module "myExternalModule" {
  export interface my_INTERFACE {}
}
```
