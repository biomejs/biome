---
"@biomejs/biome": patch
---

Added missing Mocha globals to the `Test` domain: `context`, `run`, `setup`, `specify`, `suite`, `suiteSetup`, `suiteTeardown`, `teardown`, `xcontext`, `xdescribe`, `xit`, and `xspecify`. These are injected by Mocha's BDD and TDD interfaces and were previously flagged as undeclared variables in projects using Mocha.
