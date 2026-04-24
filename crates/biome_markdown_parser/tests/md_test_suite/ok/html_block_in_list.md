- Added two new options to customise the emitted output of the CLI: `--reporter=json` and `--reporter=json-pretty`. With
  `--reporter=json`, the diagnostics and the summary will be printed in the **terminal** in JSON format. With
  `--reporter=json-pretty`, you can print the same information, but formatted using the same options of your configuration.

  NOTE: the shape of the JSON is considered experimental, and the shape of the JSON might change in the future.

  <details>
  <summary>Example of output when running `biome format` command</summary>
  ```json
  {
    "summary": {
      "changed": 0,
      "unchanged": 1,
      "errors": 1,
      "warnings": 0,
      "skipped": 0,
      "suggestedFixesSkipped": 0,
      "diagnosticsNotPrinted": 0
    },
    "diagnostics": [
      {
        "category": "format",
        "severity": "error",
        "description": "Formatter would have printed the following content:",
        "message": [
          {
            "elements": [],
            "content": "Formatter would have printed the following content:"
          }
        ],
        "advices": {
          "advices": [
            {
              "diff": {
                "dictionary": "  statement();\n",
                "ops": [
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "equal": { "range": [2, 12] } } },
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "equal": { "range": [12, 13] } } },
                  { "diffOp": { "delete": { "range": [0, 2] } } },
                  { "diffOp": { "insert": { "range": [13, 15] } } }
                ]
              }
            }
          ]
        },
        "verboseAdvices": { "advices": [] },
        "location": {
          "path": { "file": "format.js" },
          "span": null,
          "sourceCode": null
        },
        "tags": [],
        "source": null
      }
    ],
    "command": "format"
  }
  ```
  </details>
