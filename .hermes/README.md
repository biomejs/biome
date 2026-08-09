# Hermes Agent Configuration for Biome

This directory contains Hermes Agent configuration for contributing to and working with the **Biome** repository.

## What is Hermes Agent?

[Hermes Agent](https://hermes-agent.nousresearch.com/) is an open-source AI agent framework by Nous Research that runs in your terminal, desktop app, and IDE. It's built for autonomous code generation, multi-agent workflows, and seamless developer collaboration.

Unlike traditional AI coding assistants, Hermes:
- **Learns from experience** through reusable skills
- **Persists memory** across sessions and projects
- **Multi-platform** — runs on CLI, desktop, Slack, Discord, Teams, and more
- **Provider-agnostic** — swap models mid-workflow without reconfiguring
- **Self-improving** — saves procedures as skills for reuse in future projects

## Why This Config Exists

This `.hermes/` directory standardizes the development environment for Biome within Hermes Agent sessions. It provides:

- **Consistent setup instructions** — automatically installs Rust, just, pnpm, and dev tools
- **Verification commands** — quickly confirm that your build environment is working (tests, linting, formatting)
- **Metadata** — language, repository links, stars, tags for discovery
- **Developer context** — Hermes can load this config to understand the repo's structure without asking

## How to Use

### Quick Start with Hermes

```bash
hermes chat -q "Help me set up Biome for development and run the test suite."
```

Hermes will:
1. Load this `.hermes/config.yaml`
2. Follow the setup steps (install Rust, just, pnpm, dev tools)
3. Build the project
4. Run verification commands (tests, linting, formatting)
5. Report success or flag missing dependencies

### Manual Setup

If you prefer to set up without Hermes:

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install just command runner
curl --proto '=https' --tlsv1.2 -sSf https://just.systems/install.sh | bash -s -- --to /usr/local/bin

# Install pnpm
curl -fsSL https://get.pnpm.io/install.sh | sh -

# Install dev tools
just install-tools

# Build
cargo build --release
```

### Verification

Confirm everything is working:

```bash
just test-quick biome_cli     # Run quick tests
just lint                      # Lint code
just format --check            # Check formatting
cargo run --release -p biome_cli -- --version  # Binary verification
```

### Running Biome

Basic usage:

```bash
# Format code
just format src/

# Lint code
just lint src/

# Run quick tests
just test-quick biome_cli
```

## Config Structure

- **name** — Repository name (`biome`)
- **description** — What the project does
- **language** — Primary languages (Rust, JavaScript/TypeScript)
- **repository** — Upstream and fork URLs
- **setup** — Step-by-step build instructions
- **verification** — Commands to confirm a working build
- **tags** — Searchable keywords (e.g., `linter`, `formatter`, `rust`, `javascript`, `toolchain`)

## Contributing

See the upstream repository's [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines.

Biome welcomes contributions in:
- Bug fixes and improvements
- New rules and features
- Performance optimizations
- Documentation
- Test coverage

## Issues or Improvements?

If this config is outdated or incomplete:
1. Update `.hermes/config.yaml` with the corrected setup/verification steps
2. Test locally: `just install-tools && just test-quick biome_cli`
3. Commit and push to your fork
4. Open a PR with description of changes

---

**Last updated:** 2026-08-09  
**Repo:** [biomejs/biome](https://github.com/biomejs/biome)  
**Stars:** 25,537 ⭐
