use crate::html::lists::element_list::{FormatHtmlElementListOptions, HtmlChildListLayout};
use crate::utils::metadata::HTML_INLINE_TAGS;
use crate::{
    html::lists::element_list::{FormatChildrenResult, FormatHtmlElementList},
    prelude::*,
};
use biome_formatter::{format_args, write, FormatContext, FormatRuleWithOptions};
use biome_html_syntax::{HtmlElement, HtmlElementFields};

use super::{
    closing_element::{FormatHtmlClosingElement, FormatHtmlClosingElementOptions},
    opening_element::{FormatHtmlOpeningElement, FormatHtmlOpeningElementOptions},
};

/// `pre` tags are "preformatted", so we should not format the content inside them. <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre>
/// We ignore the `script` and `style` tags as well, since embedded language parsing/formatting is not yet implemented.
const HTML_VERBATIM_TAGS: &[&str] = &["script", "style", "pre"];

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatHtmlElement;

impl FormatNodeRule<HtmlElement> for FormatHtmlElement {
    fn fmt_fields(&self, node: &HtmlElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let whitespace_sensitivity = f.context().options().whitespace_sensitivity();
        let HtmlElementFields {
            opening_element,
            children,
            closing_element,
        } = node.as_fields();

        let tag_name = opening_element
            .clone()
            .and_then(|e| e.name())
            .map(|e| e.trim_trivia().map(|t| t.to_string()).unwrap_or_default())
            .unwrap_or_default();

        let should_be_verbatim = HTML_VERBATIM_TAGS
            .iter()
            .any(|tag| tag_name.eq_ignore_ascii_case(tag));
        let is_inline_tag = HTML_INLINE_TAGS
            .iter()
            .any(|tag| tag_name.eq_ignore_ascii_case(tag));

        let content_has_leading_whitespace = children
            .syntax()
            .first_token()
            .is_some_and(|tok| tok.has_leading_whitespace_or_newline())
            || opening_element
                .as_ref()
                .ok()
                .and_then(|elem| {
                    elem.r_angle_token()
                        .ok()
                        .map(|tok| tok.has_trailing_whitespace())
                })
                .unwrap_or_default();
        let content_has_trailing_whitespace = children
            .syntax()
            .last_token()
            .is_some_and(|tok| tok.has_trailing_whitespace())
            || closing_element
                .as_ref()
                .ok()
                .and_then(|elem| {
                    elem.l_angle_token()
                        .ok()
                        .map(|tok| tok.has_leading_whitespace_or_newline())
                })
                .unwrap_or_default();

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
        // This formatter is resposible for making the determination of whether or not
        // to borrow, while the child formatters are responsible for actually printing
        // the tokens. `HtmlElementList` prints them if they are borrowed, otherwise
        // they are printed by their original formatter.
        let should_borrow_opening_r_angle = whitespace_sensitivity.is_strict()
            && is_inline_tag
            && !children.is_empty()
            && !content_has_leading_whitespace;
        let should_borrow_closing_tag = whitespace_sensitivity.is_strict()
            && is_inline_tag
            && !children.is_empty()
            && !content_has_trailing_whitespace;

        let borrowed_r_angle = if should_borrow_opening_r_angle {
            opening_element
                .as_ref()
                .ok()
                .and_then(|elem| elem.r_angle_token().ok())
        } else {
            None
        };
        let borrowed_closing_tag = if should_borrow_closing_tag {
            closing_element.clone().ok()
        } else {
            None
        };

        FormatNodeRule::fmt(
            &FormatHtmlOpeningElement::default().with_options(FormatHtmlOpeningElementOptions {
                r_angle_is_borrowed: borrowed_r_angle.is_some(),
            }),
            &opening_element?,
            f,
        )?;
        if should_be_verbatim {
            format_verbatim_skipped(children.syntax()).fmt(f)?;
            write!(f, [hard_line_break()])?;
        } else {
            let format_children = FormatHtmlElementList::default()
                .with_options(FormatHtmlElementListOptions {
                    layout: HtmlChildListLayout::BestFitting,
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
                    write!(
                        f,
                        [best_fitting![
                            format_args![flat_children],
                            format_args![expanded_children]
                        ]]
                    )?;
                }
            }
        }
        FormatNodeRule::fmt(
            &FormatHtmlClosingElement::default().with_options(FormatHtmlClosingElementOptions {
                tag_borrowed: should_borrow_closing_tag,
            }),
            &closing_element?,
            f,
        )?;

        Ok(())
    }
}
