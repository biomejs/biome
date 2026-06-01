use crate::prelude::*;
use crate::utils::scss_expression::is_self_breaking_value;
use crate::utils::scss_map_layout::is_direct_each_value_map;
use biome_css_syntax::{
    ScssMapExpression, ScssMapExpressionPair, ScssMapExpressionPairList,
    is_in_scss_control_condition_sequence, is_in_scss_include_arguments,
};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPairList;

impl FormatRule<ScssMapExpressionPairList> for FormatScssMapExpressionPairList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssMapExpressionPairList, f: &mut CssFormatter) -> FormatResult<()> {
        // The map layout owns `(` and `)`, while this list owns pairs and
        // separators, including trailing comments like `(a: b, /* end */)`.
        let separated = node
            .format_separated(",")
            .with_trailing_separator(TrailingSeparator::Omit);
        // Preserve source blank lines between pairs, e.g. comment-heavy maps.
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(separated) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()?;
        write_trailing_separator(node, f)
    }
}

/// Writes the map-list comma and comments before `)`.
///
/// Example: `(key: value, /** c1 */ /* c2 */)`.
fn write_trailing_separator(
    node: &ScssMapExpressionPairList,
    f: &mut CssFormatter,
) -> FormatResult<()> {
    let Some(map) = node.syntax().parent().and_then(ScssMapExpression::cast) else {
        return Ok(());
    };

    if is_in_scss_include_arguments(map.syntax()) {
        return Ok(());
    }

    if f.comments().has_dangling_comments(node.syntax()) {
        write!(f, [token(",")])?;

        if has_block_trailing_separator_comments(node, f) {
            if has_blank_line_before_trailing_separator_comments(node, f) {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [soft_line_break_or_space()])?;
            }

            // These comments belong after the trailing comma. Generic dangling
            // comments split `/* a */ /* b */` across lines, unlike Prettier.
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_block_comments_on_line()]
            )
        } else {
            write!(f, [space(), format_dangling_comments(node.syntax())])
        }
    } else if should_omit_trailing_comma(&map, f) {
        Ok(())
    } else {
        // Prettier's `ifBreak(",")` for map-shaped parentheses.
        write!(f, [if_group_breaks(&token(","))])
    }
}

fn should_omit_trailing_comma(map: &ScssMapExpression, f: &CssFormatter) -> bool {
    // Prettier omits the closing comma in `@if (a: b)` and direct
    // `@each $k, $v in (a: b)` maps.
    is_in_scss_control_condition_sequence(map)
        || is_direct_each_value_map(map)
        || (has_comment_before_first_pair(map, f) && has_self_breaking_last_pair_value(map))
}

/// Returns `true` for maps beginning with a pair comment, e.g. `(/* c */ a: b)`.
fn has_comment_before_first_pair(map: &ScssMapExpression, f: &CssFormatter) -> bool {
    let Some(Ok(pair)) = map.pairs().first() else {
        return false;
    };

    if map.pairs().len() == 1 {
        !f.comments().leading_comments(pair.syntax()).is_empty()
    } else {
        has_source_leading_comment(&pair, f)
    }
}

/// Returns `true` when a leading comment is stored on the key token itself.
///
/// In `(a: b, /* c */ key: value)`, comment placement may expose `/* c */`
/// as leading on `key: value`, but this helper only accepts it when the
/// comment is in `key`'s leading trivia instead of the previous comma's trivia.
fn has_source_leading_comment(pair: &ScssMapExpressionPair, f: &CssFormatter) -> bool {
    f.comments()
        .leading_comments(pair.syntax())
        .iter()
        .any(|comment| {
            let comment_piece = comment.piece().as_piece();

            pair.syntax().first_token().is_some_and(|token| {
                token == comment_piece.token()
                    && token
                        .leading_trivia()
                        .text_range()
                        .contains_range(comment_piece.text_range())
            })
        })
}

/// Returns `true` for a last value that breaks itself, e.g. `key: (a: b)`.
fn has_self_breaking_last_pair_value(map: &ScssMapExpression) -> bool {
    let Some(Ok(pair)) = map.pairs().last() else {
        return false;
    };

    pair.value()
        .is_ok_and(|value| is_self_breaking_value(&value))
}

/// Returns `true` for two or more block comments after the trailing comma.
///
/// Example: `(key: value, /** c1 */ /* c2 */)`.
fn has_block_trailing_separator_comments(
    node: &ScssMapExpressionPairList,
    f: &CssFormatter,
) -> bool {
    let dangling_comments = f.comments().dangling_comments(node.syntax());

    dangling_comments.len() > 1
        && dangling_comments
            .iter()
            .all(|comment| !comment.kind().is_line())
}

/// Returns `true` for a source blank line inside the trailing comment group.
///
/// Example: `(key: value, /* c1 */\n\n/* c2 */)`.
fn has_blank_line_before_trailing_separator_comments(
    node: &ScssMapExpressionPairList,
    f: &CssFormatter,
) -> bool {
    let dangling_comments = f.comments().dangling_comments(node.syntax());
    let Some(first_comment) = dangling_comments.first() else {
        return false;
    };

    if first_comment.lines_before() > 1 {
        return true;
    }

    let mut previous_comment = first_comment;

    for comment in dangling_comments.iter().skip(1) {
        if previous_comment.lines_after() > 1 || comment.lines_before() > 1 {
            return true;
        }

        previous_comment = comment;
    }

    false
}
