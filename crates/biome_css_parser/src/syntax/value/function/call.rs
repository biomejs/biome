use crate::parser::CssParser;
use crate::syntax::css_modules::v_bind_not_allowed;
use crate::syntax::parse_error::{expected_identifier, scss_only_syntax_error};
use crate::syntax::scss::{
    is_at_scss_qualified_name, is_nth_at_scss_qualified_name, parse_scss_function_name,
};
use crate::syntax::value::attr::{is_at_attr_function, parse_attr_function};
use crate::syntax::value::r#if::{is_at_if_function, parse_if_function};
use crate::syntax::value::url::{is_at_url_function, parse_url_function_with_context};
use crate::syntax::{
    CssSyntaxFeatures, ValueParsingContext, ValueParsingMode, is_nth_at_identifier,
    parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature};

use super::parameter::ParameterList;

/// Checks if the current position is at any function recognized by CSS-only value parsing.
#[inline]
pub(crate) fn is_at_any_css_function(p: &mut CssParser) -> bool {
    is_at_any_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
pub(crate) fn is_at_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> bool {
    is_at_url_function(p)
        || is_at_css_if_function_in_context(p, context)
        || is_at_attr_function(p)
        || is_at_vue_v_bind_function(p)
        || is_at_function_with_context(p, context)
}

#[inline]
pub(crate) fn parse_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_function_with_context(p, context) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function_with_context(p, context)
    } else if is_at_css_if_function_in_context(p, context) {
        parse_if_function(p)
    } else if is_at_attr_function(p) {
        parse_attr_function(p)
    } else if is_at_vue_v_bind_function(p) {
        CssSyntaxFeatures::CssModulesWithVue.parse_exclusive_syntax(
            p,
            |p| parse_function_with_context(p, context),
            |p, marker| v_bind_not_allowed(p, marker.range(p)),
        )
    } else {
        parse_function_with_context(p, context)
    }
}

#[inline]
fn is_at_css_if_function_in_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    if !is_at_if_function(p) {
        return false;
    }

    if !context.is_scss_parsing_allowed() {
        return true;
    }

    // CSS if() branches can only start with supported condition syntax:
    // `style(...)`, `media(...)`, `supports(...)`, `sass(...)`, `not`, `else`, or a
    // parenthesized boolean expression. Anything else remains a regular
    // function call in SCSS-aware mode, such as Sass `if($cond, a, b)`.
    p.nth_at(2, T![style])
        || p.nth_at(2, T![media])
        || p.nth_at(2, T![supports])
        || p.nth_at(2, T![sass])
        || p.nth_at(2, T![not])
        || p.nth_at(2, T![else])
        || p.nth_at(2, T!['('])
}

#[inline]
pub(crate) fn is_at_function(p: &mut CssParser) -> bool {
    is_at_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::ScssAware))
}

/// Checks if the current position is at a simple function head in CSS-only mode.
#[inline]
pub(crate) fn is_at_css_function(p: &mut CssParser) -> bool {
    is_at_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn is_at_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_nth_at_function_with_context(p, 0, context) && !is_at_url_function(p)
}

#[inline]
pub(crate) fn is_nth_at_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(
        p,
        n,
        ValueParsingContext::new(p, ValueParsingMode::ScssAware),
    )
}

/// Checks if the `n`th token starts a simple function head in CSS-only mode.
#[inline]
pub(crate) fn is_nth_at_css_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(p, n, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn is_nth_at_function_with_context(
    p: &mut CssParser,
    n: usize,
    context: ValueParsingContext,
) -> bool {
    is_nth_at_identifier(p, n) && p.nth_at(n + 1, T!['('])
        || (context.is_scss_syntax_allowed()
            && is_nth_at_scss_qualified_name(p, n)
            && p.nth_at(n + 3, T!['(']))
}

#[inline]
fn is_at_vue_v_bind_function(p: &mut CssParser) -> bool {
    if !is_nth_at_css_function(p, 0) {
        return false;
    }

    p.cur_text() == "v-bind"
}

#[inline]
pub(crate) fn parse_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::ScssAware))
}

/// Parses a simple function using CSS-only branches.
#[inline]
pub(crate) fn parse_css_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn parse_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    if !is_at_function_with_context(p, context) {
        return Absent;
    }

    let m = p.start();

    if context.is_scss_syntax_allowed() && is_at_scss_qualified_name(p) {
        CssSyntaxFeatures::Scss
            .parse_exclusive_syntax(p, parse_scss_function_name, |p, marker| {
                scss_only_syntax_error(p, "SCSS qualified function names", marker.range(p))
            })
            .or_add_diagnostic(p, expected_identifier);
    } else {
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    }
    p.bump(T!['(']);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_FUNCTION))
}
