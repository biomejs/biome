---
"@biomejs/biome": minor
---

The `noRestrictedImports` rule has been enhanced with a new `patterns` option. This option allows for more flexible and powerful import restrictions using gitignore-style patterns.

You can now define patterns to restrict entire groups of modules. For example, you can disallow imports from any path under `import-foo/` except for `import-foo/baz`.

```json
{
  "options": {
    "patterns": [
      {
        "group": ["import-foo/*", "!import-foo/baz"],
        "message": "import-foo is deprecated, except for modules in import-foo/baz."
      }
    ]
  }
}
```

**Invalid examples**
```js
import foo from 'import-foo/foo';
import bar from 'import-foo/bar';
```

**Valid examples**
```js
import baz from 'import-foo/baz';
```

Additionally, the `patterns` option introduces `importNamePattern` to restrict specific import names using regular expressions.
The following example restricts the import names that match `x` , `y` or `z` letters from modules under `import-foo/`.
```json
{
  "options": {
    "patterns": [
      {
        "group": ["import-foo/*"],
        "importNamePattern": "[xyz]"
      }
    ]
  }
}
```
**Invalid examples**
```js
import { x } from 'import-foo/foo';
```

**Valid examples**
```js
import { foo } from 'import-foo/foo';
```

Furthermore, you can use the `invertImportNamePattern` boolean option to reverse this logic. When set to true, only the import names that match the `importNamePattern` will be allowed. The following configuration only allows the import names that match `x` , `y` or `z` letters from modules under `import-foo/`.
```json
{
  "options": {
    "patterns": [
      {
        "group": ["import-foo/*"],
        "importNamePattern": "[xyz]",
        "invertImportNamePattern": true
      }
    ]
  }
}
```
**Invalid examples**
```js
import { foo } from 'import-foo/foo';
```

 **Valid examples**
```js
import { x } from 'import-foo/foo';
```
