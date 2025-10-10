use crate::html::lists::element_list::{FormatHtmlElementListOptions, HtmlChildListLayout};
use crate::utils::metadata::is_element_whitespace_sensitive;
use crate::{
    html::lists::element_list::{FormatChildrenResult, FormatHtmlElementList},
    prelude::*,
};
use biome_formatter::{
    CstFormatContext, FormatRefWithRule, FormatRuleWithOptions, format_args, write,
};
use biome_html_syntax::{HtmlElement, HtmlElementFields};

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
        let HtmlElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        let closing_element = closing_element?;
        let opening_element = opening_element?;
        let tag_name = opening_element.name()?;
        let is_whitespace_sensitive = is_element_whitespace_sensitive(f, &tag_name);
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
        let should_borrow_opening_r_angle = is_whitespace_sensitive
            && !children.is_empty()
            && !content_has_leading_whitespace
            && !should_be_verbatim
            && !should_format_embedded_nodes;
        let should_borrow_closing_tag = is_whitespace_sensitive
            && !children.is_empty()
            && !content_has_trailing_whitespace
            && !should_be_verbatim
            && !should_format_embedded_nodes;

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
            let format_children = FormatHtmlElementList::default()
                .with_options(FormatHtmlElementListOptions {
                    layout: HtmlChildListLayout::BestFitting,
                    is_element_whitespace_sensitive: is_whitespace_sensitive,
                    borrowed_r_angle,
                    borrowed_closing_tag,
                })
                .fmt_children(&children, f)?;
            match format_children {
                FormatChildrenResult::ForceMultiline(multiline) => {
                    write!(f, [multiline])?;
                }
                FormatChildrenResult::BestFitting {
                    flat_children,
                    expanded_children,
                } => {
                    let expanded_children = expanded_children.memoized();
                    write!(
                        f,
                        [
                            // If the attribute group breaks, prettier always breaks the children as well.
                            &if_group_breaks(&expanded_children).with_group_id(Some(attr_group_id)),
                            // If the attribute group does NOT break, print whatever fits best for the children.
                            &if_group_fits_on_line(&best_fitting![
                                format_args![flat_children],
                                format_args![expanded_children],
                            ])
                            .with_group_id(Some(attr_group_id)),
                        ]
                    )?;
                }
            }
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
