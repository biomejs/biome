# Biome JavaScript Bindings

Official JavaScript bindings for [Biome](https://biomejs.dev/)

> **Warning**:
> The API is currently in alpha. It is not yet ready for production use. We appreciate your support and feedback as we work to make it ready for everyone.

## Installation

```shell
npm i @biomejs/js-api
npm i @biomejs/wasm-<dist>
```

You need to install one of the `@biomejs/wasm-*` package as a **peer dependency** for this package to work correctly, out of the following distributions:
- `@biomejs/wasm-bundler`: Install this package if you're using a bundler that supports importing `*.wasm` files directly
- `@biomejs/wasm-nodejs`: Install this package if you're using Node.js to load the WebAssembly bundle use the `fs` API
- `@biomejs/wasm-web`: Install this package if you are targeting the web platform to load the WASM bundle using the `fetch` API

## Usage

```js
import { Biome, Distribution } from "@biomejs/js-api";

const biome = await Biome.create({
	distribution: Distribution.NODE, // Or BUNDLER / WEB depending on the distribution package you've installed
});

const formatted = biome.formatContent("function f   (a, b) { return a == b; }", {
	filePath: "example.js",
});

console.log('Formatted content: ', formatted.content);

const result = biome.lintContent(formatted.content, {
	filePath: "example.js",
});

const html = biome.printDiagnostics(result.diagnostics, {
	filePath: "example.js",
	fileSource: formatted.content,
});

console.log('Lint diagnostics: ', html);
```

## Philosophy

The project philosophy can be found on our [website](https://biomejs.dev/internals/philosophy/).

## Community

Contribution and development instructions can be found in [CONTRIBUTING](../../CONTRIBUTING.md).

Additional project coordination and real-time discussion happens on our [Discord server](https://discord.gg/BypW39g6Yc). Remember that all activity on the Discord server is still moderated and will be strictly enforced under the project's [Code of Conduct](../../CODE_OF_CONDUCT.md).
