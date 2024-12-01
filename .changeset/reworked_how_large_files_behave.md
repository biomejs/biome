---
cli: major
---

# Reworked how large files behave

Previously, files that should exceed the configured size limit would throw an error, and the CLI would exit with an error code.

Now, the CLI ignores the file, emits a *information* diagnostic and doesn't exit with an error code.
