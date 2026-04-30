mod each_iterable_list;

use crate::prelude::*;
use crate::utils::scss_expression::single_expression_item;
use crate::utils::scss_separator_comments::FormatScssSeparatorComments;
use biome_css_syntax::{
    AnyScssExpressionItem, CssSyntaxToken, ScssEachAtRule, ScssEachBindingList, ScssExpression,
    ScssExpressionFields, ScssListExpression,
};
use biome_formatter::{FormatOwnedWithRule, FormatResult, write};
use biome_rowan::AstSeparatedList;
use each_iterable_list::FormatScssEachIterableList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssExpression;
impl FormatNodeRule<ScssExpression> for FormatScssExpression {
    fn fmt_node(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        self.fmt_node_with_scss_separator_comments(node, f)
    }

    fn fmt_fields(&self, node: &ScssExpression, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssExpressionFields { items } = node.as_fields();

        if let Some(header) = each_iterable_header(node) {
            let ScssEachIterableHeader {
                bindings,
                in_token,
                list,
            } = header;

            return write!(
                f,
                [FormatOwnedWithRule::new(
                    list,
                    FormatScssEachIterableList::new(bindings, in_token)
                )]
            );
        }

        write!(f, [items.format()])
    }

    fn fmt_leading_comments(
        &self,
        node: &ScssExpression,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if each_iterable_header(node).is_some() {
            // In `@each $x in /* list */ a, b`, the list prints the comment.
            return Ok(());
        }

        self.fmt_leading_scss_separator_comments(node, f)
    }
}

/// Header pieces for `@each $x, $y in (a, b), (c, d)`.
struct ScssEachIterableHeader {
    bindings: ScssEachBindingList,
    in_token: CssSyntaxToken,
    list: ScssListExpression,
}

/// Matches only direct list iterables in `@each`, like `@each $x in a, b`.
fn each_iterable_header(node: &ScssExpression) -> Option<ScssEachIterableHeader> {
    let Some(AnyScssExpressionItem::ScssListExpression(list)) = single_expression_item(node) else {
        return None;
    };

    let each = node.syntax().parent().and_then(ScssEachAtRule::cast)?;
    let bindings = each.bindings();

    if bindings.len() == 0 {
        return None;
    }

    let iterable = each.iterable().ok()?;

    // Only the actual `@each` iterable owns the header layout.
    if iterable.syntax() != node.syntax() {
        return None;
    }

    let in_token = each.in_token().ok()?;

    Some(ScssEachIterableHeader {
        bindings,
        in_token,
        list,
    })
}
