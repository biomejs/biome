use crate::prelude::*;
use crate::utils::scss_each::format_after_each_in;
use crate::utils::scss_expression::single_expression_item;
use biome_css_syntax::{
    AnyScssExpressionItem, CssSyntaxNode, CssSyntaxToken, ScssEachBindingList, ScssEachHeader,
    ScssEachHeaderFields, ScssExpression, ScssListExpression,
};
use biome_formatter::{FormatOwnedWithRule, FormatRule, format_args, write};
use biome_rowan::AstSeparatedList;

/// Formats the `@each $item in ...` header.
///
/// Direct comma lists use a custom fill layout so `@each $a, $b in x, y`
/// wraps as one sequence. Other iterables, such as `$items` or `(a: b)`, use
/// their own formatter.
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachHeader;
impl FormatNodeRule<ScssEachHeader> for FormatScssEachHeader {
    fn fmt_fields(&self, node: &ScssEachHeader, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachHeaderFields {
            bindings,
            in_token,
            iterable,
        } = node.as_fields();

        let in_token = in_token?;
        let iterable = iterable?;

        // Prettier treats `@each $a, $b in x, y` as one fill sequence across
        // bindings, `in`, and list items. Map iterables stay on the normal
        // expression path so the map owns its own layout.
        if bindings.len() > 0 && is_direct_list_iterable(&iterable) {
            return write!(
                f,
                [FormatOwnedWithRule::new(
                    iterable,
                    FormatScssEachIterable::new(&bindings, &in_token)
                )]
            );
        }

        write!(
            f,
            [
                bindings.format(),
                space(),
                in_token.format(),
                format_after_each_in(&in_token, iterable.syntax()),
                iterable.format()
            ]
        )
    }
}

/// Formats the direct list in `@each $x in a, b` together with the header.
struct FormatScssEachIterable<'a> {
    bindings: &'a ScssEachBindingList,
    in_token: &'a CssSyntaxToken,
}

impl<'a> FormatScssEachIterable<'a> {
    fn new(bindings: &'a ScssEachBindingList, in_token: &'a CssSyntaxToken) -> Self {
        Self { bindings, in_token }
    }
}

impl FormatRule<ScssExpression> for FormatScssEachIterable<'_> {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<ScssExpression>::fmt(self, node, f)
    }
}

impl FormatNodeRule<ScssExpression> for FormatScssEachIterable<'_> {
    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        match single_expression_item(node) {
            Some(AnyScssExpressionItem::ScssListExpression(list)) => write!(
                f,
                [FormatOwnedWithRule::new(
                    list,
                    FormatScssEachIterableList::new(self.bindings, self.in_token, node.syntax())
                )]
            ),
            _ => Ok(()),
        }
    }

    fn fmt_leading_comments(
        &self,
        _node: &ScssExpression,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Iterable comments are printed after `in`.
        Ok(())
    }
}

/// Formats `@each $x in a, b` so bindings and list items share one fill group.
struct FormatScssEachIterableList<'a> {
    bindings: &'a ScssEachBindingList,
    in_token: &'a CssSyntaxToken,
    comment_owner: &'a CssSyntaxNode,
}

impl<'a> FormatScssEachIterableList<'a> {
    fn new(
        bindings: &'a ScssEachBindingList,
        in_token: &'a CssSyntaxToken,
        comment_owner: &'a CssSyntaxNode,
    ) -> Self {
        Self {
            bindings,
            in_token,
            comment_owner,
        }
    }
}

impl FormatRule<ScssListExpression> for FormatScssEachIterableList<'_> {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<ScssListExpression>::fmt(self, node, f)
    }
}

impl FormatNodeRule<ScssListExpression> for FormatScssEachIterableList<'_> {
    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        write_each_list_header(self.bindings, self.in_token, self.comment_owner, node, f)
    }

    fn fmt_leading_comments(
        &self,
        _node: &ScssListExpression,
        _f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // The `@each` fill group prints list comments after `in`.
        Ok(())
    }
}

fn is_direct_list_iterable(iterable: &ScssExpression) -> bool {
    single_expression_item(iterable).is_some_and(|item| item.as_scss_list_expression().is_some())
}

/// Formats `@each $x, $y in (a, b), (c, d)` as one fill group.
fn write_each_list_header(
    bindings: &ScssEachBindingList,
    in_token: &CssSyntaxToken,
    comment_owner: &CssSyntaxNode,
    iterable: &ScssListExpression,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let elements = iterable.elements();
    let mut iterable_elements = elements.elements();
    let Some(first_iterable) = iterable_elements.next() else {
        return write!(
            f,
            [
                bindings.format(),
                soft_line_break_or_space(),
                in_token.format()
            ]
        );
    };
    let first_iterable_node = first_iterable.node()?;
    let first_iterable_separator = first_iterable.trailing_separator()?;

    let separator = soft_line_break_or_space();
    let has_iterable_line_comment = f
        .comments()
        .leading_comments(comment_owner)
        .iter()
        .any(|comment| comment.kind().is_line());
    let mut fill = f.fill();
    let binding_count = bindings.len();
    let mut break_after_first_iterable = false;

    for (index, binding) in bindings.elements().enumerate() {
        if index + 1 == binding_count {
            // The final binding owns `in` and the first iterable item.
            if has_iterable_line_comment {
                // `@each $x in // list\n a, b` must break before `a`.
                fill.entry(
                    &separator,
                    &group(&indent(&format_args![
                        binding.node()?.format(),
                        hard_line_break(),
                        in_token.format(),
                        space(),
                        format_leading_comments(comment_owner)
                    ])),
                );

                fill.entry(
                    &separator,
                    &format_args![
                        first_iterable_node.format(),
                        first_iterable_separator.format()
                    ],
                );
                break_after_first_iterable = true;
                continue;
            }

            fill.entry(
                &separator,
                &group(&indent(&format_args![
                    binding.node()?.format(),
                    soft_line_break_or_space(),
                    in_token.format(),
                    format_after_each_in(in_token, first_iterable_node.syntax()),
                    format_leading_comments(comment_owner),
                    format_args![
                        first_iterable_node.format(),
                        first_iterable_separator.format()
                    ]
                ])),
            );
        } else {
            fill.entry(
                &separator,
                &format_args![
                    binding.node()?.format(),
                    binding.trailing_separator()?.format()
                ],
            );
        }
    }

    for iterable in iterable_elements {
        let item_separator = if break_after_first_iterable {
            break_after_first_iterable = false;
            hard_line_break()
        } else {
            separator
        };

        fill.entry(
            &item_separator,
            &format_args![
                iterable.node()?.format(),
                iterable.trailing_separator()?.format()
            ],
        );
    }

    fill.finish()
}
