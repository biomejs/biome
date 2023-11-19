use crate::context::TabWidth;
use crate::js::auxiliary::template_chunk_element::AnyTemplateChunkElement;
use crate::js::auxiliary::template_element::{AnyTemplateElement, TemplateElementOptions};
use crate::prelude::*;
use crate::utils::test_each_template::EachTemplateTable;
use biome_formatter::FormatRuleWithOptions;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, AnyTsTemplateElement,
    JsLanguage, JsTemplateElementList, TsTemplateElementList,
};
use biome_rowan::{declare_node_union, AstNodeListIterator, SyntaxResult};
use std::iter::FusedIterator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTemplateElementList {
    options: FormatJsTemplateElementListOptions,
}

impl FormatRuleWithOptions<JsTemplateElementList> for FormatJsTemplateElementList {
    type Options = FormatJsTemplateElementListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatRule<JsTemplateElementList> for FormatJsTemplateElementList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsTemplateElementList, f: &mut JsFormatter) -> FormatResult<()> {
        if self.options.is_test_each_pattern {
            EachTemplateTable::from(node, f)?.fmt(f)
        } else {
            AnyTemplateElementList::JsTemplateElementList(node.clone()).fmt(f)
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatJsTemplateElementListOptions {
    pub(crate) is_test_each_pattern: bool,
}

pub(crate) enum AnyTemplateElementList {
    JsTemplateElementList(JsTemplateElementList),
    TsTemplateElementList(TsTemplateElementList),
}

impl Format<JsFormatContext> for AnyTemplateElementList {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let layout = if self.is_simple(f.comments()) {
            TemplateElementLayout::SingleLine
        } else {
            TemplateElementLayout::Fit
        };

        let mut indention = TemplateElementIndention::default();
        let mut after_new_line = false;

        for element in self.elements() {
            match element {
                AnyTemplateElementOrChunk::AnyTemplateElement(element) => {
                    let options = TemplateElementOptions {
                        after_new_line,
                        indention,
                        layout,
                    };

                    match &element {
                        AnyTemplateElement::JsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                        AnyTemplateElement::TsTemplateElement(element) => {
                            element.format().with_options(options).fmt(f)?;
                        }
                    }
                }
                AnyTemplateElementOrChunk::AnyTemplateChunkElement(chunk) => {
                    match &chunk {
                        AnyTemplateChunkElement::JsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                        AnyTemplateChunkElement::TsTemplateChunkElement(chunk) => {
                            chunk.format().fmt(f)?;
                        }
                    }

                    let chunk_token = chunk.template_chunk_token()?;
                    let chunk_text = chunk_token.text();

                    let tab_width = f.options().tab_width();

                    indention = TemplateElementIndention::after_last_new_line(
                        chunk_text, tab_width, indention,
                    );
                    after_new_line = chunk_text.ends_with('\n');
                }
            }
        }

        Ok(())
    }
}

impl AnyTemplateElementList {
    /// Returns `true` for `JsTemplate` if all elements are simple expressions that should be printed on a single line.
    ///
    /// Simple expressions are:
    /// * Identifiers: `this`, `a`
    /// * Members: `a.b`, `a[b]`, `a.b[c].d`, `a.b[5]`, `a.b["test"]`
    fn is_simple(&self, comments: &JsComments) -> bool {
        match self {
            AnyTemplateElementList::JsTemplateElementList(list) => {
                if list.is_empty() {
                    return false;
                }

                let mut expression_elements = list.iter().filter_map(|element| match element {
                    AnyJsTemplateElement::JsTemplateElement(element) => Some(element),
                    _ => None,
                });

                expression_elements.all(|expression_element| {
                    match expression_element.expression() {
                        Ok(expression) => {
                            is_simple_member_expression(expression, comments).unwrap_or(false)
                        }
                        Err(_) => false,
                    }
                })
            }
            AnyTemplateElementList::TsTemplateElementList(_) => false,
        }
    }

    fn elements(&self) -> TemplateElementIterator {
        match self {
            AnyTemplateElementList::JsTemplateElementList(list) => {
                TemplateElementIterator::JsTemplateElementList(list.iter())
            }
            AnyTemplateElementList::TsTemplateElementList(list) => {
                TemplateElementIterator::TsTemplateElementList(list.iter())
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum TemplateElementLayout {
    /// Applied when all expressions are identifiers, `this`, static member expressions, or computed member expressions with number or string literals.
    /// Formats the expressions on a single line, even if their width otherwise would exceed the print width.
    SingleLine,

    /// Tries to format the expression on a single line but may break the expression if the line otherwise exceeds the print width.
    #[default]
    Fit,
}

declare_node_union! {
    AnyTemplateElementOrChunk = AnyTemplateElement | AnyTemplateChunkElement
}

fn is_simple_member_expression(
    expression: AnyJsExpression,
    comments: &JsComments,
) -> SyntaxResult<bool> {
    let mut current = expression;

    loop {
        if comments.has_comments(current.syntax()) {
            return Ok(false);
        }

        current = match current {
            AnyJsExpression::JsStaticMemberExpression(expression) => expression.object()?,
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                if matches!(
                    expression.member()?,
                    AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsStringLiteralExpression(_)
                            | AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                    ) | AnyJsExpression::JsIdentifierExpression(_)
                ) {
                    expression.object()?
                } else {
                    break;
                }
            }
            AnyJsExpression::JsIdentifierExpression(_) | AnyJsExpression::JsThisExpression(_) => {
                return Ok(true);
            }
            _ => {
                break;
            }
        }
    }

    Ok(false)
}

enum TemplateElementIterator {
    JsTemplateElementList(AstNodeListIterator<JsLanguage, AnyJsTemplateElement>),
    TsTemplateElementList(AstNodeListIterator<JsLanguage, AnyTsTemplateElement>),
}

impl Iterator for TemplateElementIterator {
    type Item = AnyTemplateElementOrChunk;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    AnyJsTemplateElement::JsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    AnyJsTemplateElement::JsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };
                Some(result)
            }
            TemplateElementIterator::TsTemplateElementList(inner) => {
                let result = match inner.next()? {
                    AnyTsTemplateElement::TsTemplateChunkElement(chunk) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateChunkElement::from(chunk))
                    }
                    AnyTsTemplateElement::TsTemplateElement(element) => {
                        AnyTemplateElementOrChunk::from(AnyTemplateElement::from(element))
                    }
                };

                Some(result)
            }
        }
    }
}

impl ExactSizeIterator for TemplateElementIterator {
    fn len(&self) -> usize {
        match self {
            TemplateElementIterator::JsTemplateElementList(inner) => inner.len(),
            TemplateElementIterator::TsTemplateElementList(inner) => inner.len(),
        }
    }
}

impl FusedIterator for TemplateElementIterator {}

/// The indention derived from a position in the source document. Consists of indention level and spaces
#[derive(Debug, Copy, Clone, Default)]
pub struct TemplateElementIndention(u32);

impl TemplateElementIndention {
    /// Returns the indention level
    pub(crate) fn level(&self, tab_width: TabWidth) -> u32 {
        self.0 / u32::from(u8::from(tab_width))
    }

    /// Returns the number of space indents on top of the indent level
    pub(crate) fn align(&self, tab_width: TabWidth) -> u8 {
        (self.0 % u32::from(u8::from(tab_width))) as u8
    }

    /// Computes the indention after the last new line character.
    fn after_last_new_line(
        text: &str,
        tab_width: TabWidth,
        previous_indention: TemplateElementIndention,
    ) -> Self {
        let by_new_line = text.rsplit_once('\n');

        let size = match by_new_line {
            None => previous_indention.0,
            Some((_, after_new_line)) => {
                let tab_width: u32 = u8::from(tab_width).into();
                let mut size: u32 = 0;

                for c in after_new_line.chars() {
                    match c {
                        '\t' => {
                            // Tabs behave in a way that they are aligned to the nearest
                            // multiple of tab_width:
                            // number of spaces -> added size
                            // 0 -> 4, 1 -> 4, 2 -> 4, 3 -> 4
                            // 4 -> 8, 5 -> 8, 6 -> 8, 7 -> 8 ..
                            // Or in other words, it clips the size to the next multiple of tab width.
                            size = size + tab_width - (size % tab_width);
                        }
                        ' ' => {
                            size += 1;
                        }
                        _ => break,
                    };
                }

                size
            }
        };

        TemplateElementIndention(size)
    }
}
