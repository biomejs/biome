---
name: formatter-development
description: Use this skill when implementing or debugging Biome formatter behavior, IR composition, node rules, layout selection, source-comment handling, verbatim formatting, idempotency, internal specs, or Prettier comparison. Do not use for generic snapshot commands or parser changes.
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
6. Run formatter codegen for the language, then format and lint.

## Node Rules

Generated node rules implement `FormatNodeRule`. In `fmt_fields`:

- destructure the generated `*Fields` type explicitly;
- format source tokens through their typed accessors;
- use `_` for an intentionally ignored field rather than `..`;
- preserve every token and comment unless the formatter contract intentionally replaces it;
- keep layout decisions near the type that owns them.

`format_verbatim_node` preserves a node's source text. Replace verbatim formatting with structured formatting only when tests cover valid, malformed, and commented forms of the node.

## IR Composition

Use semantic IR rather than writing whitespace as arbitrary text:

- `space()` for required spaces;
- soft line breaks for optional wrapping;
- hard line breaks for mandatory breaks;
- groups to choose flat versus expanded layout;
- indentation primitives matching the enclosing construct;
- conditional content tied to the group whose fit decision controls it.

For a distinct formatting concern, prefer a named type implementing `Format`. A cluster of free functions that pass `&mut Formatter` obscures what has already been written and which layout invariants apply.

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

After changing a language formatter:

```shell
just gen-formatter <lang>
just f
just l
```

Run the narrowest formatter crate or spec test first. Review snapshots before accepting them.

## Review Checklist

- Every generated field is handled explicitly.
- Tokens use typed formatting rather than recreated static text where source tokens exist.
- Comments survive in the intended position.
- Layout is selected once and composed with semantic IR.
- Error and bogus syntax remains representable without formatter panics.
- Internal specs cover the changed behavior.
- Reformatting converges in one test invocation.
- Required formatter generation is checked in.

## References

- Formatter guide: `crates/biome_formatter/CONTRIBUTING.md`
- JavaScript formatter guide: `crates/biome_js_formatter/CONTRIBUTING.md`
- Formatter test infrastructure: `crates/biome_formatter_test/src/spec.rs`
- Prettier comparison: `packages/prettier-compare/README.md`
