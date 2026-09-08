# Rust Safety and Syntax Review

Use relevant sections for affected Rust code only; exclude unrelated audits and speculative redesign.

## Partial Operations

Check added or modified production operations, plus existing ones whose safety preconditions or reachability the diff affects:

```text
.unwrap()  .expect(...)  indexing  slicing  panic!  unreachable!
todo!  unimplemented!  assert!  remove  swap  split_at  integer / or %
```

Apply [production totality](../SKILL.md#production-totality), including caller guarantees. No concrete failing input is required; test-only code is exempt.

| Construct | Review rule |
| --- | --- |
| Panic macros and `assert!` | Establish that the panic path is unreachable or the assertion always holds |
| `unwrap`, `expect`, index, slice | Establish success, presence, or valid bounds through guards, types, or caller contracts |
| Partial collection/string methods | Identify the receiver and establish that every index and UTF-8 boundary is valid |
| Integer division/remainder | Establish a nonzero divisor and absence of arithmetic overflow |

Comments, debug assertions, and passing tests are supporting evidence, not release-mode guards. Consolidate shared missing guarantees or failures; exclude unrelated legacy code.

Suggest establishing the invariant or using a total operation while preserving error/recovery behavior, not blanket fallible-API conversions.

## Syntax Text and Ranges

Load `syntax-text-handling` for implementation contracts.

- `SyntaxNode::text_trimmed()` retains trivia between child tokens. Do not compare a multi-token node's text to a semantic literal.
- Token comparisons and hashes use trimmed token accessors so attached trivia cannot change behavior.
- Check language-specific quote semantics and string boundaries when slicing; helper choice alone is not a finding.
- A token-relative range used as a diagnostic must be translated to the file range exactly once, including any quote offset.

Treat trivia and range mistakes as correctness issues, not merely performance issues.

## Allocation and Analyzer Phases

Cite the affected path and violated requirement or avoidable cost. A possible borrowed representation alone does not justify a rewrite.

- `run()` decides whether to signal; `action()` constructs a fix. Data used only by the action should not be built in `run()`.
- Prefer borrows or `Cow` when the source buffer remains available.
- Report an owning field type once at its definition rather than every allocation it forces.
- Avoid caches, memoization, and hand-rolled fast paths without benchmark evidence and a correct invalidation strategy.

## API Shape

For affected APIs, distinguish contracts from heuristics. Report violations or concrete impact, not preferred signatures or abstractions; keep remediation local.

- Language-specific behavior belongs in its language crate.
- Generic CST questions belong in syntax extension traits; consumer-specific policy belongs in the analyzer or formatter using it.
- Functions inspecting CST should accept typed nodes or a declared node union rather than raw syntax nodes.
- A helper should isolate a domain operation or invariant, not merely rename an expression.
- Repeated parameters across a cluster of free functions often indicate state that belongs in a struct.
- Derive only traits used by the implementation or callers.
- Put generic bounds on implementations unless a field or language rule requires them on the type.
- Prefer `Option<&T>` to `&Option<T>`. A function that immediately applies `?` to an optional parameter probably should require `T` instead.

## Recursion and Worklists

Check affected user-controlled traversals for reachable overflow or cycles; recursion alone is not a finding.

For a hand-written worklist, verify:

- frame processing follows the required traversal order;
- every early exit restores externally owned state;
- LIFO reversal preserves intended source or declaration order.

## Error Handling

- Do not discard errors needed by the reporting boundary with `.ok()`, `let _ =`, or an unrelated default.
- Check required reporting through changed wrappers and fallbacks; abstraction preference alone is not a finding.
