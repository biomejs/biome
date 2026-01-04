//! Classified children for HTML formatting.
//!
//! This module provides a two-phase approach to formatting HTML children:
//! 1. First, classify all children with their whitespace sensitivity and CSS display
//! 2. Then, format them with proper separators based on the classification
//!
//! This approach matches Prettier's behavior where decisions about whitespace and
//! line breaks are made based on the relationship between adjacent nodes.
//!
//! NOTE: This module is not yet integrated into the main formatter. It is prepared
//! for future use when element_list.rs is refactored to use the classification system.

#![allow(dead_code)]

use biome_formatter::Format;
use biome_formatter::prelude::*;
use biome_formatter::write;
use biome_html_syntax::AnyHtmlElement;

use crate::context::HtmlFormatContext;
use crate::context::WhitespaceSensitivity;
use crate::utils::children::HtmlChild;
use crate::utils::css_display::{CssDisplay, get_css_display};
use crate::utils::whitespace::WhitespaceInfo;

/// A child node with its classification metadata.
///
/// This struct captures everything needed to make formatting decisions:
/// - The underlying child (word, element, whitespace, etc.)
/// - CSS display value (for elements)
/// - Whitespace sensitivity information
#[derive(Debug)]
pub(crate) struct ClassifiedChild {
    /// The underlying child content
    pub child: HtmlChild,

    /// CSS display value for element children, None for text/whitespace
    pub display: Option<CssDisplay>,

    /// Whitespace sensitivity information based on context
    pub whitespace_info: WhitespaceInfo,
}

impl ClassifiedChild {
    /// Creates a new classified child with default whitespace info.
    pub(crate) fn new(child: HtmlChild) -> Self {
        let display = Self::compute_display(&child);
        Self {
            child,
            display,
            whitespace_info: WhitespaceInfo::default(),
        }
    }

    /// Creates a new classified child with explicit whitespace info.
    pub(crate) fn with_whitespace_info(child: HtmlChild, whitespace_info: WhitespaceInfo) -> Self {
        let display = Self::compute_display(&child);
        Self {
            child,
            display,
            whitespace_info,
        }
    }

    /// Computes the CSS display value for a child.
    fn compute_display(child: &HtmlChild) -> Option<CssDisplay> {
        match child {
            HtmlChild::NonText(element) | HtmlChild::Verbatim(element) => {
                Some(get_element_display(element))
            }
            // Text nodes (words, comments) and whitespace don't have CSS display
            _ => None,
        }
    }

    /// Returns true if this child is a block-level element.
    pub(crate) fn is_block(&self) -> bool {
        self.display.is_some_and(|d| d.is_block_like())
    }

    /// Returns true if this child is an inline-level element.
    pub(crate) fn is_inline(&self) -> bool {
        self.display.is_some_and(|d| d.is_inline_like())
    }

    /// Returns true if this child is a text node (word or comment).
    pub(crate) fn is_text(&self) -> bool {
        matches!(self.child, HtmlChild::Word(_) | HtmlChild::Comment(_))
    }

    /// Returns true if this child is a comment.
    pub(crate) fn is_comment(&self) -> bool {
        matches!(self.child, HtmlChild::Comment(_))
    }

    /// Returns true if this child is a comment.
    pub(crate) fn is_word(&self) -> bool {
        matches!(self.child, HtmlChild::Word(_))
    }

    /// Returns true if this child is whitespace (space, newline, empty line).
    pub(crate) fn is_whitespace(&self) -> bool {
        matches!(
            self.child,
            HtmlChild::Whitespace | HtmlChild::Newline | HtmlChild::EmptyLine
        )
    }

    /// Returns true if this child represents an empty line.
    pub(crate) fn is_empty_line(&self) -> bool {
        matches!(self.child, HtmlChild::EmptyLine)
    }

    /// Returns true if this child represents a newline.
    pub(crate) fn is_newline(&self) -> bool {
        matches!(self.child, HtmlChild::Newline)
    }

    /// Returns true if leading whitespace is significant for this child.
    pub(crate) fn is_leading_whitespace_sensitive(&self) -> bool {
        self.whitespace_info.is_leading_space_sensitive()
    }

    /// Returns true if trailing whitespace is significant for this child.
    pub(crate) fn is_trailing_whitespace_sensitive(&self) -> bool {
        self.whitespace_info.is_trailing_space_sensitive()
    }
}

/// Gets the CSS display value for an AnyHtmlElement.
fn get_element_display(element: &AnyHtmlElement) -> CssDisplay {
    match element {
        AnyHtmlElement::HtmlElement(el) => {
            let Some(tag_name) = el.tag_name() else {
                return CssDisplay::Inline;
            };
            get_css_display(&tag_name)
        }
        AnyHtmlElement::HtmlSelfClosingElement(el) => {
            let Some(tag_name) = el.tag_name() else {
                return CssDisplay::Inline;
            };
            get_css_display(&tag_name)
        }
        // Content is treated as inline
        AnyHtmlElement::AnyHtmlContent(_) => CssDisplay::Inline,
        // Other node types default to inline
        _ => CssDisplay::Inline,
    }
}

/// A list of classified children with methods for querying relationships.
#[derive(Debug)]
pub(crate) struct ClassifiedChildren {
    children: Vec<ClassifiedChild>,
}

impl ClassifiedChildren {
    /// Creates a new empty classified children list.
    pub(crate) fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Creates classified children from an iterator of HtmlChild.
    pub(crate) fn from_children(children: Vec<HtmlChild>) -> Self {
        let classified: Vec<ClassifiedChild> =
            children.into_iter().map(ClassifiedChild::new).collect();
        Self {
            children: classified,
        }
    }

    /// Returns the number of children.
    pub(crate) fn len(&self) -> usize {
        self.children.len()
    }

    /// Returns true if there are no children.
    pub(crate) fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    /// Returns an iterator over the classified children.
    pub(crate) fn iter(&self) -> impl Iterator<Item = &ClassifiedChild> {
        self.children.iter()
    }

    /// Returns the child at the given index.
    pub(crate) fn get(&self, index: usize) -> Option<&ClassifiedChild> {
        self.children.get(index)
    }

    /// Returns the previous non-whitespace child before the given index.
    pub(crate) fn prev_non_whitespace(&self, index: usize) -> Option<&ClassifiedChild> {
        if index == 0 {
            return None;
        }
        self.children[..index]
            .iter()
            .rev()
            .find(|c| !c.is_whitespace())
    }

    /// Returns the next non-whitespace child after the given index.
    #[allow(dead_code)]
    pub(crate) fn next_non_whitespace(&self, index: usize) -> Option<&ClassifiedChild> {
        self.children[index + 1..]
            .iter()
            .find(|c| !c.is_whitespace())
    }

    /// Returns the first child.
    #[allow(dead_code)]
    pub(crate) fn first(&self) -> Option<&ClassifiedChild> {
        self.children.first()
    }

    /// Returns the last child.
    #[allow(dead_code)]
    pub(crate) fn last(&self) -> Option<&ClassifiedChild> {
        self.children.last()
    }

    /// Removes and returns the last child if it's whitespace.
    #[allow(dead_code)]
    pub(crate) fn pop_trailing_whitespace(&mut self) -> Option<ClassifiedChild> {
        if self.children.last().is_some_and(|c| c.is_whitespace()) {
            self.children.pop()
        } else {
            None
        }
    }

    /// Removes and returns the first child if it's whitespace.
    #[allow(dead_code)]
    pub(crate) fn remove_leading_whitespace(&mut self) -> Option<ClassifiedChild> {
        if self.children.first().is_some_and(|c| c.is_whitespace()) {
            Some(self.children.remove(0))
        } else {
            None
        }
    }

    /// Returns true if any child contains a block-level element.
    #[allow(dead_code)]
    pub(crate) fn has_block_element(&self) -> bool {
        self.children.iter().any(|c| c.is_block())
    }

    /// Returns true if children contain meaningful text (non-whitespace words).
    #[allow(dead_code)]
    pub(crate) fn has_meaningful_text(&self) -> bool {
        self.children
            .iter()
            .any(|c| matches!(c.child, HtmlChild::Word(_)))
    }
}

impl Default for ClassifiedChildren {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for ClassifiedChildren {
    type Item = ClassifiedChild;
    type IntoIter = std::vec::IntoIter<ClassifiedChild>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl<'a> IntoIterator for &'a ClassifiedChildren {
    type Item = &'a ClassifiedChild;
    type IntoIter = std::slice::Iter<'a, ClassifiedChild>;

    fn into_iter(self) -> Self::IntoIter {
        self.children.iter()
    }
}

/// Classification context for making whitespace sensitivity decisions.
#[derive(Debug, Clone)]
pub(crate) struct ClassificationContext<'a> {
    /// The whitespace sensitivity mode from formatter options.
    pub sensitivity: WhitespaceSensitivity,
    /// The parent element's tag name, if known. AKA the element that
    /// contains the element list children that we're classifying.
    pub parent_tag: Option<&'a str>,
    /// Whether the parent element preserves whitespace (like <pre>).
    pub is_whitespace_preserving: bool,
}

impl<'a> ClassificationContext<'a> {
    /// Creates a new classification context.
    pub(crate) fn new(sensitivity: WhitespaceSensitivity) -> Self {
        Self {
            sensitivity,
            parent_tag: None,
            is_whitespace_preserving: false,
        }
    }

    /// Sets the parent tag name.
    #[allow(dead_code)]
    pub(crate) fn with_parent_tag(mut self, tag: Option<&'a str>) -> Self {
        self.parent_tag = tag;
        self
    }

    /// Sets whether the parent preserves whitespace.
    #[allow(dead_code)]
    pub(crate) fn with_whitespace_preserving(mut self, preserving: bool) -> Self {
        self.is_whitespace_preserving = preserving;
        self
    }
}

/// Classifies children with whitespace sensitivity information.
///
/// This is the main entry point for the two-phase formatting approach:
/// 1. Call this function to get classified children
/// 2. Use the classification to make formatting decisions
pub(crate) fn classify_children(
    children: Vec<HtmlChild>,
    ctx: &ClassificationContext<'_>,
) -> ClassifiedChildren {
    let mut classified = ClassifiedChildren::from_children(children);

    // Now compute whitespace sensitivity for each child based on neighbors
    let len = classified.len();
    for i in 0..len {
        let prev = if i > 0 { classified.get(i - 1) } else { None };
        let next = classified.get(i + 1);

        let is_leading_sensitive = compute_leading_sensitivity(ctx, classified.get(i), prev);
        let is_trailing_sensitive = compute_trailing_sensitivity(ctx, classified.get(i), next);

        // Check for actual whitespace in source
        let (has_leading, has_trailing) = check_source_whitespace(classified.get(i), prev, next);

        // update the child's whitespace info
        if let Some(child) = classified.children.get_mut(i) {
            child
                .whitespace_info
                .set_leading_space_sensitive(is_leading_sensitive);
            child
                .whitespace_info
                .set_trailing_space_sensitive(is_trailing_sensitive);
            child.whitespace_info.set_has_leading_spaces(has_leading);
            child.whitespace_info.set_has_trailing_spaces(has_trailing);
        }
    }

    classified
}

/// Computes whether leading whitespace is sensitive for a child.
fn compute_leading_sensitivity(
    ctx: &ClassificationContext<'_>,
    current: Option<&ClassifiedChild>,
    prev: Option<&ClassifiedChild>,
) -> bool {
    let Some(current) = current else {
        return false;
    };

    match ctx.sensitivity {
        WhitespaceSensitivity::Strict => true,
        WhitespaceSensitivity::Ignore => false,
        WhitespaceSensitivity::Css => {
            // In whitespace-preserving elements, everything is sensitive
            if ctx.is_whitespace_preserving {
                return true;
            }

            // Text and comment nodes are always sensitive
            if current.is_text() {
                return true;
            }

            // Inline elements are sensitive
            if current.is_inline() {
                return true;
            }

            // If previous sibling is inline or text, we're sensitive
            if let Some(prev) = prev
                && (prev.is_inline() || prev.is_text())
            {
                return true;
            }

            // If parent is inline, we're sensitive
            if let Some(parent_tag) = ctx.parent_tag
                && get_css_display(parent_tag).is_inline_like()
            {
                return true;
            }

            false
        }
    }
}

/// Computes whether trailing whitespace is sensitive for a child.
fn compute_trailing_sensitivity(
    ctx: &ClassificationContext<'_>,
    current: Option<&ClassifiedChild>,
    next: Option<&ClassifiedChild>,
) -> bool {
    let Some(current) = current else {
        return false;
    };

    match ctx.sensitivity {
        WhitespaceSensitivity::Strict => true,
        WhitespaceSensitivity::Ignore => false,
        WhitespaceSensitivity::Css => {
            // In whitespace-preserving elements, everything is sensitive
            if ctx.is_whitespace_preserving {
                return true;
            }

            // Text nodes are always sensitive
            if current.is_text() {
                return true;
            }

            // Inline elements are sensitive
            if current.is_inline() {
                return true;
            }

            // If next sibling is inline or text, we're sensitive
            if let Some(next) = next
                && (next.is_inline() || next.is_text())
            {
                return true;
            }

            // If parent is inline, we're sensitive
            if let Some(parent_tag) = ctx.parent_tag
                && get_css_display(parent_tag).is_inline_like()
            {
                return true;
            }

            false
        }
    }
}

/// Checks if there's whitespace in the source before/after a child.
fn check_source_whitespace(
    _current: Option<&ClassifiedChild>,
    prev: Option<&ClassifiedChild>,
    next: Option<&ClassifiedChild>,
) -> (bool, bool) {
    let has_leading = prev.is_some_and(|p| p.is_whitespace());
    let has_trailing = next.is_some_and(|n| n.is_whitespace());
    (has_leading, has_trailing)
}

/// Determines what kind of separator to use between two adjacent children.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChildSeparator {
    /// No separator needed (elements directly adjacent).
    None,
    /// A space character.
    Space,
    /// A soft line break (space in flat mode, newline in expanded).
    SoftLine,
    /// A soft line break or space.
    SoftLineOrSpace,
    /// A hard line break (always newline).
    HardLine,
    /// An empty line (two newlines).
    EmptyLine,
}

impl ChildSeparator {
    /// Returns true if this separator will always cause a line break.
    pub(crate) fn is_hard(&self) -> bool {
        matches!(self, Self::HardLine | Self::EmptyLine)
    }
}

impl Format<HtmlFormatContext> for ChildSeparator {
    fn fmt(&self, f: &mut Formatter<HtmlFormatContext>) -> FormatResult<()> {
        match self {
            Self::None => Ok(()),
            Self::Space => write!(f, [space()]),
            Self::SoftLine => write!(f, [soft_line_break()]),
            Self::SoftLineOrSpace => write!(f, [soft_line_break_or_space()]),
            Self::HardLine => write!(f, [hard_line_break()]),
            Self::EmptyLine => write!(f, [empty_line()]),
        }
    }
}

/// Determines the appropriate separator between two classified children.
pub(crate) fn get_separator_between(
    current: &ClassifiedChild,
    existing_separator: Option<&ClassifiedChild>,
    next: &ClassifiedChild,
    ctx: &ClassificationContext<'_>,
) -> ChildSeparator {
    if let Some(existing_separator) = existing_separator {
        debug_assert!(
            existing_separator.is_whitespace(),
            "Existing separator must be whitespace, got {:?}",
            existing_separator.child
        );
    }

    // If in whitespace-preserving mode, preserve exact whitespace
    if ctx.is_whitespace_preserving {
        return ChildSeparator::None;
    }

    match ctx.sensitivity {
        WhitespaceSensitivity::Strict => {
            let keep_existing_separator = (current.is_comment() && next.is_text())
                || (current.is_text() && next.is_comment());

            if keep_existing_separator && let Some(existing_separator) = existing_separator {
                return match existing_separator.child {
                    HtmlChild::Whitespace => ChildSeparator::Space,
                    HtmlChild::Newline => ChildSeparator::HardLine,
                    HtmlChild::EmptyLine => ChildSeparator::EmptyLine,
                    _ => ChildSeparator::None,
                };
            }

            ChildSeparator::None
        }
        WhitespaceSensitivity::Ignore => {
            // In ignore mode, we can freely add line breaks
            if current.is_block() || next.is_block() {
                ChildSeparator::HardLine
            } else {
                ChildSeparator::SoftLineOrSpace
            }
        }
        WhitespaceSensitivity::Css => {
            // CSS mode: respect display values
            let current_is_text = current.is_text();
            let next_is_text = next.is_text();
            let current_is_inline = current.is_inline() || current_is_text;
            let next_is_inline = next.is_inline() || next_is_text;
            let keep_existing_separator = (current.is_comment() && next.is_text())
                || (current.is_text() && next.is_comment());

            if keep_existing_separator && let Some(existing_separator) = existing_separator {
                return match existing_separator.child {
                    HtmlChild::Whitespace => ChildSeparator::Space,
                    HtmlChild::Newline => ChildSeparator::HardLine,
                    HtmlChild::EmptyLine => ChildSeparator::EmptyLine,
                    _ => ChildSeparator::None,
                };
            }

            // Adjacent words/text always get a space between them.
            // This is because html_split_children doesn't emit whitespace nodes
            // between words on the same line - it only emits the words themselves.
            // The original source had whitespace separating them, so we need a space.
            if current_is_text && next_is_text {
                return ChildSeparator::SoftLineOrSpace;
            }

            if current.is_block() && next.is_block() {
                // Block to block: hard line break
                ChildSeparator::HardLine
            } else if current_is_inline && next_is_inline {
                // Inline to inline: soft line or space
                if current.whitespace_info.has_trailing_spaces()
                    || next.whitespace_info.has_leading_spaces()
                {
                    ChildSeparator::SoftLineOrSpace
                } else {
                    ChildSeparator::SoftLine
                }
            } else if current.is_block() && next_is_inline {
                // Block to inline: hard line break
                ChildSeparator::HardLine
            } else if current_is_inline && next.is_block() {
                // Inline to block: hard line break
                ChildSeparator::HardLine
            } else {
                // Default: soft line or space
                ChildSeparator::SoftLineOrSpace
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classified_child_whitespace() {
        let child = ClassifiedChild::new(HtmlChild::Whitespace);
        assert!(child.is_whitespace());
        assert!(!child.is_text());
        assert!(!child.is_block());
        assert!(child.display.is_none());
    }

    #[test]
    fn test_classified_child_newline() {
        let child = ClassifiedChild::new(HtmlChild::Newline);
        assert!(child.is_whitespace());
        assert!(child.is_newline());
        assert!(!child.is_empty_line());
    }

    #[test]
    fn test_classified_child_empty_line() {
        let child = ClassifiedChild::new(HtmlChild::EmptyLine);
        assert!(child.is_whitespace());
        assert!(!child.is_newline());
        assert!(child.is_empty_line());
    }

    #[test]
    fn test_classified_children_operations() {
        let children = vec![HtmlChild::Whitespace, HtmlChild::Newline];

        let mut classified = ClassifiedChildren::from_children(children);
        assert_eq!(classified.len(), 2);
        assert!(!classified.has_meaningful_text());

        // Remove trailing whitespace
        let removed = classified.pop_trailing_whitespace();
        assert!(removed.is_some());
        assert_eq!(classified.len(), 1);
    }

    #[test]
    fn test_whitespace_sensitivity_strict_mode() {
        let ctx = ClassificationContext::new(WhitespaceSensitivity::Strict);

        // Everything is sensitive in strict mode
        let whitespace_child = ClassifiedChild::new(HtmlChild::Whitespace);
        assert!(compute_leading_sensitivity(
            &ctx,
            Some(&whitespace_child),
            None
        ));
        assert!(compute_trailing_sensitivity(
            &ctx,
            Some(&whitespace_child),
            None
        ));
    }

    #[test]
    fn test_whitespace_sensitivity_ignore_mode() {
        let ctx = ClassificationContext::new(WhitespaceSensitivity::Ignore);

        // Nothing is sensitive in ignore mode
        let whitespace_child = ClassifiedChild::new(HtmlChild::Whitespace);
        assert!(!compute_leading_sensitivity(
            &ctx,
            Some(&whitespace_child),
            None
        ));
        assert!(!compute_trailing_sensitivity(
            &ctx,
            Some(&whitespace_child),
            None
        ));
    }

    // #[test]
    // fn test_separator_strict_mode() {
    //     let ctx = ClassificationContext::new(WhitespaceSensitivity::Strict);
    //     let child1 = ClassifiedChild::new(HtmlChild::Whitespace);
    //     let child2 = ClassifiedChild::new(HtmlChild::Whitespace);

    //     let sep = get_separator_between(&child1, &child2, &ctx);
    //     assert_eq!(sep, ChildSeparator::None);
    // }

    // #[test]
    // fn test_separator_ignore_mode() {
    //     let ctx = ClassificationContext::new(WhitespaceSensitivity::Ignore);
    //     let child1 = ClassifiedChild::new(HtmlChild::Whitespace);
    //     let child2 = ClassifiedChild::new(HtmlChild::Whitespace);

    //     let sep = get_separator_between(&child1, &child2, &ctx);
    //     // Neither is block, so soft line or space
    //     assert_eq!(sep, ChildSeparator::SoftLineOrSpace);
    // }

    #[test]
    fn test_classification_context_builder() {
        let ctx = ClassificationContext::new(WhitespaceSensitivity::Css)
            .with_parent_tag(Some("div"))
            .with_whitespace_preserving(false);

        assert_eq!(ctx.sensitivity, WhitespaceSensitivity::Css);
        assert_eq!(ctx.parent_tag, Some("div"));
        assert!(!ctx.is_whitespace_preserving);
    }
}
