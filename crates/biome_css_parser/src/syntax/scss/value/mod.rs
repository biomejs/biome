mod any;
mod function;
mod interpolated_string;
mod interpolated_value;
mod parent_selector;

use crate::parser::CssParser;
use crate::syntax::scss::parse_scss_expression_until;
use crate::syntax::value::function::is_at_any_function_with_context;
use crate::syntax::{
    CssSyntaxFeatures, ValueParsingContext, ValueParsingMode, is_at_any_value_with_context,
    is_at_css_wide_keyword, is_at_identifier,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::{Parser, SyntaxFeature, TokenSet, token_set};

pub(crate) use any::{is_at_any_scss_value, parse_any_scss_value_with_context};
pub(crate) use function::{
    is_at_scss_function, is_nth_at_scss_function, parse_scss_function,
    parse_scss_function_call_from_name,
};
pub(crate) use interpolated_string::{
    is_at_scss_interpolated_string, parse_scss_interpolated_string,
};
pub(crate) use interpolated_value::{
    is_at_scss_interpolated_function_or_value, parse_scss_interpolated_function_or_value,
    parse_scss_interpolated_function_or_value_until, parse_scss_interpolated_value,
};
pub(crate) use parent_selector::{
    is_at_scss_parent_selector_value, parse_scss_parent_selector_value,
};

pub(crate) const SCSS_BRACKETED_VALUE_EXPRESSION_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![']'], T![,], T![/]];

/// Parses a Sass expression item inside the shared bracketed-value parser.
///
/// This keeps plain CSS custom identifiers on the fallback path, while parsing
/// Sass-only values and CSS-wide keywords as expressions.
///
/// Examples:
/// ```scss
/// $list: [1, $value, []];
/// $keywords: [inherit, initial, unset];
///
/// a {
///   c: [(c,) (d e)];
/// }
/// ```
#[inline]
pub(crate) fn parse_scss_bracketed_value_expression_item(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_bracketed_value_expression_item(p) {
        return Absent;
    }

    parse_scss_expression_until(p, SCSS_BRACKETED_VALUE_EXPRESSION_END_SET)
}

#[inline]
fn is_at_scss_bracketed_value_expression_item(p: &mut CssParser) -> bool {
    if !CssSyntaxFeatures::Scss.is_supported(p) {
        return false;
    }

    if p.at(T!['(']) {
        return true;
    }

    let context = ValueParsingContext::new(p, ValueParsingMode::ScssAware);
    let is_plain_css_custom_identifier = is_at_identifier(p)
        && !is_at_css_wide_keyword(p)
        && !is_at_any_function_with_context(p, context);

    !is_plain_css_custom_identifier && is_at_any_value_with_context(p, context)
}
