---
"@biomejs/biome": patch
---

Added the nursery rule [`useGithubWorkflowJobName`](https://biomejs.dev/linter/rules/use-github-workflow-job-name/), which requires the jobs of GitHub Actions workflows to have a non-empty `name`.

```yaml
# .github/workflows/ci.yml
name: CI
on: push
jobs:
  build:
    runs-on: ubuntu-latest
```
