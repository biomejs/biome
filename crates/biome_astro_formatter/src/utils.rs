use crate::prelude::*;
use biome_astro_syntax::{AstroSyntaxNode, AstroSyntaxToken};

/// Check if the given node is an inline element that should not have line breaks around it
pub fn is_inline_element(node: &AstroSyntaxNode) -> bool {
    // This is a simplified implementation
    // In a real formatter, we'd have a comprehensive list of inline HTML elements
    false
}

/// Check if the given node is a block element that should have line breaks around it
pub fn is_block_element(node: &AstroSyntaxNode) -> bool {
    // This is a simplified implementation
    // In a real formatter, we'd have a comprehensive list of block HTML elements
    true
}

/// Check if the given text node should preserve whitespace
pub fn should_preserve_whitespace(node: &AstroSyntaxNode) -> bool {
    // Check if we're inside a <pre> tag or similar
    // This is a simplified implementation
    false
}

/// Trim whitespace from the start of a text node
pub fn trim_text_start(text: &str) -> &str {
    text.trim_start()
}

/// Trim whitespace from the end of a text node
pub fn trim_text_end(text: &str) -> &str {
    text.trim_end()
}

/// Check if the text contains only whitespace
pub fn is_only_whitespace(text: &str) -> bool {
    text.chars().all(|c| c.is_whitespace())
}

/// Check if the text starts with a line break
pub fn starts_with_line_break(text: &str) -> bool {
    text.starts_with('\n') || text.starts_with("\r\n")
}

/// Check if the text ends with a line break
pub fn ends_with_line_break(text: &str) -> bool {
    text.ends_with('\n') || text.ends_with("\r\n")
}

/// Get the preferred quote character for attribute values
pub fn get_preferred_quote(value: &str, default_quote: char) -> char {
    if value.contains(default_quote) && !value.contains(if default_quote == '"' { '\'' } else { '"' }) {
        if default_quote == '"' { '\'' } else { '"' }
    } else {
        default_quote
    }
}

/// Check if an element should be formatted on a single line
pub fn should_format_on_single_line(node: &AstroSyntaxNode) -> bool {
    // This is a simplified implementation
    // Factors to consider:
    // - Element type (inline vs block)
    // - Number of attributes
    // - Length of content
    // - Line width configuration
    false
}

/// Check if attributes should be formatted on separate lines
pub fn should_format_attributes_on_separate_lines(
    attributes: &[AstroSyntaxNode],
    line_width: usize,
) -> bool {
    // This is a simplified implementation
    // In a real formatter, we'd calculate the total length and compare with line width
    attributes.len() > 3
}