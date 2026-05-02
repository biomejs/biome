use crate::{CssSyntaxNode, ScssIncludeArgumentList, ScssKeywordArgument};
use biome_rowan::AstNode;

/// Returns `true` for nodes nested anywhere inside `@include mix(...)`.
pub fn is_in_scss_include_arguments(node: &CssSyntaxNode) -> bool {
    node.ancestors()
        .any(|ancestor| ScssIncludeArgumentList::can_cast(ancestor.kind()))
}

/// Finds a keyword argument before the surrounding include argument list.
///
/// Example: in `@include mix($arg: (a, b))`, `(a, b)` maps to `$arg`.
pub fn include_keyword_argument_before_argument_list(
    node: &CssSyntaxNode,
) -> Option<ScssKeywordArgument> {
    node.ancestors()
        .take_while(|ancestor| !ScssIncludeArgumentList::can_cast(ancestor.kind()))
        .find_map(ScssKeywordArgument::cast)
}
