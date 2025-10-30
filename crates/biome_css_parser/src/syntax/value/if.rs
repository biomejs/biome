use biome_css_syntax::CssSyntaxKind;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecovery;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::ParsedSyntax;

use crate::parser::CssParser;
use crate::syntax::at_rule::container::parse_any_container_style_query;
use crate::syntax::at_rule::feature::parse_any_query_feature;
use crate::syntax::at_rule::media::is_at_any_media_condition;
use crate::syntax::at_rule::media::parse_any_media_condition;
use crate::syntax::at_rule::supports::parse_any_supports_condition;
use crate::syntax::is_at_declaration;
use crate::syntax::parse_declaration;
use crate::syntax::parse_regular_identifier;
use crate::syntax::property::GenericComponentValueList;
use crate::syntax::property::parse_generic_component_value;
use crate::syntax::value::parse_error::expected_url_modifier;

// TODO: BOGUS

pub(crate) fn is_at_if_function(p: &mut CssParser) -> bool {
    p.at(T![if]) && p.nth_at(1, T!['('])
}

pub(crate) fn parse_if_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_function(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![if]);
    p.bump(T!['(']);

    CssIfBranchList.parse_list(p);

    p.expect(T![')']);

    Present(m.complete(p, CSS_IF_FUNCTION))
}

#[inline]
fn is_at_if_supports_test(p: &mut CssParser) -> bool {
    p.at(T![supports]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_if_supports_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_supports_test(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![supports]);
    p.bump(T!['(']);

    if is_at_declaration(p) {
        parse_declaration(p).ok();
    } else {
        parse_any_supports_condition(p).ok();
    }

    p.bump(T![')']);

    Present(m.complete(p, CSS_IF_SUPPORTS_TEST))
}

#[inline]
fn is_at_if_style_test(p: &mut CssParser) -> bool {
    p.at(T![style]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_if_style_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_style_test(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![style]);
    p.bump(T!['(']);

    parse_any_container_style_query(p).ok();

    p.bump(T![')']);

    Present(m.complete(p, CSS_IF_STYLE_TEST))
}

#[inline]
fn is_at_if_media_test(p: &mut CssParser) -> bool {
    p.at(T![media]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_if_media_test(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_media_test(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![media]);
    p.bump(T!['(']);

    if is_at_any_media_condition(p) {
        parse_any_media_condition(p).ok();
    } else {
        parse_any_query_feature(p).ok();
    }

    p.bump(T![')']);

    Present(m.complete(p, CSS_IF_MEDIA_TEST))
}

#[inline]
fn parse_if_test(p: &mut CssParser) -> ParsedSyntax {
    if is_at_if_supports_test(p) {
        return parse_if_supports_test(p);
    }

    if is_at_if_style_test(p) {
        return parse_if_style_test(p);
    }

    if is_at_if_media_test(p) {
        return parse_if_media_test(p);
    }

    Absent
}

#[inline]
fn parse_any_if_test_boolean_expr_group(p: &mut CssParser) -> ParsedSyntax {
    // ( <boolean-expr> )
    if p.at(T!['(']) {
        let m = p.start();
        p.bump(T!['(']);
        parse_any_if_test_boolean_expr(p).ok();
        p.expect(T![')']);
        return Present(m.complete(p, CSS_IF_TEST_BOOLEAN_EXPR_IN_PARENS));
    }

    // <if-test>
    parse_if_test(p)
}

fn is_at_if_test_boolean_not_expr(p: &mut CssParser) -> bool {
    p.at(T![not])
}

/// not <boolean-expr-group>
#[inline]
fn parse_if_test_boolean_not_expr(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_if_test_boolean_not_expr(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_if_test_boolean_expr_group(p).ok();

    Present(m.complete(p, CSS_IF_TEST_BOOLEAN_NOT_EXPR))
}

/// Parse <boolean-expr-group> and <boolean-expr-group> ...
#[inline]
fn parse_if_test_boolean_and_expr(p: &mut CssParser) -> ParsedSyntax {
    let group = parse_any_if_test_boolean_expr_group(p);

    if p.at(T![and]) {
        let m = group.precede(p);
        p.expect(T![and]);
        parse_if_test_boolean_and_expr(p).ok();
        Present(m.complete(p, CSS_IF_TEST_BOOLEAN_AND_EXPR))
    } else {
        group
    }
}

/// Parse <boolean-expr-group> or <boolean-expr-group> ...
#[inline]
fn parse_if_test_boolean_or_expr(p: &mut CssParser) -> ParsedSyntax {
    let group = parse_if_test_boolean_and_expr(p);

    if p.at(T![or]) {
        let m = group.precede(p);
        p.expect(T![or]);
        parse_if_test_boolean_or_expr(p).ok();
        Present(m.complete(p, CSS_IF_TEST_BOOLEAN_OR_EXPR))
    } else {
        group
    }
}

#[inline]
fn parse_any_if_test_boolean_expr(p: &mut CssParser) -> ParsedSyntax {
    if is_at_if_test_boolean_not_expr(p) {
        return parse_if_test_boolean_not_expr(p);
    }

    let group = parse_any_if_test_boolean_expr_group(p);

    match p.cur() {
        T![and] => {
            let m = group.precede(p);
            p.expect(T![and]);
            parse_if_test_boolean_and_expr(p).ok();
            Present(m.complete(p, CSS_IF_TEST_BOOLEAN_AND_EXPR))
        }
        T![or] => {
            let m = group.precede(p);
            p.expect(T![or]);
            parse_if_test_boolean_or_expr(p).ok();
            Present(m.complete(p, CSS_IF_TEST_BOOLEAN_OR_EXPR))
        }
        _ => group,
    }
}

#[inline]
fn parse_any_if_condition(p: &mut CssParser) -> ParsedSyntax {
    if p.at(T![else]) {
        let m = p.start();
        p.bump(T![else]);
        return Present(m.complete(p, CSS_ELSE_KEYWORD));
    }

    parse_any_if_test_boolean_expr(p)
}

// TODO: BOGUS
#[inline]
fn parse_if_branch(p: &mut CssParser) -> ParsedSyntax {
    let m = p.start();

    parse_any_if_condition(p).ok();
    p.bump(T![:]);
    GenericComponentValueList.parse_list(p);

    Present(m.complete(p, CSS_IF_BRANCH))
}

struct IfBranchListParseRecovery;

impl ParseRecovery for IfBranchListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;

    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_IF_BRANCH;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;]) || p.at(T![')'])
    }
}

struct CssIfBranchList;

impl ParseSeparatedList for CssIfBranchList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_IF_BRANCH_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_if_branch(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        // TODO:handle right expected kind
        parsed_element.or_recover(p, &IfBranchListParseRecovery, expected_url_modifier)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}
