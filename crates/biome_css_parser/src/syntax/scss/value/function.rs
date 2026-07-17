use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::{is_nth_at_scss_module_member_access, parse_scss_function_name};
use crate::syntax::{ValueParsingContext, ValueParsingMode};
use biome_css_syntax::CssSyntaxKind::CSS_FUNCTION;
use biome_css_syntax::T;
use biome_parser::CompletedMarker;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

use crate::syntax::value::function::{ParameterList, is_nth_at_source_tight_l_paren};

#[inline]
pub(crate) fn is_at_scss_function(p: &mut CssParser) -> bool {
    is_nth_at_scss_function(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_scss_function(p: &mut CssParser, n: usize) -> bool {
    if !is_nth_at_scss_module_member_access(p, n) {
        return false;
    }

    // `module.name(` has `(` at `n + 3`, while invalid function names such as
    // `module.$name(` include the `$` token and place `(` at `n + 4`.
    let l_paren_offset = if p.nth_at(n + 2, T![$]) { 4 } else { 3 };
    is_nth_at_source_tight_l_paren(p, n + l_paren_offset)
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

/// Parses a function call from an already parsed SCSS function name.
///
/// The caller must leave the parser at `(`.
///
/// Examples:
/// ```scss
/// math.pow(2, 3)
/// #{fn}(arg)
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/function/#plain-css-functions
#[inline]
pub(crate) fn parse_scss_function_call_from_name(
    p: &mut CssParser,
    name: CompletedMarker,
) -> ParsedSyntax {
    let m = name.precede(p);
    p.expect(T!['(']);
    let context = ValueParsingContext::new(p, ValueParsingMode::ScssAware);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}
