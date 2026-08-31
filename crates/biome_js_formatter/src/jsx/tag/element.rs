use crate::prelude::*;

use crate::jsx::lists::child_list::{FormatChildrenResult, FormatJsxChildList, JsxChildListLayout};
use crate::utils::jsx::{is_astro_raw_text_element, is_jsx_suppressed, is_meaningful_jsx_child};
use crate::verbatim::format_verbatim_skipped;
use biome_formatter::{CstFormatContext, FormatResult, FormatRuleWithOptions, format_args, write};
use biome_js_syntax::{
    AnyJsExpression, AnyJsxChild, JsxChildList, JsxElement, JsxExpressionChild, JsxFragment,
    JsxText,
};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug, Clone, Default)]
pub struct FormatJsxElement;

impl FormatNodeRule<JsxElement> for FormatJsxElement {
    fn fmt_fields(&self, node: &JsxElement, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsxTagWithChildren::from(node.clone()).fmt(f)
    }

    fn is_suppressed(&self, node: &JsxElement, f: &JsFormatter) -> bool {
        is_jsx_suppressed(&node.clone().into(), f.comments())
    }

    /// [`AnyJsxTagWithChildren`] prints them between the tags instead of after `</name>`.
    fn fmt_dangling_comments(&self, _: &JsxElement, _: &mut JsFormatter) -> FormatResult<()> {
        Ok(())
    }
}

declare_node_union! {
    pub(super) AnyJsxTagWithChildren = JsxElement | JsxFragment
}

impl Format<JsFormatContext> for AnyJsxTagWithChildren {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let format_opening = format_with(|f| self.fmt_opening(f));
        let format_closing = format_with(|f| self.fmt_closing(f));

        let layout = self.layout(f)?;

        match layout {
            ElementLayout::NoChildren => {
                let dangling = format_dangling_comments(self.syntax()).with_block_indent();
                write!(f, [format_opening, dangling, format_closing])
            }

            ElementLayout::RawText(text) => {
                write!(
                    f,
                    [
                        format_opening,
                        format_verbatim_skipped(text.syntax()),
                        format_closing
                    ]
                )
            }

            ElementLayout::Template(expression) => {
                write!(f, [format_opening, expression.format(), format_closing])
            }

            ElementLayout::Default => {
                let mut format_opening = format_opening.memoized();
                let opening_breaks = format_opening.inspect(f)?.will_break();

                let multiple_attributes = match self {
                    Self::JsxElement(element) => element.opening_element()?.attributes().len() > 1,
                    Self::JsxFragment(_) => false,
                };

                let list_layout = if multiple_attributes || opening_breaks {
                    JsxChildListLayout::Multiline
                } else {
                    JsxChildListLayout::BestFitting
                };

                let children = self.children();
                let format_children = FormatJsxChildList::default()
                    .with_options(list_layout)
                    .fmt_children(&children, f)?;

                match format_children {
                    FormatChildrenResult::ForceMultiline(multiline) => {
                        write!(f, [format_opening, multiline, format_closing])
                    }
                    FormatChildrenResult::BestFitting {
                        flat_children,
                        expanded_children,
                    } => {
                        let format_closing = format_closing.memoized();
                        write!(
                            f,
                            [best_fitting![
                                format_args![format_opening, flat_children, format_closing],
                                format_args![format_opening, expanded_children, format_closing]
                            ]]
                        )
                    }
                }
            }
        }
    }
}

impl AnyJsxTagWithChildren {
    fn fmt_opening(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            Self::JsxElement(element) => {
                write!(f, [element.opening_element().format()])
            }
            Self::JsxFragment(fragment) => {
                write!(f, [fragment.opening_fragment().format()])
            }
        }
    }

    fn fmt_closing(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            Self::JsxElement(element) => {
                write!(f, [element.closing_element().format()])
            }
            Self::JsxFragment(fragment) => {
                write!(f, [fragment.closing_fragment().format()])
            }
        }
    }

    fn children(&self) -> JsxChildList {
        match self {
            Self::JsxElement(element) => element.children(),
            Self::JsxFragment(fragment) => fragment.children(),
        }
    }

    fn layout(&self, f: &mut JsFormatter) -> SyntaxResult<ElementLayout> {
        use AnyJsExpression::*;
        use AnyJsxChild::*;

        let children = self.children();

        if let Self::JsxElement(element) = self
            && let Some(JsxText(text)) = children.iter().next()
            && children.len() == 1
            && f.options()
                .source_type()
                .as_embedding_kind()
                .is_astro_template()
            && is_astro_raw_text_element(&element.opening_element()?)
        {
            return Ok(ElementLayout::RawText(text));
        }

        // Text carrying no meaning must not sway the layout, or the passes disagree.
        let mut meaningless_texts = Vec::new();
        let mut meaningful_children = Vec::new();
        for child in &children {
            if is_meaningful_jsx_child(&child) {
                meaningful_children.push(child);
            } else if let JsxText(text) = child {
                meaningless_texts.push(text);
            }
        }

        let layout = match meaningful_children.as_slice() {
            [] => {
                for text in &meaningless_texts {
                    // Text nodes can't have suppressions
                    f.context_mut()
                        .comments()
                        .mark_suppression_checked(text.syntax());
                    f.state_mut().track_token(&text.value_token()?);
                }

                ElementLayout::NoChildren
            }
            [JsxExpressionChild(expression)] if meaningless_texts.is_empty() => {
                match expression.expression() {
                    Some(JsTemplateExpression(_)) => ElementLayout::Template(expression.clone()),
                    _ => ElementLayout::Default,
                }
            }
            _ => ElementLayout::Default,
        };

        Ok(layout)
    }
}

#[derive(Debug, Clone)]
enum ElementLayout {
    /// Empty Tag with no children or contains no meaningful text.
    NoChildren,

    /// The children of an Astro raw-text element, printed byte for byte.
    ///
    /// ```astro
    /// <div is:raw>{ this   is   not   JSX }</div>
    /// ```
    RawText(JsxText),

    /// Prefer breaking the template if it is the only child of the element
    /// ```javascript
    /// <div>{`A Long Template String That uses ${
    ///   5 + 4
    /// } that will eventually break across multiple lines ${(40 / 3) * 45}`}</div>;
    /// ```
    ///
    /// instead of
    ///
    /// ```javascript
    /// <div>
    ///   {`A Long Template String That uses ${
    ///     5 + 4
    ///   } that will eventually break across multiple lines ${(40 / 3) * 45}`}
    /// </div>;
    /// ```
    Template(JsxExpressionChild),

    /// Default layout used for all elements that have children and [ElementLayout::Template] does not apply.
    ///
    /// ```javascript
    ///<Element2>
    ///   Some more content
    ///   <Sub />
    ///   <Sub />
    ///   <Sub />
    /// </Element2>;
    /// ```
    Default,
}
