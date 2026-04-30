use crate::prelude::*;
use crate::utils::scss_expression::single_expression_item;
use biome_css_syntax::{
    CssLanguage, CssSyntaxNode, CssSyntaxToken, ScssEachAtRule, ScssEachAtRuleFields,
    ScssEachBindingList, ScssExpression, ScssListExpression, ScssListExpressionElement,
    ScssVariable,
};
use biome_formatter::{format_args, write};
use biome_rowan::{AstNode, AstSeparatedElement, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachAtRule;

impl FormatNodeRule<ScssEachAtRule> for FormatScssEachAtRule {
    fn fmt_fields(&self, node: &ScssEachAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachAtRuleFields {
            each_token,
            bindings,
            in_token,
            iterable,
            block,
        } = node.as_fields();

        if let (Ok(in_token), Ok(iterable)) = (in_token.as_ref(), iterable.as_ref())
            && let Some(iterable_list) = each_iterable_list(iterable)
        {
            let has_iterable_suppression =
                has_suppression(iterable.syntax(), f) || has_suppression(iterable_list.syntax(), f);

            if !has_iterable_suppression {
                return write!(
                    f,
                    [
                        each_token.format(),
                        group(&format_args![
                            space(),
                            indent(&group(&format_with(|f| {
                                write_each_list_header(&bindings, in_token, &iterable_list, f)
                            }))),
                            soft_line_break_or_space()
                        ]),
                        block.format()
                    ]
                );
            }
        }

        write!(
            f,
            [
                each_token.format(),
                space(),
                bindings.format(),
                space(),
                in_token.format(),
                space(),
                iterable.format(),
                space(),
                block.format()
            ]
        )
    }
}

fn each_iterable_list(iterable: &ScssExpression) -> Option<ScssListExpression> {
    single_expression_item(iterable).and_then(|item| item.as_scss_list_expression().cloned())
}

fn has_suppression(node: &CssSyntaxNode, f: &CssFormatter) -> bool {
    f.comments().is_suppressed(node) || f.comments().is_global_suppressed(node)
}

fn write_each_list_header(
    bindings: &ScssEachBindingList,
    in_token: &CssSyntaxToken,
    iterable: &ScssListExpression,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let mut binding_elements: Vec<_> = bindings.elements().collect();
    let Some(last_binding) = binding_elements.pop() else {
        return Ok(());
    };

    let elements = iterable.elements();
    let mut iterable_elements = elements.elements();
    let Some(first_iterable) = iterable_elements.next() else {
        return write!(
            f,
            [
                bindings.format(),
                space(),
                in_token.format(),
                space(),
                iterable.format()
            ]
        );
    };

    let separator = soft_line_break_or_space();
    let mut fill = f.fill();

    for binding in binding_elements {
        fill.entry(
            &separator,
            &format_with(|f| write_separated_binding(&binding, f)),
        );
    }

    fill.entry(
        &separator,
        &group(&indent(&format_with(|f| {
            write_last_binding_with_first_iterable(&last_binding, in_token, &first_iterable, f)
        }))),
    );

    for iterable in iterable_elements {
        fill.entry(
            &separator,
            &format_with(|f| write_separated_iterable(&iterable, f)),
        );
    }

    fill.finish()
}

fn write_last_binding_with_first_iterable(
    binding: &AstSeparatedElement<CssLanguage, ScssVariable>,
    in_token: &CssSyntaxToken,
    iterable: &AstSeparatedElement<CssLanguage, ScssListExpressionElement>,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    write!(
        f,
        [
            binding.node()?.format(),
            soft_line_break_or_space(),
            in_token.format(),
            space(),
            format_with(|f| write_separated_iterable(iterable, f))
        ]
    )
}

fn write_separated_binding(
    binding: &AstSeparatedElement<CssLanguage, ScssVariable>,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    write!(f, [binding.node()?.format()])?;
    write_separator(binding.trailing_separator()?, f)
}

fn write_separated_iterable(
    iterable: &AstSeparatedElement<CssLanguage, ScssListExpressionElement>,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    write!(f, [iterable.node()?.format()])?;
    write_separator(iterable.trailing_separator()?, f)
}

fn write_separator(separator: Option<&CssSyntaxToken>, f: &mut CssFormatter) -> FormatResult<()> {
    if let Some(separator) = separator {
        write!(f, [separator.format()])
    } else {
        Ok(())
    }
}
