---
"@biomejs/biome": minor
---

Added a new reporter named `rdjson`. This reporter prints diagnostics following the [RDJSON format](https://deepwiki.com/reviewdog/reviewdog/3.2-reviewdog-diagnostic-format):

The following command:

```shell
biome check --reporter=rdjson
```

Will emit diagnostics in the following format:

```json
{
  "source": {
    "name": "Biome",
    "url": "https://biomejs.dev"
  },
  "diagnostics": [
    {
      "code": {
        "url": "https://biomejs.dev/linter/rules/no-unused-imports",
        "value": "lint/correctness/noUnusedImports"
      },
      "location": {
        "path": "index.ts",
        "range": {
          "end": {
            "column": 11,
            "line": 0
          },
          "start": {
            "column": 7,
            "line": 0
          }
        }
      },
      "message": "This import is unused."
    },
    {
      "code": {
        "url": "https://biomejs.dev/linter/rules/no-unused-imports",
        "value": "lint/correctness/noUnusedImports"
      },
      "location": {
        "path": "index.ts",
        "range": {
          "end": {
            "column": 10,
            "line": 1
          },
          "start": {
            "column": 9,
            "line": 1
          }
        }
      },
      "message": "Several of these imports are unused."
    }
  ]
}
```
