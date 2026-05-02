use biome_css_syntax::{CssSyntaxNode, is_in_scss_include_arguments};
use biome_formatter::separated::TrailingSeparator;

/// Chooses the trailing separator policy for SCSS-aware separated lists.
pub(crate) fn trailing_separator_for_node(node: &CssSyntaxNode) -> TrailingSeparator {
    if is_in_scss_include_arguments(node) {
        TrailingSeparator::Omit
    } else {
        TrailingSeparator::Disallowed
    }
}
