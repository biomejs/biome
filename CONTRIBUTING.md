# 🚀 Contributing

We can use help in a bunch of areas and any help is greatly appreciated!

# Table of Contents
- [Asking Questions, Making Proposals](#asking-questions-making-proposals)
- [Reporting Bugs](#reporting-bugs)
- [Getting Started](#getting-started)
- [Install the Required Tools](#install-the-required-tools)
- [Crates Development](#crates-development)
- [VS Code Extension Development](#vs-code-extension-development)
- [IntelliJ Extension Development](#intellij-plugin-development)
- [Node.js Development](#nodejs-development)
- [Website Development](#website-development)
- [Commit Messages](#commit-messages)
- [Creating Pull Requests](#creating-pull-requests)
- [Releasing](#releasing)
- [Resources](#resources)
- [Current Members](#current-members)

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

Alternatively, you can use our official [CodeSandbox template](https://codesandbox.io/p/sandbox/biome-starter-cbs-rky6zq).

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

## Crates development

### Analyzers and lint rules

To know the technical details of how our analyzer works, how to create a rule and how to write tests, please check our [internal page](https://docs.rs/biome_analyze/latest/biome_analyze/)

### Parser

To know the technical details of how our parser works and how to write test, please check our [internal page](https://docs.rs/biome_parser/latest/biome_parser/)

### Formatter

To know the technical details of how our formatter works and how to write test, please check our [internal page](https://docs.rs/biome_js_formatter/latest/biome_js_formatter/)

### Testing

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

### Checks

When you finished your work, and you are ready to **commit and open a PR**,
run the following command:

```shell
just ready
```

This command will run the same commands of the CI: format, lint, tests and code generation.
Eventually everything should be "green" 🟢 and commit all the code that was generated.

#### Generated files

If you work on some parser and create new nodes or modify existing ones, you must run a command to update some auto-generated files.

##### `cargo codegen grammar`

This command will update the syntax of the parsers.

The source is generated from the [`ungram` files](https://github.com/biomejs/biome/blob/main/xtask/codegen/js.ungram).

##### `cargo codegen test`

This command will create new tests for JS or JSON parser.
These tests are generated from inline comments found inside the source code.

On the other hand, we are moving away from this approach and have a straightforward process in other parser implementation like CSS.


##### `cargo codegen analyzer`

This command will detect linter rules declared in the `analyzers`, `assists` and `syntax` directories in the analyzer crates, e.g. `biome_js_analyze`, `biome_json_analyze`, etc., and regenerate the `registry.rs` file and its dependents to include all the rules.

### crate dependencies

[Workspace dependencies](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table) are used, and many dependencies are defined in Cargo.toml in the root.

Internal crates are loaded with `workspace = true` for each crate. About `dev-dependencies`, we use [path dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies) to avoid requiring the published version of these crates.

## VS Code extension development

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

### Debugging the VS Code extension

The Biome VS Code extension can be debugged by running the `Debug Extension` launch configuration
in VS Code. This will compile the extension, watch for modifications and start a separate VS Code
instance with only the Biome extension installed.

### User files

If files specific to your local development environment should be ignored, please add these files to a global git ignore file rather than to a git ignore file within Biome.

You can find more information on this process [here](https://help.github.com/en/github/using-git/ignoring-files#configuring-ignored-files-for-all-repositories-on-your-computer).

## Intellij plugin development

To start development from source, navigate to the `editors/intellij` directory.

Build and run the plugin requires:

- Java development kit 17+
- IntelliJ IDEA Ultimate edition

### Running the plugin on IDEA
```bash
./gradlew runIde
```

### Build the plugin
```bash
./gradlew buildPlugin
```

### UI Testing intellij plugin

Before start testing the plugin you will need to start IDE by invoking the `./gradlew runIdeForUiTests &` and wait IDE startup

You can now run the test task:

```bash
./gradlew test
```

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

## Website development

The [Biome website](https://biomejs.dev/) is built with [Astro](https://astro.build).
To start a development server you can run the following commands:

```bash
cd website
pnpm install
pnpm start
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

## Creating pull requests

When creating a new pull request, it's preferable to use a conventional commit-formatted title, as this title will be used as the default commit message on the squashed commit after merging.
See the [dedicated section](#Commit-messages) about conventional commit format.

Please use the template provided.

### Changelog

If the PR you're about to open is a bugfix/feature visible to Biome users, you CAN add a new bullet point to [CHANGELOG.md](./CHANGELOG.md). Although **not required**, we appreciate the effort.

At the top of the file you will see a `Unreleased` section.
The headings divide the sections by "scope"; you should be able to identify the scope that belongs to your change. If the change belongs to multiple scopes, you can copy the same sentence under those scopes.

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
  Read [how we version Biome](https://biomejs.dev/internals/versioning/) to determine if your change is breaking. A breaking change results in a major release.
- If your PR adds a new feature, enhances an existing feature, or fixes a bug, create a new heading called `#### New features`, `#### Enhancements`, or `#### Bug fixes`. Ultimately, add a bullet point that explains the change.

Make sure that the created subsections are ordered in the following order:

```md
#### BREAKING CHANGES

#### New features

#### Enhancements

#### Bug fixes
```

Because the website displays the changelog, you should update the website using the following command:

```sh
just gen-web
```

#### Writing a changelog line

- Use the present tense, e.g. "Add new feature", "Fix edge case".
- If you fix a bug, please add the link to the issue, e.g. "Fix edge case [#4444]()".
- You can add a mention `@user` for every contributor of the change.
- Whenever applicable, add a code block to show your new changes. For example, for a new
  rule you might want to show an invalid case, for the formatter you might want to show
  how the new formatting changes, and so on.

If in doubt, take a look to existing changelog lines.

### Documentation

If your PR requires some update on the website (new features, breaking changes, etc.), you should create a new PR once the previous PR is successfully merged.
When adding new features, the documentation should be part of a new PR, which will be merged right before the release.

### Magic comments

- `!bench_parser` benchmarks the parser's runtime performance and writes a comment with the results;
- `!bench_formatter` benchmarks the formatter runtime performance and writes a comment with the results;
- `!bench_analyzer` benchmarks the analyzer runtime performance and writes a comment with the results;

### Versioning

We follow the specs suggested by [the official documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#prerelease-extensions):

Odd minor versions are dedicated to pre-releases, e.g. `*.5.*` .
Even minor versions are dedicated to official releases, e.g. `*.6.*`.

## Releasing

When releasing a new version of a Biome, follow these steps:

1. [ ] Add a [changelog](./CHANGELOG.md) entry for every Pull Request that lacks one.
   You can filter [merged PRs that don't update the changelog](https://github.com/biomejs/biome/pulls?q=is%3Apr+is%3Amerged+-label%3AA-Changelog).
   Read our [guidelines for editing the changelog](#changelog).

1. [ ] Based on the [changelog](./CHANGELOG.md), determine which version number to use.
   See our [versioning guide](https://biomejs.dev/internals/versioning/) for more details.

1. [ ] Rename `Unreleased` to `<version> (iso-date)` in the [changelog](./CHANGELOG.md).
   Then update the website using `BIOME_VERSION=<version> cargo codegen-website`.

1. [ ] Update `version` in [Biome's `package.json`](./packages/@biomejs/biome/package.json) if applicable.

1. [ ] Update `version` in [Biome's LSP package.json](./editors/vscode/package.json) if applicable.
   Note that the LSP follows a [distinct versioning scheme](https://biomejs.dev/internals/versioning/#visual-studio-code-extension).

1. [ ] Update `version` in each published crates if applicable. (`Cargo.toml` and `crates/**/Cargo.toml`)

1. [ ] Linter rules have a `version` metadata directly defined in their implementation.
   This field is set to `next` for newly created rules.
   This field must be updated to the new version.
   Then execute `just gen-lint`.

1. [ ] Once the PR is merged, the CI will trigger the `Release: *` workflows. Once these workflows finish compiling the final artefact, **they need to be approved manually**.

## Resources

We have several resources explaining about Biome. They will help you understand the project and codebase.

- [Rust Dublin October 2023 - Biome - YouTube](https://youtu.be/stxiUYmHn0s?si=C9cMsc93nNrZa-r1)
- [Rome, a Modern Toolchain! by Emanuele Stoppa - GitNation](https://portal.gitnation.org/contents/rome-a-modern-toolchain)

## Current Members

Members are listed in alphabetical order. Members are free to use the full name, GitHub handle, or any other nickname they wish to be addressed. Members are free to disclose their pronouns.

### Lead team

- [Emanuele Stoppa @ematipico](https://github.com/ematipico)
- [Victorien Elvinger @Conaclos](https://github.com/Conaclos)

### Core Contributors team

- [Hiroki Ihoriya @unvalley](https://github.com/unvalley)
- [Daiki Nishikawa @nissy-dev](https://github.com/nissy-dev)
- [Denis Bezrukov @denbezrukov](https://github.com/denbezrukov)

### Maintainers team

- [Nicolas Hedger @nhedger](https://github.com/nhedger)
- [Victor Teles @victor-teles](https://github.com/victor-teles)
- [Madeline Gurriarán @SuperchupuDev](https://github.com/SuperchupuDev)
