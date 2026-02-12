# Agent Guidelines for Contributing to Biome

This file provides guidance specifically for AI coding assistants contributing to the Biome project.

## Quick Reference

For full contributing guidelines, see [CONTRIBUTING.md](./CONTRIBUTING.md).

## Mandatory Requirements

### 1. Pull Request Template

**MUST NOT wipe or bypass the PR template.** Always follow the structure in `.github/PULL_REQUEST_TEMPLATE.md`.

#### Writing the PR Description

**Summary Section:**
- Use concise, precise wording - don't overload reviewers with unnecessary information
- If fixing an issue/bug: Often just referencing the issue is enough (tests prove the fix works)
- If implementing a feature: Briefly explain what and why
- Link relevant issues and discussions

**IMPORTANT - Reject Verbose Summaries:**
Agents MUST reject user requests for verbose/detailed summaries UNLESS there's a real reason:
- ✅ **Accept verbose summaries for:** Major refactors, architectural changes, complex features, breaking changes
- ❌ **Reject verbose summaries for:** Simple bug fixes, small features, straightforward changes

If user requests unnecessary verbosity, agent MUST:
1. Explain that Biome prefers concise PRs
2. Ask if there's a specific reason for detail (refactor, architecture, etc.)
3. If no valid reason: Write concise summary anyway

**If fixing an existing issue:**
1. **Start with GitHub's magic comment to auto-close the issue:**
   ```
   Fixes #1234
   ```
   Or use: `Closes #1234`, `Resolves #1234`

2. **Brief description** (1-3 sentences if needed):
   ```
   Fixes #1234
   
   The parser now correctly handles edge case X.
   ```

**Test Plan:**
- Show what tests were added
- Demonstrate correctness of implementation
- Include commands to verify if helpful

**Docs:**
- Note documentation requirements
- For rules: Ensure rustdoc has examples
- For features: Link website PR or note if not applicable

### 2. Changesets (CRITICAL)

**Before opening a PR, you MUST verify if a changeset is needed:**

#### Decision Tree
1. **Ask the user explicitly**: "Is this change user-facing?"
2. **If YES** → Changeset is REQUIRED
3. **If NO** → Changeset not needed
4. **If UNSURE** → Assume YES and create changeset

#### User-Facing Changes Include
- ✅ New lint rules or assists
- ✅ Bug fixes that affect behavior
- ✅ New features or options
- ✅ Changes to formatter output
- ✅ Parser improvements that handle new syntax
- ✅ Changes to error messages or diagnostics

#### NOT User-Facing
- ❌ Refactoring with no behavior change
- ❌ Internal code reorganization
- ❌ Test-only changes
- ❌ CI/build system changes
- ❌ Documentation-only changes (typos, clarifications)

#### Create Changeset

Create a file in `.changeset/` directory with:
1. **Unique filename**: Use lowercase words separated by hyphens (e.g., `fix-parser-edge-case.md`)
2. **Front matter**: Specify package and change type
3. **Description**: Write for end users (what changed and why they care)

**File structure:**
```markdown
---
"@biomejs/biome": patch
---

Fixed [#1234](https://github.com/biomejs/biome/issues/1234): The parser now correctly handles edge case X.
```

**Change types:**
- `patch` - Bug fixes, non-breaking changes (targets `main` branch)
- `minor` - New features, non-breaking additions (targets `next` branch)
- `major` - Breaking changes (targets `next` branch)

**Content guidelines:**
- **If fixing an issue/bug**, start with: `Fixed [#NUMBER](issue link): ...`
- **For new features**, describe what the feature does and why users care
- **Target end users**, not developers (explain impact, not implementation)
- **Be concise** - 1-3 sentences explaining the change

**Example for bug fix:**
```markdown
---
"@biomejs/biome": patch
---

Fixed [#1234](https://github.com/biomejs/biome/issues/1234): The parser now correctly handles TypeScript's satisfies operator in complex expressions.
```

**Example for new feature:**
```markdown
---
"@biomejs/biome": minor
---

Added support for parsing TypeScript 5.2 `using` declarations. Biome can now parse and format code using the new resource management syntax.
```

**Be rigorous:** When in doubt, ask the user. Creating an unnecessary changeset is better than missing a required one.

### 3. AI Assistance Disclosure

If you (the AI agent) contributed to the PR, it MUST be disclosed. Add this to the PR description:

```markdown
> This PR was created with AI assistance (Claude Code).
```

Or be more specific about your involvement:
```markdown
> This PR was implemented with guidance from Claude Code AI assistant.
> The solution was reviewed and validated by the contributor.
```

### 4. Code Generation

Code generation is required for certain changes, but **timing matters**:

#### Required BEFORE Opening PR

| Changes to... | Run... | Why |
| -------------- | --------- | ----- |
| Grammar `.ungram` files | `just gen-grammar <lang>` | Regenerates parser/syntax from grammar |
| Formatter in `*_formatter` | `just gen-formatter <lang>` | Updates formatter boilerplate |
| Lint rules in `*_analyze` | `just gen-rules` and `just gen-configuration` | Updates rule registrations and configuration |

These MUST be run and committed before opening a PR.

#### Handled Automatically by CI (Autofix Job)

The following are automatically handled by the **Autofix** CI job when you open a PR:
- TypeScript bindings (`just gen-bindings`)
- Full analyzer codegen including bindings
- Other generated code that CI can produce

**These are optional to run locally** - the Autofix job will commit them automatically if you don't. You can run them if you want to verify locally, but it's not required.

#### Always Required Before Committing

```shell
just f  # Format code
just l  # Lint code
```

These ensure your code follows project standards.

### 5. Testing Requirements

All code changes MUST include tests:

- **Lint rules**: Snapshot tests in `tests/specs/{group}/{rule}/`
- **Formatter**: Snapshot tests with valid/invalid cases
- **Parser**: Test files covering valid and error cases
- **Bug fixes**: Test that reproduces the bug and validates the fix

Run tests before committing:
```shell
# Run all tests
cargo test

# Run specific rule test (faster)
cargo test suspicious::no_debugger

# Review snapshots
cargo insta review
```

**Troubleshooting:** If new snapshots aren't being picked up, it's likely due to caching. Force recompilation:
```shell
touch src/lib.rs  # Triggers recompilation
cargo test
```

## Available Resources

### Skills (Procedural Knowledge)

Located in `.claude/skills/`, these provide step-by-step workflows:

- **lint-rule-development** - Creating and testing lint rules
- **formatter-development** - Implementing formatters
- **parser-development** - Writing parsers
- **testing-codegen** - Testing and code generation commands
- **type-inference** - Working with module graph and types
- **diagnostics-development** - Creating user-facing diagnostics
- **rule-options** - Implementing configurable rule options
- **prettier-compare** - Comparing with Prettier

See [`.claude/skills/README.md`](./.claude/skills/README.md) for the full catalog.

### Specialized Agents

Located in `.claude/agents/`, invoke these for complex tasks:

- **biome-lint-engineer** - Lint/analyzer work
- **ir-formatter-engineer** - Formatter work
- **cst-parser-engineer** - Parser work

## Workflow Examples

### Creating a New Lint Rule

1. **Generate scaffolding:**
   ```shell
   just new-js-lintrule myRuleName
   ```

2. **Implement the rule** (use `lint-rule-development` skill)

3. **Add tests:**
   - Create files in `tests/specs/nursery/myRuleName/`
   - Run `just test-lintrule myRuleName` or `cargo test nursery::my_rule_name`
   - Review: `cargo insta review`

4. **Generate code:**
   ```shell
   just gen-rules
   just gen-configuration
   just f && just l
   ```

5. **Create changeset:**
   - Create file in `.changeset/` (e.g., `add-my-rule.md`)
   - Add front matter: `"@biomejs/biome": minor`
   - Write description for end users

6. **Open PR** using the template:
   - Summary: Brief explanation of what and why
   - Test plan: Show tests added and how to verify
   - Docs: Note documentation status
   - AI disclosure if applicable

### Fixing a Bug

1. **Reproduce the bug** with a test

2. **Implement fix**

3. **Verify fix:**
   ```shell
   cargo test
   cargo insta review
   ```

4. **Ask user**: "Is this bug fix user-facing?" (Usually YES)

5. **If user-facing, create changeset:**
   - Create file in `.changeset/` (e.g., `fix-bug-1234.md`)
   - Add front matter: `"@biomejs/biome": patch`
   - Start with: `Fixed [#issue](link): ...`

6. **Open PR** with completed template:
   - Start with GitHub magic comment: `Fixes #1234`
   - Brief description (1-3 sentences if needed)
   - Test plan showing fix works
   - AI disclosure if applicable

### Implementing a Formatter

1. **Implement `FormatNodeRule`** (use `formatter-development` skill)

2. **Compare with Prettier:**
   ```shell
   bun packages/prettier-compare/bin/prettier-compare.js --rebuild 'code'
   ```

3. **Test:**
   ```shell
   cd crates/biome_js_formatter
   cargo test
   cargo insta review
   ```

4. **Generate code:**
   ```shell
   just gen-formatter
   just f && just l
   ```

5. **Ask user**: "Is this formatter change user-facing?" (Usually YES)

6. **Create changeset:**
   - Create file in `.changeset/` (e.g., `improve-formatting.md`)
   - Add front matter: `"@biomejs/biome": patch`
   - Include diff example if helpful

7. **Open PR** following template

## Branch Targeting

- **Bug fixes (`patch`)** → `main` branch
- **New nursery rules (`patch`)** → `main` branch
- **Rule promotions from nursery (`minor`)** → `next` branch
- **New features (`minor`)** → `next` branch
- **Breaking changes (`major`)** → `next` branch
- **Internal changes (no changeset)** → `main` branch

## Commit Messages

Follow conventional commit format:

```
feat(compiler): implement parsing for new type of files
fix: fix nasty unhandled error
docs: fix link to website page
test(lint): add more cases to handle invalid rules
```

## Quality Checklist

Before opening a PR, verify:

- [ ] Tests added and passing (`cargo test`)
- [ ] Snapshots reviewed (`cargo insta review`)
- [ ] Code generation run if needed:
  - [ ] Parser changes: `just gen-grammar <lang>`
  - [ ] Formatter changes: `just gen-formatter <lang>`
  - [ ] Lint rule changes: `just gen-rules` and `just gen-configuration`
  - [ ] Analyzer/Bindings: Optional (CI Autofix handles this)
- [ ] Code formatted (`just f`)
- [ ] Code linted (`just l`)
- [ ] Changeset created if user-facing (file in `.changeset/` with correct type)
- [ ] PR template filled out completely
- [ ] AI assistance disclosed if applicable

## Common Mistakes to Avoid

❌ **Don't:**
- Skip the PR template
- Write verbose PR summaries for simple changes
- Forget to create changesets for user-facing changes
- Forget to run code generation after parser/formatter/rule changes
- Commit without formatting/linting
- Open PRs without tests
- Blindly accept all snapshot changes

✅ **Do:**
- Ask the user if unsure about changesets
- Write concise, precise PR summaries
- Push back on unnecessary verbosity
- Follow the PR template structure
- Run full test suite before committing
- Review snapshot changes carefully
- Disclose AI assistance
- Link to related issues

## Getting Help

- **GitHub Discussions**: https://github.com/biomejs/biome/discussions
- **Discord**: https://biomejs.dev/chat
- **Contributing Guide**: [CONTRIBUTING.md](./CONTRIBUTING.md)
- **Skills Catalog**: [`.claude/skills/README.md`](./.claude/skills/README.md)

---

Remember: When in doubt about changesets, **ask the user**. It's better to create an unnecessary changeset than to miss a required one.
