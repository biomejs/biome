use crate::prelude::*;
use biome_css_syntax::{CssSyntaxToken, ScssEachBindingList, ScssListExpression};
use biome_formatter::{FormatRule, format_args, write};
use biome_rowan::AstSeparatedList;

/// Formats `@each $x in a, b` so bindings and list items share one fill group.
pub(super) struct FormatScssEachIterableList {
    bindings: ScssEachBindingList,
    in_token: CssSyntaxToken,
}

impl FormatScssEachIterableList {
    pub(super) fn new(bindings: ScssEachBindingList, in_token: CssSyntaxToken) -> Self {
        Self { bindings, in_token }
    }
}

impl FormatRule<ScssListExpression> for FormatScssEachIterableList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        FormatNodeRule::<ScssListExpression>::fmt(self, node, f)
    }
}

impl FormatNodeRule<ScssListExpression> for FormatScssEachIterableList {
    fn fmt_fields(&self, node: &ScssListExpression, f: &mut CssFormatter) -> FormatResult<()> {
        write_each_iterable(&self.bindings, &self.in_token, node, f)
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

/// Formats an `@each` header and list iterable in one fill group.
///
/// This matches Prettier wrapping for `@each $a, $b in (x, y), (z, w)`.
fn write_each_iterable(
    bindings: &ScssEachBindingList,
    in_token: &CssSyntaxToken,
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

    let separator = soft_line_break_or_space();
    let has_iterable_line_comment = f
        .comments()
        .leading_comments(iterable.syntax())
        .iter()
        .any(|comment| comment.kind().is_line());
    let mut fill = f.fill();
    let binding_count = bindings.len();
    let mut break_after_first_iterable = false;

    for (index, binding) in bindings.elements().enumerate() {
        if index + 1 == binding_count {
            // The final binding owns `in` and the first iterable item.
            if has_iterable_line_comment {
                // `@each $x in // list\n a, b` breaks before the first item.
                fill.entry(
                    &separator,
                    &group(&indent(&format_args![
                        binding.node()?.format(),
                        soft_line_break_or_space(),
                        in_token.format(),
                        space(),
                        format_leading_comments(iterable.syntax())
                    ])),
                );

                fill.entry(
                    &separator,
                    &format_args![
                        first_iterable.node()?.format(),
                        first_iterable.trailing_separator()?.format()
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
                    space(),
                    format_leading_comments(iterable.syntax()),
                    format_args![
                        first_iterable.node()?.format(),
                        first_iterable.trailing_separator()?.format()
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
