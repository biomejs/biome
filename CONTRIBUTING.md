# 🚀 Contributing

We can use help in a bunch of areas and any help is greatly appreciated!

## Table of Contents
- [🚀 Contributing](#-contributing)
  * [Asking questions, making proposals](#asking-questions-making-proposals)
  * [Reporting bugs](#reporting-bugs)
  * [Getting Started](#getting-started)
  * [Install the required tools](#install-the-required-tools)
  * [Testing](#testing)
    + [Debugging](#debugging)
  * [Debug binaries](#debug-binaries)
  * [Production binaries](#production-binaries)
  * [Checks](#checks)
  * [Crates development](#crates-development)
    + [Create new crates](#create-new-crates)
    + [Analyzers and lint rules](#analyzers-and-lint-rules)
    + [Parser](#parser)
    + [Formatter](#formatter)
  * [Crate dependencies](#crate-dependencies)
  * [Node.js development](#nodejs-development)
    + [Translations](#translations)
  * [Commit messages](#commit-messages)
  * [Creating pull requests](#creating-pull-requests)
    + [Changelog](#changelog)
      - [Choose the correct packages](#choose-the-correct-packages)
      - [Choose the correct type of change](#choose-the-correct-type-of-change)
      - [Writing a changeset](#writing-a-changeset)
    + [Documentation](#documentation)
    + [Versioning](#versioning)
  * [Releasing](#releasing)
  * [Resources](#resources)
  * [Current Members](#current-members)
    + [Lead team](#lead-team)
    + [Core Contributors team](#core-contributors-team)
    + [Maintainers team](#maintainers-team)
    + [Past Maintainers](#past-maintainers)

## Asking questions, making proposals

If you have any questions, proposals, or feedbacks, open a [GitHub discussion](https://github.com/biomejs/biome/discussions).
Make sure your comment adds value: [don't post a comment just to get attention](https://jacobtomlinson.dev/posts/2022/dont-be-that-open-source-user-dont-be-me/).

Our [Discord server](https://biomejs.dev/chat) is open for help and more ad-hoc discussion.
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

You can use cargo to run Biome CLI in development mode:

```bash
# This is like running "biome --help"
cargo biome-cli-dev --help
```

## Install the required tools

We use [Just](https://just.systems/man/en/) to run scripts and tasks, to make our life easier.

You can install `just` using cargo:

```shell
cargo install just
```

But we **highly recommend** to [install it using an OS package manager](https://github.com/casey/just#packages),  so you won't need to prefix every command with `cargo`.

Once installed, run the following command install the required tools:

```shell
just install-tools
```

This command will install:
- `cargo-binstall`, to install binary extensions for `cargo`.
- `cargo-insta`, a `cargo` extension to manage snapshot testing inside the repository.
- `taplo-cli`, a small tool for formatting TOML files.
- `wasm-pack` and `wasm-tools` for managing the WASM build of Biome.

And you're good to go hack with Biome and Rust! 🚀

## Testing

You can either use `cargo` or `just` to run tests. For simplicity and running tests real quick, use `cargo`.

With `cargo`, you can run tests with using the `test` command:

```shell
# run tests
cargo test

# or use the shortcut
cargo t
```

If you run `cargo t` from the root, it will run **all** tests of the whole repository. If you're inside a crate folder, `cargo` will run **tests of that crate**:

```shell
cd crates/biome_cli

# it will run only the tests of the `biome_cli` crate
cargo t
```

You can run **a single test** with cargo by passing the test name after the `test` command:

```shell
cd crates/biome_js_formatter

cargo t quick_test
```

This will run the `quick_test` test inside he `biome_js_formatter` crate. You should see an output similar to this:

```shell
running 1 test
test quick_test::quick_test ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 224 filtered out; finished in 0.02s
```

You can also use `just` for running tests. With `just`, the scripts will use the same test runner we use in the CI.

```shell
just test
```

If you want to test the tests for a single crate:

```shell
just test-crate biome_cli
```

Rust has a concept of **doctest**. A doc test is a doc comment that runs some code. Usually, it looks something like this:

```rust
/// I am a doc test
/// ```
/// assert_eq!(true, true) // this is a doc test, and the assertion must pass
/// ```
fn some_fn() {

}
```
The code inside the code blocks is **run** during the testing phase.

To run only the doctest, you can run the command:

```shell
just test-doc
```

In some crates, we use snapshot testing. The majority of snapshot testing is done using [`insta`](https://insta.rs). `insta` is already installed by the command `just install-tools`.

When a snapshot test fails, you can run:

- `cargo insta accept` to accept all the changes and update all the snapshots;
- `cargo insta reject` to reject all the changes;
- `cargo insta review` to review snapshots singularly.

### Debugging

Sometimes you want to debug something when running tests. Like `console.log`, in JavaScript, in Rust you can use the macro `dbg!()` to print something during debugging something. Then, pass the option `--show-output` to `cargo`:

```rust
fn some_function() -> &'static str {
    let some_variable = "some_variable";
    dbg!(&some_variable);
    some_variable
}
#[test]
fn test_some_function() {
    let result = some_function();
    assert_eq!(result, "some_variable")
}
```

```shell
cargo t test_some_function --show-output
```

## Debug binaries

Creating a development binary is very useful in case you need to triage a reproduction, and you require more information like logging, trace logs, etc.

Additionally, you can use this binary when you need to debug issues related to LSP clients.

From the root of the repository, run the following command:

```shell
cargo build --bin biome
```
`cargo` will create a binary called `biome` in the `target/debug/` directory.

If you're debugging a CLI reproduction, copy the `biome` binary inside the root of the reproduction, and change any script that uses the npm package to use the binary instead:

```diff
{
  "scripts": {
-    "lint": "biome lint",
+    "lint": "./biome lint"
  }
}
```

If you're debugging an LSP reproduction, make sure that the client allows to use custom binary, like VSCode and Zed. Provide an absolute URL to the binary that was emitted.

```json
{
  "biome.lspBin": "/Users/john/www/biome/target/debug/biome"
}
```

## Production binaries

_Usually_, the easiest way to create a production build is to use the `--release` flag, **however** Biome requires an environment variable called `BIOME_VERSION` to generate different code at compile time.

When you provide a `BIOME_VERSION` that is _different_ from `0.0.0`, the build will turn off all the nursery rules that are recommended. The value of `BIOME_VERSION` doesn't matter, as long as it's different from `0.0.0`. This means that you'll have to provide a command similar to this:

```shell
BIOME_VERSION=0.0.1 cargo build --bin biome --release
```

## Checks

When you finished your work, and you are ready to **commit and open a PR**, there are few other
things you would need to run and check:
- `just f` (alias for `just format`), formats Rust and TOML files.
- `just l` (alias for `just lint`), run the linter for the whole project.
- Code generation. The code generation of the repository is spread in the different parts of the code base. Sometimes is needed and sometime it isn't:
  - run `just gen-analyzer` when you're working on the **linter**;
  - run `just gen-bindings` in case you worked around the **workspace**.

> [!NOTE]
> You can run `just ready` as well, although it's a command that runs the codegen of the whole repository, which will take some time

## Crates development

### Create new crates

If you happen to create a new _crate_ inside the workspace, use the command `just new-crate`, e.g.:

```shell
just new-crate biome_new_crate
```

Where `biome_new_crate` is going to be the name of the new crate. This script takes care of adding the correct template for the `Cargo.toml` file, and it adds the crate
to the `knope.toml` file, which we use for changelog generation.

### Analyzers and lint rules

To know the technical details of how our analyzer works, how to create a rule and how to write tests, please check our [internal page](https://github.com/biomejs/biome/blob/main/crates/biome_analyze/CONTRIBUTING.md)

### Parser

To know the technical details of how our parser works and how to write test, please check our [internal page](https://github.com/biomejs/biome/blob/main/crates/biome_parser/CONTRIBUTING.md)

### Formatter

To know the technical details of how our formatter works and how to write test, please check our [internal page](https://github.com/biomejs/biome/blob/main/crates/biome_formatter/CONTRIBUTING.md)


## Crate dependencies

[Workspace dependencies](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table) are used, and many dependencies are defined in Cargo.toml in the root.

Internal crates are loaded with `workspace = true` for each crate. About `dev-dependencies`, we use [path dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies) to avoid requiring the published version of these crates.

## Node.js development

The npm module `packages/@biomejs/biome` contains Biome's Node.js API that supports different backends:

- `wasm-nodejs` (WebAssembly)
- `backend-jsonrpc` (Connection to the daemon)

For testing and developing, you need to build these packages, following the steps:

1. install pnpm via [corepack](https://nodejs.org/api/corepack.html) by running `corepack enable`;
2. install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) globally;
3. run `pnpm --filter "@biomejs/backend-jsonrpc" build`;
4. run the `pnpm --filter "@biomejs/js-api" build:wasm-dev` and `pnpm --filter "@biomejs/js-api" build` commands;
5. run `pnpm i --filter "@biomejs/js-api" --frozen-lockfile` to link the WebAssembly bindings and the JSON-RPC bindings

The tests are run against the compiled files, which means that you need to run the
`build` script after you implemented features/bug fixes.

### Translations

For more information on how to help with translation, please see the [translation contribution guidelines for our docs](https://github.com/biomejs/website/blob/main/TRANSLATIONS.md).

## Commit messages

Internally, the Biome team adheres as closely as possible to the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0-beta.2/).
The following this convention encourages commit best-practices and facilitates commit-powered features like change log generation.

The following commit prefixes are supported:

- `build:`, a change that affects the build system or external dependencies
- `chore:`, project housekeeping
- `ci:`, a change that affects CI
- `docs:`, a documentation update
- `feat:`, a new feature
- `fix:`, a bugfix
- `perf:`, project performance
- `refactor:`, refactor of the code without change in functionality
- `release:`, release of a new version
- `revert:`, revert a previous change
- `test:`, a test update

Below are examples of well-formatted commits:

```txt
feat(compiler): implement parsing for new type of files
fix: fix nasty unhandled error
docs: fix link to website page
test(lint): add more cases to handle invalid rules
```

We are using [action-semantic-pull-request](https://github.com/amannn/action-semantic-pull-request) to lint the titles of pull requests. If the 'Lint Pull Request Titles' workflow fails, please correct the title.

## Creating pull requests

When creating a new pull request, it's preferable to use a conventional commit-formatted title, as this title will be used as the default commit message on the squashed commit after merging.
See the [dedicated section](#commit-messages) about conventional commit format.

Please use the template provided.

### Changelog

This repository uses [knope](https://knope.tech/) to automate the releases of Biome's binaries, the Rust crates, the JavaScript libraries and the creation of the `CHANGELOG.md` for each library/crate.

If the PR you're about to open is a bugfix/feature visible to users of the Biome toolchain or of the published Biome crates, you are encouraged to provide a **changeset** . To *create* a changeset, use the following command (*don't create it manually*):

```shell
just new-changeset
```
The command will present a prompt where you need to choose the libraries involved by the PR, the type of change (`major`, `minor` or `patch`) for each library, and a description of the change. The description will be used as name of the file.

The command will create the changeset(s) in the `.changeset` folder. You're free to open the file, and add more information in it.

#### Choose the correct packages

In the vast majority of cases, you want to choose the `cli` package, which represents the main binary (AKA the npm package `@biomejs/biome`). If your PR also changes any *public* crate that is published, you'll have to select it too.

For example, if your PR fixes a *JavaScript* lint rule and changes a public of the crate `biome_js_syntax`, you'll select:
- `cli`
- `biome_js_analyze`
- `biome_js_syntax`

And the frontmatter of the changset will look like this:

```markdown
---
cli: patch
biome_js_analyze: patch
biome_js_syntax: patch
---
```
With this, the description will be the same for all three packages.

However, there are cases where **you don't want** that, because we want to write a specific message for `cli` and another for `biome_js_syntax`. In this case, you're free to create **multiple changesets** in the *same PR*. In the example above, you would need to run `just new-chageset` twice, select `cli` the first time, and select `biome_js_syntax` and `biome_js_analyze` the second time.

#### Choose the correct type of change

We are very strict about `major` changes in the `cli` package. To better understand type of your change *for this package*, please refer to our [versioning page](https://biomejs.dev/internals/versioning/). Generally:
- `patch`: any sort of change that fixes a bug.
- `minor`: new features available to the users.
- `major`: a change that breaks a user API.

We are very liberal for the `biome_` crates, so don't be afraid to break the developer-facing APIs, as long as the change is properly described.

#### Writing a changeset

`knope` allows to generate two types of changesets, and you can use the one that you see fit:
1. A changeset with a single header (`#`). When there's a single header, `knope` will transform it into a single bullet point inside the `CHANGELOG.md`.
2. A changeset with a single header (`#`) and a paragraph beneath. `knope` will transform it into a specialised header inside the `CHANGELOG.md`.

For example, you have a PR that ships a bug fix, and a new feature, and you want to document both of them. The bugfix doesn't require a long description, while the feature might need some explanation. So create two changesets, that might look like this:

This is the changeset for the bugfix:
```markdown
---
biome_js_syntax: patch
---

# Fix [#000](https://path/to/000), where the parameter `foo` wasn't read by the function `apply_foo()`
```

This is the changeset for the feature, where you show the feature usage by having an example of configuration, with the shell result:
``````markdown
---
cli: minor
---

# Add the X feature

This feature does wonders, and you can use it with this configuration:

```json
{
    "configuration": "here"
}
```

And this happens:

```shell

```
``````

These two changesets will be compiled by `knope`, and the final result in the `CHANGELOG.md` will be as follows:

``````markdown
### Patches

- Fix [#000](https://path/to/000), where the parameter `foo` wasn't read by the function `apply_foo()`

### Features

#### Add the X feature

This feature does wonders, and you can use it with this configuration:

```json
{
    "configuration": "here"
}
```

And this happens:

```shell

```
``````

Regarding the description of the changeset, try to follow the these guidelines:

- Use the present tense, e.g. "Add new feature", "Fix edge case".
- If you fix a bug, please add the link to the issue, e.g. "Fix edge case [#4444]()".
- Whenever applicable, add a code block to show your new changes. For example, for a new
  rule you might want to show an invalid case, for the formatter you might want to show
  how the new formatting changes, and so on.

If in doubt, take a look at existing or past changesets.

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

1. [ ] Update `version` in [Biome's `package.json`](./packages/@biomejs/biome/package.json) if applicable.

1. [ ] **Update to the same `version` in all crates** if you publish crates. (`Cargo.toml` and `crates/**/Cargo.toml`)

1. [ ] Linter rules have a `version` metadata directly defined in their implementation.
   This field is set to `next` for newly created rules.
   This field must be updated to the new version.

1. [ ] Once the PR is merged, the CI will trigger the `Release: *` workflows. Once these workflows finish compiling the final artefact, **they need to be approved manually**.

1. [ ] Open a new PR in the [website repository](https://github.com/biomejs/website) to update the website with the new version number:
   `BIOME_VERSION=<version> pnpm run codegen:all`.
   This will also copy the configuration schema in the right place.

## Resources

We have several resources explaining about Biome. They will help you understand the project and codebase.

- [Rust Dublin October 2023 - Biome - YouTube](https://youtu.be/stxiUYmHn0s?si=C9cMsc93nNrZa-r1)
- [Rome, a Modern Toolchain! by Emanuele Stoppa - GitNation](https://portal.gitnation.org/contents/rome-a-modern-toolchain)
- [How to create a lint rule in Biome](https://www.youtube.com/watch?v=zfzMO3nW_Wo&t=343s)

## Current Members

Members are listed in alphabetical order. Members are free to use the full name, GitHub handle, or any other nickname they wish to be addressed. Members are free to disclose their pronouns.

### Lead team

- [Arend van Beelen @arendjr](https://github.com/arendjr)
- [Emanuele Stoppa @ematipico](https://github.com/ematipico)
- [Victorien Elvinger @Conaclos](https://github.com/Conaclos)

### Core Contributors team

- [Carson McManus @dyc3](https://github.com/dyc3)
- [Denis Bezrukov @denbezrukov](https://github.com/denbezrukov)
- [Hiroki Ihoriya @unvalley](https://github.com/unvalley)
- [Jon Egeland @faultyserver](https://github.com/faultyserver)
- [Nicolas Hedger @nhedger](https://github.com/nhedger)
- [Ze-Zheng Wu @Sec-ant](https://github.com/Sec-ant)

### Maintainers team

- [Dani Guardiola @DaniGuardiola](https://github.com/DaniGuardiola)
- [Justinas Delinda @minht11](https://github.com/minht11)
- [Madeline Gurriarán @SuperchupuDev](https://github.com/SuperchupuDev)
- [Vasu Singh @vasucp1207](https://github.com/vasucp1207)
- [Vo Hoang Long @vohoanglong0107](https://github.com/vohoanglong0107)
- [Yagiz Nizipli @anonrig](https://github.com/anonrig)
- [Yoshiaki Togami @togami2864](https://github.com/togami2864)
- [Yusuke Abe @chansuke](https://github.com/chansuke)
- [Zheyu Zhang @ah-yu](https://github.com/ah-yu)
- [zoomdong @fireairforce](https://github.com/fireairforce)

### Past Maintainers

- [Daiki Nishikawa @nissy-dev](https://github.com/nissy-dev) (Core contributor)
- [Takayuki Maeda @TaKO8Ki](https://github.com/TaKO8Ki) (Maintainer)
- [Victor Teles @victor-teles](https://github.com/victor-teles) (Maintainer)


