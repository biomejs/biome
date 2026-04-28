use biome_css_syntax::{CssSyntaxNode, ScssIncludeArgumentList};
use biome_rowan::AstNode;

/// Returns `true` for nodes nested anywhere inside `@include mix(...)`.
pub(crate) fn is_in_scss_include_arguments(node: &CssSyntaxNode) -> bool {
    node.ancestors()
        .any(|ancestor| ScssIncludeArgumentList::can_cast(ancestor.kind()))
}
