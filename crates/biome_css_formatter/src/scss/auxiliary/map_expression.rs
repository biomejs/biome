use crate::prelude::*;
use crate::utils::scss_map::is_scss_map_key;
use biome_css_syntax::{ScssMapExpression, ScssMapExpressionFields};
use biome_formatter::{CstFormatContext, format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpression;
impl FormatNodeRule<ScssMapExpression> for FormatScssMapExpression {
    fn fmt_fields(&self, node: &ScssMapExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionFields {
            l_paren_token,
            pairs,
            r_paren_token,
        } = node.as_fields();

        let group_id = f.group_id("scss_map_expression");
        let pair_count = node.pairs().len();
        let dangling_comments = f.context().comments().dangling_comments(node.syntax());
        let has_inline_trailing_separator_comments = pair_count > 0
            && !dangling_comments.is_empty()
            && dangling_comments
                .iter()
                .all(|comment| comment.kind().is_inline() && comment.lines_before() == 0);

        // Keep comment-only maps inside the parentheses, e.g.
        // `$map: (/* comment */);`.
        if pair_count == 0 && f.context().comments().has_dangling_comments(node.syntax()) {
            let dangling_comments = f.context().comments().dangling_comments(node.syntax());
            let has_line_comment = dangling_comments
                .iter()
                .any(|comment| comment.kind().is_line());

            // `(// comment)` would comment out the closing `)` and produce
            // invalid SCSS.
            if has_line_comment {
                return write!(
                    f,
                    [group(&format_args![
                        l_paren_token.format(),
                        indent(&format_args![
                            hard_line_break(),
                            format_dangling_comments(node.syntax())
                        ]),
                        hard_line_break(),
                        r_paren_token.format()
                    ])
                    .with_group_id(Some(group_id))]
                );
            }

            return write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()),
                    r_paren_token.format()
                ])
                .with_group_id(Some(group_id))]
            );
        }

        let should_expand = should_expand_map_expression(node);
        let comma = token(",");
        let trailing_comma = format_with(|f| {
            if has_inline_trailing_separator_comments {
                write!(f, [token(",")])
            } else {
                if_group_breaks(&comma).with_group_id(Some(group_id)).fmt(f)
            }
        });

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                indent(&format_args![
                    soft_line_break(),
                    pairs.format(),
                    trailing_comma,
                    has_inline_trailing_separator_comments.then_some(space()),
                    has_inline_trailing_separator_comments
                        .then_some(format_dangling_comments(node.syntax()))
                ]),
                soft_line_break(),
                r_paren_token.format()
            ])
            .with_group_id(Some(group_id))
            .should_expand(should_expand)]
        )
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssMapExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        // `fmt_fields` already handles:
        // - empty comment-only maps like `(/* comment */)`
        // - inline comments after the last comma like `a: b, /* end */)`
        //
        // Avoid printing those comments a second time at the end of the node.
        if f.context().comments().has_dangling_comments(node.syntax())
            && (node.pairs().len() == 0
                || f.context()
                    .comments()
                    .dangling_comments(node.syntax())
                    .iter()
                    .all(|comment| comment.kind().is_inline() && comment.lines_before() == 0))
        {
            Ok(())
        } else {
            format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f)
        }
    }
}

/// Returns `true` when the map expression should force a multiline layout.
///
/// A single-pair map stays inline only when the map itself is the direct key of
/// an enclosing pair, e.g. `("key": "value"): ...`.
///
/// Multi-pair maps, values, and nested maps inside a larger key all expand.
fn should_expand_map_expression(node: &ScssMapExpression) -> bool {
    let pair_count = node.pairs().len();

    if pair_count == 0 {
        return false;
    }

    if pair_count > 1 {
        return true;
    }

    !is_scss_map_key(node)
}
