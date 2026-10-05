---
name: doc-comments
description: Use this skill whenever writing or editing Rust `//`, `///`, or `//!` comments in Biome, including contributor documentation and rustdoc exposed to users through configuration schemas, CLI help, workspace or daemon APIs, and lint or assist declarations. For lint or assist rustdoc, also load lint-rule-development. Do not use for formatter handling of comments in user code.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

## Purpose

Rust syntax does not determine a comment's audience. Some comments explain the
implementation to Biome contributors. Others are published for Biome users or
client authors. Before editing a comment, trace where it is consumed by checking
nearby derives, attributes, macros, generators, generated artifacts, and help
snapshots.

## Identify the Audience

| Source context | Reader and destination |
| --- | --- |
| Ordinary implementation `//`, internal item rustdoc, and most module docs | Biome contributors reading the Rust source or rustdoc |
| Rustdoc on configuration types that derive `JsonSchema`, including their fields | Biome users reading configuration descriptions from the JSON Schema |
| Rustdoc consumed by `Bpaf`, including command variants, arguments, and configuration fields | Biome users reading CLI help |
| Daemon-facing `Workspace` methods and their serialized request and response types | Authors of daemon clients and users of generated backend bindings |
| Rustdoc inside `declare_lint_rule!` or the assist macro `declare_source_rule!` | Biome users reading rule or assist documentation on the website |

A file can mix audiences. For example,
[`workspace.rs`](../../../crates/biome_service/src/workspace.rs) contains
contributor-facing module docs and user-facing daemon contracts, while
[`FormatterConfiguration`](../../../crates/biome_configuration/src/formatter.rs)
has user-facing field docs and contributor-facing methods.

A comment can also feed multiple destinations. Write for the least specialized
reader, use only formatting supported by every destination, and inspect the
rendered output. Neither `///` nor `pub` proves who the reader is; follow the
text to its destination.

### Contributor reader

Write for a contributor who knows Rust but has no access to this conversation,
the pull request, the issue, or the diff. Describe the code at HEAD. Never
narrate change history or address a reviewer.

### End-user reader

Write for an intelligent adult who may be using Biome or the documented feature
for the first time. Do not require knowledge of Rust, Biome internals, advanced
knowledge of the target language's idioms or terminology, or unstated ecosystem
concepts.

Use an ELI18 register: beginner-friendly, technically complete, and never
childish. Prefer the most familiar accurate term. In JavaScript documentation,
for example, prefer `variable` to `binding` when both are correct. If the
distinction matters, define it on first use: "a binding (a name introduced by
code, such as a variable, parameter, or import)." Do not assume client authors
know Biome's Rust implementation.

Follow the [Biome philosophy](https://biomejs.dev/internals/philosophy/):

- reduce jargon, including target-language idioms, and define necessary terms;
- set clear expectations by stating relevant defaults, prerequisites, effects,
  limitations, fallback behavior, and error recovery;
- be specific, inclusive, and neutral; avoid vague phrases, idioms, and
  assumptions about the reader;
- keep CLI text understandable without color or other visual styling.

Use public names, such as `files.includes`, `--write`, or `openFile`, with the
spelling shown in the destination. Do not leak a Rust identifier merely because
it is convenient for the implementation.

## Comment Kinds and Value

| Kind | Job |
| --- | --- |
| `//!` module docs | Explain why a module exists, its core concepts, how its pieces relate, and durable design rationale. |
| `///` item docs | Describe the item's contract for its actual audience. Contributor contracts cover behavior, inputs and outputs, invariants, panics, and errors; end-user contracts describe public behavior. |
| `//` inline comments | Explain constraints, workarounds, non-obvious coupling, or why the obvious implementation is wrong. These normally target contributors. |

Ask what the intended reader can recover from the surface they see. Contributors
can inspect names, types, and control flow; improve the code or delete comments
that only restate them. End users may see only a help entry, editor hover,
generated API, or rule page, so retain the self-contained behavior summary even
when the Rust name appears descriptive. Keep item contracts in `///`; put
implementation rationale beside the code as `//`, not in user-facing docs.

## Writing End-User Documentation

Start with a short, plain-language sentence that says what the item does. Add
only relevant details: when to use it, prerequisites, accepted values and units,
default or omission behavior, interactions, side effects, persistence, results,
limitations, failures, and recovery. Use an example when prose leaves the result
ambiguous.

Write in the present tense and active voice, with one main idea per sentence.
Use familiar, concrete words; explain necessary terms in the same paragraph.
Avoid vague pronouns, unexplained acronyms, idioms, and dismissive words such as
"obviously", "simply", or "just". Proofread grammar and terminology.

Use the reader's interface in examples: configuration snippets, shell commands,
or the public binding or wire format, not Rust for a non-Rust surface. Introduce
what each example demonstrates and its expected result.

| Surface | Include |
| --- | --- |
| Configuration | The setting's effect, public key, default or omission behavior, accepted range or units, and relevant interactions. |
| CLI help | The action and scope, prerequisites, implications or conflicts, output, and exit behavior. Keep the first sentence useful on its own. |
| Shared configuration and CLI rustdoc | A first paragraph complete in both contexts, using only links or formatting verified in every renderer. |
| Workspace and daemon APIs | The public client contract: required prior state, state changes and persistence, interpretation of fields such as versions and positions, result meaning, and recoverable failures. Use public API concepts rather than Rust concepts such as borrowing, `Option`, or implementation structs. |
| Lint rules and assist actions | End-user website content. Also load [lint-rule-development](../lint-rule-development/SKILL.md) for required structure, examples, and option documentation; its content requirements take precedence. |

## Writing Contributor Documentation

Write documentation for a human reader, not as a translation of the
implementation.

- Start with a plain-language description of what the function returns or
  accomplishes.
- Use short or medium-length sentences. Keep one main idea per sentence.
- Avoid internal jargon. If a technical term is necessary, explain what it means
  in the same paragraph.
- Describe business-logic caveats that can surprise callers. Examples include
  fallback behavior, work limits, ambiguous results, overload ordering, and
  conditions that return `None`, `Unknown`, or an indeterminate result.
- Do not describe implementation details unless callers need them to understand
  the behavior.

Add an example when the signature cannot clearly show a relationship such as
overload selection, argument mapping, import traversal, fallback behavior, or
an otherwise ambiguous result. Introduce what the example demonstrates and its
expected result. Keep it minimal and self-contained.

Module docs should describe a durable concept or design reason, not list items
that will become stale. If there is no durable concept to explain, use a brief
one-line description.

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

**Reviewer-addressed justification.** Move the argument to the pull request:

```rust
// BAD: This correctly handles the overload case from the bug report.
// GOOD: Overloads are matched by arity before parameter types, so a
//       partial-arity call cannot select the wrong candidate.
```

**Restated contributor rustdoc.** A doc comment that only rewords the item name
adds nothing for a contributor:

```rust
// BAD:
/// Handles type inference.
fn infer_types(...)

// GOOD:
/// Infers the type of `expr` in `module`, returning `TypeData::Unknown`
/// when the expression references an unresolved import.
fn infer_types(...)
```

A self-contained summary can still be necessary in CLI help or an editor hover.

**Implementation language in end-user documentation.** Describe public behavior,
not its Rust representation:

```rust
// BAD:
/// Stores an `Option<IndentStyle>` consumed by the formatter.

// GOOD:
/// Uses tabs or spaces for indentation. Defaults to tabs.
```

Replace phrases such as "returns `Some`", "sets this enum variant", or "the
struct contains" with the result the user observes.

**Vague hedging.** Name the cases and reasons or remove the sentence. Avoid
"some cases", "various reasons", "handles edge cases", and "etc."

**Ad-hoc section banners** (`// ----- helpers -----`, `// ==== TYPES ====`).
Use the region comment pattern below instead.

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
uses it to group its methods. Editors fold on these markers; they exist for
navigation, not documentation.

- Pair every `// #region` with `// #endregion`.
- Name what the group contains. Use a plain label, or anchor the name to a
  function when the region holds one entry point and its private support code.
- Use regions only when folding helps in a long file, `impl`, or `trait` block.
- Never use a region name instead of rustdoc on its items.

## Editing Existing Code

- Edit the source comment, not a generated schema, binding, or help snapshot.
- Preserve accurate details. If behavior changes, correct the specific prose
  instead of replacing it with generic text.
- Match nearby density and terminology only for the same audience and
  destination.
- Keep the change focused; do not rewrite unrelated comments to impose a voice.

## Self-Check Before Finishing

Read only the comments in the diff, without the implementation:

1. Identify the reader and every destination.
2. Make each comment understandable without the conversation, change history,
   reviewer, issue, or diff. Keep issue links only as supplemental context for
   constraints or tracked workarounds.
3. Delete contributor comments recoverable from the code.
4. Ensure end-user comments stand alone without Rust, Biome-internal, or
   advanced target-language knowledge and state the details needed to predict
   behavior.
5. Check public names, examples, terminology, inclusivity, and grammar.
6. Inspect generated or rendered output when applicable.

Delete redundant contributor comments, but retain descriptions required by a
user-facing surface.

## References

- [Biome philosophy](https://biomejs.dev/internals/philosophy/) — principles for user communication.
- [Diátaxis](https://diataxis.fr/) — explanation, reference, and rationale.
- [lint-rule-development](../lint-rule-development/SKILL.md) — lint and assist documentation requirements.
