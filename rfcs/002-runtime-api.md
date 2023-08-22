# Overview

Biome is a toolchain where users can use only Biome to manage their code. On the other hand, users should be free to use only specific tools in their workflow, without too much hassle.

While the latter is desirable, using Biome as all-in-one tool is preferred on the long run because it creates a homogeneous user experience in terms of:
- performance
- error messaging
- caching
- overall workflow between tools
- good defaults

Considering what written above, the Node.js API should reflect what stated above.

## Terminology

From now on the document will use some terms, here's their context inside the document:

- **Frontend**: the code written for the runtimes (Node.js, Deno, browser, etc.);
- **Rust Workspace**: the main hub where the bulk of the logic resides. It's the Rust code that live
  inside the `rome_service` crate;
- **Runtime Workspace**: it's the shared code used by the different Frontends;


## Use case for Biome

The first and foremost important tool that users should access to is Biome's compiler. The compiler,
as this time, it's not ready yet - at least not all the features we intend to implement,
but this was a design decision when it was first created.

Exposing the compiler should allow a user to do multiple things (not limited to this list):
- allow to transform some code (transplilation, minification, compile to another language, etc.)
- analyze some code (linter, control flow analysis, type check, etc.)
- bundle some code
- generate documentation

The end goal of how Biome can expose its compiler is via plugins. **Plugins are a non-goal for this
RFC**, but this first proposal should start taking them into consideration.

As for today, Biome can expose a formatter and a linter (analyzer).

## The aim of the APIs

The APIs should only be a **medium** to eventually communicate with the Rust Workspace,
which means that the APIs can't provide more features than what the Rust Workspace can provide.

While designing the APIs, it's important to take this in consideration.


# The current ecosystem

As for today, different tools have different approaches

## `prettier`

Their APIs are not aligned with the CLI. The CLI allows to format a single file, a list of files or
some code passed via `stdin`. The APIs mostly orbit around formatting some content, and they are not
aware if prettier is installed already. This works well for most environments but the majority of the
work is responsibility of the user:
- read files and their contents
- resolve the configuration in case prettier is already installed in the project

## `esbuild`

Their APIs and CLI are almost aligned, meaning that the options passed via APIs are mapped 100% to the
CLI, for example:
```js
require('esbuild').buildSync({
  entryPoints: ['app.js'],
  outfile: 'out.js',
  bundle: true,
  platform: 'node',
  external: ['fsevents'],
})
```

```shell
esbuild app.js --bundle --external:fsevents --platform=node
```

> It seems to me that esbuild is not designed to be a standalone dev server, then caching strategies
> and whatnot are designed for this goal. I might be wrong.

`esbuild` is mostly designed to be a bundler, so their APIs and design decisions might orbit
around that.

## `eslint`

`eslint` takes a different approach compared to `prettier`. Their approach emulates their CLI but
with extra steps:
```js
// 1. Create an instance.
const eslint = new ESLint();

// 2. Lint files.
const results = await eslint.lintFiles(["lib/**/*.js"]);

// 3. Format the results.
const formatter = await eslint.loadFormatter("stylish");
const resultText = formatter.format(results);

// 4. Output it.
console.log(resultText);
```

They use the instance pattern. The instance accept a [bunch of options](https://eslint.org/docs/latest/developer-guide/nodejs-api#parameters).

```shell
eslint lib/**/*.js
```

There's also one more thing to note, their Node.js API is designed mostly for plugin authors. From
their website:
> The purpose of the Node.js API is to allow plugin and tool authors to use the ESLint functionality directly, without going through the command line interface.

`eslint` also allows to lint some content:
```js
const results = await eslint.lintText(code, options);
```

## `postcss`

They use the instance pattern:

```js
let postcss = require('postcss')

postcss(plugins).process(css, { from, to }).then(result => {
  console.log(result.css)
})
```

```shell
postcss input.css -o output.css
postcss src/**/*.css --base src --dir build
cat input.css | postcss -u autoprefixer > output.css
```

Even though the approach is slightly different from `eslint`, the result is the same.

# Biome's APIs

Here's a list possible proposals of how Biome could be used.

There might be various cases, but first we need to understand how Biome is designed, and make proposal
based its design **AND** end-goal.

![Runtime design](./Runtime%20design.png)


1. Biome is configuration aware, meaning that when communicating with Rust Workspace,
   all configuration defaults are automatically applied. If the APIs are run
   inside a Biome project, the configuration is automatically picked up from the `biome.json` file.
2. Biome's APIs should reflect the CLIs commands and arguments. What it's possible to do
   via CLI, should be done also via APIs, but not vice-versa. This constraint would allow the team
   to first design and test the feature natively, and expose it once it's stable.
3. Biome should be able, in the future, to expose plugins or a way to users to use its compiler capabilities

> Point `2.` might not be true for ALL cases. For example, as for today, we can't format by
> range via CLI. Maybe this should be more of a guideline, and not a constraint.



## Instance pattern

Instance pattern

```js
import { Rome } from "rome";

let rome = new Rome({});
```

The instance pattern allows for better extensions on the long run, and allows to play better with
all the tools we want to expose.

```js
import { Rome } from "rome";

const content = "function f() { return {} }";
const rome = new Rome({
    formatter: {
        lineWidth: 120
    },
    linter: {
        recommended: false,
        js: {
            noDeadCode: "error"
        }
    }
});
const ast = rome.parseContent(content); // not part of this paragraph
const new_content = rome.formatContent(content); // not part of this paragraph
const diagnostics = rome.checkContent(content);  // not part of this paragraph
```

The first parameter of the instance should be a personalized configuration. The configuration
passed to the instance **will override** Biome's defaults  **_BUT_** not the options inside a possible
`biome.json` file.

This is an import point, because we might have cases where a user is using Biome for linting/formatting
of the project, but this project is actually using the runtime API to do some ad-hoc work. For example
scripts, some generated code, etc.

In the future the instance pattern will be useful to register plugins. Plugin need to have access
to the compiler APIs (we don't know yet what kind of APIs and what information).


## Formatter

While using functions à la `prettier` may seem quicker for prototyping, this approach might
create more friction in the future when we will extend the APIs with new features.

### Formatting a file

```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.formatFiles(["./path/to/file.js"]);
console.log(result.code); // formatted content
console.log(result.errors); // possible parse errors
```

Which reflects

```shell
rome format ./path/to/file.js
```

We can also write directly the new content to a file

```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.formatFile("./path/to/file.js", { write: true }); // `true` maps to the `--write` argument
console.log(result.code); // undefined, the new content is in the file
console.log(result.errors); // possible parse errors
```

Which reflects

```shell
rome format --write ./path/to/file.js
```

### Formatting a directory

It should be possible to format a directory

```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.formatFiles(["./path"]);
for (const [file_name, result] of result) {
    console.log(file_name);
    console.log(result.code);
    console.log(result.errors); // errors thrown while formatting this file
}
```

Which reflects

```shell
rome format ./path
```

We can also write directly the new content to files


```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.formatFiles(["./path"], { write: true });
for (const [file_name, result] of result) {
    console.log(file_name);
    console.log(result.code); // undefined, it's being written
    console.log(result.errors); // errors thrown while formatting this file
}
```

> **Note**: `formatFile` could be removed in favour of only `formatFiles`

### Formatting some content

> **Warning**: as for today the CLI doesn't allow to format from `stdin`, but it's something
> that we plan to deliver ASAP

```js
import { Rome } from "rome";

const rome = new Rome();
const content = "function f()  { return   {}}";
const result = rome.formatContent(content, { filePath: "example.js" });
console.log(result.code); // formatted content
console.log(result.errors); // possible parse errors
```

Which will translate to

```shell
echo "function f()  { return   {}}" | rome format --file-type=js
```

`filePath` is required to tell Biome how it should parse the file.

### Format range

```js
import { Rome } from "rome";

const rome = new Rome();
const content = "function f()  { return   {}}";
const result = rome.formatContent(content, { filePath: "example.js", range: [7, 10] });
console.log(result.code); // formatted content
console.log(result.errors); // possible parse errors
```

> **Note**: the CLI doesn't allow to format ranges as for today, and it might not be needed.
> It can be an exception to the rule where each API should be supported by the CLI too.

### Debugging


As you noticed each call accepts an object as second argument. This object could contain a
`debug` property, which allows us to return the IR emitted by Biome.

```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.formatFiles(["./path/to/file.js"], { debug: true });
console.log(result.code); // formatted content
console.log(result.errors); // possible parse errors
console.log(result.ir); // the IR emitted by Biome
const content = "function f()  { return   {}}";
const result2 = rome.formatContent(content, { filePath: "example.js", range: [7, 10], debug: true });
console.log(result2.ir); // the IR emitted by the call formatContent
```

## Linter

> **Note**: nowadays, Biome doesn't have a CLI command to _only lint_ files. We have the `check`
> command that does that *now*, but the command is designed to run multiple checks.

It's also possible to run the linter and retrieve possible diagnostics

### Lint files

```js
import { Rome } from "rome";

const rome = new Rome();
const result = rome.lintFiles(["./path/to/file.js"]);
console.log(result.errors); // diagnostics emitted while lint the files
```


### Lint content

```js
import { Rome } from "rome";

const rome = new Rome();
const content = "function f()  { return   {}}";
const result = rome.lintContent(content, { filePath: "example.js" });
console.log(result.errors); // diagnostics emitted while lint the files
```

## Parse

Allows to parse a file, and return the CST and AST emitted by the parsing phase.

> **Note**: nowadays, the CLI doesn't a command to parse files, this could be perfect occasion
> to actually implement it.


> **Note**: The reason why `cst` and `ast` are emitted as strings is mostly because we don't have TypeScript file types
for our nodes. If we are able to generate TypeScript types for our AST, then `ast` could be
> returned as an object

### Parse content

```js
import { Rome } from "rome";

const rome = new Rome();
const content = "function f()  { return   {}}";
const result = rome.parseContent(content, { filePath: "example.js" });
console.log(result.ast); // AST as string
console.log(result.cst); // CST as string
console.log(result.errors); // possible parse errors
```


## Errors

This proposal assumes that **all** APIs are prone to errors for different reasons. Internally, in the
Rust Workspace, we emit a `RomeError`, which might contain other variants with other errors,
form example `ConfigurationError`.

At the moment this `RomeError` is not serialized, so it's not very easy to come up with a clear
proposal, but a runtime should receive at least a `code` of the error, e.g. `RomeError:ConfigurationError`,
maybe a `sub_code` e.g. `ConfigurationError::ConfigAlreadyExists`,


## Q&A

> Why not expose also functions like `format`, `lint`, etc.?

While this can be an option, and it could be done, I think they don't exactly fit in the grand
scheme of what Biome should be.

Of course there are exceptions, for example for a future testing framework:

```js
import {test} from "rome";

test("test something", t => {
    t.assert(true)
})
```

>
