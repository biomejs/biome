use crate::parser::CssParser;
use crate::syntax::block::{expected_block, parse_declaration_or_rule_list_block};
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use crate::syntax::selector::SelectorList;
use biome_css_syntax::CssSyntaxKind::{
    SCSS_AT_ROOT_AT_RULE, SCSS_AT_ROOT_QUERY, SCSS_AT_ROOT_QUERY_CLAUSE, SCSS_AT_ROOT_SELECTOR,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

const SCSS_AT_ROOT_QUERY_MODIFIER_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![:], T![')'], T!['{'], T!['}'], T![;]];
const SCSS_AT_ROOT_QUERY_RULES_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![')'], T!['{'], T!['}'], T![;]];

/// Parses the SCSS `@at-root` at-rule.
///
/// # Example
///
/// ```scss
/// @at-root (without: media) {
///   .root-only {
///     color: red;
///   }
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/
#[inline]
pub(crate) fn parse_scss_at_root_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_at_root_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![at_root]);
    let query = parse_scss_at_root_query(p);

    if query.is_present() && (p.at(T!['}']) || p.at(CssSyntaxKind::EOF)) {
        p.error(expected_block(p, p.cur_range()));
        return Present(m.complete(p, SCSS_AT_ROOT_AT_RULE));
    }

    if query.is_present() || p.at(T!['{']) {
        parse_declaration_or_rule_list_block(p);
    } else {
        parse_scss_at_root_selector(p);
        parse_declaration_or_rule_list_block(p);
    }

    Present(m.complete(p, SCSS_AT_ROOT_AT_RULE))
}

#[inline]
fn is_at_scss_at_root_at_rule(p: &mut CssParser) -> bool {
    p.at(T![at_root])
}

/// Parses a whole-query expression or a modifier/rules clause.
///
/// # Example
///
/// ```scss
/// $query: "without: rule";
/// @at-root ($query) {}
/// @at-root (without: media supports) {}
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/#beyond-style-rules
#[inline]
fn parse_scss_at_root_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_at_root_query(p) {
        return Absent;
    }

    let query = p.start();
    p.bump(T!['(']);
    let body = p.start();
    parse_scss_expression_until(p, SCSS_AT_ROOT_QUERY_MODIFIER_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);

    if p.eat(T![:]) {
        parse_scss_expression_until(p, SCSS_AT_ROOT_QUERY_RULES_END_SET)
            .or_add_diagnostic(p, expected_scss_expression);
        body.complete(p, SCSS_AT_ROOT_QUERY_CLAUSE);
    } else {
        body.abandon(p);
    }

    p.expect(T![')']);
    Present(query.complete(p, SCSS_AT_ROOT_QUERY))
}

/// Checks for the opening parenthesis that distinguishes a query from selector shorthand.
///
/// For example, `@at-root (without: media) {}` has a query, while
/// `@at-root .root-only {}` uses selector shorthand.
#[inline]
fn is_at_scss_at_root_query(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

/// Parses the selector shorthand used by `@at-root <selector> { ... }`.
///
/// # Example
///
/// ```scss
/// @at-root .root-only, .another-root {
///          ^^^^^^^^^^^^^^^^^^^^^^^^^
///   color: red;
/// }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/at-root/
#[inline]
fn parse_scss_at_root_selector(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    SelectorList::default().parse_list(p);

    m.complete(p, SCSS_AT_ROOT_SELECTOR)
}
