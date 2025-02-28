---
"@biomejs/biome": major
---

Remove the code action `quickfix.suppressRule`.

The code action `quickfix.suppressRule` was removed in favour of two new code actions:

- `quickfix.suppressRule.inline.biome`: a code action that adds a suppression comment for each violation.
- `quickfix.suppressRule.topLevel.biome`: a code action that adds a suppression comment at the top of the file which suppresses a rule for the whole file.


Given the following code
```js
let foo = "one";
debugger
```

The code action `quickfix.suppressRule.inline.biome` will result in the following code:
```js
// biome-ignore lint/style/useConst: <explanation>
let foo = "one";
// biome-ignore lint/suspicious/noDebugger: <explanation>
debugger
```

The code action `quickfix.suppressRule.topLevel.biome`, instead, will result in the following code:
```js
/** biome-ignore lint/suspicious/noDebugger: <explanation> */
/** biome-ignore lint/style/useConst: <explanation> */

let foo = "one";
debugger;
```

