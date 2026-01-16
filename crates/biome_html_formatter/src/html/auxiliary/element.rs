use crate::html::lists::element_list::{FormatHtmlElementListOptions, HtmlChildListLayout};
use crate::utils::css_display::{CssDisplay, get_css_display, get_css_display_from_tag};
use crate::verbatim::{format_html_leading_comments, format_html_leading_comments_for_block};
use crate::{html::lists::element_list::FormatHtmlElementList, prelude::*};
use biome_formatter::{CstFormatContext, FormatRefWithRule, FormatRuleWithOptions, write};
use biome_html_syntax::{
    HtmlElement, HtmlElementFields, HtmlElementList, HtmlRoot, HtmlSelfClosingElement,
};
use biome_string_case::StrLikeExtension;

use super::{
    closing_element::{FormatHtmlClosingElement, FormatHtmlClosingElementOptions},
    opening_element::{FormatHtmlOpeningElement, FormatHtmlOpeningElementOptions},
};

/// `pre` tags are "preformatted", so we should not format the content inside them. <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre>
/// We ignore the `script` and `style` tags as well, since embedded language parsing/formatting is not yet implemented.
///
const HTML_VERBATIM_TAGS: &[&str] = &["script", "style", "pre"];

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElement;

impl FormatNodeRule<HtmlElement> for FormatHtmlElement {
    fn fmt_fields(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let formatted_element = format_with(|f| self.fmt_inner(node, f));

        let surrounding_group_id = f.group_id("element");
        write!(
            f,
            [group(&formatted_element).with_group_id(Some(surrounding_group_id))]
        )?;

        Ok(())
    }

    fn fmt_leading_comments(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        // For block elements, use hard line break after leading comments even if
        // there's no newline in the source. This ensures proper formatting like:
        // <!-- comment -->
        // <div>...</div>
        // Instead of:
        // <!-- comment --> <div>...</div>
        let css_display = node
            .opening_element()
            .ok()
            .and_then(|opening| opening.name().ok())
            .map(|tag| get_css_display_from_tag(&tag))
            .unwrap_or(CssDisplay::Block);

        if css_display.is_block_like() {
            format_html_leading_comments_for_block(node.syntax()).fmt(f)
        } else {
            format_html_leading_comments(node.syntax()).fmt(f)
        }
    }

    fn fmt_trailing_comments(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        // If there is leading whitespace before a leading comment, we need to preserve it because it's probably indentation.
        // See prettier test case: crates/biome_html_formatter/tests/specs/prettier/html/comments/hidden.html
        // The current implementation for `biome_formatter::FormatTrailingComments` actually has a ton of js specific behavior that we don't want in the html formatter.

        let comments = f.context().comments().clone();
        let trailing_comments = comments.trailing_comments(node.syntax());
        for comment in trailing_comments {
            let format_comment = FormatRefWithRule::new(
                comment,
                <HtmlFormatContext as CstFormatContext>::CommentRule::default(),
            );
            match comment.lines_before() {
                0 => {}
                1 => write!(f, [hard_line_break()])?,
                _ => write!(f, [empty_line()])?,
            }
            write!(f, [format_comment])?;
            comment.mark_formatted();
        }

        Ok(())
    }
}

impl FormatHtmlElement {
    fn fmt_inner(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let HtmlElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        let should_force_break_content = should_force_break_content(node);

        let closing_element = closing_element?;
        let opening_element = opening_element?;
        let tag_name = opening_element.name()?;
        let css_display = get_css_display_from_tag(&tag_name);
        let is_element_internally_whitespace_sensitive =
            css_display.is_internally_whitespace_sensitive(f);
        let is_root_element_list = node
            .syntax()
            .ancestors()
            .nth(2)
            .is_some_and(|ancestor| HtmlRoot::can_cast(ancestor.kind()));
        // If `<template>` is at the root level, force multiline formatting of its children.
        let is_template_element = tag_name
            .token_text_trimmed()
            .is_some_and(|tt| tt.to_ascii_lowercase_cow() == "template");

        let tag_name = tag_name
            .trim_trivia()
            .map(|t| t.value_token())
            .transpose()?;

        let should_be_verbatim = HTML_VERBATIM_TAGS.iter().any(|tag| {
            tag_name
                .as_ref()
                .is_some_and(|tag_name| tag_name.text().eq_ignore_ascii_case(tag))
        });

        let should_format_embedded_nodes = if f.context().should_delegate_fmt_embedded_nodes() {
            // Only delegate for supported <script> or <style> content
            node.is_supported_script_tag() || node.is_style_tag()
        } else {
            false
        };

        let content_has_leading_whitespace = children
            .syntax()
            .first_token()
            .is_some_and(|tok| tok.has_leading_whitespace_or_newline())
            || opening_element
                .r_angle_token()
                .ok()
                .is_some_and(|tok| tok.has_trailing_whitespace());
        let content_has_trailing_whitespace = children
            .syntax()
            .last_token()
            .is_some_and(|tok| tok.has_trailing_whitespace())
            || closing_element
                .l_angle_token()
                .ok()
                .is_some_and(|tok| tok.has_leading_whitespace_or_newline());

        // "Borrowing" in this context refers to tokens in nodes that would normally be
        // formatted by that node's formatter, but are instead formatted by a sibling
        // formatter. In this case, the opening tag's `>` and the closing tag can be
        // borrowed by the children of the element.
        //
        // This is necessary because of HTML's whitespace sensitivity. As a consequence
        // of this, the formatter must format blocks that look like this (assume a small line width, <20):
        // ```html
        // <span>really long content</span>
        // ```
        // as this, where the content hugs the tags:
        // ```html
        // <span
        //    >really long content</span
        // >
        // ```
        //
        // This formatter is responsible for making the determination of whether or not
        // to borrow, while the child formatters are responsible for actually printing
        // the tokens. `HtmlElementList` prints them if they are borrowed, otherwise
        // they are printed by their original formatter.
        //
        // Elements that force break children (like `select`, `ul`, `ol`, table elements)
        // should NOT borrow tokens because their children are always multiline.
        let forces_break_children =
            should_force_break_content || (is_root_element_list && is_template_element);

        let should_borrow_opening_r_angle = is_element_internally_whitespace_sensitive
            && !children.is_empty()
            && !content_has_leading_whitespace
            && !should_be_verbatim
            && !should_format_embedded_nodes
            && !forces_break_children;
        let should_borrow_closing_tag = is_element_internally_whitespace_sensitive
            && !children.is_empty()
            && !content_has_trailing_whitespace
            && !should_be_verbatim
            && !should_format_embedded_nodes
            && !forces_break_children;

        let borrowed_r_angle = if should_borrow_opening_r_angle {
            opening_element.r_angle_token().ok()
        } else {
            None
        };
        let borrowed_closing_tag = if should_borrow_closing_tag {
            Some(closing_element.clone())
        } else {
            None
        };

        let attr_group_id = f.group_id("element-attr-group-id");
        FormatNodeRule::fmt(
            &FormatHtmlOpeningElement::default().with_options(FormatHtmlOpeningElementOptions {
                r_angle_is_borrowed: borrowed_r_angle.is_some(),
                attr_group_id,
            }),
            &opening_element,
            f,
        )?;
        // The order here is important. First, we must check if we can delegate the formatting
        // of embedded nodes, then we check if we should format them verbatim.
        if should_format_embedded_nodes {
            write!(f, [children.format()])?;
        } else if should_be_verbatim {
            write!(f, [&format_html_verbatim_node(children.syntax())])?;
        } else {
            // Use BestFitting layout to allow the formatter to choose between
            // flat and expanded versions. The `if_group_breaks`/`if_group_fits_on_line`
            // logic below will handle breaking children when attributes break.
            // The layout is only forced to Multiline when children contain block elements.
            let layout = if forces_break_children {
                HtmlChildListLayout::Multiline
            } else {
                HtmlChildListLayout::BestFitting
            };
            FormatHtmlElementList::default()
                .with_options(FormatHtmlElementListOptions {
                    layout,
                    is_container_whitespace_sensitive: is_element_internally_whitespace_sensitive,
                    borrowed_r_angle,
                    borrowed_closing_tag,
                    opening_tag_group: Some(attr_group_id),
                })
                .fmt(&children, f)?;
        }
        FormatNodeRule::fmt(
            &FormatHtmlClosingElement::default().with_options(FormatHtmlClosingElementOptions {
                tag_borrowed: should_borrow_closing_tag,
            }),
            &closing_element,
            f,
        )?;

        Ok(())
    }
}

/// Determines if an element should force line breaks between all its children.
///
/// Elements that force break children should NOT borrow tokens because their
/// children are always formatted on multiple lines, not inline.
///
/// This is equivalent to Prettier's `function forceBreakChildren()`.
///
/// Prettier source: src/language-html/utilities/index.js:271-278
fn should_force_break_children(tag_name: &str) -> bool {
    let tag_lower = tag_name.to_ascii_lowercase_cow();

    // These elements always break children
    if matches!(tag_lower.as_ref(), "html" | "head" | "ul" | "ol" | "select") {
        return true;
    }

    // Table-related elements (except table-cell) break children
    let display = get_css_display(&tag_lower);
    display.is_table_like() && !matches!(display, CssDisplay::TableCell)
}

/// Determines if the content of an element should be forcefully broken into multiple lines.
///
/// This is equivalent to Prettier's `function forceBreakContent()`.
fn should_force_break_content(node: &HtmlElement) -> bool {
    let Some(tag_name) = node.tag_name() else {
        return false;
    };
    if (should_force_break_children(&tag_name)) {
        return true;
    }

    // prettier also considers `<script>` and `<style>` here, but we handle those elsewhere.
    // if its a `<body>` or if the grandchildren contain non-text nodes
    if !node.children().is_empty() && tag_name.eq_ignore_ascii_case("body")
        || node.children().iter().any(|child| {
            if let Some(element) = child.as_html_element() {
                has_non_text_child(&element.children())
            } else {
                false
            }
        })
    {
        return true;
    }

    // at this point, prettier would do some additional checks related to whitespace sensitivity,
    // but I'm pretty sure we handle that elsewhere. Remains to be seen.

    false
}

fn has_non_text_child(node: &HtmlElementList) -> bool {
    node.iter().any(|child| {
        HtmlElement::can_cast(child.syntax().kind())
            || HtmlSelfClosingElement::can_cast(child.syntax().kind())
    })
}
