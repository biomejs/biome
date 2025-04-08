---
"@biomejs/biome": minor
---

Added support for formatting `.html` files. The formatting is considered **experimental,** and it's only opt-in via configuration:

```json
{
  "html": {
    "formatter": {
      "enabled": true
    }
  }
}
```

Biome formatter attempts to format as Prettier, however some default options might differ.

An option `html.formatter.selfCloseVoidElements` allows to control whether the trailing `/` of [void elements](https://html.spec.whatwg.org/#void-elements) should be printed.

**By default**, Biome formatter will *remove* the `/`:

```diff
- <input />
+ <input>
```

If you come from Prettier and you want to keep the same formatting behaviour, you should set the option to `"always"`:

```json
{
  "html": {
    "formatter": {
      "selfCloseVoidElements": "always"
    }
  }
}
```

```diff
- <input>
+ <input />
```

Use to the command `biome migrate prettier` to apply this change automatically.
