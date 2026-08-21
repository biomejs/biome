---
name: parser-development
description: Implement or modify Biome grammars, lexers, token sources, parse rules, separated lists, error recovery, and parser fixtures for existing or new languages. Use for parser behavior and `.ungram` changes; not merely for consuming an existing AST/CST.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Parser Development

Use `crates/biome_parser/CONTRIBUTING.md` as the canonical parser guide. Read the section matching the current grammar, lexer, parse-rule, or recovery task.

## Workflow

1. Inspect the language grammar and neighboring parse rules.
2. Add or adjust focused `ok/` and `error/` fixtures before changing recovery behavior.
3. Update the grammar when the typed tree shape changes.
4. Regenerate syntax and factory artifacts after `.ungram` changes.
5. Implement parser logic with explicit presence tests and bounded recovery.
6. Run the language parser's focused tests and inspect snapshots.

## Grammar

Grammar files under `xtask/codegen/` define typed syntax nodes and fields. Follow existing language naming:

- prefix nodes with the language name;
- name unions with `Any`;
- use `Bogus` nodes for recoverable invalid syntax;
- end list node names with `List`;
- represent lists as present, possibly empty nodes rather than optional fields.

After a grammar change, run:

```shell
just gen-grammar <lang>
```

This updates generated syntax nodes, syntax kinds, factories, macros, and language-specific mappings. Parser rules remain hand-written.

## Presence Contract

A parse function returns `Absent` only when it has consumed no tokens. Test the first distinguishing token before calling `start`, `bump`, `eat`, `expect`, or another parser that can advance.

Use:

- `expect` for required tokens that should create a diagnostic when absent;
- `eat` for optional tokens;
- `.ok()` for optional nodes;
- `.or_add_diagnostic(...)` for required nodes;
- typed recovery for malformed nodes that should remain in the CST.

Do not use backtracking where a bounded lookahead or a more precise presence test can distinguish the syntax.

## Error Recovery

Recovery must preserve following valid syntax and produce a bogus node permitted by the grammar at that position.

A recovery set normally includes the nearest relevant:

- list separator;
- list or block terminator;
- statement boundary;
- token that starts the next valid construct.

Do not copy a recovery set from an unrelated grammar position. Verify which tokens the caller expects after the failed parse.

## Lists

Use the parser infrastructure's list traits instead of open-coded loops when their contract matches the grammar.

For separated lists, verify:

- the parser recognizes the enclosing terminator;
- separator handling agrees with trailing-separator grammar;
- a malformed element makes progress or stops;
- recovery cannot consume the enclosing terminator;
- an empty list still produces the required list node.

## Lexer and Token Source

Follow the current `Lexer` and buffered token-source traits from `biome_parser`; do not copy an old trait implementation from a skill or issue.

- Use checked byte and character accessors supplied by the lexer infrastructure.
- Keep lexing context explicit where the same bytes have context-dependent meaning.
- Ensure every lexer path advances or returns EOF.
- Test malformed UTF-8 boundaries through `&str` semantics rather than raw string slicing.

## Testing

Load `testing-codegen` for parser quick tests and snapshot mechanics.

- Use the crate's `quick_test` to inspect a CST while developing.
- Add persistent fixtures under the parser crate's current `ok/` and `error/` directories.
- A parser bug fix needs the smallest fixture that failed before the change.
- Recovery changes need malformed input followed by valid syntax to prove parsing resumes correctly.

## Review Checklist

- `Absent` paths consume no input.
- Required nodes and tokens produce useful diagnostics.
- Recovery emits a grammar-valid bogus node.
- Every loop either advances or exits.
- List recovery stops before the enclosing boundary.
- The CST retains all source text, including malformed input.
- Grammar changes include generated syntax and factory artifacts.
- Valid and malformed fixtures exercise the changed path.

## References

- Parser guide: `crates/biome_parser/CONTRIBUTING.md`
- Grammars: `xtask/codegen/*.ungram`
- Parser infrastructure: `crates/biome_parser/src/`
- Language implementations: `crates/biome_*_parser/src/`
