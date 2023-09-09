# 🚀 Contributing

We can use help in a bunch of areas and any help is greatly appreciated!

## Asking questions, making proposals

If you have any questions, proposals, or feedbacks, open a [GitHub discussion](https://github.com/bare-ts/tools/discussions).
Make sure your comment adds value: [don't post a comment just to get attention](https://jacobtomlinson.dev/posts/2022/dont-be-that-open-source-user-dont-be-me/).

Our [Discord server](https://discord.gg/BypW39g6Yc) is open for help and more ad-hoc discussion.
All activity on the Discord is still moderated and will be strictly enforced under the project's [Code of Conduct](./CODE_OF_CONDUCT.md).

Remember that we are doing this project on our own time.
We are humans: we like support, and we expect kindness :)

## Reporting bugs

Our [GitHub issues](https://github.com/biomejs/biome/issues/) serve as a place for submitting bugs.
Make sure that the bugs is not reported yet and is not fixed in the main branch.
You can test on the main branch, thanks to the [playground](https://biomejs.dev/playground/).

## Getting Started

Building this project requires a `stable` Rust toolchain, which can be installed using [rustup](https://www.rust-lang.org/tools/install).

Clone the repository and navigate to the `tools` directory:

```bash
git clone https://github.com/biomejs/biome
cd biome
```

Compile all packages and dependencies:

```bash
cargo build
```

Biome can be used via the `biome` bin:

```bash
cargo run --bin biome -- --help
```

## Install the required tools

We use [Just](https://just.systems/man/en/) to run scripts and tasks, to make our life easier.

You can install `just` using cargo:

```shell
cargo install just
```

But we **highly recommend** to [install it using an OS package manager](https://github.com/casey/just#packages),
so you won't need to prefix every command with `cargo`.

Once installed, run the following command install the required tools:

```shell
just install-tools
```

And you're good to go hack with Biome and Rust! 🚀

## Testing

To run the tests, just run

```shell
just test
```

If you want to test the tests for a single crate:

```shell
just test-crate biome_cli
```

To run only the doctests, you would need to pass an argument to the command:

```shell
just test-doc
```

In some crates, we use snapshot testing.
The majority of snapshot testing is done using [`insta`](https://insta.rs).
`insta` is already installed by the command `just install-tools`.

When a snapshot test fails, you can run:

- `cargo insta accept` to accept all the changes and update all the snapshots;
- `cargo insta reject` to reject all the changes;
- `cargo insta review` to review snapshots singularly;

## Checks

When you finished your work, and you are ready to **commit and open a PR**,
run the following command:

```shell
just ready
```

This command will run the same commands of the CI: format, lint, tests and code generation.
Eventually everything should be "green" 🟢 and commit all the code that was generated.

## Language Server and VS Code Extension Development

The Biome language server is the binary crate `biome` which can be built using the command:

```bash
cargo build --bin biome
```

If benchmarking the language server, be sure to build with the `--release` flag.

The VS Code extension can be installed from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=biomejs.biome) and can be used with a development build of the language server by setting the `"biome.lspBin"` VS Code setting to the path of the binary:

```json
{
  "biome.lspBin": "/path/to/biome/target/debug/biome"
}
```

Please note that Windows disallows modifying an executable while it's running,
meaning you won't be able to recompile the Biome binary once the extension was activated in your editor.

The server is spawned as a background daemon, and continues to run even after the editor is closed.

To stop the running daemon instance use the `biome stop` command, with the editor closed as the extension
will try to restart it otherwise.

To build the VS Code extension from source, navigate to the `editors/vscode` directory and run:

```bash
npm install
npm run build
```

This will create a `biome_lsp.vsix` which you can install into VS Code by running:

```bash
npm run install-extension
```

The `"biome.lspBin"` VS Code setting will still need to be set as described above.

When the extension is running, it will connect to a daemon server - or it will bootstrap one.

When you apply changes to the binary, you need to do two things:

- compile the binary
- kill the daemon process, so you can spawn a new server session
  with the new changes

When the daemon is running, it's possible to inspect its logs in the folder `biome-logs`, placed
in the temporary folder of the operative system.

### User files

If files specific to your local development environment should be ignored, please add these files to a global git ignore file rather than to a git ignore file within Biome.

You can find more information on this process [here](https://help.github.com/en/github/using-git/ignoring-files#configuring-ignored-files-for-all-repositories-on-your-computer).

## Node.js development

The npm module `npm/biome` contains Biome's Node JS API that supports different backends:

- `wasm-nodejs` (WebAssembly)
- `backend-jsonrpc` (Connection to the daemon)

For testing and developing, you need to build these packages, following the steps:

1. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) globally;
2. run the `build` command inside the package `backend-jsonrpc`;
3. run the `build` and `build:wasm-node-dev` commands inside the package `js-api` (folder `npm/js-api`);
4. run `pnpm i` inside the package `js-api` (folder `npm/js-api`), this will link the WebAssembly bindings and the
   JSON-RPC bindings;

The tests are run against the compiled files, which means that you need to run the
`build` command after you implemented features/bug fixes.

## Website

The [Biome website](https://biomejs.dev/) is built with [Astro](https://astro.build).
To start a development server you can run the following commands:

```bash
cd website
pnpm install
pnpm start
```

### Generated files

If you work on some parser and you create new nodes or modify existing ones, will need to run a command to update some files that are auto-generated.

#### `cargo codegen grammar`

This command will update the syntax of the parsers.

The source is generated from the [`ungram` files](https://github.com/biomejs/biome/blob/main/xtask/codegen/js.ungram).

#### `cargo codegen test`

This command will create new tests for your parser. We currently have a neat infrastructure
where tests for parser are generated com inline comments found inside
the source code. Please read [the proper chapter for more information](#write-tests-for-a-parser)

It's strongly advised to **run this command before committing new changes**.

#### `cargo codegen analyzer`

This command will detect linter rules declared in the `analyzers` and `assists` directories in `biome_analyze`, regenerate the index modules `analyzers.rs` and `assists.rs` to import these files, and update the registry builder function in `registry.rs` to include all these rules.
It will also regenerate the configuration of the rules.

#### `cargo coverage`

This command will check and report parser conformance against different test suites.
We currently target the [Official ECMAScript Conformance Test Suite](https://github.com/tc39/test262) and
the [Typescript Test Suite](https://github.com/microsoft/TypeScript/tree/main/tests)

The test suites are included as git submodules and can be pulled using:

```bash
git submodule update --init --recursive
```

## Commit messages

Internally, the Biome team adheres as closely as possible to the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0-beta.2/).
The following this convention encourages commit best-practices and facilitates commit-powered features like change log generation.

The following commit prefixes are supported:

- `feat:`, a new feature
- `fix:`, a bugfix
- `docs:`, a documentation update
- `test:`, a test update
- `chore:`, project housekeeping
- `perf:`, project performance
- `refactor:`, refactor of the code without change in functionality

Below are examples of well-formatted commits:

```
feat(compiler): implement parsing for new type of files
fix: fix nasty unhandled error
docs: fix link to website page
test(lint): add more cases to handle invalid rules
```

### Creating pull requests

When creating a new pull request, it's preferable to use a conventional commit-formatted title, as this title will be used as the default commit message on the squashed commit after merging.

Please use the template provided.

#### Changelog

If the PR you're about to open is a bugfix/feature around Biome, you can add a new line to the `CHANGELOG.md`, but it's not mandatory.

At the top of the file you will see a `Unreleased` section. The headings divide the sections by "feature", make sure
to add a new bullet point.

Here's a sample of the headings:

```markdown
## Unreleased

### Analyzer

### CLI

### Configuration

### Editors

### Formatter

### JavaScript APIs

### Linter

### Parser

### VSCode
```

When you edit a blank section:

- If your PR adds a **breaking change**, create a new heading called `#### BREAKING CHANGES` and add
  bullet point that explains the breaking changes; provide a migration path if possible.
- If your PR adds a new feature of a fix, create a new heading called `#### Other changes` and
  add a bullet point that explains the fix or the new feature. Make sure that this new heading
  appears after the `#### BREAKING CHANGES` heading.

##### Writing a changelog line

- Use the present tense, e.g. "Add new feature", "Fix edge case".
- If you fix a bug, please add the link to the issue, e.g. "Fix edge case [#4444]()".
- Whenever applicable, add a code block to show your new changes. For example, for a new
  rule you might want to show an invalid case, for the formatter you might want to show
  how the new formatting changes, and so on.

#### Documentation

If your PR requires some update on the website (new features, breaking changes, etc.), you should create a new PR once the previous PR is successfully merged.
When adding new features, the documentation should be part of a new PR, which will be merged right before the release.

#### Magic comments

- `!bench_parser` benchmarks the parser's runtime performance and writes a comment with the results;
- `!bench_formatter` benchmarks the formatter runtime performance and writes a comment with the results;
- `!bench_analyzer` benchmarks the analyzer runtime performance and writes a comment with the results;

### Analyzers and lint rules

To know the technical details of how our analyzer works, how to create a rule and how to write tests, please check our [internal
documentation page](https://rustdocs.rome.tools/biome_analyze/index.html)

### JavaScript Parser

To know the technical details of how our JavaScript works and how to write test, please check our [internal
documentation page](https://rustdocs.rome.tools/rome_js_parser/index.html)

### Formatter

To know the technical details of how our formatter works and how to write test, please check our [internal
documentation page](https://rustdocs.rome.tools/rome_js_formatter/index.html)

### Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.

### Playground

- [run the playground locally](/website/playground/README.md)
