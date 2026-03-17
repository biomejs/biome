use crate::parser::CssParser;
use crate::syntax::scss::{is_at_scss_interpolation, is_nth_at_scss_interpolation};
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier};
use biome_css_syntax::CssSyntaxKind::{
    EOF, SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST,
};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, ParserProgress, SyntaxFeature};

#[inline]
pub(crate) fn is_at_scss_interpolated_identifier(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_nth_at_identifier(p, n) || is_nth_at_scss_interpolation(p, n))
}

/// Parses an identifier that may be formed by adjacent identifier and
/// interpolation fragments with no intervening trivia.
///
/// This lower-level helper is reused by selector-specific wrappers that need
/// different fragment parsing behavior.
pub(super) fn parse_scss_interpolated_identifier_with(
    p: &mut CssParser,
    parse_fragment: fn(&mut CssParser) -> ParsedSyntax,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_identifier(p) {
        return Absent;
    }

    let first_is_interpolation = is_at_scss_interpolation(p);
    let Present(first_fragment) = parse_fragment(p) else {
        return Absent;
    };

    if !first_is_interpolation && !is_at_scss_interpolated_identifier(p) {
        return Present(first_fragment);
    }

    let list = first_fragment.precede(p);
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && is_at_scss_interpolated_identifier(p) {
        progress.assert_progressing(p);

        if parse_fragment(p).is_absent() {
            break;
        }
    }

    let list = list.complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
    let m = list.precede(p);
    Present(m.complete(p, SCSS_INTERPOLATED_IDENTIFIER))
}
