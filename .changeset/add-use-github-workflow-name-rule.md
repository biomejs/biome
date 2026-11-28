---
"@biomejs/biome": patch
---

Added the nursery rule [`useGithubWorkflowName`](https://biomejs.dev/linter/rules/use-github-workflow-name/), which requires GitHub Actions workflows to have a non-empty top-level `name`.

```yaml
# .github/workflows/ci.yml
on: push
jobs:
  build:
    runs-on: ubuntu-latest
```
