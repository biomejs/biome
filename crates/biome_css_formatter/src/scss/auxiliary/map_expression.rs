use crate::prelude::*;
use crate::utils::scss_map::is_scss_map_key;
use biome_css_syntax::{ScssMapExpression, ScssMapExpressionFields};
use biome_formatter::{CstFormatContext, GroupId, format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpression;
impl FormatNodeRule<ScssMapExpression> for FormatScssMapExpression {
    fn fmt_fields(&self, node: &ScssMapExpression, f: &mut CssFormatter) -> FormatResult<()> {
        ScssMapLayout::new(node, f.group_id("scss_map_expression")).fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        node: &ScssMapExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if ScssMapLayout::new(node, f.group_id("scss_map_expression")).handles_dangling_comments(f)
        {
            Ok(())
        } else {
            format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f)
        }
    }
}

struct ScssMapLayout<'a> {
    node: &'a ScssMapExpression,
    group_id: GroupId,
}

impl<'a> ScssMapLayout<'a> {
    fn new(node: &'a ScssMapExpression, group_id: GroupId) -> Self {
        Self { node, group_id }
    }

    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_comment_only_map(f) {
            self.fmt_comment_only_map(f)
        } else {
            self.fmt_non_empty_map(f)
        }
    }

    fn handles_dangling_comments(&self, f: &CssFormatter) -> bool {
        // `fmt_fields` already handles:
        // - empty comment-only maps like `(/* comment */)`
        // - inline comments after the last comma like `a: b, /* end */)`
        //
        // Avoid printing those comments a second time at the end of the node.
        f.context()
            .comments()
            .has_dangling_comments(self.node.syntax())
            && (self.node.pairs().len() == 0 || self.has_inline_closing_comments(f))
    }

    fn is_comment_only_map(&self, f: &CssFormatter) -> bool {
        self.node.pairs().len() == 0
            && f.context()
                .comments()
                .has_dangling_comments(self.node.syntax())
    }

    fn has_inline_closing_comments(&self, f: &CssFormatter) -> bool {
        self.node.pairs().len() > 0
            && !f
                .context()
                .comments()
                .dangling_comments(self.node.syntax())
                .is_empty()
            && f.context()
                .comments()
                .dangling_comments(self.node.syntax())
                .iter()
                .all(|comment| comment.kind().is_inline() && comment.lines_before() == 0)
    }

    fn fmt_comment_only_map(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionFields {
            l_paren_token,
            r_paren_token,
            ..
        } = self.node.as_fields();
        let dangling_comments = f.context().comments().dangling_comments(self.node.syntax());

        // Keep comment-only maps inside the parentheses, e.g.
        // `$map: (/* comment */);`.
        if dangling_comments
            .iter()
            .any(|comment| comment.kind().is_line())
        {
            // `(// comment)` would comment out the closing `)` and produce
            // invalid SCSS.
            return write!(
                f,
                [group(&format_args![
                    l_paren_token.format(),
                    indent(&format_args![
                        hard_line_break(),
                        format_dangling_comments(self.node.syntax())
                    ]),
                    hard_line_break(),
                    r_paren_token.format()
                ])
                .with_group_id(Some(self.group_id))]
            );
        }

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                format_dangling_comments(self.node.syntax()),
                r_paren_token.format()
            ])
            .with_group_id(Some(self.group_id))]
        )
    }

    fn fmt_non_empty_map(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionFields {
            l_paren_token,
            pairs,
            r_paren_token,
        } = self.node.as_fields();
        let has_inline_closing_comments = self.has_inline_closing_comments(f);
        let trailing_comma = self.fmt_trailing_comma(has_inline_closing_comments);

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                indent(&format_args![
                    soft_line_break(),
                    pairs.format(),
                    trailing_comma,
                    has_inline_closing_comments.then_some(space()),
                    has_inline_closing_comments
                        .then_some(format_dangling_comments(self.node.syntax()))
                ]),
                soft_line_break(),
                r_paren_token.format()
            ])
            .with_group_id(Some(self.group_id))
            .should_expand(should_expand_map_expression(self.node))]
        )
    }

    fn fmt_trailing_comma(
        &self,
        has_inline_closing_comments: bool,
    ) -> impl Format<CssFormatContext> {
        let group_id = self.group_id;

        format_with(move |f| {
            if has_inline_closing_comments {
                write!(f, [token(",")])
            } else {
                if_group_breaks(&token(","))
                    .with_group_id(Some(group_id))
                    .fmt(f)
            }
        })
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
