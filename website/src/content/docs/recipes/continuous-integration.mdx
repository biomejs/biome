---
title: Continuous Integration
description: Using Biome in a CI environment
---

Running Biome in a CI environment is easy. Check out the following examples for some inspiration.

## GitHub Actions

We provide a first-party [GitHub Action](https://github.com/marketplace/actions/setup-biome) to setup Biome in your runner.
Here's what a simple workflow might look like:

```yaml title="pull_request.yml"
name: Code quality

on:
  push:
  pull_request:

jobs:
  quality:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4
      - name: Setup Biome
        uses: biomejs/setup-biome@v2
        with:
          version: latest
      - name: Run Biome
        run: biome ci .
```

### Third-party actions

These are actions maintained by other communities, that you use in your runner:

- [reviewdog-action-biome](https://github.com/marketplace/actions/run-biome-with-reviewdog): run Biome with reviewdog and make comments and commit suggestions on the pull request.

```yaml title="pull_request.yml"
name: reviewdog
on: [pull_request]
jobs:
  biome:
    name: runner / Biome
    runs-on: ubuntu-latest
    permissions:
      contents: read
      pull-requests: write
    steps:
      - uses: actions/checkout@v4
      - uses: mongolyy/reviewdog-action-biome@v1
        with:
          github_token: ${{ secrets.github_token }}
          reporter: github-pr-review
```
