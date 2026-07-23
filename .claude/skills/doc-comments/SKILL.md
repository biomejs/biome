---
name: doc-comments
description: How to write inline comments, rustdoc, and module documentation in the Biome codebase. The audience is Biome developers reading the source, not end users. Use whenever writing or editing `//` comments, `///` item docs, or `//!` module docs — including comments added incidentally while fixing bugs or implementing features.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Comments and doc comments in this repository are read by contributors, months
or years after they were written, with none of the context you have right now.
This skill defines who that reader is, what each kind of comment is for, and
which patterns are banned.

**Scope boundary:** rustdoc inside `declare_lint_rule!` / `declare_assist_rule!`
blocks is end-user documentation — it is generated into the website. This skill
does not apply there; see [lint-rule-development](../lint-rule-development/SKILL.md).

## The Reader

Write for a Biome contributor who is competent in Rust but has **no access to
your current context**: not this conversation, not the pull request, not the
issue, not the diff. They see only the repository at HEAD.

Two consequences follow directly:

1. **Never narrate change history.** Words like "now", "previously",
   "no longer", "the new approach" are meaningless at HEAD, where only one
   approach exists. State how the code works, not how it came to be.
2. **Never address the reviewer.** A comment that argues your change is
   correct ("this properly handles X") belongs in the PR description, not in
   the source. The comment must justify the code as it stands, permanently.

## Three Kinds of Documentation, Three Different Jobs

| Kind | Job | Contains |
| ---- | --- | -------- |
| `//!` module docs | Explanation | Why the module exists, core concepts and terminology, how the pieces relate, design rationale |
| `///` item docs | Reference | The contract: behavior, inputs and outputs, invariants, panics, errors. Neutral and factual |
| `//` inline comments | Rationale | Only what the code cannot say: constraints, workarounds (with issue links), non-obvious coupling, why the obvious alternative is wrong |

Do not mix the jobs. Implementation details do not belong in `///` docs — put
them as `//` comments inside the body. The contract does not belong scattered
across inline comments — put it on the item.

## The Deletion Test

Before writing any comment, ask: **does this state something the reader cannot
recover from the code itself?**

- If the information is already carried by names, types, or structure, do not
  write the comment. If the name fails to carry it, improve the name.
- Information that legitimately needs a comment: an invariant, a rationale, a
  coupling to code elsewhere, a workaround with a link, surprising behavior of
  a dependency, a term of art the module defines.

When editing later, the same test applies in reverse: a comment that no longer
passes it should be deleted, not left to rot.

## Banned Patterns

**Narrating the next line.** Delete these on sight:

```rust
// Increment the generation counter
generation += 1;
```

**Change-history narration.** Rewrite as present-tense rationale:

```rust
// BAD: We now intern types instead of cloning them.
// GOOD: Interning avoids cloning these types on every lookup.
```

**Reviewer-addressed justification.** Move the argument to the PR:

```rust
// BAD: This correctly handles the overload case from the bug report.
// GOOD: Overloads are matched by arity before parameter types, so a
//       partial-arity call cannot select the wrong candidate.
```

**Restated rustdoc.** A `///` doc that rewords the item name says nothing:

```rust
// BAD:
/// Handles the type inference.
fn infer_types(...)

// GOOD:
/// Infers the type of `expr` in the scope of `module`, returning
/// `TypeData::Unknown` when the expression references an unresolved import.
fn infer_types(...)
```

**Vague hedging.** "Some cases", "various reasons", "handles edge cases",
"etc." — either name them or drop the sentence.

**Ad-hoc section banners** (`// ----- helpers -----`, `// ==== TYPES ====`).
For grouping in long files, use the region comment pattern below instead.

## Region Comments

Long files group related items with paired region markers:

```rust
// #region FILE-LEVEL METHODS
...
// #endregion
```

This is an established convention across the codebase (`biome_service`,
`biome_module_graph`, `biome_rowan`, the parsers). The `Workspace` trait in
[`crates/biome_service/src/workspace.rs`](../../../crates/biome_service/src/workspace.rs)
uses it to group its methods (`PROJECT-LEVEL METHODS`, `FILE-LEVEL METHODS`,
`SEARCH-RELATED METHODS`). Editors fold on these markers, which is the point:
they exist for navigation, not documentation.

Rules:

- Every `// #region` has a matching `// #endregion`. An unpaired marker breaks
  editor folding silently.
- The name states what the group contains. It can be a plain label
  (`Shared helpers`) or anchored to a function (`#region parse_thematic_break_parts`)
  when the region holds one entry point and its private support code.
- Use regions only where they earn their keep: files or `impl`/`trait` blocks
  long enough that folding helps. A file that fits on two screens does not
  need them.
- A region name is organization, not documentation. It never substitutes for
  rustdoc on the items inside it.

## Editing Existing Code

- Preserve existing doc comments. If your change alters behavior, extend or
  correct the specific prose — never replace it with generic text. Deleting
  hard-won context is worse than leaving a comment slightly stale.
- Match the surrounding density. A heavily documented module deserves the same
  level on new items; do not blanket a sparse module with comments.

## Exemplar

The `//!` module docs at the top of
[`crates/biome_service/src/workspace.rs`](../../../crates/biome_service/src/workspace.rs)
show the target register. They define a term the rest of the module depends on
("open documents") and give its meaning in both the LSP and CLI contexts; they
explain a design decision the signatures alone would make confusing (the
workspace is stateful, yet every method takes `&self`, because the trait must
be thread-safe and caching happens internally); and they state the error
philosophy once, at the top, instead of repeating it on every method.
Everything is present tense; nothing mentions how the design evolved or
defends a change.

## Self-Check Before Finishing

After completing any task that touched comments, re-read **only the comments
in your diff**, in isolation from the code changes:

1. Does each one pass the deletion test?
2. Does any reference the conversation, the change itself, or the reviewer?
3. Would a reader without access to the diff understand each one?

Fix or delete what fails. Deletion is the default; a missing comment is
cheaper than a misleading one.

## References

- [Diátaxis](https://diataxis.fr/) — the framework behind the
  explanation / reference / rationale split above.
- [lint-rule-development](../lint-rule-development/SKILL.md) — for rule
  rustdoc, which is end-user documentation.
