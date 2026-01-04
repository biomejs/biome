//! Whitespace sensitivity helpers for HTML formatting.
//!
//! This module provides utilities for determining whitespace sensitivity
//! at various positions in the HTML document, matching Prettier's behavior.
//!
//! The key insight from Prettier is that whitespace sensitivity is not just
//! about the element itself, but about the relationship between adjacent nodes.
//! For example, whether we can add a line break between two children depends on:
//! - The whitespace mode (css/strict/ignore)
//! - Whether the previous/next siblings are inline or block elements
//! - Whether there's actual whitespace in the source
//! - Whether we're at the start/end of a parent element

use biome_html_syntax::{AnyHtmlElement, HtmlElement, HtmlSelfClosingElement};
use biome_rowan::TokenText;
use biome_string_case::StrLikeExtension;
use enumflags2::{BitFlags, bitflags, make_bitflags};

use crate::HtmlFormatter;
use crate::context::WhitespaceSensitivity;
use crate::utils::css_display::{CssDisplay, get_css_display, get_css_display_from_tag};

/// Whitespace information flags for a node.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[bitflags]
#[repr(u8)]
pub enum WhitespaceFlag {
    /// Whether the leading position is whitespace-sensitive.
    /// If true, we cannot freely add/remove whitespace before this node.
    LeadingSpaceSensitive = 1 << 0,

    /// Whether there are actual leading spaces in the source.
    HasLeadingSpaces = 1 << 1,

    /// Whether the trailing position is whitespace-sensitive.
    /// If true, we cannot freely add/remove whitespace after this node.
    TrailingSpaceSensitive = 1 << 2,

    /// Whether there are actual trailing spaces in the source.
    HasTrailingSpaces = 1 << 3,
}

/// Information about whitespace around a node.
///
/// This struct captures the whitespace sensitivity context that Prettier uses
/// to make formatting decisions.
///
/// Prettier source: src/language-html/print-preprocess.js:255
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WhitespaceInfo(BitFlags<WhitespaceFlag>);

impl WhitespaceInfo {
    /// Creates a new WhitespaceInfo with all fields set to false.
    pub const fn new() -> Self {
        Self(BitFlags::EMPTY)
    }

    /// Creates WhitespaceInfo where both positions are sensitive.
    pub const fn all_sensitive() -> Self {
        Self(make_bitflags!(WhitespaceFlag::{LeadingSpaceSensitive | TrailingSpaceSensitive}))
    }

    /// Creates WhitespaceInfo where neither position is sensitive.
    pub const fn not_sensitive() -> Self {
        Self::new()
    }

    /// Returns whether the leading position is whitespace-sensitive.
    pub fn is_leading_space_sensitive(&self) -> bool {
        self.0.contains(WhitespaceFlag::LeadingSpaceSensitive)
    }

    /// Sets whether the leading position is whitespace-sensitive.
    pub fn set_leading_space_sensitive(&mut self, value: bool) {
        if value {
            self.0.insert(WhitespaceFlag::LeadingSpaceSensitive);
        } else {
            self.0.remove(WhitespaceFlag::LeadingSpaceSensitive);
        }
    }

    /// Returns whether there are actual leading spaces in the source.
    pub fn has_leading_spaces(&self) -> bool {
        self.0.contains(WhitespaceFlag::HasLeadingSpaces)
    }

    /// Sets whether there are actual leading spaces in the source.
    pub fn set_has_leading_spaces(&mut self, value: bool) {
        if value {
            self.0.insert(WhitespaceFlag::HasLeadingSpaces);
        } else {
            self.0.remove(WhitespaceFlag::HasLeadingSpaces);
        }
    }

    /// Returns whether the trailing position is whitespace-sensitive.
    pub fn is_trailing_space_sensitive(&self) -> bool {
        self.0.contains(WhitespaceFlag::TrailingSpaceSensitive)
    }

    /// Sets whether the trailing position is whitespace-sensitive.
    pub fn set_trailing_space_sensitive(&mut self, value: bool) {
        if value {
            self.0.insert(WhitespaceFlag::TrailingSpaceSensitive);
        } else {
            self.0.remove(WhitespaceFlag::TrailingSpaceSensitive);
        }
    }

    /// Returns whether there are actual trailing spaces in the source.
    pub fn has_trailing_spaces(&self) -> bool {
        self.0.contains(WhitespaceFlag::HasTrailingSpaces)
    }

    /// Sets whether there are actual trailing spaces in the source.
    pub fn set_has_trailing_spaces(&mut self, value: bool) {
        if value {
            self.0.insert(WhitespaceFlag::HasTrailingSpaces);
        } else {
            self.0.remove(WhitespaceFlag::HasTrailingSpaces);
        }
    }
}

impl Default for WhitespaceInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Determines if an element is whitespace-sensitive based on its CSS display value.
///
/// In "css" whitespace mode, inline-like elements (inline, inline-block, table-cell, etc.)
/// are whitespace-sensitive because adding/removing spaces affects rendering.
pub fn is_element_whitespace_sensitive_by_css(tag_name: &str) -> bool {
    get_css_display(tag_name).is_inline_like()
}

/// Determines if whitespace between two adjacent children is significant.
///
/// This is the core function for determining whether we can add line breaks
/// between children in an element. The logic matches Prettier's behavior.
///
/// Arguments:
/// - `sensitivity`: The whitespace sensitivity mode from formatter options
/// - `prev`: The previous child (if any)
/// - `next`: The next child (if any)
/// - `has_whitespace`: Whether there's whitespace between prev and next in source
pub fn is_whitespace_between_significant(
    sensitivity: WhitespaceSensitivity,
    prev: Option<&AnyHtmlElement>,
    next: Option<&AnyHtmlElement>,
    _has_whitespace: bool,
) -> bool {
    match sensitivity {
        WhitespaceSensitivity::Strict => true,
        WhitespaceSensitivity::Ignore => false,
        WhitespaceSensitivity::Css => {
            // In CSS mode, whitespace is significant if either adjacent node
            // is inline-like.
            let prev_is_inline = prev.is_some_and(is_element_inline_like);
            let next_is_inline = next.is_some_and(is_element_inline_like);

            // Whitespace is significant between two inline elements
            // or at the boundary of an inline element with text
            if prev_is_inline || next_is_inline {
                return true;
            }

            // Also significant if there's a text node involved
            let prev_is_text = prev.is_some_and(is_text_like);
            let next_is_text = next.is_some_and(is_text_like);

            prev_is_text || next_is_text
        }
    }
}

/// Determines if the leading whitespace of a node is significant.
///
/// This considers both the node itself and what comes before it.
pub fn is_leading_whitespace_sensitive(
    sensitivity: WhitespaceSensitivity,
    node: &AnyHtmlElement,
    prev_sibling: Option<&AnyHtmlElement>,
    parent_tag: Option<&str>,
) -> bool {
    match sensitivity {
        WhitespaceSensitivity::Strict => true,
        WhitespaceSensitivity::Ignore => false,
        WhitespaceSensitivity::Css => {
            // Check if this node itself is inline-like
            if is_element_inline_like(node) {
                return true;
            }

            // Check if previous sibling is inline-like
            if let Some(prev) = prev_sibling
                && (is_element_inline_like(prev) || is_text_like(prev))
            {
                return true;
            }

            // Check if parent is inline-like
            if let Some(parent) = parent_tag
                && get_css_display(parent).is_inline_like()
            {
                return true;
            }

            // Text-like nodes are always sensitive
            is_text_like(node)
        }
    }
}

/// Determines if the trailing whitespace of a node is significant.
///
/// This considers both the node itself and what comes after it.
pub fn is_trailing_whitespace_sensitive(
    sensitivity: WhitespaceSensitivity,
    node: &AnyHtmlElement,
    next_sibling: Option<&AnyHtmlElement>,
    parent_tag: Option<&str>,
) -> bool {
    match sensitivity {
        WhitespaceSensitivity::Strict => true,
        WhitespaceSensitivity::Ignore => false,
        WhitespaceSensitivity::Css => {
            // Check if this node itself is inline-like
            if is_element_inline_like(node) {
                return true;
            }

            // Check if next sibling is inline-like
            if let Some(next) = next_sibling
                && (is_element_inline_like(next) || is_text_like(next))
            {
                return true;
            }

            // Check if parent is inline-like
            if let Some(parent) = parent_tag
                && get_css_display(parent).is_inline_like()
            {
                return true;
            }

            // Text-like nodes are always sensitive
            is_text_like(node)
        }
    }
}

/// Checks if an element has inline-like CSS display.
fn is_element_inline_like(element: &AnyHtmlElement) -> bool {
    match element {
        AnyHtmlElement::HtmlElement(el) => {
            if let Ok(opening) = el.opening_element()
                && let Ok(tag_name) = opening.name()
            {
                return get_css_display_from_tag(&tag_name).is_inline_like();
            }
            // Unknown elements default to inline
            true
        }
        AnyHtmlElement::HtmlSelfClosingElement(el) => {
            if let Ok(tag_name) = el.name() {
                return get_css_display_from_tag(&tag_name).is_inline_like();
            }
            true
        }
        // Content (text) is always considered inline
        AnyHtmlElement::AnyHtmlContent(_) => true,
        // Other node types
        _ => false,
    }
}

/// Checks if a node is text-like (content that should be treated as inline text).
fn is_text_like(element: &AnyHtmlElement) -> bool {
    matches!(element, AnyHtmlElement::AnyHtmlContent(_))
}

/// Determines if we should preserve the exact whitespace for a parent element.
///
/// Some elements like `<pre>`, `<textarea>`, `<script>`, `<style>` should
/// preserve all internal whitespace.
pub fn is_whitespace_preserving_parent(tag_name: &str) -> bool {
    let tag_name = tag_name.to_ascii_lowercase_cow();
    matches!(
        tag_name.as_ref(),
        "pre" | "textarea" | "script" | "style" | "xmp" | "listing" | "plaintext"
    )
}

/// Gets the tag name from an HtmlElement.
#[deprecated(note = "use .tag_name()")]
pub fn get_element_tag_name(element: &HtmlElement) -> Option<TokenText> {
    element.tag_name()
}

/// Gets the tag name from a HtmlSelfClosingElement.
#[deprecated(note = "use .tag_name()")]
pub fn get_self_closing_tag_name(element: &HtmlSelfClosingElement) -> Option<TokenText> {
    element.tag_name()
}

/// Gets the tag name from any element type.
#[deprecated(note = "use .name()")]
pub fn get_any_element_tag_name(element: &AnyHtmlElement) -> Option<TokenText> {
    element.name()
}

/// Determines the CSS display for any HTML element.
pub fn get_any_element_display(element: &AnyHtmlElement) -> CssDisplay {
    match element.name() {
        Some(name) => get_css_display(&name),
        None => CssDisplay::default(),
    }
}

/// Checks if an element should cause a line break before it.
///
/// Block-level elements typically cause line breaks.
pub fn should_break_before_element(
    element: &AnyHtmlElement,
    sensitivity: WhitespaceSensitivity,
) -> bool {
    match sensitivity {
        WhitespaceSensitivity::Strict => false,
        WhitespaceSensitivity::Ignore => true,
        WhitespaceSensitivity::Css => {
            let display = get_any_element_display(element);
            display.is_block_like()
        }
    }
}

/// Checks if an element should cause a line break after it.
///
/// Block-level elements typically cause line breaks.
pub fn should_break_after_element(
    element: &AnyHtmlElement,
    sensitivity: WhitespaceSensitivity,
) -> bool {
    match sensitivity {
        WhitespaceSensitivity::Strict => false,
        WhitespaceSensitivity::Ignore => true,
        WhitespaceSensitivity::Css => {
            let display = get_any_element_display(element);
            display.is_block_like()
        }
    }
}

/// Computes WhitespaceInfo for a child node given its context.
pub fn compute_whitespace_info(
    f: &HtmlFormatter,
    node: &AnyHtmlElement,
    prev_sibling: Option<&AnyHtmlElement>,
    next_sibling: Option<&AnyHtmlElement>,
    parent_tag: Option<&str>,
    has_leading_spaces_in_source: bool,
    has_trailing_spaces_in_source: bool,
) -> WhitespaceInfo {
    let sensitivity = f.options().whitespace_sensitivity();

    let mut info = WhitespaceInfo::new();

    info.set_leading_space_sensitive(is_leading_whitespace_sensitive(
        sensitivity,
        node,
        prev_sibling,
        parent_tag,
    ));
    info.set_has_leading_spaces(has_leading_spaces_in_source);
    info.set_trailing_space_sensitive(is_trailing_whitespace_sensitive(
        sensitivity,
        node,
        next_sibling,
        parent_tag,
    ));
    info.set_has_trailing_spaces(has_trailing_spaces_in_source);

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whitespace_preserving_elements() {
        assert!(is_whitespace_preserving_parent("pre"));
        assert!(is_whitespace_preserving_parent("PRE"));
        assert!(is_whitespace_preserving_parent("textarea"));
        assert!(is_whitespace_preserving_parent("script"));
        assert!(is_whitespace_preserving_parent("style"));
        assert!(!is_whitespace_preserving_parent("div"));
        assert!(!is_whitespace_preserving_parent("span"));
    }

    #[test]
    fn test_inline_sensitive_elements() {
        // Inline elements should be sensitive
        assert!(is_element_whitespace_sensitive_by_css("span"));
        assert!(is_element_whitespace_sensitive_by_css("a"));
        assert!(is_element_whitespace_sensitive_by_css("strong"));

        // Block elements should not be sensitive
        assert!(!is_element_whitespace_sensitive_by_css("div"));
        assert!(!is_element_whitespace_sensitive_by_css("p"));
        assert!(!is_element_whitespace_sensitive_by_css("section"));
    }

    #[test]
    fn test_table_cells_are_sensitive() {
        // Table cells have inline content and are whitespace-sensitive
        assert!(is_element_whitespace_sensitive_by_css("td"));
        assert!(is_element_whitespace_sensitive_by_css("th"));
    }
}
