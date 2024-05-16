use crate::prelude::*;
use biome_formatter::prelude::tag::Tag;
use biome_formatter::{
    format_args, write, CstFormatContext, FormatRuleWithOptions, RemoveSoftLinesBuffer,
};

use crate::context::TabWidth;
use crate::js::expressions::array_expression::FormatJsArrayExpressionOptions;
use crate::js::lists::template_element_list::TemplateElementIndention;
use biome_js_syntax::{
    AnyJsExpression, JsSyntaxNode, JsSyntaxToken, JsTemplateElement, TsTemplateElement,
};
use biome_rowan::{declare_node_union, AstNode, NodeOrToken, SyntaxResult};

enum TemplateElementLayout {
    /// Tries to format the expression on a single line regardless of the print width.
    SingleLine,

    /// Tries to format the expression on a single line but may break the expression if the line otherwise exceeds the print width.
    Fit,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTemplateElement {
    options: TemplateElementOptions,
}

impl FormatRuleWithOptions<JsTemplateElement> for FormatJsTemplateElement {
    type Options = TemplateElementOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<JsTemplateElement> for FormatJsTemplateElement {
    fn fmt_fields(
        &self,
        node: &JsTemplateElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        let element = AnyTemplateElement::from(node.clone());

        FormatTemplateElement::new(element, self.options).fmt(formatter)
    }
}

declare_node_union! {
    pub(crate) AnyTemplateElement = JsTemplateElement | TsTemplateElement
}

#[derive(Debug, Copy, Clone, Default)]
pub struct TemplateElementOptions {
    /// The indention to use for this element
    pub(crate) indention: TemplateElementIndention,

    /// Does the last template chunk (text element) end with a new line?
    pub(crate) after_new_line: bool,
}

pub(crate) struct FormatTemplateElement {
    element: AnyTemplateElement,
    options: TemplateElementOptions,
}

impl FormatTemplateElement {
    pub(crate) fn new(element: AnyTemplateElement, options: TemplateElementOptions) -> Self {
        Self { element, options }
    }
}

impl Format<JsFormatContext> for FormatTemplateElement {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let format_expression = format_with(|f| match &self.element {
            AnyTemplateElement::JsTemplateElement(template) => match template.expression()? {
                AnyJsExpression::JsArrayExpression(expression) => {
                    let option = FormatJsArrayExpressionOptions {
                        is_force_flat_mode: true,
                    };

                    write!(f, [expression.format().with_options(option)])
                }
                expression => write!(f, [expression.format()]),
            },
            AnyTemplateElement::TsTemplateElement(template) => {
                write!(f, [template.ty().format()])
            }
        });

        let interned_expression = f.intern(&format_expression)?;

        let layout = if !self.element.has_new_line_in_range() {
            let will_break = if let Some(element) = &interned_expression {
                element.will_break()
            } else {
                false
            };

            // make sure the expression won't break to prevent reformat issue
            if will_break {
                TemplateElementLayout::Fit
            } else {
                TemplateElementLayout::SingleLine
            }
        } else {
            TemplateElementLayout::Fit
        };

        let format_inner = format_with(|f: &mut JsFormatter| match layout {
            TemplateElementLayout::SingleLine => {
                let mut buffer = RemoveSoftLinesBuffer::new(f);

                match &interned_expression {
                    Some(element) => buffer.write_element(element.clone()),
                    None => Ok(()),
                }
            }
            TemplateElementLayout::Fit => {
                use AnyJsExpression::*;

                let expression = self.element.expression();

                // It's preferred to break after/before `${` and `}` rather than breaking in the
                // middle of some expressions.
                let indent = f
                    .context()
                    .comments()
                    .has_comments(&self.element.inner_syntax()?)
                    || matches!(
                        expression,
                        Some(
                            JsStaticMemberExpression(_)
                                | JsComputedMemberExpression(_)
                                | JsConditionalExpression(_)
                                | JsSequenceExpression(_)
                                | TsAsExpression(_)
                                | TsSatisfiesExpression(_)
                                | JsBinaryExpression(_)
                                | JsLogicalExpression(_)
                                | JsInstanceofExpression(_)
                                | JsInExpression(_)
                                | JsIdentifierExpression(_)
                        )
                    );

                match &interned_expression {
                    Some(element) if indent => {
                        write!(
                            f,
                            [soft_block_indent(&format_with(
                                |f| f.write_element(element.clone())
                            ))]
                        )
                    }
                    Some(element) => f.write_element(element.clone()),
                    None => Ok(()),
                }
            }
        });

        let format_indented = format_with(|f: &mut JsFormatter| {
            if self.options.after_new_line {
                write!(f, [dedent_to_root(&format_inner)])
            } else {
                write_with_indention(
                    &format_inner,
                    self.options.indention,
                    f.options().tab_width(),
                    f,
                )
            }
        });

        write!(
            f,
            [group(&format_args![
                self.element.dollar_curly_token().format(),
                format_indented,
                line_suffix_boundary(),
                self.element.r_curly_token().format()
            ])]
        )
    }
}

impl AnyTemplateElement {
    fn dollar_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.dollar_curly_token(),
            AnyTemplateElement::TsTemplateElement(template) => template.dollar_curly_token(),
        }
    }

    fn inner_syntax(&self) -> SyntaxResult<JsSyntaxNode> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => {
                template.expression().map(AstNode::into_syntax)
            }
            AnyTemplateElement::TsTemplateElement(template) => {
                template.ty().map(AstNode::into_syntax)
            }
        }
    }

    fn expression(&self) -> Option<AnyJsExpression> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.expression().ok(),
            AnyTemplateElement::TsTemplateElement(_) => None,
        }
    }

    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyTemplateElement::JsTemplateElement(template) => template.r_curly_token(),
            AnyTemplateElement::TsTemplateElement(template) => template.r_curly_token(),
        }
    }

    fn has_new_line_in_range(&self) -> bool {
        fn has_new_line_in_node(node: &JsSyntaxNode) -> bool {
            node.children_with_tokens().any(|child| match child {
                NodeOrToken::Token(token) => token
                    // no need to check for trailing trivia as it's not possible to have a new line
                    .leading_trivia()
                    .pieces()
                    .any(|trivia| trivia.is_newline()),
                NodeOrToken::Node(node) => has_new_line_in_node(&node),
            })
        }
        has_new_line_in_node(self.syntax())
    }
}

/// Writes `content` with the specified `indention`.
fn write_with_indention<Content>(
    content: &Content,
    indention: TemplateElementIndention,
    tab_width: TabWidth,
    f: &mut JsFormatter,
) -> FormatResult<()>
where
    Content: Format<JsFormatContext>,
{
    let level = indention.level(tab_width);
    let spaces = indention.align(tab_width);

    if level == 0 && spaces == 0 {
        return write!(f, [content]);
    }

    // Adds as many nested `indent` elements until it reaches the desired indention level.
    let format_indented = format_with(|f| {
        for _ in 0..level {
            f.write_element(FormatElement::Tag(Tag::StartIndent))?;
        }

        write!(f, [content])?;

        for _ in 0..level {
            f.write_element(FormatElement::Tag(Tag::EndIndent))?;
        }

        Ok(())
    });

    // Adds any necessary `align` for spaces not covered by indent level.
    let format_aligned = format_with(|f| {
        if spaces == 0 {
            write!(f, [format_indented])
        } else {
            write!(f, [align(spaces, &format_indented)])
        }
    });

    write!(f, [dedent_to_root(&format_aligned)])
}
