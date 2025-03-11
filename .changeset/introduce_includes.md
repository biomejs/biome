---
"@biomejs/biome": major
---

Removed `include` and `ignore` fields in favor of the new filed `includes`.

The Biome configuration file allows users to specify which files should be processed using [glob patterns](https://en.wikipedia.org/wiki/Glob_(programming)).
Prior to Biome 2.0, this was done using the `include` and `ignore` fields.
In Biome 2.0, `include` and `ignore` are removed and replaced by `includes`.
You can run `biome migrate` to convert `include` and `ignore` into `includes` automatically.

`includes` uses a different glob pattern format that fixes [many](https://github.com/biomejs/biome/issues/2421) [issues](https://github.com/biomejs/biome/issues/3345) and many other limitations that Biome users reported.

`includes` accepts an array of glob patterns.
A glob pattern starting with a `!` is a negated pattern also called exception.
This replaces `ignore` patterns and allows users to create chains of include and ignore patterns.
Thus, it is now possible to include again a file previously ignored.
This was not possible with `include` and `ignore`, because `ignore` has priority over `include`.

The semantics of `*` and `**/*` have changed too.
Before, with `include` and `ignore`, the glob `*`  was interpreted as `**/*`.
Now, with `includes`, the globs `*` and `**/*` are interpreted differently.
The first pattern matches all files that are inside a folder.
The second pattern recursively matches all files **and sub-folders** inside a folder.

Let's take an example.
Given the following file hierarchy of a project...

```
├── biome.json
├── src
│   ├── file.js
│   ├── file.ts
│   ├── out.gen.js
│   ├── file.test.js
│   └── test
│       └── special.test.js
└── test ...
```

...we want:

1. Ignore all files ending with `.test.js`, except `special.test.ts`.
2. Ignore all files of the `test` directory.
   The `test` directory is located at the root of the project.
3. Execute the linter on files in the `src` directory, that don't end with `.gen.js`.
   The `src` directory is located at the root of the project.
4. Enable the `noDefaultExport` lint rule on files ending with `.ts`.

Prior to Biome 2.0, the configuration might look like:

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

Unfortunately, the configuration doesn't quite fit what we want:

1. There is no way to ignore files and unignore one of them.
   Thus, we ignore all files ending with `.test.js`, including `special.test.ts`.
2. The configuration ignores all directories named `test`, including `src/test`.
3. The linter is executed on all files of all directories named `src` 

All these issues and limitations are fixed with `includes`.
Here the migrated configuration:

```json
{
    "files": {
        "includes": ["**", "!**/*.test.js", "**/special.test.ts", "!test"]
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

1. All files named `special.test.ts` are unignored because the pattern appear after the pattern that ignore files ending with `.test.js`.
2. Only the `test` directory at the project's root is ignored because the pattern doesn't start with `**/`.
3. The linter is executed on the `src` directory at the project's root only.

Because `includes` pattern have a different pattern format than `include` and `ignore` we made some adjustments:

- We added the pattern `**` in `files.includes` to ensure that all files are included before ignoring some of them.
- We added the prefix `**/` for patterns that must match at any level of the file hierarchy.
