use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::{
    is_at_scss_interpolation, is_nth_at_scss_interpolation, is_nth_at_scss_qualified_name,
    parse_scss_function_name, parse_scss_identifier_or_interpolation,
    parse_scss_interpolated_identifier,
};
use crate::syntax::{ValueParsingContext, ValueParsingMode, is_nth_at_identifier};
use biome_css_syntax::CssSyntaxKind::{
    CSS_FUNCTION, SCSS_INTERPOLATED_IDENTIFIER, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST,
    SCSS_INTERPOLATION,
};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

use crate::syntax::value::function::ParameterList;

#[inline]
pub(crate) fn is_at_scss_function(p: &mut CssParser) -> bool {
    is_nth_at_scss_function(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_function(p: &mut CssParser, n: usize) -> bool {
    if !is_nth_at_scss_qualified_name(p, n) {
        return false;
    }

    // `module.name(` has `(` at `n + 3`, while invalid function names such as
    // `module.$name(` include the `$` token and place `(` at `n + 4`.
    let l_paren_offset = if p.nth_at(n + 2, T![$]) { 4 } else { 3 };
    p.nth_at(n + l_paren_offset, T!['('])
}

/// Parses an SCSS function call whose head uses a module-qualified function
/// name.
///
/// Examples:
/// ```scss
/// color.adjust($c, $lightness: 10%);
/// math.pow(2, 3);
/// ```
///
/// Docs: https://sass-lang.com/documentation/modules
#[inline]
pub(crate) fn parse_scss_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_function(p) {
        return Absent;
    }

    let m = p.start();
    parse_scss_function_name(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T!['(']);
    let context = ValueParsingContext::new(p, ValueParsingMode::ScssAware);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}

#[inline]
pub(crate) fn is_at_scss_interpolated_function_or_value(p: &mut CssParser) -> bool {
    is_at_scss_interpolation(p) || is_at_identifier_with_interpolation_suffix(p)
}

/// Parses an SCSS interpolation-led value and upgrades it to a function call
/// when the interpolation-shaped name is followed by `(`.
///
/// Examples:
///
/// ```scss
/// #{foo}(arg)
/// foo#{1 + 1}(arg)
/// #{$value}
/// ```
///
/// Docs: https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_scss_interpolated_function_or_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_function_or_value(p) {
        return Absent;
    }

    let name = if is_at_scss_interpolation(p) {
        parse_scss_identifier_or_interpolation(p)
    } else {
        parse_scss_interpolated_identifier(p)
    };

    // Guarded by is_at_scss_interpolated_function_or_value above.
    let Some(name) = name.ok() else {
        return Absent;
    };

    let name = if name.kind(p) == SCSS_INTERPOLATION && p.at(T!['(']) {
        let list = name
            .precede(p)
            .complete(p, SCSS_INTERPOLATED_IDENTIFIER_PART_LIST);
        list.precede(p).complete(p, SCSS_INTERPOLATED_IDENTIFIER)
    } else {
        name
    };

    if !p.at(T!['(']) {
        return Present(name);
    }

    let m = name.precede(p);
    p.bump(T!['(']);
    let context = ValueParsingContext::new(p, ValueParsingMode::ScssAware);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}

#[inline]
fn is_at_identifier_with_interpolation_suffix(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && is_nth_at_scss_interpolation(p, 1)
        && !p.has_nth_preceding_whitespace(1)
}
