---
"@biomejs/biome": minor
---

Added a new option to Biome's Javascript formatter, `operator_linebreak`, to configure whether long lines should be broken before or after binary operators.

For example:

```json
{
  "formatter": {
    "javascript": {
      "operatorLinebreak": "before" // defaults to "after"
    }
  }
}
```

Will cause this JavaScript file

```js
const VERY_LONG_CONDITION_1234123412341234123412341234 = false;

if (VERY_LONG_CONDITION_1234123412341234123412341234 && VERY_LONG_CONDITION_1234123412341234123412341234 && VERY_LONG_CONDITION_1234123412341234123412341234 && VERY_LONG_CONDITION_1234123412341234123412341234) {
  console.log("DONE")
}
```

to be formatted like this:

```js
const VERY_LONG_CONDITION_1234123412341234123412341234 = false;
if (
  VERY_LONG_CONDITION_1234123412341234123412341234
  && VERY_LONG_CONDITION_1234123412341234123412341234
  && VERY_LONG_CONDITION_1234123412341234123412341234
  && VERY_LONG_CONDITION_1234123412341234123412341234
) {
  console.log("DONE")
}
```
