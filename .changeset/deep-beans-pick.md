---
"@biomejs/biome": patch
---

Fixed [#9097](https://github.com/biomejs/biome/issues/9097): [`organizeImports`](https://biomejs.dev/assist/actions/organize-imports/) no longer adds a blank line between a never-matched group and a matched group.

Given the following `organizeImports` options:

```json
{
  "groups": [
    ":NODE:",
    ":BLANK_LINE:",
    ":PACKAGE:",
    ":BLANK_LINE:",
    ":PATH:"
  ]
}
```

The following code...

```js
// Comment
import "package";
import "./file.js";
```

...was organized as:

```diff
+
  // Comment
  import "package";
+
  import "./file.js";
```

A blank line was added even though the group ':NODE:' doesn't match any imports here.
`:BLANK_LINE:` between never-matched groups and matched groups are now ignored.
The code is now organized as:

```diff
  // Comment
  import "package";
+
  import "./file.js";
```
