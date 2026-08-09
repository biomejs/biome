# Hermes Agent Guidelines for Biome

This folder helps Hermes agents (AI coding assistants) understand and safely contribute to this repository.

## Repository Overview

- **Purpose:** Fast formatter and linter for JavaScript, TypeScript, JSON, CSS, and more
- **Language:** Rust (+ TypeScript/Node.js for tooling)
- **Build tool:** cargo + just
- **Test command:** `just test`

## What Hermes Should Do

✓ Fix bugs from reported issues labeled `help wanted` or `good first issue`  
✓ Add or improve tests for specific formatters/linters  
✓ Improve documentation and error messages  
✓ Refactor code for clarity (small scope)  
✓ Add type hints and docstrings  

## What Hermes Should NOT Do

✗ Major architectural changes without discussion  
✗ Add new public APIs or core behaviors without consensus  
✗ Add external dependencies without discussion  
✗ Modify CI/CD workflows (`.github/workflows/**`)  
✗ Touch lock files (`Cargo.lock`, `pnpm-lock.yaml`)  
✗ Use AI to write PR descriptions or communication (manual only)  

## Setup Instructions

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install required tools
cargo install just
just install-tools

# Install Node deps
pnpm install

# Clone and build
git clone https://github.com/biomejs/biome.git
cd biome
cargo build --release
```

## Verification Commands

Before submitting a PR, Hermes must verify:

```bash
# Format and lint (Rust)
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings

# Format and lint (Node)
pnpm run format
pnpm run lint

# Run tests
just test
```

## Key Files to Understand

- `CONTRIBUTING.md` — contribution guidelines and AI disclosure requirement
- `README.md` — project overview
- `Cargo.toml` — Rust workspace manifest
- `package.json` — Node.js tooling
- `crates/` — organized by subsystem
- `src/` — source code
- `tests/` — test suite

## Issue Labels to Target

Good for Hermes contributions:
- `good first issue` — lower barrier to entry
- `help wanted` — explicitly good for community
- `bug` — concrete, scoped fixes
- `documentation` — writing improvements

Avoid:
- `blocked-by-upstream` — external blocker
- Issues without clear community indication

## Important: AI Disclosure

**⚠️ If you use ANY AI assistance, you MUST disclose it in your PR.** Include:
- Which AI tool(s) you used (e.g., Claude Code, ChatGPT)
- What extent of usage (e.g., "generated tests", "consulted for design")

Example: "This PR was written primarily by Claude Code. I manually reviewed all changes."

## Quick Tips

1. Read recent merged PRs to understand patterns
2. Always run `just test` locally before committing
3. Follow Rust idioms and existing code style
4. Keep PRs focused — one feature/fix per PR
5. Reference the issue number in your commit message
6. Disclose any AI assistance used (see above)

---

For more about Hermes Agent, see: https://hermes-agent.nousresearch.com
