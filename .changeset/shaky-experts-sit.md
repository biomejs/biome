---
"@biomejs/biome": patch
---

Fixed [#6799](https://github.com/biomejs/biome/issues/6799): The [`noImportCycles`](https://biomejs.dev/linter/rules/no-import-cycles/) rule now ignores type-only imports if the new `ignoreTypes` option is enabled (enabled by default).

> [!WARNING]
> **Breaking Change**: The `noImportCycles` no longer detect import cycles that includes one or more type-only imports.
> To keep the old behaviour, you can turn off the `ignoreTypes` option explicitly:
>
> ```json
> {
>   "linter": {
>     "rules": {
>       "nursery": {
>         "noImportCycles": {
>           "options": {
>             "ignoreTypes": false
>           }
>         }
>       }
>     }
>   }
> }
> ```
