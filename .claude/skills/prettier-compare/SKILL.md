---
name: prettier-compare
description: Compares code formatting and formatting IR between Biome and Prettier to ensure that Biome's formatting is consistent and correct.
---

## Purpose

Use `packages/prettier-compare/` to inspect any differences between Biome and Prettier formatting (including IR output) before shipping formatter changes.

## Prerequisites

1. Run every command from the repository root so relative paths resolve correctly.
2. Use `bun` (the CLI is a Bun script) and ensure dependencies have been installed.
3. Always pass `--rebuild` so the Biome WASM bundle matches your current Rust changes.

## Common workflows

Snippets passed as CLI args:

```
bun packages/prettier-compare/bin/prettier-compare.js --rebuild 'const x={a:1,b:2}'
```

Force a language (useful when the tool cannot infer it from a filename):

```
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -l ts 'const x: number = 1'
```

Compare files on disk:

```
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -f src/example.tsx
```

Read from stdin (great for piping editor selections):

```
echo 'const x = 1' | bun packages/prettier-compare/bin/prettier-compare.js --rebuild -l js
```

## Tips

- Use `-l/--language` when formatting code without an extension so both formatters pick the correct parser.
- Use `-f/--file` for large samples or snapshot tests so you can iterate directly on project fixtures.
- Reference `packages/prettier-compare/README.md` for deeper CLI details; mirror any updates here, keeping the hard requirement that commands include `--rebuild`.
- Use single quotes for code snippets passed as CLI arguments to avoid shell interpretation issues.
- "\n" does not get escaped into a newline when passed as a CLI argument. You should write a literal newline or use a file instead.
