use crate::prelude::*;
use crate::utils::component_value_list::write_component_value_list_with_separator_rule;
use biome_css_syntax::{AnyCssValue, AnyScssExpressionItem, ScssExpressionItemList};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpressionItemList;
impl FormatRule<ScssExpressionItemList> for FormatScssExpressionItemList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &ScssExpressionItemList, f: &mut CssFormatter) -> FormatResult<()> {
        write_component_value_list_with_separator_rule(
            node,
            f,
            omit_separator_between_unquoted_concatenation_parts,
        )
    }
}

/// Preserves direct adjacency for the currently supported interpolation-bearing
/// unquoted concatenation forms.
///
/// Examples:
/// - `$variable#{something}`
/// - `#{something}$variable`
/// - `$a#{b}$c`
///
/// These forms currently parse as adjacent `ScssVariable` and
/// `ScssInterpolation` expression items rather than a dedicated concatenation
/// node, so the expression-item formatter must not insert a separator between
/// those item pairs when the source had no trivia between them.
fn omit_separator_between_unquoted_concatenation_parts(
    left: &AnyScssExpressionItem,
    right: &AnyScssExpressionItem,
) -> bool {
    has_no_trivia_between(left, right) && is_unquoted_concatenation_pair(left, right)
}

fn has_no_trivia_between(left: &AnyScssExpressionItem, right: &AnyScssExpressionItem) -> bool {
    left.syntax()
        .last_token()
        .is_some_and(|token| token.trailing_trivia().is_empty())
        && right
            .syntax()
            .first_token()
            .is_some_and(|token| token.leading_trivia().is_empty())
}

fn is_unquoted_concatenation_pair(
    left: &AnyScssExpressionItem,
    right: &AnyScssExpressionItem,
) -> bool {
    matches!(
        (left, right),
        (
            AnyScssExpressionItem::AnyCssValue(AnyCssValue::ScssVariable(_)),
            AnyScssExpressionItem::ScssInterpolation(_),
        ) | (
            AnyScssExpressionItem::ScssInterpolation(_),
            AnyScssExpressionItem::AnyCssValue(AnyCssValue::ScssVariable(_)),
        )
    )
}
