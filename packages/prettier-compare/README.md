# `prettier-compare` Package

## Overview

A CLI tool that compares Prettier and Biome formatting output and IR side-by-side.

## Quick start for agents

Without `--watch`, the tool prints plain text to stdout and exits. It starts no TUI and needs no TTY. Use it instead of writing your own Prettier script.

```bash
# From the repo root
bun packages/prettier-compare/bin/prettier-compare.js --rebuild --output-only -f path/to/input.svelte
```

- The first line of output ends in `[MATCH]` or `[DIFF]`. Biome's output follows, then Prettier's.
- `--output-only` drops the IR. `--ir-only` prints the IR alone. With neither flag you get both.
- The tool formats with the WASM bundle, not `target/debug/biome`. Pass `--rebuild` after every Rust change, or you compare stale output. It runs `just build-wasm-node-dev`.
- The language comes from the extension of the `-f` file. A snippet or stdin defaults to JavaScript, so pass `-l <lang>`. Svelte and Astro run through their Prettier plugins.
- No flag sets formatter options. Biome's configuration is in `src/biome.ts` and Prettier's is in `src/prettier.ts`. To compare under another option, such as `whitespaceSensitivity` or the line width, edit both files locally and leave the edit out of your commit.
- The JS dependencies come from `pnpm install` at the repo root.

## Architecture

```
packages/prettier-compare/
├── src/
│   ├── index.tsx         # Main CLI entry point + React TUI app
│   ├── biome.ts          # Biome formatting via @biomejs/js-api
│   ├── prettier.ts       # Prettier formatting via npm package
│   ├── languages.ts      # Language detection and config mapping
│   ├── components/
│   │   ├── App.tsx           # Main app component
│   │   ├── DiffView.tsx      # Side-by-side diff display
│   │   ├── DiagnosticsView.tsx # Error/diagnostics section
│   │   └── Spinner.tsx       # Loading spinner for rebuilds
│   └── watch.ts          # Watch mode with cargo rebuild + debounce
├── bin/
│   └── prettier-compare.js  # Bin script
├── package.json
├── tsconfig.json
└── README.md
```

## Key Features

1. **Multiple Input Sources**: Snippet argument, file (`--file`), or stdin
2. **Language Detection**: Auto-detect from file extension or specify with `--language`
3. **Side-by-Side Diff**: Show Biome vs Prettier formatted output
4. **IR Comparison**: Show intermediate representation from both formatters
5. **Diagnostics Section**: Display syntax errors from both tools
6. **Watch Mode**: Rebuild WASM on Rust file changes with debounce and spinner
7. **All Languages**: Support JS/TS, JSON, CSS, HTML, Vue, GraphQL, etc.

## Usage

In order to run the tool, you must have the WASM build of Biome available. You can use the `--rebuild` flag to build it automatically if needed, or you can run `just build-wasm-node-dev`.

To run the tool, from the repo root run:

```bash
# Format a snippet
bun packages/prettier-compare/bin/prettier-compare.js "const x={a:1,b:2}"

# Specify language
bun packages/prettier-compare/bin/prettier-compare.js -l ts "const x: number = 1"

# From file
bun packages/prettier-compare/bin/prettier-compare.js -f src/example.tsx

# From stdin
echo "const x = 1" | bun packages/prettier-compare/bin/prettier-compare.js -l js

# Watch mode (rebuilds WASM on Rust changes)
bun --hot packages/prettier-compare/bin/prettier-compare.js -w --watch "function f() { return 1 }"
bun --hot packages/prettier-compare/bin/prettier-compare.js -l html --watch '<tr> <th> A </th> <th> B </th> <th> C </th> </tr>'
```

Note: In watch mode, if you want it to reload the wasm after building biome, then you must use `bun --hot` to run the bin script.
