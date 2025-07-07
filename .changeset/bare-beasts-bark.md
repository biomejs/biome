---
"@biomejs/biome": patch
---

Fixed [#6573](https://github.com/biomejs/biome/issues/6573): Grit plugins can
now match bare imports.

## Example

The following snippet:

```grit
`import $source`
```

will now match:

```ts
import "main.css";
```
