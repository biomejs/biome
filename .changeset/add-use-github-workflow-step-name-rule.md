---
"@biomejs/biome": patch
---

Added the nursery rule [`useGithubWorkflowStepName`](https://biomejs.dev/linter/rules/use-github-workflow-step-name/), which requires the steps of GitHub Actions workflows to have a non-empty `name`.

```yaml
# .github/workflows/ci.yml
name: CI
on: push
jobs:
  build:
    name: Build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
```
