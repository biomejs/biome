---
"@biomejs/biome": major
---

Replace `include` and `ignore` with `includes`.

The Biome configuration file allows users to specify which files should be processed using [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).
Prior to Biome 2.0, this was done using the `include` and `ignore` fields.
In Biome 2.0, `include` and `ignore` are removed and replaced by `includes`.
You can run `biome migrate` to convert `include` and `ignore` into `includes` automatically.

`includes` uses a different glob pattern format that fixes [many](https://github.com/biomejs/biome/issues/2421) [issues](https://github.com/biomejs/biome/issues/3345) and limitations that Biome users reported.

`includes` accepts an array of glob patterns.
A glob pattern starting with a `!` is an exception.
This replaces `ignore` patterns and allows users to create chains of include and ignore patterns.
Thus, it is now possible to include again a file previously ignored.
This was not possible with `include` and `ignore`, because `ignore` was always applied after `include`.

`includes` handles `*` and `**/*` differently.
The first pattern matches all files in the directory in which the Biome configuration file is.
The second pattern matches all files.
`include` and `ignore` interprets `*` as `**/*`.

Let's take a Biome configuration with `include`/`ignore` and convert it into the new `includes` field.
In the following configuration, all files of the `test` directory and all files ending with the extension `.test.js` are ignored.
The linter is only executed on files if the `src` directory that doesn't end with the `.gen.js` extension.
Also, an extra rule is enabled on files ending with the `.ts` extension.

```json
{
    "files": {
        "ignore": ["*.test.js", "test"]
    },
    "linter": {
        "include": ["src/**"],
        "ignore": ["*.gen.js"],
        "enabled": true
    },
    "overrides": [{
        "include": ["*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
    }]
}
```

The Biome configuration is migrated to the following one:

```json
{
    "files": {
        "includes": ["**", "!**/*.test.js", "!test"]
    },
    "linter": {
        "includes": ["src/**", "!**/*.gen.js"],
        "enabled": true
    },
    "overrides": [{
        "includes": ["**/*.ts"],
        "linter": { "rules": { "style": { "noDefaultExport": "on" } } }
    }]
}
```

Note that we have to add the glob pattern `**` to ensure that files got included.
Otherwise, no files are included.
