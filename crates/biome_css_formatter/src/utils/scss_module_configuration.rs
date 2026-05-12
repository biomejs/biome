use biome_css_syntax::{ScssModuleConfigurationList, ScssWithClause};
use biome_rowan::AstNode;

/// Returns true for the source-separated `with (` shape that Prettier expands.
///
/// ```scss
/// @use "theme" with ($x: red);
/// ```
pub(crate) fn is_source_separated_with_configuration(node: &ScssModuleConfigurationList) -> bool {
    let Some(with_clause) = node
        .syntax()
        .parent()
        .and_then(|parent| ScssWithClause::cast_ref(&parent))
    else {
        return false;
    };

    let Ok(with_token) = with_clause.with_token() else {
        return false;
    };
    let Ok(l_paren_token) = node.l_paren_token() else {
        return false;
    };

    with_token.has_trailing_whitespace()
        || with_token.has_trailing_comments()
        || l_paren_token.has_leading_whitespace_or_newline()
        || l_paren_token.has_leading_comments()
}
