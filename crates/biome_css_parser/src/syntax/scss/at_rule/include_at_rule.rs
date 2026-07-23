use super::expect_scss_semicolon_at_rule;
use super::parameter::parse_scss_parameter_list;
use crate::parser::CssParser;
use crate::syntax::ValueParsingContext;
use crate::syntax::ValueParsingMode;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::scss::parse_scss_function_name;
use crate::syntax::value::function::ParameterList;
use biome_css_syntax::CssSyntaxKind::{
    SCSS_INCLUDE_ARGUMENT_LIST, SCSS_INCLUDE_AT_RULE, SCSS_INCLUDE_USING_CLAUSE,
};
use biome_css_syntax::T;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_rowan::TextRange;

/// Parses the SCSS `@include` at-rule.
///
/// # Example
///
/// ```scss
/// @include button($radius: 4px);
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/
#[inline]
pub(crate) fn parse_scss_include_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_include_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![include]);
    parse_scss_function_name(p).or_add_diagnostic(p, expected_identifier);
    // The include argument list is optional in the grammar, so a missing `(` is valid.
    parse_scss_include_argument_list(p).ok();
    // The content-parameter clause is optional, but `using (...)` requires a block.
    let using_clause = parse_scss_include_using_clause(p);

    if using_clause.is_present() || p.at(T!['{']) {
        parse_declaration_or_rule_list_block(p);
    } else {
        expect_scss_semicolon_at_rule(p);
    }

    Present(m.complete(p, SCSS_INCLUDE_AT_RULE))
}

#[inline]
fn is_at_scss_include_at_rule(p: &mut CssParser) -> bool {
    p.at(T![include])
}

/// Parses the optional SCSS argument list used by `@include` and `@content`.
///
/// # Example
///
/// ```scss
/// @include button($radius: 4px, $args...);
///                ^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/
#[inline]
pub(crate) fn parse_scss_include_argument_list(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_include_argument_list(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    ParameterList::new(ValueParsingContext::new(p, ValueParsingMode::ScssAware)).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, SCSS_INCLUDE_ARGUMENT_LIST))
}

#[inline]
fn is_at_scss_include_argument_list(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

/// Parses the optional content-parameter clause used by `@include`.
///
/// # Example
///
/// ```scss
/// @include button using ($value) {
///                 ^^^^^^^^^^^^^^
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/mixin/#passing-content-blocks-to-mixins
#[inline]
fn parse_scss_include_using_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_include_using_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![using]);
    parse_scss_parameter_list(p).or_add_diagnostic(p, expected_scss_include_using_parameters);

    Present(m.complete(p, SCSS_INCLUDE_USING_CLAUSE))
}

#[inline]
fn is_at_scss_include_using_clause(p: &mut CssParser) -> bool {
    p.at(T![using])
}

#[inline]
fn expected_scss_include_using_parameters(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder("Expected content parameters.", range)
        .with_hint("Add a parameter list like `($value)` after `using`.")
}
