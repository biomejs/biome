use biome_css_syntax::{
    CssGenericProperty, CssSyntaxKind, CssSyntaxNode, ScssExpression, ScssExpressionItemList,
    ScssListExpression, ScssListExpressionElement, ScssListExpressionElementList,
};
use biome_rowan::{AstNode, AstSeparatedList};

/// Finds the nearest SCSS declaration-list group containing `node`.
///
/// Given the second list element or any descendant within it, this returns the
/// expression item list containing `$z $w`:
///
/// ```scss
/// a {
///   box-shadow: $x $y, $z $w;
/// }
/// ```
pub(crate) fn find_scss_declaration_list_group(
    node: &CssSyntaxNode,
) -> Option<ScssExpressionItemList> {
    let items = ScssListExpressionElement::cast_ref(node)
        .and_then(|element| element.value().ok())
        .and_then(|value| value.as_scss_expression().map(ScssExpression::items))
        .or_else(|| node.ancestors().find_map(ScssExpressionItemList::cast))?;
    is_declaration_list_group(&items).then_some(items)
}

/// Returns whether `items` is one comma group in an SCSS property value.
///
/// Both `$x $y` and `$z $w` are groups in this declaration:
///
/// ```scss
/// a {
///   box-shadow: $x $y, $z $w;
/// }
/// ```
fn is_declaration_list_group(items: &ScssExpressionItemList) -> bool {
    let Some(elements) = items
        .parent::<ScssExpression>()
        .and_then(|expression| expression.parent::<ScssListExpressionElement>())
        .and_then(|element| element.parent::<ScssListExpressionElementList>())
    else {
        return false;
    };

    elements
        .separators()
        .any(|separator| separator.is_ok_and(|token| token.kind() == CssSyntaxKind::COMMA))
        && elements
            .parent::<ScssListExpression>()
            .and_then(|list| list.parent::<ScssExpressionItemList>())
            .and_then(|items| items.parent::<ScssExpression>())
            .and_then(|expression| expression.parent::<CssGenericProperty>())
            .is_some()
}
