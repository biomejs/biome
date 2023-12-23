---
title: useFilenamingConvention (since vnext)
---

**Diagnostic Category: `lint/nursery/useFilenamingConvention`**

:::caution
This rule is part of the [nursery](/linter/rules/#nursery) group.
:::

Enforce naming conventions for JavaScript and TypeScript filenames.

Enforcing [naming conventions](https://en.wikipedia.org/wiki/Naming_convention_(programming)) helps to keep the codebase consistent.

A filename consists of two parts: a name and a set of consecutive extension.
For instance, `my-filename.test.js` has `my-filename` as name, and two consecutive extensions: `.test` and `.js`.

The name of a filename can start with a dot, be prefixed and suffixed by underscores `_`.
For example, `.filename.js`, `__filename__.js`, or even `.__filename__.js`.

By default, the rule ensures that the filename is either in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case),
or equal to the name of one export in the file.

## Options

The rule provides two options that are detailed in the following subsections.

```json
{
    "//": "...",
    "options": {
        "strictCase": false,
        "enumMemberCase": ["camelCase", "export"]
    }
}
```

### strictCase

When this option is set to `true`, it forbids consecutive uppercase characters in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case).
For instance,  when the option is set to `true`, `agentID` will throw an error.
This name should be renamed to `agentId`.

When the option is set to `false`, consecutive uppercase characters are allowed.
`agentID` is so valid.

Default: `true`

### filenameCases

By default, the rule enforces that the filename  is either in [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case), or equal to the name of one export in the file.

You can enforce a stricter convention by setting `filenameCases` option.
`filenameCases` accepts an array of cases among the following cases: [`camelCase`](https://en.wikipedia.org/wiki/Camel_case), [`kebab-case`](https://en.wikipedia.org/wiki/Letter_case#Kebab_case), [`snake_case`](https://en.wikipedia.org/wiki/Snake_case), and `export`.

## Related links

- [Disable a rule](/linter/#disable-a-lint-rule)
- [Rule options](/linter/#rule-options)
