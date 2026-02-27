//! Markdown-specific parse error diagnostics.

use crate::MarkdownParser;
use biome_parser::Parser;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::TextRange;

/// Default maximum nesting depth for block quotes and lists.
pub(crate) const DEFAULT_MAX_NESTING_DEPTH: usize = 100;

/// Unclosed emphasis (bold/italic).
///
/// ```markdown
/// *text
///     ^ expected closing *
/// ```
pub(crate) fn unclosed_emphasis(
    p: &MarkdownParser,
    opening_range: TextRange,
    marker: &str,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Unclosed emphasis, expected closing `{marker}`."),
        opening_range,
    )
    .with_detail(opening_range, "emphasis started here")
    .with_hint(format!(
        "Add closing `{marker}` or remove the opening delimiter."
    ))
}

/// Unclosed inline link.
///
/// ```markdown
/// [text
///     ^ expected closing ] and (url)
/// ```
pub(crate) fn unclosed_link(
    p: &MarkdownParser,
    opening_range: TextRange,
    missing_part: &str,
) -> ParseDiagnostic {
    p.err_builder(format!("Unclosed link, {missing_part}."), opening_range)
        .with_detail(opening_range, "link started here")
        .with_hint("Format: [link text](url)")
}

/// Unclosed inline image.
///
/// ```markdown
/// ![alt
///     ^ expected closing ] and (src)
/// ```
pub(crate) fn unclosed_image(
    p: &MarkdownParser,
    opening_range: TextRange,
    missing_part: &str,
) -> ParseDiagnostic {
    p.err_builder(format!("Unclosed image, {missing_part}."), opening_range)
        .with_detail(opening_range, "image started here")
        .with_hint("Format: ![alt text](image-url)")
}

/// ATX heading with too many hashes (>6).
///
/// ```markdown
/// ####### heading
/// ^^^^^^^ too many hashes (max 6)
/// ```
pub(crate) fn too_many_hashes(
    p: &MarkdownParser,
    range: TextRange,
    count: usize,
) -> ParseDiagnostic {
    p.err_builder(
        format!("ATX heading has {count} hashes, but maximum is 6."),
        range,
    )
    .with_detail(range, "heading started here")
    .with_hint("Use 1-6 `#` characters for headings. This will be parsed as a paragraph.")
}

/// Unterminated fenced code block.
///
/// ```markdown
/// ```rust
/// fn main() {}
/// <EOF>
/// ^ expected closing ```
/// ```
pub(crate) fn unterminated_fenced_code(
    p: &MarkdownParser,
    opening_range: TextRange,
    fence_type: &str,
) -> ParseDiagnostic {
    let fence_name = if fence_type == "```" {
        "triple backticks (```)"
    } else {
        "triple tildes (~~~)"
    };
    p.err_builder(
        format!("Unterminated fenced code block, expected closing {fence_name}."),
        opening_range,
    )
    .with_detail(opening_range, "code block started here")
    .with_hint(format!(
        "Add closing {fence_name} at the start of a new line."
    ))
}

/// Block quote nesting too deep.
///
/// ```markdown
/// >>>>>>>>...>>>>  (100+ levels)
/// ^^^^^^^^^^^^^^^^ nesting too deep
/// ```
pub(crate) fn quote_nesting_too_deep(
    p: &MarkdownParser,
    range: TextRange,
    max_nesting_depth: usize,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Block quote nesting exceeds maximum depth of {max_nesting_depth}."),
        range,
    )
    .with_detail(range, "nesting limit reached here")
    .with_hint("Reduce nesting depth. Additional levels will be treated as content.")
}

/// List nesting too deep.
///
/// ```markdown
/// - - - - ... - (100+ levels)
/// ^^^^^^^^^^^^^^ nesting too deep
/// ```
pub(crate) fn list_nesting_too_deep(
    p: &MarkdownParser,
    range: TextRange,
    max_nesting_depth: usize,
) -> ParseDiagnostic {
    p.err_builder(
        format!("List nesting exceeds maximum depth of {max_nesting_depth}."),
        range,
    )
    .with_detail(range, "nesting limit reached here")
    .with_hint("Reduce nesting depth. Additional levels will be treated as content.")
}

/// Parser made no progress while parsing a block.
pub(crate) fn parse_any_block_no_progress(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Parser made no progress while parsing a block.", range)
        .with_detail(range, "stuck token skipped")
        .with_hint("This is likely a parser bug; the token was skipped to recover.")
}
