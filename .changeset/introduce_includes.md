---
cli: minor
---

# Introduce `includes`

Biome allows users to `include` and `ignore` files in its configuration using glob patterns.

For example, in the following configuration, all files of the `src/` directory are checked except the ones ending with the extension `.test.js`.

```json
{
    "files": {
        "include": ["src/**"],
        "ignore": ["**/*.test.js"]
    }
}
```

Some Biome users have requested the ability to ignore a set of files except some of the files.
With the current system, this is not possible because `include` is always applied before `ignore`.

Also, many Biome users [reported](https://github.com/biomejs/biome/issues/2421) [issues](https://github.com/biomejs/biome/issues/3345) with the behavior of the glob patterns.
Notably:

- `src/**` is interpreted as `**/src/**`
- `*.js` is interpreted as `**/*.js`

To solve all these issues, we introduce a new field `includes`, which replaces both `include` and `ignore`.
`includes` accepts an array of glob patterns with a stricter and more intuitive behavior than the previous glob pattern format.
A glob starting with a `!` is an exception.
This replaces `ignore` patterns.

The previous configuration must be updated as follows:

```json
{
    "files": {
        "includes": ["src/**", "!**/*.test.js"]
    }
}
```
