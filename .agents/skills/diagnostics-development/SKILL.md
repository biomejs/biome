---
name: diagnostics-development
description: Use this skill when designing or implementing Biome user-facing diagnostic presentation or APIs, including messages, advice, markup, details, code frames, categories, severity, and standalone `Diagnostic` types. Do not use for lint matching logic or code-action mutations.
compatibility: Designed for coding agents working on the Biome codebase (github.com/biomejs/biome).
---

# Diagnostics Development

Use `crates/biome_diagnostics/CONTRIBUTING.md` as the canonical API and design guide. Inspect current diagnostics in the same subsystem before choosing an advice type or derive shape.

## Choose the Diagnostic API

Use `RuleDiagnostic` for lint and assist rules. Use a standalone type deriving `Diagnostic` when a parser, service, CLI, configuration, or infrastructure boundary needs its own structured diagnostic.

Do not introduce a standalone type solely to wrap a one-line lint message. Do not force a complex diagnostic with locations, sub-diagnostics, or conditional advice into a `RuleDiagnostic` chain when an owned type expresses the contract more clearly.

## Three Questions

A complete diagnostic answers:

1. What condition was found?
2. Why does it matter?
3. What can the user do?

Keep those jobs separate:

- the primary message identifies the condition;
- a note or detail explains the consequence or rationale;
- an action, diff, command, or final advice gives the next step.

Do not combine rationale and remediation into vague prose. When an automated action exists, its label normally carries the remediation rather than repeating it in a note.

## Messages and Markup

- Name the actual problem, not only the syntax construct.
- Highlight the smallest source range that helps the user understand it.
- Interpolate values directly in `markup!`; do not allocate with `format!` first.
- Use emphasis for source terms or configuration values where it improves scanning.
- Keep the primary message short; put explanation in advice.
- Do not claim a consequence the implementation does not establish.

## Advice Selection

Choose advice based on the information users need:

| Need | Mechanism |
| --- | --- |
| Explanation or next step | note or log advice |
| Point to a related source span | detail or code-frame advice |
| Show an exact textual change | diff advice or code action |
| Show a command to run | command advice |
| Extra opt-in context | verbose advice |

Prefer source evidence over a paragraph describing where the issue is. Avoid adding multiple notes that repeat the same fact in different words.

## Standalone Diagnostics

For `#[derive(Diagnostic)]`, verify the current guide and derive implementation for supported attributes. Typical concerns include:

- severity and category;
- primary message and plain-text description;
- location path, source, and span;
- advice and verbose advice;
- tags such as fixable, unnecessary, deprecated, or internal.

The diagnostic type owns stable data needed at the reporting boundary. Avoid retaining large source buffers or allocating rendered strings when a range and structured value suffice.

## Categories

Do not edit generated diagnostic-category registries by hand. Use the rule scaffolding or generator responsible for the category, then verify the generated entry and documentation URL.

For non-rule diagnostics, inspect neighboring categories and the current code-generation source before making changes; do not infer the workflow from the generated file.

## Severity

Match established subsystem policy. Severity communicates operational impact, not how strongly the author feels about the message. Inspect nearby diagnostics and the current contributing guide before introducing a different severity or tag.

## Validation

- Add the narrowest test or fixture that renders the diagnostic.
- Inspect snapshots for message, advice ordering, markup, path, and highlighted range.
- Test conditional advice in every branch.
- Test source ranges with surrounding trivia and multibyte text when offsets are computed manually.
- For lint diagnostics, load `lint-rule-development` and `testing-codegen` for rule-specific fixtures.

## Review Checklist

- The primary message states what is wrong.
- Advice explains why and gives a concrete next step.
- The highlighted range is minimal and correct.
- Structured markup avoids preformatted string allocation.
- The selected API matches diagnostic complexity.
- Category changes use the supported source or generator.
- Snapshot coverage exercises every conditional message or advice path.

## References

- Diagnostic guide: `crates/biome_diagnostics/CONTRIBUTING.md`
- Diagnostic trait: `crates/biome_diagnostics/src/diagnostic.rs`
- Advice APIs: `crates/biome_diagnostics/src/advice.rs`
- Existing derive examples: search for `#[derive(Diagnostic)]`
