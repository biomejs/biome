use crate::prelude::*;
use crate::utils::scss_closing_comments::owns_map_closing_comments;
use biome_css_syntax::{
    AnyScssExpressionItem, ScssEachHeader, ScssEachValueList, ScssExpression, ScssMapExpression,
    ScssMapExpressionFields, is_in_scss_control_condition_sequence, is_in_scss_include_arguments,
    is_scss_map_key, single_expression_item,
};
use biome_formatter::{CstFormatContext, format_args, write};

/// Shared map layout policy for `ScssMapExpression`.
///
/// Example:
/// `("key": "value"): "hello"`
pub(crate) struct ScssMapLayout<'a> {
    node: &'a ScssMapExpression,
}

impl<'a> ScssMapLayout<'a> {
    pub(crate) fn new(node: &'a ScssMapExpression) -> Self {
        Self { node }
    }

    pub(crate) fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        if self.is_comment_only_map(f) {
            self.fmt_comment_only_map(f)
        } else {
            self.fmt_non_empty_map(f)
        }
    }

    /// Returns `true` when this layout already prints the map dangling comments.
    pub(crate) fn owns_dangling_comments(&self, f: &CssFormatter) -> bool {
        // `fmt` already handles:
        // - empty comment-only maps like `(/* comment */)`
        // - inline comments after the last comma like `a: b, /* end */)`
        //
        // Avoid printing those comments a second time at the end of the node.
        f.context()
            .comments()
            .has_dangling_comments(self.node.syntax())
            && (self.node.pairs().len() == 0 || owns_map_closing_comments(self.node, f))
    }

    fn is_comment_only_map(&self, f: &CssFormatter) -> bool {
        self.node.pairs().len() == 0
            && f.context()
                .comments()
                .has_dangling_comments(self.node.syntax())
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
                ])]
            );
        }

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                format_dangling_comments(self.node.syntax()),
                r_paren_token.format()
            ])]
        )
    }

    fn fmt_non_empty_map(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionFields {
            l_paren_token,
            pairs,
            r_paren_token,
        } = self.node.as_fields();
        // Include arguments accept any closing comment shape before `)`, while
        // standalone maps only special-case inline closing comments.
        let has_closing_comments = owns_map_closing_comments(self.node, f);
        // `@if (a: b, c: d)` uses the map as a parenthesized condition.
        // Prettier does not synthesize a closing comma there.
        let should_omit_trailing_comma =
            !has_closing_comments && is_in_scss_control_condition_sequence(self.node);
        let trailing_comma = (!should_omit_trailing_comma).then_some(format_with(|f| {
            if has_closing_comments {
                write!(f, [token(",")])
            } else {
                write!(f, [if_group_breaks(&token(","))])
            }
        }));
        let closing_comment_separator = format_with(|f| {
            if is_in_scss_include_arguments(self.node.syntax()) {
                write!(f, [soft_line_break_or_space()])
            } else {
                write!(f, [space()])
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
                    has_closing_comments.then_some(closing_comment_separator),
                    has_closing_comments.then_some(format_dangling_comments(self.node.syntax()))
                ]),
                soft_line_break(),
                r_paren_token.format()
            ])
            .should_expand(self.should_expand())]
        )
    }

    fn should_expand(&self) -> bool {
        // Prettier keeps direct `@each` maps inline when they fit:
        // `@each $k, $v in (a: 1, b: 2)`.
        !is_direct_each_value_map(self.node) && should_expand_map_expression(self.node)
    }
}

/// Returns `true` for the direct map value in `@each $name in (a: b)`.
fn is_direct_each_value_map(node: &ScssMapExpression) -> bool {
    let Some(expression) = node
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(ScssExpression::cast)
    else {
        return false;
    };

    let Some(values) = expression
        .syntax()
        .parent()
        .and_then(ScssEachValueList::cast)
    else {
        return false;
    };

    if !values
        .syntax()
        .parent()
        .is_some_and(|parent| ScssEachHeader::can_cast(parent.kind()))
    {
        return false;
    }

    single_expression_item(&expression).is_some_and(|item| match item {
        AnyScssExpressionItem::ScssMapExpression(map) => map.syntax() == node.syntax(),
        _ => false,
    })
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
