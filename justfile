_default:
  just --list -u

alias f := format
alias t := test
alias r := ready
alias l := lint


# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-insta cargo-nextest taplo-cli wasm-pack wasm-tools cargo-workspaces

# Upgrades the tools needed to develop
upgrade-tools:
	cargo install cargo-binstall --force
	cargo binstall cargo-insta cargo-nextest taplo-cli wasm-pack wasm-tools cargo-workspaces --force

# Generate all files across crates and tools. You rarely want to use it locally.
codegen:
  cargo codegen all
  cargo codegen-configuration
  just codegen-bindings
  cargo format

# Generates TypeScript types and JSON schema of the configuration
codegen-bindings:
  cargo codegen-schema
  cargo codegen-bindings

# Generates code generated files for the linter
codegen-linter:
  cargo codegen analyzer
  cargo codegen-configuration
  just codegen-bindings
  cargo lintdoc

# Generates the linter documentation and Rust documentation
documentation:
  cargo lintdoc
  cargo documentation

# Creates a new lint rule in the given path, with the given name. Name has to be camel case.
new-lintrule path rulename:
  cargo run -p xtask_codegen -- newlintrule --path={{path}} --name={{rulename}}
  just codegen-linter
  just documentation

# Promotes a rule from the nursery group to a new group
promote-rule rulename group:
	cargo run -p xtask_codegen -- promoterule --rule={{rulename}} --group={{group}}
	just codegen-linter
	just documentation
	-cargo test -p rome_js_analyze -- {{snakecase(rulename)}}
	cargo insta accept


# Format Rust files and TOML files
format:
	cargo format
	taplo format



[unix]
_touch file:
  touch {{file}}

[windows]
_touch file:
  (gci {{file}}).LastWriteTime = Get-Date

# Run tests of all crates
test:
	cargo nextest run --no-fail-fast

# Run tests for the crate passed as argument e.g. just test-create rome_cli
test-crate name:
	cargo nextest run -E 'package({{name}})' --no-fail-fast

# Run doc tests
test-doc:
	cargo test --doc

# Tests a lint rule. The name of the rule needs to be camel case
test-lintrule name:
  just _touch crates/rome_js_analyze/tests/spec_tests.rs
  just _touch crates/rome_json_analyze/tests/spec_tests.rs
  cargo test -p rome_js_analyze -- {{snakecase(name)}}
  cargo test -p rome_json_analyze -- {{snakecase(name)}}

# Tests a lint rule. The name of the rule needs to be camel case
test-transformation name:
  just _touch crates/rome_js_transform/tests/spec_tests.rs
  cargo test -p rome_js_transform -- {{snakecase(name)}}


# Alias for `cargo lint`, it runs clippy on the whole codebase
lint:
	cargo lint

# When you finished coding, run this command to run the same commands in the CI.
ready:
  git diff --exit-code --quiet
  just codegen
  just documentation
  just format
  just lint
  just t
  cargo test --doc
  git diff --exit-code --quiet
