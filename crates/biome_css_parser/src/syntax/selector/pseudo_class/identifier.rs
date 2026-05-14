use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::parse_scss_interpolated_pseudo_class_function_arguments;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};

use super::super::{is_at_selector_identifier, parse_selector_identifier};

/// Parses a pseudo-class identifier.
///
/// Examples:
/// ```scss
/// :hover
/// :foo-#{$name}
/// :foo-#{$name}(bar)
/// ```
#[inline]
pub(crate) fn parse_pseudo_class_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_selector_identifier(p) {
        return Absent;
    }

    let m = p.start();
    let name = parse_selector_identifier(p);
    let is_interpolated_name = name.kind(p) == Some(SCSS_INTERPOLATED_IDENTIFIER);
    name.or_add_diagnostic(p, expected_identifier);

    if is_interpolated_name && p.at(T!['(']) {
        p.bump(T!['(']);
        // `:#{$name}()` has no arguments; invalid non-empty bodies are reported by `)`.
        parse_scss_interpolated_pseudo_class_function_arguments(p).ok();
        p.expect(T![')']);

        return Present(m.complete(p, SCSS_INTERPOLATED_PSEUDO_CLASS_FUNCTION));
    }

    Present(m.complete(p, CSS_PSEUDO_CLASS_IDENTIFIER))
}
