# 🚀 Contributing

We can use help in a bunch of areas and any help is greatly appreciated!

## Table of Contents

* [Asking questions, making proposals](#asking-questions-making-proposals)
* [Reporting bugs](#reporting-bugs)
* [Getting Started](#getting-started)
* [Install the required tools](#install-the-required-tools)
* [Testing](#testing)
  + [Debugging](#debugging)
* [Checks](#checks)
* [Crates development](#crates-development)
  + [Analyzers and lint rules](#analyzers-and-lint-rules)
  + [Parser](#parser)
  + [Formatter](#formatter)
* [Crate dependencies](#crate-dependencies)
* [Node.js development](#nodejs-development)
  + [Translations](#translations)
* [Commit messages](#commit-messages)
* [Creating pull requests](#creating-pull-requests)
  + [Changelog](#changelog)
    - [Writing a changelog line](#writing-a-changelog-line)
  + [Documentation](#documentation)
  + [Versioning](#versioning)
* [Releasing](#releasing)
* [Resources](#resources)
* [Current Members](#current-members)
  + [Lead team](#lead-team)
  + [Core Contributors team](#core-contributors-team)
  + [Maintainers team](#maintainers-team)

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
- `cargo-nextest`, a `cargo` extension to for optionally running tests faster.
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

## Checks

When you finished your work, and you are ready to **commit and open a PR**, there are few other
things you would need to run and check:
- `just f` (alias for `just format`), formats Rust and TOML files.
- `just l` (alias for `just lint`), run the linter for the whole project.
- Code generation. The code generation of the repository is spread in the different parts of the code base. Sometimes is needed and sometime it isn't:
  - run `just gen-lint` when you're working on the **linter**;
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

1. [ ] Update `version` in [Biome's `package.json`](./packages/@biomejs/biome/package.json) if applicable.

1. [ ] **Update to the same `version` in all crates** if you publish crates. (`Cargo.toml` and `crates/**/Cargo.toml`)

1. [ ] Linter rules have a `version` metadata directly defined in their implementation.
   This field is set to `next` for newly created rules.
   This field must be updated to the new version.
   Then execute `just gen-lint`.

1. [ ] Update the website with the new version number:
   `BIOME_VERSION=<version> just gen-web`.
   This will also copy the configuration schema in the right place.

1. [ ] Once the PR is merged, the CI will trigger the `Release: *` workflows. Once these workflows finish compiling the final artefact, **they need to be approved manually**.

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
- [Ze-Zheng Wu @Sec-ant](https://github.com/Sec-ant)
- [Zheyu Zhang @ah-yu](https://github.com/ah-yu)

### Past Maintainers

- [Daiki Nishikawa @nissy-dev](https://github.com/nissy-dev) (Core contributor)


