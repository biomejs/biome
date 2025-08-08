# Biome Development Instructions

**ALWAYS follow these instructions first. Only fallback to additional search and context gathering if the information here is incomplete or found to be in error.**

Biome is a Rust-based monorepo implementing a high-performance web development toolchain for JavaScript, TypeScript, CSS, JSON, HTML, GraphQL, and Grit. The architecture follows a modular, language-agnostic design using Just task runner and Cargo.

## Quick Start

```bash
# Clone and navigate
git clone https://github.com/biomejs/biome
cd biome

# Install Rust toolchain (if not present)
# Requires Rust 1.88.0 - check rust-toolchain.toml

# Install Just task runner
cargo install just

# Install development tools - TAKES 15-30 MINUTES
# NEVER CANCEL: Tool installation may have network timeouts but will eventually succeed
just install-tools

# Install pnpm for changesets
npm install -g pnpm@10.12.1

# Install dependencies
pnpm install
```

## Critical Build and Test Timings

**NEVER CANCEL long-running commands. Set generous timeouts and wait for completion.**

- `just install-tools`: **15-30 minutes** (network timeouts are normal)
- `cargo check --workspace`: **3-5 minutes** 
- `cargo build --workspace`: **5-8 minutes** 
- `cargo test --workspace`: **8-15 minutes** 
- `just gen-all`: **3-5 minutes**
- `just ready`: **15-25 minutes** (runs full CI suite)
- `just format`: **30 seconds** (requires taplo-cli)
- `just lint`: **4-5 minutes**

**Always use timeouts of 60+ minutes for build commands and 30+ minutes for test commands.**

## Essential Development Commands

### Primary Commands (Use Just)

```bash
# Install/upgrade development tools
just install-tools        # NEVER CANCEL: 15-30 minutes
just upgrade-tools

# Format code (Rust + TOML) - requires taplo-cli
just format               # 30 seconds

# Run all tests - NEVER CANCEL: 8-15 minutes  
just test

# Run linting - NEVER CANCEL: 4-5 minutes
just lint

# Check if ready for CI - NEVER CANCEL: 15-25 minutes
just ready

# Generate all code-generated files
just gen-all              # 3-5 minutes
```

### Cargo Commands (Direct Rust)

```bash
# Run Biome CLI in development mode (first run: 4 minutes)
cargo biome-cli-dev --help
cargo biome-cli-dev check --write

# Build workspace - NEVER CANCEL: 5-8 minutes
cargo build --workspace

# Check workspace - NEVER CANCEL: 3-5 minutes  
cargo check --workspace

# Run tests for specific crate
cargo test -p biome_html_formatter

# Build release mode
cargo build --release

# Run linting only (bypasses just dependencies)
cargo lint                # 4-5 minutes
```

### Rule Development Commands

```bash
# Create new lint rules (name must be camelCase)
just new-js-lintrule ruleName
just new-css-lintrule ruleName
just new-json-lintrule ruleName
just new-graphql-lintrule ruleName

# Test specific lint rule
just test-lintrule ruleName

# Move rule from nursery to stable group
just move-rule stable ruleName
```

## Validation and CI Preparation

**Always run these before committing:**

```bash
# Format code
just format

# Generate code if you modified grammars/rules
just gen-all

# Lint code
just lint

# Run full CI check - NEVER CANCEL: 15-25 minutes
just ready

# Run tests
just test
```

## Common Development Scenarios

### Making Code Changes

1. Make your changes
2. Run `just format` 
3. If you modified grammars or added rules: `just gen-all`
4. Run `just lint` to check for issues
5. Run `just test` to verify changes
6. Run `just ready` before submitting PR

### Creating a New Lint Rule

```bash
# Create rule (use camelCase name)
just new-js-lintrule myNewRule

# This automatically runs gen-analyzer which:
# - Generates code
# - Updates configuration 
# - Updates migration
# - Updates bindings
# - Formats code

# Test the rule
just test-lintrule myNewRule

# Run full validation
just ready
```

### Working with Snapshots

```bash
# Update snapshots when tests change
cargo insta accept

# Review snapshots individually  
cargo insta review

# Reject all snapshot changes
cargo insta reject
```

## Manual Validation Requirements

**Always test actual functionality after making changes:**

### CLI Testing
```bash
# Test basic CLI functionality
cargo biome-cli-dev --help
cargo biome-cli-dev version
cargo biome-cli-dev --version

# Test formatting
echo "let x=1" > test.js
cargo biome-cli-dev format test.js
cargo biome-cli-dev check test.js

# Test linting
cargo biome-cli-dev lint test.js

# Test with actual files
cargo biome-cli-dev check . --write
```

### Build Validation
```bash
# Check workspace (3-5 minutes)
cargo check --workspace

# Build workspace (5-8 minutes) 
cargo build --workspace

# Test compilation and basic functionality
cargo biome-cli-dev --version
```

### End-to-End Testing
```bash
# Create test files and run complete workflows
cargo biome-cli-dev init
cargo biome-cli-dev check --write ./
```

## Repository Structure

### Core Components
- **biome_service**: Central workspace service providing unified APIs
- **biome_rowan**: Lossless syntax tree foundation
- **biome_formatter**: Pretty-printing infrastructure  
- **biome_analyze**: Rule-based analysis framework

### Language Pattern
Each language follows this structure:
```
biome_{lang}_syntax     # AST definitions
biome_{lang}_parser     # Parser implementation  
biome_{lang}_formatter  # Formatting logic
biome_{lang}_analyze    # Linting rules
biome_{lang}_factory    # AST construction
biome_{lang}_semantic   # Semantic analysis (where applicable)
```

### Key Locations
- Core crates: `/crates/biome_*/`
- Grammar definitions: `/xtask/codegen/*.ungram`
- Test snapshots: `tests/specs/` in each crate
- Configuration: `biome_configuration/src/`

## Troubleshooting

### Tool Installation Issues
`just install-tools` commonly experiences network timeouts but will eventually succeed:
- Network timeouts are normal - NEVER CANCEL, wait 15-30 minutes
- If it completely fails, install tools individually:
  ```bash
  cargo install cargo-insta
  cargo install taplo-cli  
  cargo install wasm-pack
  cargo install wasm-tools
  ```
- If `just format` fails with "taplo: not found", install taplo separately:
  ```bash
  cargo install taplo-cli
  ```

### Build Issues
- Always run `just format` first (or install taplo-cli if it fails)
- For grammar changes: `just gen-all`
- Check that Rust 1.88.0 is installed: `rustc --version`
- Clean build: `cargo clean && cargo check`
- Use `cargo lint` instead of `just lint` if taplo-cli is missing

### Test Failures
- Update snapshots: `cargo insta accept`
- Run specific tests: `cargo test -p crate_name`
- For rule tests: `just test-lintrule ruleName`

## Performance Notes

- Uses arena allocation for memory efficiency
- Implements incremental parsing/analysis
- Zero-copy string handling where possible
- All formatters use same IR for consistency

## Network Limitations

Tool installation and some dependency downloads may experience timeouts due to network restrictions. This is normal - commands will eventually succeed with patience. NEVER CANCEL builds or installations that appear to hang.

## Changes That Require Code Generation

Run `just gen-all` after:
- Modifying `.ungram` files
- Adding new syntax nodes
- Creating new lint rules
- Changing analyzer configurations

## Before Submitting PRs

**Required checklist:**
- [ ] Run `just format`
- [ ] Run `just gen-all` (if needed)
- [ ] Run `just lint` 
- [ ] Run `just test`
- [ ] Run `just ready` (full CI check)
- [ ] Manually test CLI functionality
- [ ] Create changeset with `just new-changeset` (if user-visible changes)

## Common File Patterns

```bash
# Repository root structure
ls -la
.github/        # CI/CD workflows and configs
crates/         # All Rust crates
xtask/          # Code generation and utilities  
packages/       # Node.js packages
scripts/        # Utility scripts
justfile        # Task definitions
Cargo.toml      # Workspace definition
biome.json      # Biome configuration
```

Remember: Biome aims for high performance and correctness. All changes should maintain these principles while following the established architectural patterns.