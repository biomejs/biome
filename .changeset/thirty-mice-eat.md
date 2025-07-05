---
"@biomejs/biome": minor
---

Added a new CLI argument `--verbosity`, which replaced the now deprecated `--verbose` argument. The argument accepts three values, which behave as follow:
- `--verboity=simple`, the default value, which prints simple diagnostics.
- `--verbosity=full`, which replaces the functionality provided by `--verbose`. It prints additional diagnostics and information.
- `--verbosity=minimal`, which prints only the "header" of non-verbose diagnostics
  ```
  check.js:1:6 lint/correctness/noConstantCondition
  check.js:1:6 lint/correctness/noConstantCondition
  check.js:1:6 lint/correctness/noConstantCondition
  ```
