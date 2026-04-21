# Biome Agent Skills Catalog

This directory contains specialized skills for AI coding assistants working on Biome. Skills provide step-by-step workflows, code snippets, and best practices for specific development tasks.

## What Are Skills?

**Skills** are domain-specific instruction sets that can be loaded into the conversation context. They provide:
- Exact commands to run for common tasks
- Code snippet templates
- Testing workflows
- Best practices and tips
- Links to detailed documentation

Skills complement the specialized **agents** in `.claude/agents/` - agents are personas that do the work, skills are the procedural knowledge they reference.

## Universal Coding Standards

**CRITICAL: No Emojis Policy**

Emojis are BANNED in all code contributions and documentation:
- NO emojis in source code
- NO emojis in comments (code comments, rustdoc, etc.)
- NO emojis in diagnostic messages
- NO emojis in test files
- NO emojis in commit messages
- NO emojis in PR descriptions
- NO emojis in skill documents or agent instructions
- NO emojis in any generated code or text

This applies to all agents, all skills, and all contributions. Keep code and documentation professional and emoji-free.

## Available Skills

### Core Development Skills

| Skill | Purpose | Best Used With |
| ------- | --------- | ---------------- |
| **[lint-rule-development](./lint-rule-development/SKILL.md)** | Create and implement lint rules with semantic analysis | `biome-lint-engineer` |
| **[formatter-development](./formatter-development/SKILL.md)** | Implement formatting rules using IR-based formatter | `ir-formatter-engineer` |
| **[parser-development](./parser-development/SKILL.md)** | Write parsers with error recovery and grammar authoring | `cst-parser-engineer` |

### Supporting Skills

| Skill | Purpose | Best Used With |
| ------- | --------- | ---------------- |
| **[biome-developer](./biome-developer/SKILL.md)** | General development best practices, common gotchas, Biome-specific patterns | Any agent |
| **[testing-codegen](./testing-codegen/SKILL.md)** | Run tests, manage snapshots, generate code | Any agent |
| **[changeset](./changeset/SKILL.md)** | Create and write proper changesets for the CHANGELOG | Any agent |
| **[pull-request](./pull-request/SKILL.md)** | Create PRs with proper titles, descriptions, and branch targeting | Any agent |
| **[type-inference](./type-inference/SKILL.md)** | Work with module graph and type inference system | `biome-lint-engineer` |
| **[diagnostics-development](./diagnostics-development/SKILL.md)** | Create user-friendly error messages and diagnostics | Any agent |
| **[prettier-compare](./prettier-compare/SKILL.md)** | Compare Biome and Prettier formatting output and IR | `ir-formatter-engineer` |
| **[eslint-migrate-options](./eslint-migrate-options/SKILL.md)** | Write custom ESLint-to-Biome option migrators | Any agent |

## Quick Workflow Guide

### "I want to create a new lint rule"

1. Load the `lint-rule-development` skill
2. Run: `just new-js-lintrule myRuleName`
3. Implement the rule using patterns from the skill
4. Use `testing-codegen` skill to test and generate code
5. Optionally invoke `biome-lint-engineer` agent for guidance

**Example commands:**
```shell
just new-js-lintrule noDebugger
# Edit crates/biome_js_analyze/src/lint/nursery/no_debugger.rs
cargo test quick_test -- --show-output
just test-lintrule noDebugger
cargo insta review
just gen-analyzer
```

### "I'm working on the formatter"

1. Load the `formatter-development` skill
2. Implement `FormatNodeRule` for your node
3. Compare with Prettier using `packages/prettier-compare/` tool
4. Run snapshot tests
5. Optionally invoke `ir-formatter-engineer` agent

**Example commands:**
```shell
# Edit formatter implementation
cargo test
bun packages/prettier-compare/bin/prettier-compare.js --rebuild 'code snippet'
cargo insta review
```

### "I'm adding parser support"

1. Load the `parser-development` skill
2. Author `.ungram` grammar
3. Generate parser skeleton
4. Implement lexer and parse rules
5. Optionally invoke `cst-parser-engineer` agent

**Example commands:**
```shell
# Edit xtask/codegen/html.ungram
just gen-grammar html
# Implement parser in crates/biome_html_parser/
cargo test
```

### "I need to understand type inference"

1. Load the `type-inference` skill
2. Review module graph concepts
3. Understand three resolution phases
4. Implement type-aware rule logic

### "I'm creating diagnostics"

1. Load the `diagnostics-development` skill
2. Use `#[derive(Diagnostic)]` macro
3. Implement `Advices` trait
4. Follow "show don't tell" principle

### "I'm adding rule options"

1. Load the `lint-rule-development` skill
2. Follow the "Adding Configurable Options" section
3. See `lint-rule-development/references/OPTIONS.md` for detailed patterns
4. Test with `options.json` files

## Agent + Skill Combinations

### Lint Rule Development
**Agent:** `biome-lint-engineer`
**Skills:** `lint-rule-development` + `testing-codegen`
**Use for:** Implementing new lint rules, adding semantic analysis, creating code actions, adding rule options

### Formatter Development
**Agent:** `ir-formatter-engineer`
**Skills:** `formatter-development` + `testing-codegen`
**Use for:** Implementing formatting rules, handling comments, comparing with Prettier

### Parser Development
**Agent:** `cst-parser-engineer`
**Skills:** `parser-development` + `testing-codegen`
**Use for:** Creating parsers, writing grammars, implementing error recovery

### Type-Aware Rules
**Agent:** `biome-lint-engineer`
**Skills:** `lint-rule-development` + `type-inference` + `testing-codegen`
**Use for:** Rules that need type information, semantic analysis across modules

### Quality Diagnostics
**Agent:** Any
**Skills:** `diagnostics-development` + `testing-codegen`
**Use for:** Improving error messages, adding helpful advice, creating code frames

## Skill Format

Each skill follows this structure:

```markdown
---
name: skill-name
description: Brief description with use case examples
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose
When to use this skill (1-2 sentences)

## Prerequisites
Required setup and tools

## Common Workflows
### Workflow Name
Exact commands and code snippets

## Tips
Non-obvious knowledge and gotchas

## References
Links to full documentation
```

Skills are designed to be **quick reference cards** - scan in 30 seconds and know exactly what to do.

## File Organization

```
.claude/skills/
├── README.md (this file)
├── biome-developer/
│   └── SKILL.md
├── changeset/
│   └── SKILL.md
├── lint-rule-development/
│   ├── SKILL.md
│   └── references/
│       └── OPTIONS.md
├── formatter-development/
│   └── SKILL.md
├── parser-development/
│   └── SKILL.md
├── pull-request/
│   └── SKILL.md
├── testing-codegen/
│   └── SKILL.md
├── type-inference/
│   └── SKILL.md
├── diagnostics-development/
│   └── SKILL.md
├── prettier-compare/
│   └── SKILL.md
└── eslint-migrate-options/
    └── SKILL.md
```

## Contributing New Skills

When adding a new skill:

1. Create directory: `.claude/skills/skill-name/`
2. Create `SKILL.md` with standard structure
3. Include frontmatter with `name`, `description`, and `compatibility`
4. Provide exact, copy-pasteable commands
5. Use real examples from Biome codebase
6. Keep `SKILL.md` concise (under 500 lines)
7. For detailed reference material, use a `references/` subdirectory
8. Link to detailed CONTRIBUTING.md docs
9. Update this README with the new skill

## Additional Resources

- **Agent guidelines:** `../../AGENTS.md` (mandatory rules for AI assistants)
- **Main contributing guide:** `../../CONTRIBUTING.md`
- **Specialized agents:** `../agents/`
- **Settings:** `../settings.json`
- **Biome documentation:** https://biomejs.dev
- **Internals documentation:** https://biomejs.dev/internals

## Questions or Feedback

- **GitHub Discussions:** https://github.com/biomejs/biome/discussions
- **Discord:** https://biomejs.dev/chat
- **Issues:** https://github.com/biomejs/biome/issues
