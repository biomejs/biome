# @biomejs/resolver

> [!WARNING]
> This package is currently shipped as alpha. Its APIs could change from one release to another.

<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="700">
  </picture>

  <br>
  <br>

[![Discord chat][discord-badge]][discord-url]
[![npm version][npm-badge]][npm-url]

[discord-badge]: https://badgen.net/discord/online-members/BypW39g6Yc?icon=discord&label=discord&color=60a5fa
[discord-url]: https://biomejs.dev/chat
[npm-badge]: https://badgen.net/npm/v/@biomejs/resolver?icon=npm&color=60a5fa&label=%40biomejs%2Fresolver
[npm-url]: https://www.npmjs.com/package/@biomejs/resolver/v/latest

</div>

<br>


A WebAssembly-based module resolver that implements the [Node.js module resolution
algorithm](https://nodejs.org/api/esm.html#resolution-algorithm-specification), including support for `package.json` `exports`/`imports` maps,
TypeScript path aliases, extension aliases, and more.

This package is part of the [Biome](https://biomejs.dev) project. It exposes the
same resolver that Biome uses internally for its module graph, project lint rules, and type-aware lint rules.

Because it is compiled to WebAssembly, it requires no native binaries, has no
platform-specific dependencies, and is fully synchronous.

## Installation

Install the main package together with the WASM peer dependency that matches
your environment. There are two peer packages: one for Node.js and one for
browser environments.

```sh
# Install the Node.js distribution
npm install @biomejs/resolver @biomejs/wasm-resolver-nodejs

# Install the web distribution
npm install @biomejs/resolver @biomejs/wasm-resolver-web

```

> [!NOTE]
> All the examples from now on will target the Node.js distribution.
> Head to the [relevant section](#using-the-web-distribution) if you wish to know how to use
> the web distribution.

## Quick start

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";
import { ResolveErrorKind } from "@biomejs/resolver";

const resolver = createNodeResolver({
  extensions: ["ts", "js"],
  defaultFiles: ["index"],
  conditionNames: ["node", "import"],
});

const result = resolver.resolve("./utils", "/project/src");

if (result.path) {
  console.log(result.path); // "/project/src/utils/index.ts"
} else if (result.errorKind === ResolveErrorKind.ModuleNotFound) {
  console.error("Not found:", result.error);
} else {
  console.error("Resolution failed:", result.error);
}

resolver.free();
```

## Choosing a distribution

The package ships two entry points that differ in how they access the filesystem.

`@biomejs/resolver/nodejs` talks to the real filesystem using Node.js built-in
`node:fs` APIs. Use this when writing CLI tools, build scripts, language server
plugins, or any program that runs in Node.js, Bun, or Deno and needs to resolve
modules from the disk.

`@biomejs/resolver/web` uses an in-memory filesystem that you populate yourself.
Use this when writing browser-based tools such as online code playgrounds or
browser IDEs, where access to the host filesystem is not available. This entry
point is also a good fit for unit tests because you control every file precisely
without touching the disk.

**The web distribution must be loaded asynchronously** using `await import()`
because the WASM binary needs to be fetched and compiled by the browser before
it can be used. This is different from the Node.js distribution, which can load
WASM synchronously from disk and therefore supports static imports.

## Important: how extensions work

Extensions must be provided **without a leading dot**. The resolver adds the dot
itself when it constructs candidate file paths. For example, passing `"js"`
causes the resolver to look for files ending in `.js`.

The correct way to set extensions is:

```ts
const resolver = createNodeResolver({
  extensions: ["js", "ts", "json"],
});
```

Passing extensions with a leading dot will cause the resolver to look for files
whose names literally begin with a dot, which is almost certainly not what you
want:

```ts
const resolver = createNodeResolver({
  extensions: [".js", ".ts", ".json"], // Wrong
});
```

The same rule applies to `defaultFiles`. This option accepts bare filename stems,
not full filenames. The resolver combines the stem with each extension to form
the candidates it tries. Passing `"index"` with extensions `["js", "ts"]` causes
the resolver to try `index.js` and then `index.ts`.

```ts
const resolver = createNodeResolver({
  defaultFiles: ["index"],
  extensions: ["js", "ts"],
});
```

Passing a full filename like `"index.js"` as a default file will not work as
expected, because the resolver will append the extension again, producing
`index.js.js`.

## How `package.json` fields map to resolver options

When the resolver encounters a bare package specifier such as `import "lodash"`,
it walks up the directory tree from `baseDir` until it finds a `package.json`,
then reads fields from it to determine where the package's entry point is. Which
fields it reads and how it interprets them depend on the options you pass.

### `exports`

The `exports` field is the modern way for a package to declare its entry points.
It supports conditional mapping, where the same specifier can resolve to different
files depending on the environment. The resolver evaluates these conditions against
the `conditionNames` option you provide. Without `conditionNames`, the resolver
cannot match any condition and will skip the `exports` field entirely.

The order of keys inside `exports` determines which condition is matched first —
not the order of your `conditionNames` array.

In the following example, `my-package` ships both an ESM and a CommonJS build.
Resolving with `conditionNames: ["import"]` returns the ESM build, while
`conditionNames: ["require"]` returns the CommonJS build.

```json5
{
  "name": "my-package",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.cjs"
    }
  }
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({ conditionNames: ["node", "import"] });
const result = resolver.resolve("my-package", "/project/src");
// => { path: "/project/node_modules/my-package/dist/index.mjs" }

resolver.free();
```

### `imports`

The `imports` field works like `exports` but for internal package imports —
specifiers that start with `#` and map to files within the same package. It also
supports conditional mapping evaluated against `conditionNames`.

In the following example, a file inside `my-package` imports `#utils`. With
`conditionNames: ["import"]`, the resolver returns the ESM version of that
internal module.

```json5
{
  "name": "my-package",
  "imports": {
    "#utils": {
      "import": "./src/utils.mjs",
      "require": "./src/utils.cjs"
    }
  }
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({ conditionNames: ["node", "import"] });
const result = resolver.resolve("#utils", "/project/node_modules/my-package/src");
// => { path: "/project/node_modules/my-package/src/utils.mjs" }

resolver.free();
```

### `main` and `module`

When a package does not have an `exports` field, the resolver falls back to `main`
as the entry point. If `main` is also absent and the resolution does not require
CommonJS, the resolver tries `module`, which is a convention for ESM entry points
used by some older packages.

In the following example, `legacy-package` has no `exports` field. The resolver
returns the path from `main` directly.

```json5
{
  "name": "legacy-package",
  "main": "./dist/index.js",
  "module": "./dist/index.mjs"
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({ conditionNames: ["node", "require"] });
const result = resolver.resolve("legacy-package", "/project/src");
// => { path: "/project/node_modules/legacy-package/dist/index.js" }

resolver.free();
```

### `types`

When `resolveTypes` is set to `true`, the resolver reads the `types` field as the
package's type declaration entry point. This is used as a fallback when no
`"types"` export condition is found in the `exports` map.

In the following example, `my-package` declares its types via the `types` field.
With `resolveTypes: true`, the resolver returns the `.d.ts` path instead of the
runtime entry point.

```json5
{
  "name": "my-package",
  "main": "./dist/index.js",
  "types": "./dist/index.d.ts"
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  resolveTypes: true,
  conditionNames: ["types", "import", "default"],
  extensions: ["js", "ts"],
});
const result = resolver.resolve("my-package", "/project/src");
// => { path: "/project/node_modules/my-package/dist/index.d.ts" }

resolver.free();
```

## How `tsconfig.json` fields map to resolver options

The resolver automatically discovers `tsconfig.json` by walking up from the
`baseDir` you pass to `resolve()`. When a `tsconfig.json` is found, the resolver
reads certain fields from it automatically, without requiring any extra options
on your part.

### `paths`

The `paths` field in `compilerOptions` defines path aliases — mappings from short
import names to concrete file paths. These are applied automatically whenever a
`tsconfig.json` is discovered. No extra option is needed.

In the following example, any import of `@utils/string` resolves to
`./src/utils/string` relative to the project root. The trailing `/*` on both the
key and the value means the alias applies to all specifiers under that prefix, not
just the exact string.

```json5
{
  "compilerOptions": {
    "paths": {
      "@utils/*": ["./src/utils/*"]
    }
  }
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({ extensions: ["ts", "js"] });
const result = resolver.resolve("@utils/string", "/project/src/app");
// => { path: "/project/src/utils/string.ts" }

resolver.free();
```

### `baseUrl`

The `baseUrl` field shifts the root from which non-relative imports are resolved.
Normally, a bare specifier like `"utils/string"` would be looked up in
`node_modules`. With `baseUrl` set, the resolver also tries to find it relative
to the `baseUrl` directory. Like `paths`, this is applied automatically when the
`tsconfig.json` is found.

In the following example, `baseUrl` is set to `./src`, so an import of
`"utils/string"` resolves to `./src/utils/string` rather than a package in
`node_modules`.

```json5
{
  "compilerOptions": {
    "baseUrl": "./src"
  }
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({ extensions: ["ts", "js"] });
const result = resolver.resolve("utils/string", "/project/src/app");
// => { path: "/project/src/utils/string.ts" }

resolver.free();
```

### `typeRoots`

The `typeRoots` field lists directories where the resolver looks for `@types`
packages. It is only consulted when `resolveTypes` is set to `true`. When
`resolveTypes` is enabled and `typeRoots` is absent from `tsconfig.json`, the
resolver defaults to searching `node_modules/@types`.

In the following example, `typeRoots` points to a local `types` directory
alongside the standard `@types` location. With `resolveTypes: true`, the resolver
searches both directories when looking up type declarations.

```json5
{
  "compilerOptions": {
    "typeRoots": ["./types", "./node_modules/@types"]
  }
}
```

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  resolveTypes: true,
  conditionNames: ["types", "import", "default"],
  extensions: ["js", "ts"],
});
const result = resolver.resolve("my-package", "/project/src");
// Searches /project/types and /project/node_modules/@types for type declarations

resolver.free();
```

## Examples

### Resolve a relative path

When one file imports another using a path that starts with `./` or `../`, the
resolver treats it as a relative path and looks for the file on disk relative to
`baseDir`. The `baseDir` must be an absolute path to a directory, not a file. If
you only have a file path, use `path.dirname()` to get its directory.

```ts
import path from "node:path";
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver();

const currentFile = "/project/src/index.ts";
const result = resolver.resolve("./utils.js", path.dirname(currentFile));

if (result.path) {
  console.log(result.path); // "/project/src/utils.js"
} else {
  console.error(result.error);
}

resolver.free();
```

### Resolve a bare package specifier

A bare specifier is one that does not start with `./`, `../`, or `/` — for
example, `"lodash"` or `"react/jsx-runtime"`. The resolver looks for the package
in a `node_modules` directory, walking up from `baseDir` until it finds one.

To resolve the entry point correctly, pass the condition names that match your
environment. For a Node.js ESM context, `"node"` and `"import"` are the typical
choices.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  conditionNames: ["node", "import"],
});

const result = resolver.resolve("lodash", "/project/src");

if (result.path) {
  console.log(result.path); // "/project/node_modules/lodash/lodash.js"
}

resolver.free();
```

### Resolve using `exports` condition names (ESM vs CJS)

Many modern packages ship both an ESM and a CommonJS version and expose them
through the `exports` field using the `"import"` and `"require"` conditions.
Which file you get depends entirely on which condition names you pass to the
resolver.

Given a package whose `package.json` looks like this:

```json5
{
  "name": "my-package",
  "exports": {
    ".": {
      "import": "./dist/index.mjs",
      "require": "./dist/index.cjs"
    }
  }
}
```

Passing `"import"` as a condition name gives you the ESM build; passing
`"require"` gives you the CommonJS build.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const esmResolver = createNodeResolver({
  conditionNames: ["node", "import"],
});
const esmResult = esmResolver.resolve("my-package", "/project/src");
// => { path: "/project/node_modules/my-package/dist/index.mjs" }

const cjsResolver = createNodeResolver({
  conditionNames: ["node", "require"],
});
const cjsResult = cjsResolver.resolve("my-package", "/project/src");
// => { path: "/project/node_modules/my-package/dist/index.cjs" }

esmResolver.free();
cjsResolver.free();
```

### Resolve a TypeScript file via extension aliases

TypeScript encourages writing import specifiers with the compiled output
extension — `"./helper.js"` — even when the file that actually exists on disk is
`helper.ts`. This means that if you ask the resolver for `"./helper.js"` and
only `helper.ts` exists, a plain resolve will fail.

The `extensionAliases` option lets you tell the resolver to try one or more
alternative extensions whenever it sees a specific extension in an import
specifier. Each entry maps a source extension to a list of extensions to try, in
order. Mapping `"js"` to `["ts", "js"]` means the resolver first tries the `.ts`
file, then falls back to `.js` if no `.ts` is found. Like `extensions`, all
values must be written without a leading dot.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  extensionAliases: [
    { extension: "js", aliases: ["ts", "js"] },
    { extension: "jsx", aliases: ["tsx", "jsx"] },
  ],
});

const result = resolver.resolve("./helper.js", "/project/src");
// Resolves to helper.ts if it exists, otherwise helper.js

resolver.free();
```

### Resolve a JSON file

The resolver does not try JSON files by default. To make it consider `.json`
files as candidates when resolving a specifier without an explicit extension,
include `"json"` in the `extensions` list.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  extensions: ["json"],
});

const result = resolver.resolve("./config.json", "/project/src");
// => { path: "/project/src/config.json" }

resolver.free();
```

When a package exposes a JSON file through the `exports` field, you do not need
`"json"` in `extensions`. The resolver follows the `exports` map directly, so
condition names are all that is needed. Given this `package.json`:

```json5
{
  "name": "my-package",
  "exports": {
    "./data": "./data/index.json"
  }
}
```

```ts
const result = resolver.resolve("my-package/data", "/project/src");
// => { path: "/project/node_modules/my-package/data/index.json" }
```

### Resolve a directory index file

When a specifier points to a directory rather than a file, the resolver looks for
an index file inside that directory. This only works when you provide both
`defaultFiles` and `extensions`. The resolver combines each stem in `defaultFiles`
with each extension in `extensions` to form the list of candidates, trying them
in the order you specify.

For example, `defaultFiles: ["index"]` and `extensions: ["ts", "js"]` causes the
resolver to try `index.ts` first, then `index.js`. Putting `"ts"` before `"js"`
gives TypeScript files priority over JavaScript files.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  defaultFiles: ["index"],
  extensions: ["ts", "js"],
});

const result = resolver.resolve("./utils", "/project/src");
// Resolves to /project/src/utils/index.ts if it exists,
// otherwise /project/src/utils/index.js

resolver.free();
```

### Resolve TypeScript declaration files

When building a tool that works with types rather than runtime code — such as a
type checker, a documentation generator, or an IDE plugin — you want the resolver
to return `.d.ts` paths instead of source paths. Set `resolveTypes: true` to
enable this behaviour.

With `resolveTypes` enabled, the resolver changes how it interprets `package.json`.
It prefers the `"types"` export condition over `"import"` or `"require"`, falls
back to the `types` field when no `"types"` condition is found, and ignores the
`main` field. For any import that explicitly names a JavaScript file extension,
the resolver also automatically tries the corresponding declaration extension
first — `.d.ts` for `.js`, `.d.mts` for `.mjs`, and so on.

You should pair `resolveTypes: true` with `conditionNames: ["types", "import",
"default"]` and include both JavaScript and TypeScript extensions in `extensions`.
Do not include `.d.ts` yourself — the resolver inserts declaration extensions
automatically at the right priority.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";

const resolver = createNodeResolver({
  resolveTypes: true,
  conditionNames: ["types", "import", "default"],
  extensions: ["js", "ts", "mjs", "mts", "cjs", "cts"],
});

const result = resolver.resolve("my-package", "/project/src");
// For a package with "types": "./dist/index.d.ts" in package.json:
// => { path: "/project/node_modules/my-package/dist/index.d.ts" }

resolver.free();
```

For a package that ships a `"types"` export condition, the resolver picks that up
directly:

```json5
{
  "name": "my-package",
  "exports": {
    ".": {
      "types": "./dist/index.d.ts",
      "import": "./dist/index.mjs"
    }
  }
}
```

```ts
const result = resolver.resolve("my-package", "/project/src");
// => { path: "/project/node_modules/my-package/dist/index.d.ts" }
```

### Handle Node.js built-in modules

Node.js ships a set of built-in modules — `node:fs`, `node:path`, `node:crypto`,
and others — that are part of the runtime itself and cannot be resolved to a file
path. By default, when the resolver encounters one of these, it treats it like
any other specifier and returns a generic "module not found" error, because no
file with that name exists on disk.

Setting `resolveNodeBuiltins: true` changes this behaviour. Instead of a generic
error, the resolver returns `ResolveErrorKind.NodeBuiltIn` the moment it
recognises a built-in specifier, without walking the filesystem at all. This
lets you tell the difference between a genuinely missing package and a deliberate
import of a built-in, so you can handle each case appropriately — for example,
by skipping the built-in, substituting a polyfill, or reporting a targeted error
message.

This option is particularly useful when writing tools that need to support
multiple runtimes. Bun and Deno share most of the Node.js built-in names, but a
browser environment has none of them. Detecting built-ins explicitly lets you
branch per-runtime rather than treating them all as missing packages.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";
import { ResolveErrorKind } from "@biomejs/resolver";

const resolver = createNodeResolver({
  resolveNodeBuiltins: true,
});

const result = resolver.resolve("node:fs", "/project/src");

if (result.errorKind === ResolveErrorKind.NodeBuiltIn) {
  console.log("node:fs is a built-in — substituting polyfill");
} else if (result.error) {
  console.error("Resolution failed:", result.error);
} else {
  console.log("Resolved to:", result.path);
}

resolver.free();
```

### Resolve in a browser playground (web distribution)

The web distribution is designed for environments where the host filesystem is
not accessible, such as browser-based code playgrounds or in-browser IDEs. It
exposes an in-memory filesystem that you populate with `insertFile()` calls
before creating a resolver.

Because the WASM binary must be fetched and compiled by the browser before it
can be used, the web distribution must be loaded with a dynamic `import()`. The
`await` ensures the WASM is ready before you call any resolver APIs.

To set up a virtual project, insert every file the resolver might need: your
source files, any `package.json` manifests (both your project's and any packages
in `node_modules`), and any other files that imports may reference. The resolver
walks this virtual filesystem exactly as it would a real one.

```ts
// The web distribution must be loaded dynamically because browsers require
// asynchronous fetching and compilation of the WASM binary.
const { createMemoryFileSystem, createWebResolver } = await import(
  "@biomejs/resolver/web"
);

const fs = createMemoryFileSystem();

// Populate the file system
fs.insertFile(
  "/project/package.json",
  JSON.stringify({ name: "my-app", version: "1.0.0" }),
);
fs.insertFile("/project/src/index.ts", "");
fs.insertFile("/project/src/greet.ts", "");

fs.insertFile(
  "/project/node_modules/lodash/package.json",
  JSON.stringify({ name: "lodash", version: "4.17.21", main: "./lodash.js" }),
);
fs.insertFile("/project/node_modules/lodash/lodash.js", "");

// Create resolver
const resolver = createWebResolver(fs, {
  extensions: ["ts", "js"],
  defaultFiles: ["index"],
  conditionNames: ["import", "default"],
});

const relativeResult = resolver.resolve("./greet.ts", "/project/src");
// => { path: "/project/src/greet.ts" }

const packageResult = resolver.resolve("lodash", "/project/src");
// => { path: "/project/node_modules/lodash/lodash.js" }

resolver.free();
fs.free();
```

## Error handling

`resolver.resolve()` never throws. All failures are returned as a value. A
failed result carries two fields: `error`, a human-readable string suitable for
logging, and `errorKind`, a `ResolveErrorKind` enum value for programmatic
branching. A successful result carries only `path`. Exactly one of `path` or
`error` is always present.

```ts
type ResolveResult =
  | { path: string; error?: never; errorKind?: never }
  | { path?: never; error: string; errorKind: ResolveErrorKind };
```

Check for the presence of `path` to distinguish the two cases. TypeScript will
narrow the type correctly inside each branch. Use `error` for display and
logging; use `errorKind` for programmatic branching.

```ts
import { createNodeResolver } from "@biomejs/resolver/nodejs";
import { ResolveErrorKind } from "@biomejs/resolver";

const resolver = createNodeResolver();
const result = resolver.resolve("./utils.js", "/project/src");

if (result.path) {
  processFile(result.path);
} else {
  // Use errorKind for branching, error for display.
  if (result.errorKind === ResolveErrorKind.ModuleNotFound) {
    console.error("File not found:", result.error);
  } else {
    console.error("Resolution failed:", result.error);
  }
}

resolver.free();
```

This design means you never need a `try`/`catch` around `resolve()` calls. All
exceptional conditions, including malformed manifests and broken symlinks, are
surfaced as values rather than thrown exceptions.

### `ResolveErrorKind.ModuleNotFound`

The specifier could not be found anywhere the resolver looked. This is the most
common error and almost always means one of the following:

- The file does not exist at the path you specified. Check the path for typos.
- The package is not installed. Run your package manager's install command.
- The `extensions` option does not include the extension of the file you are
  trying to resolve. Add the missing extension without a leading dot.
- The `conditionNames` option does not match any condition in the package's
  `exports` map. Check which conditions the package supports and include the
  right ones.
- The `baseDir` you passed is wrong. It must be an absolute path to a
  **directory**, not a file path.

### `ResolveErrorKind.DirectoryWithoutIndex`

The specifier resolves to a directory, but the resolver does not know which file
inside that directory to use. Fix this by providing both `defaultFiles` and
`extensions`.

```ts
const resolver = createNodeResolver({
  defaultFiles: ["index"],
  extensions: ["ts", "js"],
});
```

With this configuration, resolving `"./utils"` when `./utils/` is a directory
will try `./utils/index.ts` and then `./utils/index.js`.

### `ResolveErrorKind.NodeBuiltIn`

This is only returned when `resolveNodeBuiltins: true` is set. It means the
specifier names a built-in module such as `node:fs` or `node:path`. This is not
a failure — it signals that the import refers to the runtime itself rather than a
file on disk. Handle it by skipping the specifier, recording it as a built-in,
or substituting a polyfill.

```ts
const result = resolver.resolve("node:fs", "/project/src");

if (result.errorKind === ResolveErrorKind.NodeBuiltIn) {
  // not an error — the import is intentional
} else if (result.error) {
  console.error("Resolution failed:", result.error);
}
```

Without `resolveNodeBuiltins: true`, built-in specifiers produce
`ModuleNotFound` instead, because the resolver treats them as ordinary package
names and finds no matching directory in `node_modules`.

### `ResolveErrorKind.ManifestNotFound`

No `package.json` was found walking up from `baseDir`. This typically happens
when `baseDir` is set to a path outside your project root or to a temporary
directory that has no manifest. Confirm that `baseDir` is inside a directory
tree that contains a `package.json`.

### `ResolveErrorKind.ErrorLoadingManifest`

A `package.json` or `tsconfig.json` was found on disk but could not be parsed.
The file likely contains invalid JSON. Validate it with a JSON linter.

### `ResolveErrorKind.BrokenSymlink`

A symlink in the resolution chain points to a target that does not exist. This
usually means a broken symlink in `node_modules` left behind by an interrupted
package install. Re-running your package manager's install command normally
fixes it.

### `ResolveErrorKind.InvalidExportsTarget`

The matched condition in a `package.json` `exports` or `imports` map points to
an invalid target. A valid target must be a string starting with `./`, an array
of fallbacks, a conditions object, or `null`. This is a bug in the package's
`package.json`. If you control the package, fix the manifest; otherwise check
for a newer version.

### `ResolveErrorKind.InvalidPackageName`

The specifier contains characters that are not valid in a package name, such as
uppercase letters in a scoped package name or a path segment that begins with
`.`. Check the specifier for typos.

## Memory management

`Resolver` and `MemoryFileSystem` objects hold memory inside the WebAssembly
heap. When you are done with them, call `.free()` to release that memory. If you
do not, the memory will not be reclaimed for the lifetime of the process or page.

```ts
const resolver = createNodeResolver();

for (const specifier of specifiers) {
  const result = resolver.resolve(specifier, baseDir);
    // handle result...
}

resolver.free();

```

Calling `free()` more than once on the same object throws an error. Do not
retain a reference to a resolver after calling `free()`.

If you create a single resolver at startup and reuse it for the lifetime of your
process — a common pattern in long-running tools — there is no need to call
`free()` at all.

## Using the web distribution

The web distribution uses an in-memory filesystem that you populate before
creating a resolver. Because it has no access to the host filesystem, every file
the resolver might need must be explicitly inserted with `insertFile()`. This
includes not just your source files, but also:

- Every `package.json` along the resolution path — both your project root
  manifest and the manifests of any packages in `node_modules`. Without a
  `package.json`, the resolver cannot locate package entry points and will
  return `ManifestNotFound`.
- Any `tsconfig.json` files whose `paths`, `baseUrl`, or `typeRoots` settings
  you want the resolver to apply. If a `tsconfig.json` is absent, the resolver
  resolves as if no TypeScript configuration exists.

The paths you use must be absolute and consistent. The resolver treats the
virtual filesystem exactly like a real one: it walks parent directories looking
for manifests, so the directory structure implied by the paths you insert must
match what you expect the resolver to traverse.

Because the WASM binary must be fetched over the network and compiled by the
browser before it can be used, the web distribution must be loaded with a
dynamic `await import()`. The `await` ensures the WASM module is fully
initialized before you call any resolver APIs. This is required in browser
environments — synchronous WASM loading is not possible when the binary comes
over HTTP. Node.js can load WASM synchronously from disk, which is why
`@biomejs/resolver/nodejs` supports static imports.

```ts
const { createMemoryFileSystem, createWebResolver } = await import(
  "@biomejs/resolver/web"
);

const fs = createMemoryFileSystem();

// Project manifest — required for bare package specifier resolution.
fs.insertFile(
  "/project/package.json",
  JSON.stringify({ name: "my-app", version: "1.0.0" }),
);

// TypeScript configuration — required for path aliases and baseUrl.
fs.insertFile(
  "/project/tsconfig.json",
  JSON.stringify({
    compilerOptions: {
      baseUrl: "./src",
      paths: { "@utils/*": ["./src/utils/*"] },
    },
  }),
);

// Source files.
fs.insertFile("/project/src/index.ts", "");
fs.insertFile("/project/src/utils/format.ts", "");

// A package in node_modules — both the manifest and the entry point
// must be present for the resolver to return a path.
fs.insertFile(
  "/project/node_modules/lodash/package.json",
  JSON.stringify({ name: "lodash", version: "4.17.21", main: "./lodash.js" }),
);
fs.insertFile("/project/node_modules/lodash/lodash.js", "");

const resolver = createWebResolver(fs, {
  extensions: ["ts", "js"],
  defaultFiles: ["index"],
  conditionNames: ["import", "default"],
});

// Resolves using the path alias from tsconfig.json.
const aliasResult = resolver.resolve("@utils/format", "/project/src");
// => { path: "/project/src/utils/format.ts" }

// Resolves the package entry point from the node_modules manifest.
const pkgResult = resolver.resolve("lodash", "/project/src");
// => { path: "/project/node_modules/lodash/lodash.js" }

resolver.free();
fs.free();
```

If you are building a browser playground that lets users edit multiple files,
keep a single `MemoryFileSystem` instance and call `insertFile()` or `remove()`
as files change. You can reuse the same `Resolver` instance across edits because
it reads from the filesystem on every `resolve()` call — there is no internal
cache to invalidate.
