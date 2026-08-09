---
name: biome-code-review
description: Static, read-only code review of a Biome (github.com/biomejs/biome) pull request, local branch, commit range, diff, or working tree being prepared as a PR. Applies Biome's own conventions - nursery placement and naming for lint rules, diagnostics quality, allocation and borrow discipline around TokenText and SyntaxToken, parser error recovery, formatter IR and idempotency, codegen freshness, insta snapshots, dev-dependency paths, doc style, changesets, branch targeting, PR titles, and AI disclosure. Only when asked to review a Biome pull request, branch, commit range, diff, or working tree. Not for issue triage, bug reproduction, or implementing fixes. Reports findings only - never edits files or runs cargo, just, pnpm, tests, builds, or codegen.
compatibility: Designed for read-only review of the Biome codebase (github.com/biomejs/biome).
metadata:
  repository: biomejs/biome
  mode: read-only
---

# Biome Code Review

Review a proposed change for correctness and fit with the Biome codebase.

## Safety Boundary

This is a static review. Preserve the developer's working tree exactly as found.

- Do not create, edit, move, or delete files. Do not apply patches. Do not create a changeset.
- Do not run project code: no `cargo` (including `check`, `test`, `clippy`, `insta`), no `just`, `pnpm`, `npm`, `bun`, `node`, no codegen, benchmarks, `prettier-compare`, LSP, or daemon.
- Do not run mutating Git: no pull, merge, rebase, checkout, switch, worktree, reset, restore, clean, stash, commit, push, or `gh pr review|comment|edit|merge|close`.
- Do not use `curl`, `wget`, or arbitrary network tools.
- Do not use shell pipelines, `sed`, `awk`, or scripts to analyze source. Use file-reading, glob, and text-search tools.
- Do not delegate the review to another agent — the restrictions may not carry over. Loading a skill from the repository's `.claude/skills/` is **not** delegation; it pulls maintainer-authored instructions into this context and is expected.

Shell is limited to this allowlist:

```
git fetch origin main
git fetch origin next
git status --short --branch
git branch --show-current
git rev-parse ...
git merge-base ...
git --no-pager diff --no-ext-diff --no-textconv ...
git --no-pager show --no-ext-diff --no-textconv ...
git --no-pager log ...
git ls-files ...
gh pr view <number> [--json ...]
gh pr diff <number>
gh issue view <number> [--json ...]
```

Permitted network operations, and nothing else:

1. One `git fetch` of the base branch. It updates Git metadata, not source files. If it fails, continue with the local `origin/<base>` and disclose that the comparison may be stale; do not retry.
2. The read-only `gh` reads above, when the user supplies a PR number or the description cites `Fixes #N` and you need the issue to judge changeset accuracy or test coverage.
3. Documentation lookups when the checked-out source cannot settle the question: `biomejs.dev`, `docs.rs`, `doc.rust-lang.org`, `rust-lang.github.io/rust-clippy`, and MDN or a language spec when the change encodes a claim about JS, CSS, HTML, or GraphQL semantics.

Prefer the checked-out source over documentation and over memory. A rule's behavior, an API's contract, and a lint's registration are all determinable from the repository.

## Delegating This Review

An orchestrator may spawn agents that each run this skill; the Safety Boundary forbids only delegating outward from inside a review. The brief is the failure mode: a hint narrowing where the agent looks returns that suspicion with citations attached, and steering destroys the findings the brief did not anticipate.

**Brief the agent with the scope and nothing else.** A PR number, a branch, a commit range, or a diff. Add the feature's intended behavior only where the repository cannot supply it — a product requirement, an off-repo decision, an issue the agent cannot read. That is the complete brief.

**Never include** the area you suspect, the file you think is wrong, a defect class to prioritize, a severity or finding count you expect, or the earlier review whose gaps prompted this one. Both "check the allocations in `x.rs`" and "focus on the parser" substitute your judgment for the agent's, and guarantee the rest of the diff is reviewed at lower attention or not at all.

**Running this skill under a brief that steers:** keep the scope, discard the steer. Review the whole diff against this file's priorities as though the pointer had not arrived, and report what that produces — including no finding in the named area when it is clean. Note the steer under `## Review Status` so a confirmation is not read as an independent result.

## Establish the Review Scope

Prefer a scope the user supplies: files, a diff, a commit range, a base branch, or a PR number.

Biome has two long-lived branches. `main` takes bug fixes, new nursery rules, and internal-only changes; `next` takes features, nursery promotions, and breaking changes. Determine the **actual** base first, then judge separately whether it is the **correct** one.

For a PR number, read `gh pr view <n> --json title,body,baseRefName,files` and `gh pr diff <n>`. Do not check it out.

Otherwise review the current branch and working tree:

1. Read the branch and upstream with `git status --short --branch`.
2. Pick the base: the tracking branch if it is `origin/main` or `origin/next`; otherwise compute the merge base against both and take the branch whose merge base is a descendant of the other. If genuinely ambiguous, ask. Never fall back to `HEAD`: it reviews only uncommitted work and drops every commit on the branch without saying so.
3. Fetch that base once.
4. Diff merge-base to working tree — this covers committed, staged, and unstaged changes.
5. Use `git status` to identify untracked file paths, then read the contents of each with the file-reading tool. New rules, specs, and `.changeset/*.md` entries are frequently untracked.

Establish intended behavior from the description, commit messages, the linked issue, changed tests, and surrounding code. Where correctness depends on requirements not available locally, state the assumption or ask.

## Gather Context

Read enough surrounding code to review the change rather than the diff in isolation.

- Read every changed file in full before inspecting related files.
- Inspect callers, the rule registry, and the configuration surface the change touches.
- Inspect existing tests and conventions in the same `tests/specs/` directory.
- Search for an existing helper before accepting a new abstraction. The utility surface is large: `biome_rowan`, `biome_string_case`, the `*_ext.rs` traits, `biome_analyze` services.
- Read `AGENTS.md` and `CONTRIBUTING.md` at the root, plus the crate guide governing the changed area (`crates/biome_analyze/CONTRIBUTING.md`, `biome_parser`, `biome_formatter`, `biome_js_formatter`, `biome_diagnostics`, `biome_js_type_info`).
- **Load the matching skill under `.claude/skills/` for every area the diff touches**: `lint-rule-development`, `formatter-development`, `parser-development`, `diagnostics-development`, `type-inference`, `testing-codegen`, `doc-comments`, `changeset`, `pull-request`, `biome-developer`, `eslint-migrate-options`. Read the `SKILL.md` directly or use the skill tool; both are read-only. They are maintainer-authored and versioned with the code, so **this file deliberately does not restate them** — it adds only what a reviewer needs on top. If the two ever disagree, the skill wins and the disagreement is worth reporting.

Focus findings on the proposed change. Mention a pre-existing problem only when the change depends on it, worsens it, or makes it newly reachable.

## Repository Map

Locate the counterparts a change must keep in sync.

| Area | Source of truth | Also touched |
| --- | --- | --- |
| Grammar | `xtask/codegen/<lang>.ungram` | `biome_<lang>_syntax/src/generated/`, `biome_<lang>_factory/src/generated/` |
| Lint rule | `biome_<lang>_analyze/src/lint/<group>/<rule>.rs` | `../<group>.rs`, `../registry.rs`, `biome_configuration/src/analyzer/linter/rules.rs`, `biome_diagnostics_categories/src/categories.rs`, `packages/@biomejs/biome/configuration_schema.json` |
| Assist | `biome_<lang>_analyze/src/assist/source/<action>.rs` | `../source.rs`, `../registry.rs`, `biome_configuration/src/analyzer/assist/actions.rs`, `categories.rs` |
| Rule options | `biome_rule_options/src/<snake_case_rule>.rs` | rule's `type Options`, rustdoc `## Options`, `tests/.../options.json` |
| Generic CST helpers | `biome_<lang>_syntax/src/*_ext.rs` | the consumer that needed them |
| Formatter | `biome_<lang>_formatter/src/**` | `tests/specs/**`, `tests/prettier/**.snap` |
| Parser | `biome_<lang>_parser/src/**` | `tests/**/ok/`, `tests/**/error/` |
| Types / module graph | `biome_js_type_info/`, `biome_module_graph/` | `biome_db/`, `biome_service/` |
| Changeset | `.changeset/<name>.md` | — |

Rule groups: `a11y`, `complexity`, `correctness`, `nursery`, `performance`, `security`, `style`, `suspicious` — not every language crate has every group. New rules land in `nursery`, but most existing rules do not live there, so locate an established rule by group before assuming its path or severity constraints.

Files starting with `//! Generated file, do not edit by hand` are generated. Review the human-authored source, confirm the checked-in output corresponds to it, and never run the generator.

## Review Method

Two passes. **Design:** understand the goal, decide whether the change sits in the right crate and layer, trace how it meets the analyzer, parser, formatter, workspace, and configuration surfaces. **Implementation:** review every human-written changed line and the relevant tests for correctness, failure behavior, allocation cost, and maintainability.

If part of the change cannot be understood locally, ask or state the limitation — do not silently skip complex code. Judge whether the change improves the codebase, not whether it is perfect.

**Check the change against the property it claims.** When the title, description, changeset, or a comment asserts a property — zero-copy, no new allocations, no behavior change, non-breaking, faster, no new dependency — that assertion is reviewable, and the first counterexample is a finding rather than a question. Read the claim first, then look for what would falsify it. A stated property also reorders [Review Priorities](#review-priorities): the dimension the change exists to improve is reviewed first, whatever its rank in the list. A single line defeating the premise is `high` — merging it closes the ticket without doing the work, and the claim then misleads everyone who reads the code afterwards.

## Review Priorities

Review in this order so correctness is not displaced by style.

1. Design and crate-layer placement
2. Workspace access: CLI stateless, LSP cancellable
3. Functional correctness and false positives
4. Panics and partial operations
5. Text access, trivia, and allocation
6. API shape and type design
7. Duplication, simplicity, and consistency
8. Codegen and registration completeness
9. Subsystem conventions
10. Error handling and recursion
11. Tests and snapshots
12. Rust idioms, hygiene, and enforced lints
13. Documentation
14. Changeset, branch target, and PR hygiene

### Design and Crate-Layer Placement

- The change belongs in the crate that owns the behavior. Language-specific logic goes in that language's crate, not `biome_analyze`, `biome_formatter`, or `biome_rowan`.
- Use the subsystem that already owns the behavior: semantic model for bindings, module graph for cross-file facts, `biome_resolver` for paths, `biome_configuration` for user-facing options.
- No dependency that inverts the layering (syntax → factory → parser → analyze/formatter → service → cli).
- Unrelated refactors or reformatting mixed in make review and rollback harder.
- A new feature should not add generic machinery or public surface beyond its requirement. A single-use trait, an unrequested config knob, or a new abstraction layer needs justification in the diff.
- `biome_wasm` and anything reachable from it cannot depend on filesystem or process APIs. Trace new dependencies.

### Workspace Access: CLI Stateless, LSP Cancellable

The `Workspace` trait hides two execution models behind one interface — every method takes `&self` and returns `Result`, so no signature tells you which rules apply. The architecture is settled; the risk is a **new operation** adopting the wrong model. Check this whenever a change adds a `Workspace` method, touches `crates/biome_service/src/workspace/`, or adds a CLI or LSP path. Every violation named below is `high`.

Two salsa storage modes back the split (`workspace/db.rs:26-29`):

| Client | Constructed with | Storage | Consequence |
| --- | --- | --- | --- |
| CLI | `DbState::default()` (`workspace/server.rs:393`) | `Shared` | No setters, so no cancellation exists |
| LSP | `DbState::lsp()` (`biome_lsp/src/server.rs:791-805`) | `Owned` | Setters, pending-write cancellation, deadlock risk |

**CLI is read-only after the scan; writes go to the filesystem.** Once `scan_project` finishes every per-file operation is a read: workers call `process_file` and the runner writes final bytes to disk (`biome_cli/src/runner/process_file.rs:319`). The module graph in use is the one the scan produced. A worker must never call `change_file` — publishing module data while another worker holds a snapshot for type-aware analysis is the race this design removes. The one exception is `--use-server`, where synchronization is **deferred and sequential**: `crawl_inputs` returns, all workers join, then a plain `for` loop replays the changes one at a time (`runner/crawler.rs:70-86`, draining at `:288`). Without `--use-server` there is no synchronization; the workspace dies with the command.

A `change_file` or other state-publishing call inside a worker, inside `crawl_inputs`, or inside a parallel iterator; any attempt to parallelize the deferred loop or move it back into the crawl.

**LSP reads must be cancellable.** `OwnedDb::fork` unwinds with `salsa::Cancelled::PendingWrite` rather than blocking when a setter is pending (`db.rs:155-162`), because a thread holding a clone would wait forever on itself. Cancellation is normal traffic, not an error. `salsa::Cancelled::catch` wraps operations at the boundary (`biome_lsp/src/server.rs:46`, `session.rs:1477`) and `PendingWrite` maps to `content_modified()` (`utils.rs:419-422`), telling the editor to re-request. Non-LSP callers use `retry_on_pending_write` (`workspace.rs:1870`).

A handler bypassing the catch wrapper, turning routine cancellation into a panic that escapes to the client; any code that `unwrap()`s, swallows, or hard-errors a `Cancelled` unwind instead of letting the read retry.

**Never hold a database fork across a write.** `OwnedDb::with_setter` waits for all clones to drop, so a thread holding its own fork deadlocks against itself. A thread-local `LIVE_READS` counter (`db.rs:33`) catches this before it hangs, asserting in debug and logging a `Cancelled` in release (`db.rs:183-191`). The shape that satisfies it is extract, resolve, commit: hold the fork only long enough to pull out owned data, drop it, do the expensive AST work fork-free, then write. `update_module_graph_internal` is the reference implementation; production code contains no `drop(db)` calls.

A function that both reads through `DbState::fork` and performs a `DbState` write anywhere in the same call stack, even when tests pass — the guard only fires on an exercised path, so review is the earlier defence. `clone_untracked_db` and `into_untracked_db` deliberately escape the counter (`db.rs:61`, `db.rs:71`); a new call needs a stated reason that the handle serves a read-only leaf.

### Functional Correctness and False Positives

Trace real inputs through the affected code. For lint rules the dominant defect class is the false positive.

- **Globals and shadowing.** A rule banning an identifier (`console`, `Array`, a framework import) must consult the semantic model to confirm the binding is global and not locally redeclared. `crates/biome_analyze/CONTRIBUTING.md` calls this the most common rule bug.
- **Variant coverage.** Check every relevant node variant, not the first one found. Union nodes (`AnyJs*`, `AnyCss*`) and directive families (`VueDirective` plus each `VueV*ShorthandDirective`; `AnySvelteDirective` plus prefixed `HtmlAttribute`) commonly have siblings a change misses.
- **Value shapes.** Vue attribute values arrive as quoted `HtmlString`, Svelte values as `HtmlAttributeSingleTextExpression`. Handling one silently skips the other.
- **Ranges and offsets.** A range built from token-relative offsets must add the token's absolute start, and skip the opening quote when derived from `inner_string_text()`. An off-by-one points the diagnostic at the wrong span, which the snapshot then records as correct.
- **Suppression and options.** Verify `// biome-ignore` still works and that a new option's default preserves existing behavior.
- **Fix safety.** `FixKind::Safe` must not change semantics in any reachable case; one counterexample makes it `Unsafe`. Applying the action must produce code the rule no longer reports.

Do not report a theoretical edge case without showing how the changed code reaches it and what fails. This governs rule behavior; it does **not** govern [Panics and Partial Operations](#panics-and-partial-operations) below, where presence is the finding.

### Panics and Partial Operations

Biome runs as a long-lived daemon and language server. A panic ends the session, not the operation. This check is mechanical and runs on every diff.

**Scan before judging.** List every added or modified production line containing:

`.unwrap()` · `.expect(` · `[i]` index · `[a..b]` slice · `panic!` · `unreachable!` · `todo!` · `unimplemented!` · `assert!` · `.remove(` · `.swap(` · `.split_at(` · integer `/` or `%`

Production is everything outside `#[cfg(test)]` modules, `tests/`, `*.tests.rs` (some live inside `src/`), `biome_test_utils`, `biome_formatter_test`, and `tests_macros` — including code that only runs behind a debug flag. Test code may use all of these freely.

**Then classify each.**

| Construct | Accepted when | Otherwise |
| --- | --- | --- |
| `panic!` `unreachable!` `todo!` `unimplemented!` `assert!` | never in production | `medium`; `high` if user input reaches it |
| `.unwrap()` `.expect(..)` `[i]` `[a..b]` | release-mode control flow, a checked API, a type invariant, or an API contract proves the operation cannot fail | `medium` |
| Partial collection or string `.remove(..)` `.swap(..)` `.split_at(..)` | the receiver type is identified and release-mode evidence proves every index and boundary is valid; total map and set removals are not findings | `medium` |
| Integer `/` `%` | release-mode control flow or a type invariant proves the divisor is nonzero, or the code uses checked division or remainder | `medium` |

**Presence is the finding.** You need not show that input reaches the panic; the reachability requirement in [Finding Threshold](#finding-threshold) does not apply to this section. Establishing totality is the author's burden, and a reviewer who cannot reconstruct the invariant from the diff has found the defect. "This branch cannot happen" is not a defence — if it truly cannot, the invariant is cheap to write down.

**Comments and debug assertions are supporting evidence only.** A `// SAFETY:` comment can state the invariant and a `debug_assert!` can check it during development, but neither prevents the operation from failing in release mode. Accept a partial operation only when release-mode control flow, a type, or an API contract establishes the invariant.

**Take the model from the code, not from this file.** Search the touched crate for `// SAFETY:` and for `debug_assert!`, read the two or three occurrences nearest the change, and judge the diff against those. Any exemplar named here would be a snapshot of a codebase that keeps moving; the search returns the current one. Check three things in each: the invariant is named rather than gestured at, release-mode code or an API contract establishes it before the operation, and any comment or debug assertion accurately documents that proof. A justification the code does not establish is worse than none — it tells the next reader the case was considered.

**Indexing is the class most often missed.** A constant index (`[0]`, `[len - 1]`) is easy to see; a variable one is not. Flag every `slice[i]`, `self.bytes[self.position]`, `&self.source[start..end]`, and `map[&key]` whose index is not proven in range in the same function. Range slicing a `&str` also panics on a non-char boundary, not only out of bounds.

Name the replacement:

- Lexer byte and slice access → read the `Lexer` trait in `biome_parser` before reporting. It already exposes bounds-checked accessors that return `Option<u8>` for the current byte, a peeked byte, a byte at a forward or backward offset, and a char-boundary check. Name the one that fits from what the trait currently declares; a new raw index in a lexer is a finding with a named fix.
- Any slice or `Vec` → `.get(i)`, `.get(a..b)`, `.first()`, `.last()`, `.split_first()`.
- Absent value → `Option<T>` with `?`, `.ok()?`, or `let ... else`.
- Fallible operation → `Result<T, E>` propagated to the reporting boundary.
- Unexpected CST shape → `None` from `run()`, or a `BOGUS_*` node. For CST access `.ok()?` and `?` need no justification at all.
- Unresolvable type → `TypeData::Unknown`.
- A genuinely upheld invariant → encode it in the type so the branch stops existing.

**`expect` messages.** `.expect("")`, and a message that merely restates the call (`.expect("failed to get value")`), are `low` findings on their own. That message is the only diagnostic a user gets from a daemon crash, so it must name the violated invariant.

**Density is not licence.** Production `unwrap()` and `expect()` calls are common in the codebase and most carry no justification. They are legacy. A neighbouring `unwrap()` is not precedent, and a change may not extend a match whose arms already panic. Report new occurrences at the same bar regardless of what surrounds them.

`debug_assert!` is also the right suggestion where a contributor reached for `assert!` to document an invariant: it compiles out in release and is used widely.

Consolidate repeats — many occurrences of one pattern in one function is a single finding listing the lines.

### Text Access, Trivia, and Allocation

Biome runs on every keystroke in the LSP. This is the densest source of real defects, and they are correctness bugs as much as performance ones. Check every text access the diff introduces.

**`text_trimmed()` on a node is not "text without trivia".** `SyntaxNode::text_trimmed()` excludes only the first token's leading and last token's trailing trivia — its rustdoc says "All other trivia is included" (`biome_rowan/src/syntax/node.rs:97`). Every comment, newline, and space *between* child tokens survives. So comparing a node's trimmed text to a literal is wrong for any node spanning more than one token: `foo.bar`, `foo . bar`, and `foo /* c */ . bar` yield three different strings from the same AST shape. Report this whenever the diff matches node text against an expected value. Correct approaches, in order: compare the specific typed token's trimmed text; use the typed accessor the syntax crate provides (`static_value()`, `name()`, and friends); or match structurally against the AST instead of a string.

**On tokens, always use the trimmed variant.** The doctests are explicit: `token.text()` returns `"\n\t let \t\t"` (`token.rs:90`) while `text_trimmed()` returns `"let"` (`token.rs:144`); `token_text()` keeps trivia (`token.rs:111`), `token_text_trimmed()` does not. Any comparison, match, or hash of token text must use a trimmed accessor — otherwise a leading newline or attached comment becomes part of the value and the check passes or fails with surrounding formatting. This is a correctness finding, not a style nit.

**`syntax().to_*` allocates.** `to_trimmed_string()` carries its own rustdoc warning, "This function allocates a [String]" (`ast/mod.rs:215-220`); `to_trimmed_text()` (`ast/mod.rs:225`) is banned by name. `.to_string()` on a node or token, `String::from(...)`, and `format!(...)` on syntax text are the same defect spelled differently. Treat any `syntax().to_*` in the diff as a finding unless an owned `String` is genuinely required and cannot be a `TokenText` or `&str` — usually the code just wants to compare against a `&str`, which needs no allocation.

**Allocation discipline is not only a CST question.** The accessors above are this codebase's spelling of a general defect: producing an owned `String` where a borrow would do. Apply the same bar to every changed line that holds a `&str`, a `&[u8]`, or any buffer outliving the value being built from it. Flag `.to_string()`, `.to_owned()`, `String::from`, `format!`, `.collect::<String>()`, and `push_str` accumulation, and name the zero-copy replacement: the borrow itself (`&'a str`), a `Cow<'a, str>` where only an escape or normalization path must own, or a range into the source resolved on demand. Judge the type before the call site — a `String` or `Vec<String>` field forces every producer to allocate, so report it once at the definition as `design` rather than at each `to_string()` as `performance`. An owned value stands only where the diff shows the borrow cannot be held: a lifetime that genuinely cannot be threaded, a value that must outlive its buffer, or a transformation with no borrowed representation.

Also:

- `run()` decides whether to signal; `action()` builds the fix. Building strings or collections in `run()` that only `action()` consumes pays that cost on every node that does not signal.
- A `String` or `Box<str>` in a rule's `State` signals an avoidable allocation. Prefer `TokenText`, or `TokenText` plus a token-relative `TextRange` for words split out of one token. `TokenText::clone()` is a refcount bump.
- Extract quoted contents with `inner_string_text()` — never manual quote-slicing, never `text_trimmed()`.
- `format!()` inside `markup!` allocates; `markup!` interpolates directly.
- `clippy.toml` bans `str::to_lowercase`, `str::to_ascii_lowercase`, `OsStr::to_ascii_lowercase`, and `to_trimmed_text()`; the `biome_string_case` cow variants and trimmed token accessors replace them.
- **Module graph and type inference:** Module graph entries may not copy or clone data from another module, even behind `Arc`. Use `TypeReference` for cross-module type data.

### API Shape and Type Design

- **Free-function sprawl.** A cluster of new free functions threading the same values through every call should be a type: a struct owning that state with `&self` / `&mut self` methods, or an enum when the cases are closed. The parameters repeated across the signatures *are* the state the type should hold. Report the cluster once, naming the type that replaces it.
- **Indirection has a cost.** A helper that only renames a trivial expression, or forces a file jump without hiding complexity, is worse than inline code. Extraction earns its place when it names a domain operation, isolates an invariant, or makes non-trivial branching testable. Reuse alone does not justify a function; one call site alone does not condemn one.
- **Prefer the stronger construct.** Iterator chains over collect-then-loop; a struct with `&mut self` over threading `&mut Vec` through free functions; `impl Iterator` over an intermediate `Vec`. When functions must run in a fixed order on shared state, that ordering belongs in a type where it is enforced, not in a convention. In the formatter this is concrete — see the `Format` rule below.
- **Typed nodes in signatures.** A function inspecting the CST takes typed nodes (`HtmlAttribute`, `JsCallExpression`), never `HtmlSyntaxNode` or `SyntaxNode<L>`. An untyped parameter pushes the kind check onto every caller and lets the wrong node through. For several node types, declare a `declare_node_union!` and take that.
- **Derive discipline.** Every derive must be used. `Clone` on a type never cloned, `Debug` never formatted, `PartialEq`/`Hash` "for completeness" are findings — unused derives inflate compile time and generated code. `Copy` needs particular scrutiny: it silently changes move semantics and hides accidental copies at the call site. Do not accept it on anything past a couple of machine words without evidence the copies are cheap and intended.

**Which crate owns the helper** — judge by how broad the logic is, not who calls it today. Broad CST questions (does this expression have a static value, is this binding exported, what is the callee's name) belong in the syntax crate as an extension trait in `biome_<lang>_syntax/src/*_ext.rs`, alongside `expr_ext.rs` and `binding_ext.rs`. Consumer-specific business logic (how many blank lines precede this node, does it hold a dangling comment) belongs in the formatter or analyzer — `get_lines_before` sits at `biome_formatter/src/builders.rs:2832` for that reason. Both directions are findings: a generic accessor buried in one rule file guarantees the next rule reimplements it with a subtly different contract, and a formatter-specific query pushed into the syntax crate pollutes a crate every consumer depends on.

### Duplication, Simplicity, and Consistency

DRY and KISS pull against each other. Duplication is sometimes simpler than an abstraction coupling two things that merely look alike today.

**Consistency carries unusual weight here.** Biome holds hundreds of rules and dozens of formatter node implementations, each family written to the same shape, and contributors copy the nearest example. Make the comparison concretely: read two or three existing rules in the same group, or the neighbouring node implementations in the same directory, and check the change against them. Report a divergence in structure, naming, or error handling that has no stated reason — but not one the surrounding subsystem is itself inconsistent about.

**Report a DRY problem** when the copies express the same domain rule or invariant, must change together to stay correct, or reimplement something the repository already provides. That last case is the common one, and the surface is large: `biome_rowan`, `biome_string_case`, the `*_ext.rs` traits, `biome_analyze` services, the `TokenText`-plus-range pattern. Two rules separately reimplementing "is this identifier a global" with different shadowing handling is a genuine defect. **Do not report** code that is merely syntactically similar — two match arms with the same shape and different meanings should stay apart.

**Report a KISS problem** when the change adds indirection nothing needs, unrequested configurability, generic machinery for one case, or a layer obscuring a straightforward tree walk. Biome-specific: reaching for a semantic or type-inference service where a plain AST query answers the question (type-aware rules are markedly more expensive), and inventing an abstraction for a pattern `biome_analyze` already names.

**Premature optimization is distinct from allocation discipline** and the two are easy to confuse. Avoiding an allocation in `run()` or a formatter hot path is the standing requirement, not premature. Adding a cache, memo, fast path, or hand-rolled structure *is* an optimization needing evidence — the project benchmarks through CodSpeed (`.github/workflows/benchmark.yml`). Where one is added, check its invalidation and fallback: a stale cache in a language server is a correctness bug, not a slow one.

### Codegen and Registration Completeness

`AGENTS.md` section 4 states which generator to run for which change and which outputs CI Autofix produces. What matters for review:

- **Verify the artifact, not the command.** The author's terminal history is not in the diff. Use the [Repository Map](#repository-map) to confirm each edited source of truth has its generated counterpart, consistent with the edit. A `.ungram` change with no matching `generated/` change, or a new rule missing its `registry.rs` / group module / `rules.rs` / `categories.rs` entry, fails to compile or silently omits the rule.
- **Do not report what Autofix regenerates** — bindings, the JSON schema, full `gen-analyzer` output. Raising them is noise; mention one only when its absence blocks verifying another finding.
- **Look for the registration points nobody lists.** For a new enum member, feature flag, manifest field, or option, search the subsystem for parallel sites (serializer, deserializer, migration, preset lists, docs). No checklist covers this because the set differs per subsystem.

### Subsystem Conventions

Apply only the subsections the diff touches. Each names the skill that owns the convention and adds the reviewer's angle.

**Lint rules and assists** — load `lint-rule-development` (deeper: `crates/biome_analyze/CONTRIBUTING.md`). Beyond it: the two failure modes that reach `main` most often are a rule firing on a shadowed binding and a `FixKind::Safe` action that is not semantics-preserving, both scored `high` in [Functional Correctness](#functional-correctness-and-false-positives). Grade a naming or severity mismatch `low` unless the rule is leaving nursery, where the constraint binds and fixing it later is breaking. An option with no stated reason is a `design` finding — the bar is conflicting community preferences, not future flexibility.

**Parser** — load `parser-development` (deeper: `crates/biome_parser/CONTRIBUTING.md`). Beyond it: the highest-value check is a presence test that consumes input before returning `Absent`, because it corrupts the caller and surfaces far from the change. Next is a recovery set targeting a `BOGUS_*` kind the grammar does not permit there. A parser change with no `error/` fixture is a test gap however green the `ok/` cases are.

**Diagnostics** — load `diagnostics-development` (deeper: `crates/biome_diagnostics/CONTRIBUTING.md`). Beyond it: the recurring finding is a message naming the construct but not the problem — a restatement of the rule name, or "Invalid syntax". Grade `low`, or `medium` when it is the rule's only output and leaves the user with nothing to act on.

**Types and module graph** — load `type-inference` (deeper: `crates/biome_js_type_info/CONTRIBUTING.md`). Beyond it: treating `TypeData::Unknown` as a negative result rather than "could be anything" produces false positives and is `high`. A Salsa query reading an input it does not track serves stale results after an edit — `high`, and invisible to snapshot tests, so it must be caught by inspection.

For every changed resolution boundary, trace the changed path in review notes
before judging it:

```text
required result <- consuming query <- raw starting reference
                -> inference layer -> reachable data -> fallback
```

The trace names the caller's exact result, the narrowest reference containing
the required information, every parameter, argument, member, import, or module
the resolution may visit, and the condition that broadens or abandons the
lookup. For imported references, it also names the module that owns the
reference. Treat an unexplained visit, a broader-than-necessary starting point,
or a fallback that changes the caller's semantics as a finding. Require semantic
tests for the type structures the changed path actually traverses; require
query-event tests only when the change claims a narrower dependency or inference
scope.

**Formatter** — load `formatter-development` for IR primitives, `format_verbatim_node`, `space()` vs `token(" ")`, dangling comments, idempotency, and the internal-vs-Prettier test split. Two things it does not cover, both derived from the crates:

*Formatting logic belongs in a type implementing `Format`.* The unit of composition is a type implementing `Format<Context>` (`biome_formatter/src/lib.rs:1338`), and the trait's own rustdoc example is a struct. A change that instead grows free functions is a design finding, reported once against the cluster.

- A distinct formatting concern gets a named type — `ArrowChain` (`js/expressions/arrow_function_expression.rs:482`, impl at `:508`), `MemberChain` (`utils/member_chain/mod.rs:124`), and the `FormatCallArgument` family in `js/expressions/call_arguments.rs`. None is a function.
- Prefer a named type over a free function returning `impl Format<Context> + '_`: it can hold its inputs and chosen layout, it is greppable, and a test can construct it. Reserve `format_with(|f| ...)` for a local one-off inside an enclosing `fmt`.
- **`&mut Formatter` passes only between methods of the same type.** A helper taking `f: &mut JsFormatter` must be a method on the type owning the decision, never a free function that happens to accept a formatter. `AnyJsAssignmentLike` (`utils/assignment_like.rs`) is the model: it computes `layout()` (`:654`), and `Format::fmt` (`:1056`) drives its own `write_left`, `write_operator`, `write_right(f, layout)`. A free function taking `&mut Formatter` is a finding — it has no owner, so its preconditions (active buffer, what is already written, which layout was chosen) are invisible at the call site and unenforceable.
- Model a multi-way layout as an enum computed once, then dispatch to methods on the same type (`ArrowFunctionLayout`, `ConditionalLayout`, `AssignmentLikeLayout`). Layout re-derived at several write sites drifts and breaks idempotency.

*`fmt_fields` names every field.* It destructures the generated `*Fields` struct with `_` for deliberately ignored fields; `..` is ruled out by the crate's own best-practices docs (`biome_js_formatter/src/lib.rs:34-50`) so an excluded token stays explicit. A `..` is a finding, and a field bound but never written is a dropped token.

Grading: dropped comments are `high` — silent data loss in the user's file. An idempotency break is `high`. A missing internal spec test on a change that only moved a Prettier snapshot is `medium`.

### Error Handling and Recursion

**Recursion is effectively banned.** CST depth is user-driven, and the semantic model, module graph, and type references contain cycles — a recursive walk is a stack overflow on deep input, an infinite loop on a cyclic one, or both. Report any new recursive traversal of a CST, semantic model, module graph, imports, or type references, and always name the replacement:

- Plain traversal → `ancestors()`, `descendants()`, `preorder()`, `syntax().children()`.
- Custom ordering or early exit → an explicit worklist: a `Vec` as a stack plus a frame enum, driven by `while let Some(frame) = stack.pop()`.
- Cyclic structures → the same stack plus a `visited` set. Without the set it does not terminate; the set is the fix, not an optimization.
- Mutual recursion between helpers → the shared state belongs in a struct with one driving loop.

A recursive function guarded only by a depth counter is still a finding: the counter converts a crash into a silently truncated result, which for a linter is a missed or spurious diagnostic no test notices.

**A hand-rolled worklist is a contract, not just a loop.** The replacements above tell a contributor to write an explicit stack and a frame enum. Nothing tells them what that owes the reader. Any hand-written control-flow machine — a frame enum, a phase or state enum, a manual push and pop of state the caller owns — carries a correctness argument that appears nowhere in its syntax. Ask four questions and report each one the code leaves unanswered:

- **Why does each frame exist?** A frame that is not "process this item" — `Leave`, `Exit`, `Pop`, a sentinel, a marker — exists to make something happen at a particular moment relative to the others. That moment is the contract. Unstated, the next contributor reorders the pushes and nothing fails visibly.
- **Does every exit restore the invariant?** An early `return`, `?`, `break`, or `continue` inside the loop skips the frames that would have unwound it. Where the state is a `&mut` parameter the caller owns, the residue escapes and the defect surfaces in the caller. Report `correctness` where the leaked state changes a later result, `medium` where correctness rests on callers short-circuiting and nothing says so.
- **Does the language already do this?** Cleanup at scope exit is `Drop` or a guard type; ordering is an iterator; state that must not escape is an owned local, not a `&mut` parameter. Hand-rolling one of these is a finding unless the code states why the built-in does not fit.
- **Is the traversal order the intended one?** A LIFO stack yields children in reverse push order. Where source, import, or declaration order is part of the answer, that reversal is either deliberately compensated or it is a bug, and no reader can tell which without a sentence saying which.

**Vocabulary borrowed from a pattern the code does not implement** is a finding on its own. `Visitor`, `Policy`, `Work`, `Visit`/`Leave` invoke a contract readers already hold — double dispatch, a trait over a closed set of node kinds, a `serde`-style driver. Where the code is a plain loop over a stack with none of that structure, the name advertises an extension point that does not exist, and it disguises generic machinery serving one or two call sites. Report as `maintainability`, or `design` where the borrowed name arrives together with type parameters only one implementation ever instantiates.

- Prefer a helper returning `Option<T>` or `SyntaxResult<T>` over scattered early returns, and `map`/`and_then`/`filter` chains over deeply nested `if let`.
- Treat filesystem reads, `serde` deserialization, path resolution, and process boundaries as fallible. Decide which layer owns recovery before insisting on local handling — propagating with `?` to a boundary that can report it is frequently right, and a `Result` in the signature is not a defect.
- Report handling that causes real harm: `.ok()` discarding an error the caller needed, `unwrap_or_default()` masking a corrupt config or failed read, `let _ = ...` on a `Result`, a dropped `source` chain, or a write that can leave partial state.
- Do not recommend wrapping an error merely because it can fail. Match the crate's established error type, `Diagnostic` derive, and logging rather than adding an enum for one call site.

### Tests and Snapshots

Load `testing-codegen` for spec layout, the expectation-comment rules enforced by `assert_diagnostics_expectation_comment`, `.jsonc` script semantics, and snapshot pruning. Review test code statically; never run it. Beyond the mechanics:

- **Read snapshot diffs as content, not noise.** The highest-value check here. A snapshot recording a wrong span, truncated message, or diagnostic on the wrong line is a real defect and the test still passes, because the snapshot *is* the expectation. Blanket-accepted snapshots are a common source of merged bugs and no CI job catches one.
- **A bug fix needs a case that fails without the fix.** Check the new spec exercises the changed branch, not a neighbour that already passed.
- **A group mismatch fails silently.** A spec under a directory not matching the rule's declared group produces no diagnostics, so it reads as a passing valid-case test. Verify the path against the declaration rather than trusting a green run.
- **A `.snap` deleted by hand is a finding** — the skill gives the supported pruning command; a manual `rm` usually means a failure was being made to disappear.
- **Name the untested scenario and the defect the test would catch.** Never write "add more tests".

### Rust Idioms, Hygiene, and Enforced Lints

**Enforced by the workspace and CI:**

- `[dev-dependencies]` on internal crates use `path = "../biome_<name>"`, never `workspace = true`. Regular `[dependencies]` still use `workspace = true`. Stated in both `CONTRIBUTING.md` and `AGENTS.md`.
- Import types at the top; do not inline crate paths in type positions. Do not annotate types the compiler infers. Collapse nested `if let` into let chains. Follow the file's existing style.
- The workspace denies `clippy::allow_attributes` and warns on a long list including `dbg_macro`, `map_unwrap_or`, `implicit_clone`, `inefficient_to_string`, `needless_for_each`, `ref_option_ref`, `unnested_or_patterns` (`Cargo.toml`, `[workspace.lints]`). A new `#[allow(...)]` without `#[expect(...)]` semantics or a justification is a finding, as is a leftover `dbg!` outside tests.

**Idioms CI will not catch** — not in the lint config, so the reviewer's job. Judge by the Rust API Guidelines, not taste.

- **Generic bounds belong on the `impl`, not the data structure.** `struct Foo<T: Clone + Debug>` forces the bound on every consumer and makes adding a `derive` breaking; `struct Foo<T>` with `impl<T: Clone + Debug> Foo<T>` does not. The API Guidelines state this as [C-STRUCT-BOUNDS](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-bounds) and name `Clone`, `PartialEq`, `PartialOrd`, `Debug`, `Display`, `Default`, `Error`, `Serialize`, `Deserialize`, `DeserializeOwned` as traits never to bound on the definition. Genuine exceptions: a field referencing an associated type, `?Sized`, a `Drop` impl needing the bound. Report any bound the compiler did not demand.
- **`Option<&T>` beats `&Option<T>` in a parameter.** `&Option<T>` forces the caller to own an `Option`; `Option<&T>` composes with `.as_ref()`. `clippy::ref_option` describes exactly this but is `pedantic` and **not enabled here**, so CI stays silent. The doubly-referenced `&Option<&T>` *is* caught, by `ref_option_ref`.
- **An `Option<T>` parameter the body immediately `?`-es should be `T`.** If the first line is `let x = x?;` the `None` case is refused, not handled — the signature advertises an option the function does not honor and the real check migrates to every caller. Same for a `Result` unwrapped at the top.
- **A function taking `&T` and returning that same `&T`** gives the caller nothing; it already holds it. Return something genuinely derived, or delete the function. Most common in freshly extracted helpers whose body is a match returning the input on every arm.

Panics, partial operations, and `// SAFETY:` live under [Panics and Partial Operations](#panics-and-partial-operations); apply that section, not this one, for `unwrap()`, `expect()`, indexing, and the panic macros.

### Documentation

Two audiences, two rule sets, two very different levels of CI coverage.

Load the skill for whichever the diff touches: `doc-comments` for `//` / `///` / `//!` (**mandatory** whenever a comment changes), `lint-rule-development` for rustdoc inside `declare_lint_rule!`, `diagnostics-development` for messages and advice.

**Internal documentation — not CI-checked at all.** `doc-comments` defines the reader, the jobs of each comment kind, the deletion test, and the banned patterns; apply it as written. What it does not say, because it addresses implementers:

- CI checks nothing here. `cargo documentation` with `RUSTDOCFLAGS='-D warnings'` catches malformed rustdoc and broken links; `just test-doc` runs doctests. Neither judges whether a comment is worth reading. This dimension exists only if you review it.
- `missing_docs` is not enabled, so a new item with no `///` crosses CI untouched — and visibility is not the test, which `doc-comments` never mentions. Raise it wherever the contract is not recoverable from the signature, `pub` or not.
- **A new type that defines module vocabulary.** `doc-comments` mandates documentation on no item and its deletion test runs against adding comments, so the only ground here is that skill's own list of what legitimately needs one: a term of art the module defines. A new enum is usually exactly that — a closed set of cases the rest of the module then reasons in. Report only where all three hold: the module goes on to use the type as vocabulary, the distinction between variants or fields is not recoverable from their names, and the surrounding module documents at comparable density. Where a name could carry the distinction, the finding is the name, not a missing comment. A new trait, or a state or action type threaded between phases, is the strong case: an associated type names an extension point, and what each implementor must guarantee appears in no signature. Grade `medium` where the unstated thing is an invariant implementors have to uphold — a cycle-detection scheme, an ordering, a value the caller must not mutate — and two implementors answering it differently with nothing written down is the concrete correctness risk, not a style gap.
- **Documentation the diff deletes.** A refactor that replaces a documented type with an undocumented one is a net loss the scope line below cannot see: the removed comments were not added, not changed, and not made false. Read what the deleted prose established and find where the replacement states it. Grade `medium` when it explained an invariant the new code still depends on, `low` otherwise.
- Scope: comments the diff added or changed, existing ones the new behavior made false, new type definitions per the bullets above, and documentation the diff removed whose subject survives into the replacement. Do not audit the surrounding file.
- Grading: inaccurate documentation is a finding because it misleads the next reader. Merely missing explanation is `optional`, rising to `low` for a term of art a later contributor has to preserve, and to `medium` for an unstated invariant a caller or implementor must uphold to stay correct.

**Rule documentation — partly CI-checked.** Rustdoc inside `declare_lint_rule!` is published to biomejs.dev and exempt from the rules above. `.github/workflows/lint_rule_docs.yml` runs `cargo run -p rules_check` on any change under `**/biome_*_analyze/**/*.rs`, failing when a block does not parse, an `expect_diagnostic` block yields other than exactly one diagnostic, an `expect_diff` block yields no diff, or an unmarked block yields one (`xtask/rules_check/src/lib.rs:512-538`). So do not report a snippet that "probably does not compile". Review what the validator cannot judge:

- Whether the single-line first paragraph describes the rule, since it becomes the overview-table entry.
- Whether `### Invalid` precedes `### Valid`, and whether the invalid examples show the case the rule exists for rather than a trivial one.
- Whether every option has an h3, a stated default, an options block, and an applied example.
- Whether `ignore` is dodging validation on a snippet that should be validated — the one way to silence `rules_check`, and the most common docs finding worth raising.
- Whether the prose explains *why* the pattern is a problem, not only that the rule reports it.

**Feature documentation.** Non-rule user-facing features need a PR against the `next` branch of `biomejs/website`, linked from `## Docs`. An empty `## Docs` on a feature PR is a finding; `N/A` is fine when nothing is needed.

### Changeset, Branch Target, and PR Hygiene

Load `changeset` for format and `pull-request` for titles, template, and branch targeting; `AGENTS.md` section 2 defines which changes are user-facing. All three address an author writing a PR, so these checks — the reviewer's — are not in any of them:

- **Does a changeset exist, and is the file actually one?** Look for a new `.changeset/*.md` in scope, including untracked files, where a fresh one usually sits. `.changeset/config.json` is not a changeset.
- **Does it describe what the diff actually does?** The check only a reviewer can make. A changeset copied from the issue title, or written against an earlier iteration of the fix, satisfies every format rule and still misinforms everyone reading the CHANGELOG.
- **Does the linked issue match?** When the description cites `Fixes #N`, read the issue and confirm the change addresses what was reported and that a regression test covers it.
- **Does the changeset type contradict the base branch?** Each skill states the mapping on its own page, so nothing cross-checks the two: a `minor` or `major` changeset on a PR targeting `main` is a contradiction, and it is yours to catch.
- **Does the PR meet process requirements?** Conventional-commit title, intact template sections, and the mandatory AI-assistance disclosure. `AGENTS.md` adds the no-emoji rule and the instruction to reject verbose summaries on simple changes — a wall of generated prose on a two-line fix is a process finding.

Report a missing, mistyped, or inaccurate changeset. Do not create or edit one.

## Finding Threshold

Report only issues that are actionable and supported by the inspected code.

- Explain the input, call path, or maintenance condition that triggers the problem, and the resulting incorrect behavior or concrete cost.
- **Exception:** [Panics and Partial Operations](#panics-and-partial-operations): there, presence is the finding and no reachability argument is required.
- Point to the smallest relevant changed line or range, and give a minimal remediation direction without writing the patch.
- Put uncertain requirements or design choices under questions, not findings.
- Avoid formatter, naming, and stylistic comments unless they violate an explicit repository standard.
- Consolidate repeated symptoms with one root cause into a single finding listing the locations.
- List optional improvements after required findings. Do not use severity to make a preference look mandatory.
- Do not report anything CI Autofix regenerates on its own.

## Report Format

Return the complete review as raw, unrendered Markdown inside a single fenced code block. Put no text before or after the block, and do not escape Markdown inside it. The contents must be directly pasteable into a GitHub comment.

Findings first, ordered by severity; optional improvements last. Do not lead with a summary or praise. No emojis.

**Every finding line leads with exactly one `<severity>/<area>` token** — one lowercase severity from the lists below, a `/`, one category from them, nothing else — followed by the path, a short title, and the explanation. No other spelling is accepted: a bare severity (`high`), a bare category (`correctness`), bracketed (`[high][correctness]`), or prose ("Severity: high, Area: correctness") all fail the format and must be rewritten. Inline grading elsewhere in this file ("both are `high`", "Grade `low`") supplies only the severity; the category is chosen from the list and appended to form the token.

- `high`: panic reachable from user input, dropped comments or corrupted output, broad false positives, incorrect `Safe` fix, breaking change without a `major` changeset, or a regression.
- `medium`: credible edge-case failure, missing variant coverage, allocation in a hot path, missing registration, a material test gap, a banned panic macro on a path you cannot show is reachable, or an unjustified `unwrap()`, `expect()`, index, or slice in production.
- `low`: localized correctness, maintainability, documentation, or process issue with limited impact.
- `optional`: non-blocking improvement with a concrete benefit.

Categories: `design`, `correctness`, `performance`, `completeness`, `error-handling`, `tests`, `maintainability`, `documentation`, `changeset`, `process`. Choose the root cause, not a downstream symptom. `completeness` is missing codegen, registration, or option wiring; `design` is API shape, crate placement, derive discipline, and free-function sprawl. A trivia or text-accessor defect that changes what the code matches is `correctness`, not `performance` — reserve that for computing the right answer too expensively. Recursion reachable by a cycle or deep input, a workspace-access violation, and a panic reachable from user input are all `correctness`. An unjustified `unwrap()`, `expect()`, index, or slice is `error-handling` at `medium` regardless of whether you could demonstrate reachability; reserve `maintainability` for a partial operation that is justified but awkward.

````md
```
## Findings

- `high/correctness` `crates/biome_js_analyze/src/lint/correctness/no_example.rs:42` - Short title. Explain the triggering scenario, impact, and minimal remediation direction.
- `medium/performance` `crates/biome_js_analyze/src/lint/correctness/no_example.rs:87` - Short title. Explain the triggering scenario, impact, and minimal remediation direction.
- `low/documentation` `crates/biome_js_analyze/src/lint/correctness/no_example.rs:12` - Short title. Explain the concrete problem and why it should be addressed.
- `optional/maintainability` `crates/biome_rule_options/src/no_example.rs:20` - Explain the non-blocking improvement and its concrete benefit.

## Questions

- Include only unresolved assumptions that affect correctness. Omit this section when there are none.

## Review Status

Scope: `<base-sha>` through the current working tree, `<n>` files, plus listed untracked files. State the resolved base commit and file count, never just a branch name — a base that silently resolved to `HEAD` reviews only uncommitted work.
Branch target: `<base>` is correct for a `<patch|minor|major>` change | should target `<main|next>`.
Changeset: present and correct | present but `<problem>` | missing | not required.
Brief: independent | steered toward `<area>`; the full diff was reviewed regardless.
Validation: Static review only; no build tools were run.
Fetch: updated `origin/<base>` | fetch failed and local `origin/<base>` was used | not needed for the supplied scope.
```
````

**Format self-check before returning.** Read back every line under `## Findings` and confirm each starts with a `<severity>/<area>` token built from the severities and categories listed above, nothing else. A line whose first token is a severity alone, a category alone, or anything in brackets is malformed; fix it before returning the review. Escaping the format is a finding against the review itself, not a stylistic preference.

Severity reflects impact, not confidence. If there are no findings, write `No findings.` under `## Findings`, and still include the review status plus any residual uncertainty from unavailable context.
