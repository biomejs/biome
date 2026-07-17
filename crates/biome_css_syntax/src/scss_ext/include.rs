use crate::{CssParameterList, CssSyntaxNode, ScssIncludeArgumentList, ScssKeywordArgument};
use biome_rowan::AstNode;

/// Returns `true` for nodes nested anywhere inside `@include mix(...)`.
pub fn is_in_scss_include_arguments(node: &CssSyntaxNode) -> bool {
    node.ancestors()
        .find_map(ScssIncludeArgumentList::cast)
        .is_some()
}

/// Returns the direct include keyword argument that owns `node`.
///
/// Example: in `@include mix($arg: (a, b))`, `(a, b)` maps to `$arg`.
/// Returns `None` for nested function arguments, such as
/// `@include mix(fn($arg: (a, b)))`.
pub fn scss_include_keyword_argument_owner(node: &CssSyntaxNode) -> Option<ScssKeywordArgument> {
    let include_arguments = node.ancestors().find_map(ScssIncludeArgumentList::cast)?;
    let parameter_list = node.ancestors().find_map(CssParameterList::cast)?;

    if parameter_list != include_arguments.items() {
        return None;
    }

    node.ancestors()
        .take_while(|ancestor| ancestor != parameter_list.syntax())
        .find_map(ScssKeywordArgument::cast)
}
