# Rust Safety and Syntax Review

Load the relevant sections when production Rust changes introduce partial operations, recursion, text extraction, ranges, allocations, or new APIs.

## Partial Operations

Scan added and modified production lines for:

```text
.unwrap()  .expect(...)  indexing  slicing  panic!  unreachable!
todo!  unimplemented!  assert!  remove  swap  split_at  integer / or %
```

Test code may use these freely. In production, accept a partial operation only when release-mode control flow, a type invariant, or a documented API contract proves it cannot fail.

| Construct | Review rule |
| --- | --- |
| Panic macros and `assert!` | Never acceptable on a production path; `debug_assert!` may document an invariant but does not establish it |
| `unwrap`, `expect`, index, slice | Require a proof in types, control flow, or API contract |
| Partial collection/string methods | Identify the receiver and prove every index and UTF-8 boundary |
| Integer division/remainder | Prove the divisor is nonzero or use checked arithmetic |

Comments and debug assertions are supporting evidence only. Nearby legacy partial operations are not precedent. Consolidate repeated instances in one function into one finding.

Name the total replacement where possible: `get`, `first`, `last`, `split_first`, `Option`/`Result` propagation, a checked lexer accessor, or a bogus CST node.

## Syntax Text and Ranges

Load `syntax-text-handling` for implementation contracts.

- `SyntaxNode::text_trimmed()` retains trivia between child tokens. Do not compare a multi-token node's text to a semantic literal.
- Token comparisons and hashes use trimmed token accessors so attached trivia cannot change behavior.
- `syntax().to_*`, `.to_string()`, `String::from`, and `format!` allocate. Require actual ownership or transformation before accepting them in a hot path.
- Quoted contents use the language syntax crate's `inner_string_text()` helper rather than manual slicing.
- A token-relative range used as a diagnostic must be translated to the file range exactly once, including any quote offset.
- A `String` or `Box<str>` in analyzer state often forces allocation on every candidate; prefer `TokenText` or token plus relative range where the source token outlives the state.

Treat trivia and range mistakes as correctness issues, not merely performance issues.

## Allocation and Analyzer Phases

- `run()` decides whether to signal; `action()` constructs a fix. Data used only by the action should not be built in `run()`.
- Prefer borrows or `Cow` when the source buffer remains available.
- Report an owning field type once at its definition rather than every allocation it forces.
- Avoid caches, memoization, and hand-rolled fast paths without benchmark evidence and a correct invalidation strategy.

## API Shape

- Language-specific behavior belongs in its language crate.
- Generic CST questions belong in syntax extension traits; consumer-specific policy belongs in the analyzer or formatter using it.
- Functions inspecting CST should accept typed nodes or a declared node union rather than raw syntax nodes.
- A helper should isolate a domain operation or invariant, not merely rename an expression.
- Repeated parameters across a cluster of free functions often indicate state that belongs in a struct.
- Derive only traits used by the implementation or callers.
- Put generic bounds on implementations unless a field or language rule requires them on the type.
- Prefer `Option<&T>` to `&Option<T>`. A function that immediately applies `?` to an optional parameter probably should require `T` instead.

## Recursion and Worklists

Recursive traversal of user-controlled CST depth, imports, module graphs, semantic references, or types can overflow or loop on cycles. Prefer repository traversal iterators or an explicit worklist with a visited set for cyclic structures.

For a hand-written worklist, verify:

- each frame's purpose and ordering contract are clear;
- every early exit restores externally owned state;
- LIFO reversal preserves intended source or declaration order;
- cleanup uses scope or guard types when possible;
- names such as `Visitor` or `Policy` describe an actual abstraction rather than a one-off loop.

## Error Handling and Rust Hygiene

- Do not discard errors needed by the reporting boundary with `.ok()`, `let _ =`, or an unrelated default.
- Match the crate's established error and diagnostic type instead of adding a one-use wrapper.
- Internal `biome_*` dev-dependencies use local paths as required by `AGENTS.md`.
- Reject leftover `dbg!` outside tests and unjustified lint suppression.
- Follow surrounding Rust style only when it is consistent and enforced; do not turn preferences into findings.
