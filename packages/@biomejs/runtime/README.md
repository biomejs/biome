# @biomejs/runtime

JavaScript APIs for writing and testing [Biome](https://biomejs.dev) JavaScript plugins.

```sh
npm install --save-dev @biomejs/biome @biomejs/runtime
```

The package is ESM-only.

## Writing a plugin

`@biomejs/runtime/plugin` provides the APIs available to a plugin: AST and
semantic queries, diagnostics and code fixes.

```js
// noDebugger.js
import {
	ast,
	createMutation,
	defineRule,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

export const noDebugger = defineRule({
	query: ast("JS_DEBUGGER_STATEMENT"),
	run(node) {
		const mutation = createMutation(node);
		mutation.removeNode(node);
		registerDiagnostic(node, "error", "Remove debugger.", {
			mutation,
			message: "Remove debugger",
			kind: "safe",
		});
	},
});
```

Enable it in your Biome configuration:

```json
{
  "plugins": [
    "./noDebugger.js"
  ]
}
```

Plugins run inside Biome. Importing `@biomejs/runtime/plugin` from Node.js
throws an error.

## Testing a plugin

`@biomejs/runtime/plugin/testing` provides `RuleTester`, which registers each
test case as a test of your test runner, runs the plugin in the Biome daemon and
checks the diagnostics and fixes it produces.

```js
// noDebugger.test.js
import { RuleTester } from "@biomejs/runtime/plugin/testing";

const tester = new RuleTester();

tester.run("noDebugger", "./noDebugger.js", {
	valid: ["const value = 1;"],
	invalid: [
		{
			code: "debugger;",
			diagnostics: [{ message: "Remove debugger.", severity: "error" }],
			output: "",
		},
	],
});
```

```text
▶ noDebugger
  ▶ valid
    ✔ const value = 1;
  ▶ invalid
    ✔ debugger;
```

### Test runners

`RuleTester` uses the global `describe`, `it` and `afterAll` (or `after`) of
runners such as Vitest with `globals: true`, Jest and Mocha. Without globals, it
falls back to `node:test`, so `node --test` works out of the box. To use another
runner, or Vitest without globals, set the hooks once, for example in a setup
file:

```js
import { afterAll, describe, it } from "vitest";
import { RuleTester } from "@biomejs/runtime/plugin/testing";

RuleTester.describe = describe;
RuleTester.it = it;
RuleTester.afterAll = afterAll;
```

### `new RuleTester(options?)`

| Option       | Description                                                                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| `binaryPath` | Path to the Biome binary. Defaults to the `BIOME_BINARY` environment variable, then to the prebuilt binary for the current platform. |
| `timeout`    | Milliseconds to wait for the connection and for each request. Defaults to `30000`.                                                   |
| `filename`   | Default filename of the test cases. Defaults to `input.js`.                                                                          |
| `config`     | Default parser settings: `{ javascript: { parser: { jsxEverywhere, unsafeParameterDecoratorsEnabled } } }`.                          |

### `tester.run(name, plugin, { valid, invalid })`

Registers a group named `name`, with `valid` and `invalid` subgroups and one
test per case. `plugin` is a file URL, or a path relative to the current working
directory. The connection to Biome is closed after the group's tests.

**Every rule the module exports runs**, so put the rule under test in a module of
its own.

A valid case passes if the plugin reports no diagnostic. It is a source string
or:

| Field      | Description                                                                                         |
| ---------- | --------------------------------------------------------------------------------------------------- |
| `code`     | The source code.                                                                                    |
| `name`     | The test name. Defaults to `code`.                                                                  |
| `filename` | Overrides the tester's `filename`. Its extension selects the language, for example `.ts` or `.cjs`. |
| `config`   | Overrides the tester's `config`.                                                                    |

An invalid case passes if the plugin reports the expected diagnostics. In
addition to the fields of a valid case:

| Field         | Description                                                                                             |
| ------------- | ------------------------------------------------------------------------------------------------------- |
| `diagnostics` | The number of expected diagnostics, or an array of expectations in report order.                        |
| `output`      | The expected source after applying fixes, or `null` if fixes must not change it. Omit it to skip fixes. |
| `fixMode`     | `"safe"` (default) applies safe fixes only; `"unsafe"` applies unsafe fixes too.                        |

Each expectation may check any of these fields of a diagnostic:

| Field      | Description                                                            |
| ---------- | ---------------------------------------------------------------------- |
| `message`  | The message, or a `RegExp` that matches it.                            |
| `severity` | `"hint"`, `"information"`, `"warning"`, `"error"` or `"fatal"`.        |
| `start`    | `{ line, column }` of the start, one-based; columns count code points. |
| `end`      | `{ line, column }` of the end.                                         |
