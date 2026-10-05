---
name: formatter-development
description: Use this skill whenever implementing or debugging Biome formatter behavior, IR composition, node rules, layout selection, source-comment handling, verbatim formatting, idempotency, internal specs, or Prettier comparison. Do not use it for generic snapshot commands or parser changes.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Formatter Development

Use `crates/biome_formatter/CONTRIBUTING.md` and the language formatter's guide as the canonical architecture references. Inspect neighboring node implementations before selecting IR primitives.

## Workflow

1. Reproduce the behavior with a focused internal formatter spec or `quick_test`.
2. Inspect the node fields, comments, and nearby formatting rules.
3. Implement the smallest layout change using formatter IR.
4. Run focused formatter tests and inspect snapshots.
5. Compare with Prettier when compatibility is relevant.
6. Format and lint before committing.

## Printing Discipline

Formatter output is a structured rewrite of the source tree, not a fresh pretty-printer. Treat every missed node, missed token, unchecked suppression, and untracked replacement as a formatter bug, not as style feedback.

- Every source token in the formatted range MUST be consumed exactly once. A token is consumed only by formatting it, removing it with `format_removed`, replacing it with `format_replaced`, or by a language-specific helper that does one of those operations.
- Every source node in the formatted range MUST be handled every time. Prefer `node.format()` or `node.format().with_options(...)` so the node's own rule formats the node, checks suppressions, and routes comments through the formatter infrastructure.
- A parent formatter MUST NOT inline a child node's fields just to get a convenient layout. Move the layout decision into the child rule or pass options into the child formatter.
- If an architecture-specific formatter bypasses a node's rule, it MUST NOT skip the node silently. It MUST check `f.context().comments().is_suppressed(node.syntax())`; if the node is suppressed, it MUST write the language's `format_suppressed_node(...)` helper instead of formatting the node body. Without this check, debug builds fail suppression coverage and user suppressions can be ignored.
- Source tokens MUST be printed through their typed accessors. Use `token("...")` only for syntax inserted by the formatter when no source token exists.
- Removing a source token from output MUST use `format_removed(&token)`. Do not drop the field, bind it to `_`, or omit it from `write!` without consuming it through `format_removed`; skipped trivia still belongs to that token.
- Replacing a source token's text MUST use `format_replaced(&token, &replacement)`. Do not print the replacement directly and do not use `token("...")` for replacement text, because the original token still has trivia and must be marked consumed.
- Custom formatting MUST be carried by a small struct implementing `Format<Context>`. Do not use free functions or stored closure values to carry formatter state or layout invariants. Use `format_with` only for one-off local glue that is immediately written.

## Node Rules

Generated node rules implement `FormatNodeRule`. In `fmt_fields`:

- destructure the generated `*Fields` type explicitly;
- format source tokens through their typed accessors;
- use `_` rather than `..` only when the field is consumed elsewhere in the same formatting path or deliberately handled by `format_removed` / `format_replaced`; otherwise `_` on a node or token field is a dropped-tree bug;
- preserve every token and comment unless the formatter contract intentionally removes or replaces it;
- keep layout decisions near the type that owns them.

`format_verbatim_*` methods preserve a node's source text. Replace verbatim formatting with structured formatting only when tests cover valid, malformed, and commented forms of the node.

## Token Rules

Format, replace, or remove every token. Formatter tests panic when a token is not handled, preventing accidental source loss.

Use `format_replaced` when substituting a token and `format_removed` when removing one.

## Ad-Hoc Formatting

Format a node through `node.format()` when possible. Its regular rule checks formatter-suppression comments as part of normal formatting.

When a helper formats a node or its tokens outside `FormatNodeRule`, run the formatter tests. If the suppression-check assertion reports a node, call `f.context().comments().mark_suppression_checked(node.syntax())` for that reported node. The assertion shows that the helper bypasses the node's normal suppression check.

## IR Composition

Use semantic IR rather than writing whitespace as arbitrary text:

- `space()` for required spaces;
- soft line breaks for optional wrapping;
- hard line breaks for mandatory breaks;
- groups to choose flat versus expanded layout;
- indentation primitives matching the enclosing construct;
- conditional content tied to the group whose fit decision controls it.

For a distinct formatting concern, use a named type implementing `Format`. A cluster of free functions that pass `&mut Formatter` obscures what has already been written and which layout invariants apply.

Represent multi-way layout with an enum selected once. Recomputing layout at several write sites can produce inconsistent output and idempotency failures.

## Comments

Leading and trailing comments are generally handled by formatter infrastructure. Explicitly format dangling comments when the node owns a position to which no child can attach them.

Test comments at each structural boundary affected by the change: before the first child, between children, after the last child, and around empty nodes. Dropping or moving a source comment is data loss.

## Tests and Idempotency

Load `testing-codegen` for snapshot commands and review.

Internal specs should contain the focused source shapes needed to establish canonical output. Where useful, include both already formatted and deliberately unformatted inputs that should converge to the same result.

The formatter test infrastructure reformats error-free, non-range output during the test invocation and fails when the second result differs. IR is diagnostic evidence when output differs; it is not a separate equality contract.

Add internal specs for behavior changes even when a Prettier snapshot changes or disappears. Agreement with one external corpus input does not cover the changed edge case.

## Prettier Comparison

Use the repository tool when compatibility is part of the requirement:

```shell
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -l js 'const value={a:1}'
bun packages/prettier-compare/bin/prettier-compare.js --rebuild -f path/to/file.js
```

`--rebuild` rebuilds Biome's WASM bundle and writes build outputs. It is appropriate during implementation, not during a read-only review.

Treat differences as input to design, not automatic bugs. Biome may intentionally differ when its documented behavior or architecture requires it.

## Generation and Verification

After changing source in a language formatter crate:

1. Run the narrowest formatter crate or spec test and review snapshots before accepting them.
2. Compare against Prettier when compatibility is relevant; retain reviewed intentional divergences.
3. Run `just f` and `just l` before committing.

## Review Checklist

- Every generated field is handled explicitly.
- Every child node is formatted through its own rule, or an architecture-specific bypass checks `f.context().comments().is_suppressed(node.syntax())` before formatting the node body manually.
- Every source token is formatted, removed with `format_removed`, or replaced with `format_replaced`; no source token is silently skipped, bound to `_`, or recreated as static text.
- Comments survive in the intended position.
- Custom formatting is represented by a struct implementing `Format<Context>` rather than free functions or closure values.
- Layout is selected once and composed with semantic IR.
- Error and bogus syntax remains representable without formatter panics.
- Internal specs cover the changed behavior.
- Reformatting converges in one test invocation.

## References

- Formatter guide: `crates/biome_formatter/CONTRIBUTING.md`
- JavaScript formatter guide: `crates/biome_js_formatter/CONTRIBUTING.md`
- Formatter test infrastructure: `crates/biome_formatter_test/src/spec.rs`
- Prettier comparison: `packages/prettier-compare/README.md`
