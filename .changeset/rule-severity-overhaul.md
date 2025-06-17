---
"@biomejs/biome": major
---

Update the default severity level of lint rules.

Every diagnostic emitted by Biome has a severity level set to `error`, `warn`, or `info`.
Previously, all recommended lint rules had a default severity level set to `error`.
All other lint rules had a default severity level set to `warn`.

We have adjusted the default severity level of every rule, whether recommended or not, to better communicate the _severity_ that a diagnostic highlights.

- Rules that report hard errors, likely erroneous code, dangerous code, or accessibility issues now have a default severity level of `error`.
- Rules that report possibly erroneous codes, or code that could be cleaner if rewritten in another way now have a default severity level of `warn`.
- Rules that reports stylistic suggestions now have a default severity level of `info`.

You can use the CLI option `--diagnostic-level=error` to display only errors, or `--diagnostic-level=warning` to display both errors and warnings.
By default, all diagnostics are shown.
You can also use the CLI option `--error-on-warnings` to make the command fail when warnings are emitted.
