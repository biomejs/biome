---
"@biomejs/biome": patch
---

The documentation & rule sources for [`lint/complexity/noBannedTypes`](https://biomejs.dev/linter/rules/no-banned-types) have been updated to fix a few oversights.

In addition to some general typo fixes:
- The rule now recommends `Record<keyof any, never>` instead of `Record<string, never>` (the latter of which incorrectly allows symbol-keyed properties).
- The rule mentions an alternate method to enforce object emptiness involving `unique symbol`-based guards used by [`type-fest`](https://github.com/sindresorhus/type-fest/blob/main/source/empty-object.d.ts) and [many other packages](https://github.com/search?q=lang%3ATypeScript+%2Ftype%5Cs*%5Cw%2B%5Cs*%3D%5Cs*%5C%7B%5Cs*%5C%5B%5Cw%2B%5C%5D%5C%3F%3A+never%2F&type=code):
  ```ts
  declare const mySym: unique symbol;
  
  // Since this type's only property is an unexported `unique symbol`, nothing that imports it can specify any properties directly
  // (as far as excess property checks go)
  export type EmptyObject = { [mySym]?: never };
  export type IsEmptyObject<T> = T extends EmptyObject ? true : false;
  ```

The rule's listed sources have been updated as well to reflect the original source rule (`ban-types`) having been [split into 3 separate rules](https://github.com/typescript-eslint/typescript-eslint/pull/8977) circa April 2024.
