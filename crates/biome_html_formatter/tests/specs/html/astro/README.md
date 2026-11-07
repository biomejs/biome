# Astro formatter corpus classification

This directory is the native Astro formatter corpus. Expected output is authored from Biome's syntax tree and formatter contracts. The tests do not execute or compare against `@astrojs/compiler` or Prettier Plugin Astro.

## Fixture classifications

| Fixture | Contract | Classification |
| --- | --- | --- |
| `stability_baseline.astro` | Formats frontmatter, markup, components, fragments, expressions, directives, scripts, styles, and `is:raw` content together. | Supported canonical formatting |
| `component.astro` | Preserves component identity while formatting component markup and frontmatter imports. | Supported canonical formatting |
| `directives.astro` | Formats Astro directives and their expression values. | Supported canonical formatting |
| `doctype_after_script.astro` | Preserves root-item source order when a doctype follows a script. | Source-preservation requirement |
| `empty-expression.astro` | Preserves empty expressions and formats non-empty expressions. | Supported canonical formatting |
| `fragment.astro` | Formats top-level and nested fragment shorthand. | Supported canonical formatting |
| `lowercase-member.astro` | Preserves component member expressions and dotted attribute names. | Supported canonical formatting |
| `pre-comments.astro` | Preserves preformatted text and comments while formatting surrounding markup. | Source-preservation requirement |
| `processing-instruction.astro` | Preserves processing instructions at the document root and inside SVG. | Source-preservation requirement |
| `style-is-raw.astro` | Preserves `is:raw` style content and formats ordinary CSS style content. | Deliberately opaque content |
| `template-literal-attribute.astro` | Preserves live Astro template-literal attributes instead of treating them as static strings. | Source-preservation requirement |
| `failed-embeds.astro` | Keeps malformed frontmatter, expressions, scripts, and styles unchanged while formatting surrounding markup. | Conservative error recovery |
| `opaque-embeds.astro` | Keeps unsupported script and style languages unchanged. | Unsupported language, deliberately opaque |

Static `class` and `style` attribute behavior is covered by `../attributes/class-style.astro`. Static classes use normalized whitespace, valid style values use inline CSS formatting, component props remain unchanged, and dynamic Astro template values remain source-preserving.

## Intentional Biome output

Biome defines its own canonical Astro output. Differences from other formatters are intentional when they follow these contracts:

- root items remain in source order;
- static class whitespace is collapsed to class-token separators;
- valid style attributes use Biome's CSS formatting, including trailing semicolons;
- unsupported preprocessors and custom embedded languages remain unchanged;
- malformed embedded code remains unchanged instead of being partially reformatted.

A difference is release-blocking only when it loses source, changes opaque or malformed embedded content, panics on valid Astro, fails to converge after one format pass, or produces different CLI and LSP content.

## Verification

The formatter spec harness checks whole-document idempotency for error-free fixtures. Integrated coverage also lives in:

- `crates/biome_cli/tests/cases/astro_stability.rs` for repeated CLI formatting;
- `crates/biome_lsp/src/server_astro.tests.rs` for LSP preservation and multi-project fix/format sequences.
