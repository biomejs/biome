# `@biomejs/prettier-compare` Package

## Overview

Create a Node.js CLI tool at `packages/@biomejs/prettier-compare` that compares Prettier and Biome formatting output and IR side-by-side. Uses OpenTUI (with React renderer) for the terminal UI, including spinners during WASM rebuilds and debounced watch mode.

## Architecture

```
packages/@biomejs/prettier-compare/
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
7. **All Languages**: Support JS/TS, JSON, CSS, HTML, GraphQL, etc.

## Usage

```bash
# Format a snippet
prettier-compare "const x={a:1,b:2}"

# Specify language
prettier-compare -l ts "const x: number = 1"

# From file
prettier-compare -f src/example.tsx

# From stdin
echo "const x = 1" | prettier-compare -l js

# Watch mode (rebuilds WASM on Rust changes)
prettier-compare -w "function f() { return 1 }"
```

Note: In watch mode, if you want it to reload the wasm after building biome, then you must use `bun --hot` to run the bin script.
