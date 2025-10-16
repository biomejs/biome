---
"@biomejs/biome": minor
---

Deprecated the option `files.experimentalScannerIgnores` in favour of **force-ignore** syntax in `files.includes`.

`files.includes` supports ignoring files by prefixing globs with an exclamation mark (`!`). With this change, it also supports _force_-ignoring globs by prefixing them with a double exclamation mark (`!!`).

The effect of force-ignoring is that the scanner will not index files matching the glob, even in project mode, even if those files are imported by other files, and even if they are files that receive special treatment by Biome, such as nested `biome.json` files.

#### Example

Let's take the following configuration:

```json
{
    "files": {
        "includes": [
            "**",
            "!**/generated",
            "!!**/dist",
            "fixtures/example/dist/*.js"
        ]
    },
    "linter": {
        "domains": {
            "project": "all"
        }
    }
}
```

This configuration achieves the following:

- Because the [project domain](https://biomejs.dev/linter/domains/#project) is enabled, all supported files in the project are indexed _and_ processed by the linter, _except_:
- Files inside a `generated` folder are not processed by the linter, but they will get indexed _if_ a file outside a `generated` folder imports them.
- Files inside a `dist` folder are never indexed nor processed, not even if they are imported for any purpose, _except_:
- When the `dist` folder is inside `fixtures/example/`, its `.js` files _do_ get both indexed and processed.

In general, we now recommend using the force-ignore syntax for any folders that contain _output_ files, such as `build/` and `dist/`. For such folders, it is highly unlikely that indexing has any useful benefits. For folders containing generated files, you may wish to use the regular ignore syntax so that type information can still be extracted from the files.

`experimentalScannerIgnores` will continue to work for now, but you'll see a deprecation warning if you still use it.

Run the `biome migrate --write` command to automatically update the configuration file.
