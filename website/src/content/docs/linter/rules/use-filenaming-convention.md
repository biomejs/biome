---
title: useFilenamingConvention (since v1.5.0)
---

**Diagnostic Category: `lint/style/useFilenamingConvention`**

Inspired from: <a href="https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/filename-case.md" target="_blank"><code>filename-case</code></a>

Enforce naming conventions for JavaScript and TypeScript filenames.

Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent.

A filename consists of two parts: a name and a set of consecutive extension.
For instance, `my-filename.test.js` has `my-filename` as name, and two consecutive extensions: `.test` and `.js`.

The filename can start with a dot or a plus sign, be prefixed and suffixed by underscores `_`.
For example, `.filename.js`, `+filename.js`, `__filename__.js`, or even `.__filename__.js`.

The convention of prefixing a filename with a plus sign is used by
[Sveltekit](https://kit.svelte.dev/docs/routing#page) and [Vike](https://vike.dev/route).

By default, the rule ensures that the filename is either in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case),
or equal to the name of one export in the file.

## Options

The rule provides several options that are detailed in the following subsections.

```json
{
    "//": "...",
    "options": {
        "strictCase": false,
        "requireAscii": true,
        "filenameCases": ["camelCase", "export"]
    }
}
```

### strictCase

When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case) and [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case).
For instance,  when the option is set to `true`, `agentID` will throw an error.
This name should be renamed to `agentId`.

When the option is set to `false`, consecutive uppercase characters are allowed.
`agentID` is so valid.

Default: `true`

### requireAscii

When this option is set to `true`, it forbids names that include non-ASCII characters.
For instance,  when the option is set to `true`, `café` or `안녕하세요` will throw an error.

When the option is set to `false`, a name may include non-ASCII characters.
`café` and `안녕하세요` are so valid.

Default: `false`

**This option will be turned on by default in Biome 2.0.**

### filenameCases

By default, the rule enforces that the filename  is either in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case), or equal to the name of one export in the file.

You can enforce a stricter convention by setting `filenameCases` option.
`filenameCases` accepts an array of cases among the following cases: [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`PascalCase`](https://en.wikipedia.org/wiki/Camel_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case), and `export`.

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
