use crate::prelude::*;
use crate::utils::scss_each::FormatGapAfterEachIn;
use biome_css_syntax::{
    CssSyntaxToken, ScssEachBindingList, ScssEachHeader, ScssEachHeaderFields, ScssEachValueList,
};
use biome_formatter::{format_args, write};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachHeader;
impl FormatNodeRule<ScssEachHeader> for FormatScssEachHeader {
    fn fmt_fields(&self, node: &ScssEachHeader, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachHeaderFields {
            bindings,
            in_token,
            values,
        } = node.as_fields();

        let in_token = in_token?;
        let binding_count = bindings.len();
        let value_count = values.len();

        if value_count == 0 {
            return write!(
                f,
                [
                    bindings.format(),
                    soft_line_break_or_space(),
                    in_token.format()
                ]
            );
        }

        if value_count == 1 {
            let gap_before_in = format_with(|f| {
                if binding_count > 0 {
                    write!(f, [space()])
                } else {
                    write!(f, [soft_line_break_or_space()])
                }
            });

            return write!(
                f,
                [group(&format_args![
                    bindings.format(),
                    gap_before_in,
                    in_token.format(),
                    FormatGapAfterEachIn::new(&in_token, values.syntax()),
                    values.format()
                ])]
            );
        }

        write!(
            f,
            [FormatScssEachMultiValueHeader {
                bindings: &bindings,
                in_token: &in_token,
                values: &values,
            }]
        )
    }
}

/// Formats multi-value `@each` headers as one fill sequence.
///
/// Simple headers can call `values.format()` because no value needs to share a
/// fill entry with `in`. With multiple values, Prettier keeps the first value
/// with the final binding and `in`:
///
/// ```scss
/// @each $animal, $color, $cursor in (puma, black, default),
///   (sea-slug, blue, pointer), (egret, white, move)
/// {
/// }
/// ```
///
/// A nested `values.format()` would make all values one parent fill entry, so
/// wrapping would move to the bindings instead of the value list.
struct FormatScssEachMultiValueHeader<'a> {
    bindings: &'a ScssEachBindingList,
    in_token: &'a CssSyntaxToken,
    values: &'a ScssEachValueList,
}

impl Format<CssFormatContext> for FormatScssEachMultiValueHeader<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let bindings = self.bindings;
        let in_token = self.in_token;
        let values = self.values;
        let binding_count = bindings.len();
        let mut value_elements = values.elements();
        let Some(first_value) = value_elements.next() else {
            return write!(
                f,
                [
                    bindings.format(),
                    soft_line_break_or_space(),
                    in_token.format()
                ]
            );
        };
        let first_value_node = first_value.node()?;
        let first_value_syntax = first_value_node.syntax();

        if binding_count == 0 {
            let separator = soft_line_break_or_space();
            let mut fill = f.fill();

            fill.entry(
                &separator,
                &group(&format_args![
                    bindings.format(),
                    soft_line_break_or_space(),
                    in_token.format(),
                    FormatGapAfterEachIn::new(in_token, first_value_syntax),
                    format_leading_comments(values.syntax()),
                    first_value_node.format(),
                    first_value.trailing_separator()?.format()
                ]),
            );

            for value in value_elements {
                fill.entry(
                    &separator,
                    &format_args![value.node()?.format(), value.trailing_separator()?.format()],
                );
            }

            return fill.finish();
        }

        let Some(second_value) = value_elements.next() else {
            return write!(
                f,
                [group(&format_args![
                    bindings.format(),
                    space(),
                    in_token.format(),
                    FormatGapAfterEachIn::new(in_token, first_value_syntax),
                    format_leading_comments(values.syntax()),
                    first_value_node.format(),
                    first_value.trailing_separator()?.format()
                ])]
            );
        };

        let separator = soft_line_break_or_space();
        let has_value_list_line_comment = f
            .comments()
            .leading_comments(values.syntax())
            .iter()
            .any(|comment| comment.kind().is_line());
        let mut fill = f.fill();
        let mut break_after_first_value = false;

        for (index, binding) in bindings.elements().enumerate() {
            if index + 1 == binding_count {
                // The final binding owns `in` and the first value.
                if has_value_list_line_comment {
                    // `@each $x in // list\n a, b` keeps the comment after `in`.
                    fill.entry(
                        &separator,
                        &group(&indent(&format_args![
                            binding.node()?.format(),
                            hard_line_break(),
                            in_token.format(),
                            space(),
                            format_leading_comments(values.syntax())
                        ])),
                    );
                    fill.entry(
                        &separator,
                        &format_args![
                            first_value_node.format(),
                            first_value.trailing_separator()?.format()
                        ],
                    );
                    break_after_first_value = true;
                    continue;
                }

                fill.entry(
                    &separator,
                    &group(&indent(&format_args![
                        binding.node()?.format(),
                        soft_line_break_or_space(),
                        in_token.format(),
                        FormatGapAfterEachIn::new(in_token, first_value_syntax),
                        format_leading_comments(values.syntax()),
                        first_value_node.format(),
                        first_value.trailing_separator()?.format()
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

        for value in std::iter::once(second_value).chain(value_elements) {
            let value_separator = if break_after_first_value {
                break_after_first_value = false;
                hard_line_break()
            } else {
                separator
            };

            fill.entry(
                &value_separator,
                &format_args![value.node()?.format(), value.trailing_separator()?.format()],
            );
        }

        fill.finish()
    }
}
