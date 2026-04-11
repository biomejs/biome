use crate::parser::CssParser;
use crate::syntax::scss::is_nth_at_scss_interpolation;
use crate::syntax::{CssSyntaxFeatures, is_nth_at_identifier};
use biome_css_syntax::CssSyntaxKind::{
    EOF, SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_HYPHEN,
    SCSS_INTERPOLATED_IDENTIFIER_PART_LIST,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, ParserProgress, SyntaxFeature};

#[inline]
pub(crate) fn is_at_scss_interpolated_identifier(p: &mut CssParser) -> bool {
    is_nth_at_scss_interpolated_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_interpolated_identifier(p: &mut CssParser, n: usize) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p)
        && (is_nth_at_identifier(p, n) || is_nth_at_scss_interpolation(p, n))
}

/// Returns `true` when the current token continues an interpolated identifier
/// with no separating trivia.
///
/// Examples:
/// - `foo#{$bar}`
/// - `#{$a}-#{$b}`
#[inline]
pub(super) fn is_at_identifier_continuation(p: &mut CssParser) -> bool {
    is_at_adjacent_identifier(p) || is_at_identifier_hyphen(p)
}

pub(super) fn complete_scss_interpolated_identifier(
    p: &mut CssParser,
    first_fragment: CompletedMarker,
    mut parse_part: impl FnMut(&mut CssParser) -> ParsedSyntax,
) -> CompletedMarker {
    let list = first_fragment.precede(p);
    let mut progress = ParserProgress::default();

    while !p.at(EOF) && is_at_identifier_continuation(p) {
        progress.assert_progressing(p);

        if is_at_identifier_hyphen(p) {
            // Safe: guarded by `is_at_identifier_hyphen`.
            parse_identifier_hyphen(p).ok();
        } else if parse_part(p).is_absent() {
            break;
        }
    }

    let list = list.complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
    list.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER)
}

#[inline]
fn is_at_adjacent_identifier(p: &mut CssParser) -> bool {
    !p.has_preceding_whitespace() && is_at_scss_interpolated_identifier(p)
}

/// Returns `true` when `-` belongs to the current interpolated identifier,
/// such as the hyphen in `#{$a}-#{$b}`.
#[inline]
pub(super) fn is_at_identifier_hyphen(p: &mut CssParser) -> bool {
    p.at(T![-])
        && !p.has_preceding_whitespace()
        && is_nth_at_scss_interpolated_identifier(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}

#[inline]
pub(super) fn parse_identifier_hyphen(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier_hyphen(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![-]);
    Present(m.complete(p, SCSS_INTERPOLATED_IDENTIFIER_HYPHEN))
}
