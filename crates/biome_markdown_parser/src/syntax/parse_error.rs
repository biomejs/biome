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

/// MDX expression without its closing brace.
///
/// ```mdx
/// {props.title
/// ^ expected closing }
/// ```
#[cfg(feature = "lang_mdx")]
pub(crate) fn unclosed_mdx_expression(
    p: &MarkdownParser,
    opening_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Unclosed expression, expected a closing `}`.",
        opening_range,
    )
    .with_detail(opening_range, "expression started here")
    .with_hint("Add the closing `}`. To write a literal brace in MDX, escape it as `\\{`.")
}

/// MDX `<` followed by a character that can't start a JSX tag.
///
/// ```mdx
/// 1<2 or <!-- comment -->
///  ^     ^ expected a tag name
/// ```
#[cfg(feature = "lang_mdx")]
pub(crate) fn invalid_mdx_tag_start(
    p: &MarkdownParser,
    range: TextRange,
    character: char,
    comment: bool,
) -> ParseDiagnostic {
    let hint = if comment {
        "MDX doesn't support HTML comments. Write a comment as `{/* text */}` instead."
    } else {
        "To write a literal `<` in MDX, escape it as `\\<` or put a space after it."
    };
    p.err_builder(
        format!("Unexpected character `{character}` after `<`, expected a tag name, `/`, or `>`."),
        range,
    )
    .with_hint(hint)
}

/// MDX JSX tag whose name isn't a JSX name, such as `<https://example.com>`.
#[cfg(feature = "lang_mdx")]
pub(crate) fn invalid_mdx_tag_name(
    p: &MarkdownParser,
    range: TextRange,
    character: char,
    name: &str,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Unexpected character `{character}` in the tag name `{name}`."),
        range,
    )
    .with_hint(
        "A tag name is a JavaScript name, optionally joined with `.` or a single `:`. To write a literal `<` in MDX, escape it as `\\<`.",
    )
}

/// MDX JSX tag without its closing `>`.
#[cfg(feature = "lang_mdx")]
pub(crate) fn unclosed_mdx_tag(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unclosed tag, expected a closing `>`.", range)
        .with_detail(range, "tag started here")
        .with_hint("Add the closing `>`. To write a literal `<` in MDX, escape it as `\\<`.")
}

/// MDX closing tag with attributes, such as `</Note type="tip">`.
#[cfg(feature = "lang_mdx")]
pub(crate) fn mdx_closing_tag_with_attributes(
    p: &MarkdownParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Unexpected attribute in a closing tag.", range)
        .with_hint("Move the attributes to the opening tag.")
}

/// MDX closing tag that is also self-closing, such as `</Note/>`.
#[cfg(feature = "lang_mdx")]
pub(crate) fn mdx_self_closing_closing_tag(
    p: &MarkdownParser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Unexpected `/` at the end of a closing tag.", range)
        .with_hint("A tag is either a closing tag or self-closing. Remove one of the slashes.")
}

/// MDX attribute whose value is an element, such as `icon=<Icon />`.
#[cfg(feature = "lang_mdx")]
pub(crate) fn mdx_element_attribute_value(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Unexpected element as an attribute value.", range)
        .with_hint("Wrap the element in braces, as in `icon={<Icon />}`.")
}

/// Parser made no progress while parsing a block.
pub(crate) fn parse_any_block_no_progress(p: &MarkdownParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Parser made no progress while parsing a block.", range)
        .with_detail(range, "stuck token skipped")
        .with_hint("This is likely a parser bug; the token was skipped to recover.")
}
