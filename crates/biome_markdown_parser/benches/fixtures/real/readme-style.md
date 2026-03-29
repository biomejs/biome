# Vortex

[![CI Status](https://img.shields.io/github/actions/workflow/status/vortex-org/vortex/ci.yml?branch=main)](https://github.com/vortex-org/vortex/actions)
[![npm version](https://img.shields.io/npm/v/vortex-bundler.svg)](https://www.npmjs.com/package/vortex-bundler)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Downloads](https://img.shields.io/npm/dm/vortex-bundler.svg)](https://www.npmjs.com/package/vortex-bundler)

A blazing-fast module bundler written in Rust, designed for modern web
applications. Vortex leverages incremental compilation and parallel processing
to deliver sub-second build times even on large codebases.

![Vortex Architecture Diagram](https://vortex-bundler.dev/assets/architecture.png)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [API Reference](#api-reference)
- [Plugin System](#plugin-system)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Incremental builds** with persistent caching across runs
- **Tree shaking** with full ESM and CommonJS support
- **Code splitting** with automatic chunk optimization
- **Hot Module Replacement** with sub-50ms update times
- **TypeScript** support out of the box
- **CSS Modules** and PostCSS integration built in

## Installation

Install Vortex using your preferred package manager:

```bash
# npm
npm install --save-dev vortex-bundler

# pnpm
pnpm add -D vortex-bundler
```

You can also install the CLI globally:

```bash
npm install -g vortex-bundler
```

Verify your installation:

```bash
vortex --version
```

### System Requirements

- Node.js 18.0 or later
- npm 9.0 or later (or equivalent pnpm/yarn)
- macOS, Linux, or Windows 10+

## Quick Start

Create a minimal project structure:

```bash
mkdir my-app && cd my-app
npm init -y
npm install --save-dev vortex-bundler
```

Add a build script to your `package.json`:

```json
{
  "scripts": {
    "dev": "vortex serve",
    "build": "vortex build",
    "preview": "vortex preview"
  }
}
```

Create an entry point at `src/index.ts`:

```typescript
import { createApp } from "./app";

const root = document.getElementById("root");
if (root) {
  const app = createApp();
  app.mount(root);
}
```

Then run the dev server:

```bash
npm run dev
```

Your application will be available at `http://localhost:3000`.

## Configuration

Vortex uses a `vortex.config.ts` file at the project root. All options are
optional and have sensible defaults.

```typescript
import { defineConfig } from "vortex-bundler";

export default defineConfig({
  entry: "./src/index.ts",
  output: {
    dir: "dist",
    format: "esm",
  },
  resolve: {
    alias: {
      "@": "./src",
    },
  },
});
```

### Configuration Options

- **entry**: The entry point or entry points for the bundle
    - Can be a string or an object mapping names to paths
    - Defaults to `./src/index.ts` or `./src/index.js`
- **output**: Controls the output format and location
    - `dir`: Output directory (default: `dist`)
    - `format`: Module format, one of `esm`, `cjs`, or `iife`
    - `sourcemap`: Enable source maps (default: `true`)
- **resolve**: Module resolution settings
    - `alias`: Path aliases for imports
    - `extensions`: File extensions to resolve
- **plugins**: An array of Vortex plugins
    - Plugins are applied in order
    - Each plugin can hook into the build lifecycle

#### Advanced Options

- **experimental.threads**: Number of worker threads (default: CPU count)
- **experimental.persistentCache**: Enable disk-based caching (default: `true`)
- **experimental.lazyCompilation**: Compile modules on demand in dev mode

## API Reference

Vortex exposes a programmatic API for advanced use cases.

### Core Functions

| Function | Parameters | Return Type | Description |
|----------|-----------|-------------|-------------|
| `build` | `BuildOptions` | `Promise<BuildResult>` | Run a production build |
| `serve` | `ServeOptions` | `Promise<DevServer>` | Start the dev server |
| `preview` | `PreviewOptions` | `Promise<PreviewServer>` | Preview a production build |
| `resolveConfig` | `string` | `Promise<ResolvedConfig>` | Load and resolve a config file |

### BuildResult

| Property | Type | Description |
|----------|------|-------------|
| `outputs` | `OutputFile[]` | Generated output files |
| `duration` | `number` | Build duration in milliseconds |
| `errors` | `Diagnostic[]` | Build errors, if any |
| `warnings` | `Diagnostic[]` | Build warnings |

## Plugin System

Plugins follow a simple hook-based architecture:

```typescript
import { createPlugin } from "vortex-bundler";

export const myPlugin = createPlugin({
  name: "my-plugin",
  setup(build) {
    build.onResolve({ filter: /\.csv$/ }, (args) => {
      return { path: args.path, loader: "json" };
    });

    build.onLoad({ filter: /\.csv$/ }, async (args) => {
      const contents = await readFile(args.path, "utf8");
      const parsed = parseCSV(contents);
      return { contents: JSON.stringify(parsed) };
    });
  },
});
```

## Contributing

We welcome contributions of all kinds. Please read our contributing guide before
submitting a pull request.

1. Fork the repository on GitHub
2. Clone your fork locally: `git clone https://github.com/your-name/vortex.git`
3. Create a feature branch: `git checkout -b feat/my-feature`
4. Make your changes and add tests
5. Run the test suite: `cargo test`
6. Format your code: `cargo fmt`
7. Push to your fork and open a pull request

Please make sure all tests pass and your code follows the existing style.

## License

Vortex is licensed under the [MIT License](LICENSE).

Copyright 2025 Vortex Contributors.

---

Built with Rust. Maintained by the [Vortex team](https://github.com/vortex-org).
