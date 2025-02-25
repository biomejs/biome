_default:
  just --list -u

alias f := format
alias t := test
alias r := ready
alias l := lint
alias qt := test-quick


# Installs the tools needed to develop
install-tools:
	cargo install cargo-binstall
	cargo binstall cargo-insta taplo-cli wasm-pack wasm-tools

# Upgrades the tools needed to develop
upgrade-tools:
	cargo install cargo-binstall --force
	cargo binstall cargo-insta taplo-cli wasm-pack wasm-tools --force

# Generate all files across crates and tools. You rarely want to use it locally.
gen-all:
  cargo run -p xtask_codegen -- all
  just gen-configuration
  just gen-migrate
  just gen-bindings
  just format

# Generates TypeScript types and JSON schema of the configuration
gen-bindings:
  cargo codegen-schema
  cargo run -p xtask_codegen --features schema -- bindings

# Generates code generated files for the linter
gen-analyzer:
  cargo run -p xtask_codegen -- analyzer
  just gen-configuration
  just gen-migrate
  just gen-bindings
  just lint-rules
  just format

gen-configuration:
    cargo run -p xtask_codegen --features configuration -- configuration

# Generates code for eslint migration
gen-migrate:
   cargo run -p xtask_codegen --features configuration -- migrate-eslint

# Generates the initial files for all formatter crates
gen-formatter:
  cargo run -p xtask_codegen -- formatter

# Generates the Tailwind CSS preset for utility class sorting
[working-directory: 'packages/tailwindcss-config-analyzer']
gen-tw:
  pnpm build
  pnpm execute

# Generates the code of the grammars available in Biome
gen-grammar *args='':
    cargo run -p xtask_codegen -- grammar {{args}}

# Generates the linter documentation and Rust documentation
documentation:
  RUSTDOCFLAGS='-D warnings' cargo documentation

# Creates a new js lint rule with the given name. Name has to be camel case.
new-js-lintrule rulename:
  cargo run -p xtask_codegen -- new-lintrule --kind=js --category=lint --name={{rulename}}
  just gen-analyzer
  just documentation

# Creates a new js assist rule with the given name. Name has to be camel case.
new-js-assistrule rulename:
  cargo run -p xtask_codegen -- new-lintrule --kind=js --category=assist --name={{rulename}}
  just gen-analyzer
  just documentation

# Creates a new json assist rule with the given name. Name has to be camel case.
new-json-assistrule rulename:
  cargo run -p xtask_codegen -- new-lintrule --kind=json --category=assist --name={{rulename}}
  just gen-analyzer
  just documentation

# Creates a new css lint rule with the given name. Name has to be camel case.
new-css-lintrule rulename:
  cargo run -p xtask_codegen -- new-lintrule --kind=css --category=lint --name={{rulename}}
  just gen-analyzer

# Creates a new graphql lint rule with the given name. Name has to be camel case.
new-graphql-lintrule rulename:
  cargo run -p xtask_codegen -- new-lintrule --kind=graphql --category=lint --name={{rulename}}
  just gen-analyzer


# Promotes a rule from the nursery group to a new group
promote-rule rulename group:
	cargo run -p xtask_codegen -- promote-rule --name={{rulename}} --group={{group}}
	just gen-analyzer
	just documentation
	-cargo test -p biome_js_analyze -- {{snakecase(rulename)}}
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
  powershell -Command "(Get-Item {{file}}).LastWriteTime = Get-Date"


# Run tests of all crates
test:
	cargo test run --no-fail-fast

# Run tests for the crate passed as argument e.g. just test-create biome_cli
test-crate name:
	cargo test run -p {{name}} --no-fail-fast

# Run doc tests
test-doc:
	cargo test --doc

# Tests a lint rule. The name of the rule needs to be camel case
test-lintrule name:
  just _touch crates/biome_js_analyze/tests/spec_tests.rs
  just _touch crates/biome_json_analyze/tests/spec_tests.rs
  just _touch crates/biome_css_analyze/tests/spec_tests.rs
  just _touch crates/biome_graphql_analyze/tests/spec_tests.rs
  cargo test -p biome_js_analyze -- {{snakecase(name)}} --show-output
  cargo test -p biome_json_analyze -- {{snakecase(name)}} --show-output
  cargo test -p biome_css_analyze -- {{snakecase(name)}} --show-output
  cargo test -p biome_graphql_analyze -- {{snakecase(name)}} --show-output

# Tests a lint rule. The name of the rule needs to be camel case
test-transformation name:
  just _touch crates/biome_js_transform/tests/spec_tests.rs
  cargo test -p biome_js_transform -- {{snakecase(name)}} --show-output

# Run the quick_test for the given package.
test-quick package:
  cargo test -p {{package}} --test quick_test -- quick_test --nocapture --ignored


# Alias for `cargo lint`, it runs clippy on the whole codebase
lint:
  cargo lint

# Checks if the docs of the lint rules follow Biome's requirements
lint-rules:
  cargo run -p rules_check

# When you finished coding, run this command to run the same commands in the CI.
ready:
  git diff --exit-code --quiet
  just gen-all
  just documentation
  #just format # format is already run in `just gen-all`
  just lint
  just test
  just test-doc
  git diff --exit-code --quiet

# Creates a new changeset for the final changelog
new-changeset:
    pnpm changeset
