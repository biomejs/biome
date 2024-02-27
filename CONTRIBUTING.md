# 🚀 Contributing

We can use help in a bunch of areas and any help is greatly appreciated!

## Table of Contents

- [🚀 Contributing](#-contributing)
  - [Table of Contents](#table-of-contents)
  - [Asking questions, making proposals](#asking-questions-making-proposals)
  - [Reporting bugs](#reporting-bugs)
  - [Getting Started](#getting-started)
  - [Install the required tools](#install-the-required-tools)
  - [Crates development](#crates-development)
    - [Analyzers and lint rules](#analyzers-and-lint-rules)
    - [Parser](#parser)
    - [Formatter](#formatter)
    - [Testing](#testing)
    - [Checks](#checks)
      - [Generated files](#generated-files)
        - [`cargo codegen grammar`](#cargo-codegen-grammar)
        - [`cargo codegen test`](#cargo-codegen-test)
        - [`cargo codegen analyzer`](#cargo-codegen-analyzer)
    - [crate dependencies](#crate-dependencies)
  - [Node.js development](#nodejs-development)
  - [Website development](#website-development)
    - [Translations](#translations)
  - [Commit messages](#commit-messages)
  - [Creating pull requests](#creating-pull-requests)
    - [Changelog](#changelog)
      - [Writing a changelog line](#writing-a-changelog-line)
    - [Documentation](#documentation)
    - [Magic comments](#magic-comments)
    - [Versioning](#versioning)
  - [Releasing](#releasing)
  - [Resources](#resources)
  - [Current Members](#current-members)
    - [Lead team](#lead-team)
    - [Core Contributors team](#core-contributors-team)
    - [Maintainers team](#maintainers-team)

## Asking questions, making proposals

If you have any questions, proposals, or feedbacks, open a [GitHub discussion](https://github.com/biomejs/biome/discussions).
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
To start a development please check our [README](./website/README.md)

### Translations

For more information on how to help with translation, please see the [translation contribution guidelines for our docs](./website/TRANSLATIONS.md).

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

```txt
feat(compiler): implement parsing for new type of files
fix: fix nasty unhandled error
docs: fix link to website page
test(lint): add more cases to handle invalid rules
```

## Creating pull requests

When creating a new pull request, it's preferable to use a conventional commit-formatted title, as this title will be used as the default commit message on the squashed commit after merging.
See the [dedicated section](#commit-messages) about conventional commit format.

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

2. [ ] **Update to the same `version` in all crates** if you publish crates. (`Cargo.toml` and `crates/**/Cargo.toml`)

3. [ ] Linter rules have a `version` metadata directly defined in their implementation.
   This field is set to `next` for newly created rules.
   This field must be updated to the new version.
   Then execute `just gen-lint`.

4. [ ] Once the PR is merged, the CI will trigger the `Release: *` workflows. Once these workflows finish compiling the final artefact, **they need to be approved manually**.

## Resources

We have several resources explaining about Biome. They will help you understand the project and codebase.

- [Rust Dublin October 2023 - Biome - YouTube](https://youtu.be/stxiUYmHn0s?si=C9cMsc93nNrZa-r1)
- [Rome, a Modern Toolchain! by Emanuele Stoppa - GitNation](https://portal.gitnation.org/contents/rome-a-modern-toolchain)
- [How to create a lint rule in Biome](https://www.youtube.com/watch?v=zfzMO3nW_Wo&t=343s)

## Current Members

Members are listed in alphabetical order. Members are free to use the full name, GitHub handle, or any other nickname they wish to be addressed. Members are free to disclose their pronouns.

### Lead team

- [Emanuele Stoppa @ematipico](https://github.com/ematipico)
- [Victorien Elvinger @Conaclos](https://github.com/Conaclos)

### Core Contributors team

- [Arend van Beelen @arendjr](https://github.com/arendjr)
- [Daiki Nishikawa @nissy-dev](https://github.com/nissy-dev)
- [Denis Bezrukov @denbezrukov](https://github.com/denbezrukov)
- [Hiroki Ihoriya @unvalley](https://github.com/unvalley)
- [Jon Egeland @faultyserver](https://github.com/faultyserver)
- [Nicolas Hedger @nhedger](https://github.com/nhedger)

### Maintainers team

- [Dani Guardiola @DaniGuardiola](https://github.com/DaniGuardiola)
- [Madeline Gurriarán @SuperchupuDev](https://github.com/SuperchupuDev)
- [Takayuki Maeda @TaKO8Ki](https://github.com/TaKO8Ki)
- [Vasu Singh @vasucp1207](https://github.com/vasucp1207)
- [Victor Teles @victor-teles](https://github.com/victor-teles)
- [Yagiz Nizipli @anonrig](https://github.com/anonrig)
- [Yoshiaki Togami @togami2864](https://github.com/togami2864)
