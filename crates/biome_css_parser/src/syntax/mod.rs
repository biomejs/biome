mod at_rule;
mod parse_error;
mod selector;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::{at_at_rule, parse_at_rule};
use crate::syntax::parse_error::expected_block;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::selector::CssSelectorList;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, CompletedMarker, Parser, ParserProgress, TokenSet};

use self::parse_error::expected_number;

const RULE_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![T![#], T![.], T![*], T![ident], T![:], T![::], T!['{']];
const SELECTOR_LIST_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{'], T!['}'],];
const BODY_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SELECTOR_LIST_RECOVERY_SET.union(RULE_RECOVERY_SET);

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    parse_rule_list(p);

    m.complete(p, CSS_ROOT);
}

#[inline]
pub(crate) fn parse_rule_list(p: &mut CssParser) {
    let mut progress = ParserProgress::default();

    let rules = p.start();
    while !p.at(EOF) {
        progress.assert_progressing(p);

        if at_at_rule(p) {
            parse_at_rule(p).ok();
        } else {
            parse_rule(p);
        }
    }

    rules.complete(p, CSS_RULE_LIST);
}

#[inline]
pub(crate) fn parse_rule(p: &mut CssParser) -> CompletedMarker {
    let m = p.start();

    CssSelectorList::default().parse_list(p);

    let kind = if parse_or_recover_rule_block(p).is_ok() {
        CSS_RULE
    } else {
        CSS_BOGUS_RULE
    };

    m.complete(p, kind)
}

#[inline]
pub(crate) fn parse_or_recover_rule_block(p: &mut CssParser) -> RecoveryResult {
    parse_rule_block(p).or_recover(
        p,
        &ParseRecovery::new(CSS_BOGUS_BODY, BODY_RECOVERY_SET).enable_recovery_on_line_break(),
        expected_block,
    )
}

#[inline]
pub(crate) fn parse_rule_block(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.expect(T!['{']);
    parse_declaration_list(p);
    p.expect(T!['}']);

    Present(m.complete(p, CSS_BLOCK))
}

pub(crate) fn parse_declaration_list(p: &mut CssParser) {
    let m = p.start();

    // first parse declaration item
    parse_declaration_item(p);
    loop {
        if !p.eat(T![;]) {
            break;
        }
        parse_declaration_item(p);
    }
    m.complete(p, CSS_DECLARATION_LIST);
}

// CssDeclaration =
// 	name: CssIdentifier | CssCustomProperty
// 	':'
// 	value: AnyCssValue
// 	important: CssDeclarationImportant?
#[inline]
pub(crate) fn parse_declaration_item(p: &mut CssParser) {
    if !is_at_identifier(p) {
        return;
    }
    let m = p.start();
    // name
    parse_regular_identifier(p).ok();

    p.expect(T![:]);
    loop {
        let any_css_value = parse_any_css_value(p);
        if any_css_value.is_absent() {
            break;
        }
    }
    parse_declaration_important(p);
    m.complete(p, CSS_DECLARATION);
}

#[inline]
pub(crate) fn parse_declaration_important(p: &mut CssParser) {
    if p.eat(T![!]) {
        p.expect(T![important]);
    }
}
#[inline]
pub(crate) fn is_any_css_value(p: &mut CssParser) -> bool {
    is_at_css_any_function(p)
        || is_at_identifier(p)
        || p.at(CSS_STRING_LITERAL)
        || is_at_css_dimension(p)
        || p.at(CSS_NUMBER_LITERAL)
        || is_at_css_custom_property(p)
        || is_at_css_ratio(p)
}

#[inline]
pub(crate) fn parse_any_css_value(p: &mut CssParser) -> ParsedSyntax {
    let css_any_function = parse_css_any_function(p);
    if css_any_function.is_present() {
        return css_any_function;
    }
    let css_custom_property = parse_css_custom_property(p);
    if css_custom_property.is_present() {
        return css_custom_property;
    }
    let identifier = parse_regular_identifier(p);
    if identifier.is_present() {
        return identifier;
    }
    let css_string = parse_string(p);
    if css_string.is_present() {
        return css_string;
    }
    // Before css number
    // eat dimension  or number or  ratio
    let css_dimension = parse_css_dimension(p);
    if css_dimension.is_present() {
        return css_dimension;
    }

    let css_ratio = parse_css_ratio(p);
    if css_ratio.is_present() {
        return css_ratio;
    }
    let css_number = parse_regular_number(p);
    if css_number.is_present() {
        return css_number;
    }

    Absent
}

#[inline]
pub(crate) fn is_at_css_custom_property(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.cur_text().starts_with("--")
}

#[inline]
pub(crate) fn parse_css_custom_property(p: &mut CssParser) -> ParsedSyntax {
    if is_at_css_custom_property(p) {
        let m = p.start();
        p.eat(T![-]);
        p.eat(T![-]);
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        return Present(m.complete(p, CSS_CUSTOM_PROPERTY));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_css_any_function(p: &mut CssParser) -> bool {
    p.at(T![ident]) && p.nth_at(1, T!['('])
}

#[inline]
pub(crate) fn parse_css_any_function(p: &mut CssParser) -> ParsedSyntax {
    if is_at_css_any_function(p) {
        let m = p.start();
        let simple_fn = p.start();
        // function name
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        p.eat(T!['(']);
        let func_params_m = p.start();

        while is_any_css_value(p) {
            let param = p.start();
            // cubic-bezier(0.1, 0.7, 1.0, 0.1)
            // repeating-radial-gradient(red, yellow 10%, green 15%);
            parse_any_css_value(p).ok();
            param.complete(p, CSS_PARAMETER);
            p.eat(T![,]);
        }

        func_params_m.complete(p, CSS_PARAMETER_LIST);
        p.expect(T![')']);
        simple_fn.complete(p, CSS_SIMPLE_FUNCTION);

        return Present(m.complete(p, CSS_ANY_FUNCTION));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_css_dimension(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && matches!(p.nth(1), T![%] | T![ident])
}

#[inline]
pub(crate) fn parse_css_dimension(p: &mut CssParser) -> ParsedSyntax {
    if is_at_css_dimension(p) {
        let m = p.start();
        let _css_number = parse_regular_number(p);
        let _ident = parse_regular_identifier(p);
        p.eat(T![%]);
        return Present(m.complete(p, CSS_DIMENSION));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_css_ratio(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![/])
}

#[inline]
pub(crate) fn parse_css_ratio(p: &mut CssParser) -> ParsedSyntax {
    if is_at_css_ratio(p) {
        let m = p.start();
         parse_regular_number(p).ok();
        p.eat(T![/]);
        parse_regular_number(p).or_add_diagnostic(p, expected_number);
        return Present(m.complete(p, CSS_RATIO));
    }
    Absent
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}
#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

#[inline]
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}

#[inline]
pub(crate) fn parse_regular_number(p: &mut CssParser) -> ParsedSyntax {
    parse_number(p, CssLexContext::Regular)
}
#[inline]
pub(crate) fn parse_number(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_NUMBER_LITERAL, context);

    Present(m.complete(p, CSS_NUMBER))
}

#[inline]
pub(crate) fn parse_string(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(CSS_STRING_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

