use crate::parser::HtmlParser;
use crate::syntax::AttrInitializerContext;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_expression;
use crate::syntax::parse_error::{expected_angular_name, expected_child};
use crate::syntax::parse_html_element;
use crate::token_source::HtmlReLexContext;
use crate::token_source::RestrictedExpressionStopAt;
use crate::token_source::{HtmlFramework, HtmlLexContext};
use biome_html_syntax::HtmlSyntaxKind;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parse_recovery::RecoveryResult;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub const ANGULAR_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(
    T![let],
    T![if],
    T![else],
    T![as],
    T![for],
    T![empty],
    T![track],
    T![of],
    T![switch],
    T![case],
    T![default],
    T![defer],
    T![placeholder],
    T![loading],
    T![error],
    T![minimum],
    T![when],
    T![prefetch],
    T![on],
    T![after],
    T![hydrate],
    T![never],
);

pub(crate) fn parse_angular_event_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T!['('],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    parse_angular_binding_name(p).ok();
    p.expect_with_context(
        T![')'],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, ANGULAR_EVENT_BINDING))
}

fn parse_angular_binding_name(p: &mut HtmlParser) -> ParsedSyntax {
    parse_angular_binding_name_with_context(
        p,
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    )
}

fn parse_angular_binding_name_with_context(
    p: &mut HtmlParser,
    context: HtmlLexContext,
) -> ParsedSyntax {
    let m = p.start();

    if p.at(HTML_LITERAL) {
        p.bump_with_context(HTML_LITERAL, context);
    } else {
        p.error(expected_angular_name(p, p.cur_range()));
    }

    Present(m.complete(p, ANGULAR_BINDING_NAME))
}

pub(crate) fn parse_angular_property_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T!['['],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    parse_angular_binding_name(p).ok();
    p.expect_with_context(
        T![']'],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, ANGULAR_PROPERTY_BINDING))
}

pub(crate) fn parse_angular_two_way_binding(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!["[("]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T!["[("],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    parse_angular_binding_name(p).ok();
    p.expect_with_context(
        T![")]"],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, ANGULAR_TWO_WAY_BINDING))
}

pub(crate) fn parse_angular_structural_directive(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![*]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T![*],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    parse_angular_binding_name(p).ok();
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, ANGULAR_STRUCTURAL_DIRECTIVE))
}

pub(crate) fn parse_angular_template_ref(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![#]) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T![#],
        HtmlLexContext::InsideTag {
            framework: HtmlFramework::Angular,
        },
    );
    parse_angular_binding_name(p).ok();
    if p.at(T![=]) {
        parse_attribute_initializer(p, AttrInitializerContext::Regular).ok();
    }

    Present(m.complete(p, ANGULAR_TEMPLATE_REF_VARIABLE))
}

pub(crate) fn parse_angular_block(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![@]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    match p.cur() {
        T![let] => parse_let_block(p, m),
        T![if] => parse_if_block(p, m),
        T![for] => parse_for_block(p, m),
        T![switch] => parse_switch_block(p, m),
        T![defer] => parse_defer_block(p, m),
        _ => {
            p.error(p.err_builder(
                "Expected `if`, `for`, `switch`, `let`, or `defer`",
                p.cur_range(),
            ));
            Present(m.complete(p, HTML_BOGUS_TEXT_EXPRESSION))
        }
    }
}

pub(crate) fn parse_let_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![let]) {
        marker.abandon(p);
        return Absent;
    }

    p.bump_with_context(T![let], HtmlLexContext::Angular);

    parse_angular_binding_name_with_context(p, HtmlLexContext::Angular).ok();

    parse_angular_let_initializer_clause(p).ok();

    p.re_lex(HtmlReLexContext::Angular);

    p.expect(T![;]);

    Present(marker.complete(p, ANGULAR_LET_BLOCK))
}

fn parse_angular_let_initializer_clause(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    p.expect_with_context(
        T![=],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_LET_INITIALIZER_CLAUSE))
}

fn parse_angular_text_expression(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at_ts(token_set![T![')'], T![;], T!['}'], EOF]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap(HTML_LITERAL);
    Present(m.complete(p, HTML_TEXT_EXPRESSION))
}

pub(crate) fn parse_if_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![if]) {
        marker.abandon(p);
        return Absent;
    }

    let opening = parse_if_opening_block(p, marker);
    let m = opening.precede(p);

    re_lex_angular_clause_start(p);
    AngularElseIfClauseList.parse_list(p);
    re_lex_angular_clause_start(p);
    parse_else_clause(p).ok();

    Present(m.complete(p, ANGULAR_IF_BLOCK))
}

fn parse_if_opening_block(p: &mut HtmlParser, marker: Marker) -> CompletedMarker {
    p.bump_with_context(T![if], HtmlLexContext::Angular);

    parse_angular_if_parameters(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    marker.complete(p, ANGULAR_IF_OPENING_BLOCK)
}

fn parse_else_if_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![else]) || !p.nth_at(1, T![if]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![else], HtmlLexContext::Angular);
    p.bump_with_context(T![if], HtmlLexContext::Angular);

    parse_angular_if_parameters(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_ELSE_IF_CLAUSE))
}

fn parse_else_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![else]) || p.nth_at(1, T![if]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![else], HtmlLexContext::Angular);

    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_ELSE_CLAUSE))
}

fn parse_angular_if_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T!['('],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    p.re_lex(HtmlReLexContext::Angular);
    parse_angular_if_as_clause(p).ok();
    p.re_lex(HtmlReLexContext::Angular);

    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_IF_PARAMETERS))
}

fn parse_angular_if_as_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![;]) {
        return Absent;
    }

    let has_as = p.lookahead(|p| {
        p.bump_with_context(T![;], HtmlLexContext::Angular);
        p.at(T![as])
    });

    if !has_as {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![;], HtmlLexContext::Angular);
    p.bump_with_context(
        T![as],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::ClosingParen),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_IF_AS_CLAUSE))
}

fn parse_angular_block_body(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['{']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['{'], HtmlLexContext::Regular);

    AngularElementList.parse_list(p);

    p.expect(T!['}']);

    Present(m.complete(p, ANGULAR_BLOCK_BODY))
}

fn re_lex_angular_clause_start(p: &mut HtmlParser) {
    if p.at(HTML_LITERAL) && p.cur_text().starts_with('@') {
        p.re_lex(HtmlReLexContext::Angular);
    }
}

#[derive(Debug)]
struct AngularElseIfClauseList;

impl ParseNodeList for AngularElseIfClauseList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = ANGULAR_ELSE_IF_CLAUSE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_else_if_clause(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        re_lex_angular_clause_start(p);

        if !p.at(T![@]) {
            return true;
        }

        let checkpoint = p.checkpoint();
        let m = p.start();
        p.bump_with_context(T![@], HtmlLexContext::Angular);
        let is_else_if = p.at(T![else]) && p.nth_at(1, T![if]);
        m.abandon(p);
        p.rewind(checkpoint);

        !is_else_if
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T![@], T!['}']]),
            expected_child,
        )
    }
}

#[derive(Debug)]
struct AngularElementList;

impl ParseNodeList for AngularElementList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = HTML_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_angular_html_element(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        re_lex_angular_block_boundary(p);
        p.at(T!['}']) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(
                HTML_BOGUS_ELEMENT,
                token_set![T![<], T![>], T![@], T!['}']],
            ),
            expected_child,
        )
    }
}

fn parse_angular_html_element(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_block_boundary(p);

    if p.at(T![;]) {
        let m = p.start();
        p.bump_remap(HTML_LITERAL);
        return Present(m.complete(p, HTML_CONTENT));
    }

    parse_html_element(p)
}

fn re_lex_angular_block_boundary(p: &mut HtmlParser) {
    if p.at(HTML_LITERAL) && p.cur_text().contains('}') {
        p.re_lex(HtmlReLexContext::Angular);
    }
}

pub(crate) fn parse_for_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![for]) {
        marker.abandon(p);
        return Absent;
    }

    let opening = parse_for_opening_block(p, marker);
    let m = opening.precede(p);

    re_lex_angular_clause_start(p);
    parse_empty_clause(p).ok();

    Present(m.complete(p, ANGULAR_FOR_BLOCK))
}

fn parse_for_opening_block(p: &mut HtmlParser, marker: Marker) -> CompletedMarker {
    p.bump_with_context(T![for], HtmlLexContext::Angular);

    parse_angular_for_parameters(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    marker.complete(p, ANGULAR_FOR_OPENING_BLOCK)
}

fn parse_angular_for_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T!['('],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Of),
    );

    parse_angular_for_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    p.re_lex(HtmlReLexContext::Angular);
    p.expect_with_context(T![;], HtmlLexContext::Angular);

    parse_angular_for_track_clause(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected a `track` clause in @for parameters.", range)
    });
    parse_angular_for_let_clause(p).ok();

    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_FOR_PARAMETERS))
}

fn parse_angular_for_expression(p: &mut HtmlParser) -> ParsedSyntax {
    let m = p.start();

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    p.re_lex(HtmlReLexContext::Angular);
    p.expect_with_context(
        T![of],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_FOR_EXPRESSION))
}

fn parse_angular_for_track_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![track]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T![track],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    p.re_lex(HtmlReLexContext::Angular);

    Present(m.complete(p, ANGULAR_FOR_TRACK_CLAUSE))
}

fn parse_angular_for_let_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![;]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![;], HtmlLexContext::Angular);
    p.expect_with_context(
        T![let],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Comma),
    );

    AngularForLetBindingList.parse_list(p);

    Present(m.complete(p, ANGULAR_FOR_LET_CLAUSE))
}

fn parse_empty_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![empty]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![empty], HtmlLexContext::Angular);

    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_EMPTY_CLAUSE))
}

#[derive(Debug)]
struct AngularForLetBindingList;

impl ParseSeparatedList for AngularForLetBindingList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = ANGULAR_FOR_LET_BINDING_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if !p.at(HTML_LITERAL) {
            return Absent;
        }
        let m = p.start();
        p.bump_remap_with_context(HTML_LITERAL, HtmlLexContext::Angular);
        Present(m.complete(p, HTML_TEXT_EXPRESSION))
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T![,], T![')']]),
            expected_child,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(
            T![,],
            HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Comma),
        )
    }
}

pub(crate) fn parse_switch_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![switch]) {
        marker.abandon(p);
        return Absent;
    }

    let opening = parse_switch_opening_block(p, marker);
    let m = opening.precede(p);

    p.expect(T!['{']);

    re_lex_angular_clause_start(p);
    AngularCaseClauseList.parse_list(p);
    re_lex_angular_clause_start(p);
    parse_default_clause(p).ok();

    while !p.at(T!['}']) && !p.at(EOF) {
        re_lex_angular_clause_start(p);
        if p.at(T![@]) {
            let m = p.start();
            p.bump_remap(HTML_LITERAL);
            m.complete(p, HTML_BOGUS);
            continue;
        }

        if parse_html_element(p).is_present() {
            continue;
        }

        let m = p.start();
        p.bump_remap(HTML_LITERAL);
        m.complete(p, HTML_BOGUS);
    }

    p.expect(T!['}']);

    Present(m.complete(p, ANGULAR_SWITCH_BLOCK))
}

fn parse_switch_opening_block(p: &mut HtmlParser, marker: Marker) -> CompletedMarker {
    p.bump_with_context(T![switch], HtmlLexContext::Angular);

    parse_angular_switch_parameters(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    marker.complete(p, ANGULAR_SWITCH_OPENING_BLOCK)
}

fn parse_case_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![case]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![case], HtmlLexContext::Angular);

    parse_angular_switch_parameters(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    parse_angular_block_body(p).ok();

    Present(m.complete(p, ANGULAR_CASE_CLAUSE))
}

fn parse_angular_switch_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T!['('],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::ClosingParen),
    );

    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));

    p.re_lex(HtmlReLexContext::Angular);

    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_SWITCH_PARAMETERS))
}

fn parse_default_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![default]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![default], HtmlLexContext::Angular);

    if parse_angular_block_body(p).is_absent() {
        parse_angular_default_expression_clause(p).or_add_diagnostic(p, |p, range| {
            p.err_builder("Expected a block body or expression.", range)
        });
    }

    Present(m.complete(p, ANGULAR_DEFAULT_CLAUSE))
}

fn parse_angular_default_expression_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at(T!['{']) || p.at(T![@]) || p.at(T!['}']) || p.at(EOF) {
        return Absent;
    }

    let m = p.start();
    parse_angular_text_expression(p).or_add_diagnostic(p, |p, range| expected_expression(p, range));
    p.re_lex(HtmlReLexContext::Angular);
    p.expect(T![;]);

    Present(m.complete(p, ANGULAR_DEFAULT_EXPRESSION_CLAUSE))
}

#[derive(Debug)]
struct AngularCaseClauseList;

impl ParseNodeList for AngularCaseClauseList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = ANGULAR_CASE_CLAUSE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_case_clause(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        re_lex_angular_clause_start(p);

        if !p.at(T![@]) {
            return true;
        }

        let checkpoint = p.checkpoint();
        let m = p.start();
        p.bump_with_context(T![@], HtmlLexContext::Angular);
        let is_case = p.at(T![case]);
        m.abandon(p);
        p.rewind(checkpoint);

        !is_case
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T![@], T!['}']]),
            expected_child,
        )
    }
}

pub(crate) fn parse_defer_block(p: &mut HtmlParser, marker: Marker) -> ParsedSyntax {
    if !p.at(T![defer]) {
        marker.abandon(p);
        return Absent;
    }

    let opening = parse_defer_opening_block(p, marker);
    let m = opening.precede(p);

    re_lex_angular_clause_start(p);
    parse_placeholder_clause(p).ok();
    re_lex_angular_clause_start(p);
    parse_loading_clause(p).ok();
    re_lex_angular_clause_start(p);
    parse_error_clause(p).ok();

    Present(m.complete(p, ANGULAR_DEFER_BLOCK))
}

fn parse_defer_opening_block(p: &mut HtmlParser, marker: Marker) -> CompletedMarker {
    p.bump_with_context(T![defer], HtmlLexContext::Angular);

    parse_angular_defer_parameters(p).ok();
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    marker.complete(p, ANGULAR_DEFER_OPENING_BLOCK)
}

fn parse_angular_defer_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['('], HtmlLexContext::Angular);

    AngularDeferClauseList.parse_list(p);

    p.re_lex(HtmlReLexContext::Angular);
    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_DEFER_PARAMETERS))
}

fn parse_placeholder_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![placeholder]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![placeholder], HtmlLexContext::Angular);

    parse_angular_placeholder_parameters(p).ok();
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_PLACEHOLDER_CLAUSE))
}

fn parse_angular_placeholder_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['('], HtmlLexContext::Angular);

    parse_angular_minimum_time_clause(p).or_add_diagnostic(p, |p, range| {
        p.err_builder("Expected a `minimum` clause.", range)
    });

    p.re_lex(HtmlReLexContext::Angular);
    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_PLACEHOLDER_PARAMETERS))
}

fn parse_angular_minimum_time_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![minimum]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T![minimum],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::ClosingParen),
    );

    parse_angular_expression_into_angular_context(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_MINIMUM_TIME_CLAUSE))
}

fn parse_angular_after_time_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![after]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T![after],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_expression_into_angular_context(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_AFTER_TIME_CLAUSE))
}

fn parse_loading_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![loading]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![loading], HtmlLexContext::Angular);

    parse_angular_loading_parameters(p).ok();
    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_LOADING_CLAUSE))
}

fn parse_angular_loading_parameters(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T!['('], HtmlLexContext::Angular);

    AngularLoadingParameterClauseList.parse_list(p);

    p.re_lex(HtmlReLexContext::Angular);
    p.expect(T![')']);

    Present(m.complete(p, ANGULAR_LOADING_PARAMETERS))
}

fn parse_error_clause(p: &mut HtmlParser) -> ParsedSyntax {
    re_lex_angular_clause_start(p);
    if !p.at(T![@]) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    if !p.at(T![error]) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    p.bump_with_context(T![error], HtmlLexContext::Angular);

    parse_angular_block_body(p)
        .or_add_diagnostic(p, |p, range| p.err_builder("Expected a block body.", range));

    Present(m.complete(p, ANGULAR_ERROR_CLAUSE))
}

#[derive(Debug)]
struct AngularDeferClauseList;

impl ParseSeparatedList for AngularDeferClauseList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = ANGULAR_DEFER_CLAUSE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_angular_defer_clause(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T![;], T![')']]),
            expected_child,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(T![;], HtmlLexContext::Angular)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_any_angular_defer_clause(p: &mut HtmlParser) -> ParsedSyntax {
    match p.cur() {
        T![on] => parse_angular_defer_on_clause(p),
        T![when] => parse_angular_defer_when_clause(p),
        T![prefetch] => parse_angular_defer_prefetch_clause(p),
        T![hydrate] => parse_angular_defer_hydrate_clause(p),
        _ => Absent,
    }
}

fn parse_angular_defer_on_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T![on],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_expression_into_angular_context(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_DEFER_ON_CLAUSE))
}

fn parse_angular_defer_when_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![when]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(
        T![when],
        HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
    );

    parse_angular_expression_into_angular_context(p)
        .or_add_diagnostic(p, |p, range| expected_expression(p, range));

    Present(m.complete(p, ANGULAR_DEFER_WHEN_CLAUSE))
}

fn parse_angular_defer_prefetch_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![prefetch]) {
        return Absent;
    }

    let next = p.lookahead(|p| {
        p.bump_with_context(T![prefetch], HtmlLexContext::Angular);
        p.cur()
    });

    let m = p.start();
    p.bump_with_context(T![prefetch], HtmlLexContext::Angular);

    if next == T![on] {
        p.bump_with_context(
            T![on],
            HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
        );
        parse_angular_expression_into_angular_context(p)
            .or_add_diagnostic(p, |p, range| expected_expression(p, range));
        Present(m.complete(p, ANGULAR_DEFER_PREFETCH_ON_CLAUSE))
    } else {
        p.expect_with_context(
            T![when],
            HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
        );
        parse_angular_expression_into_angular_context(p)
            .or_add_diagnostic(p, |p, range| expected_expression(p, range));
        Present(m.complete(p, ANGULAR_DEFER_PREFETCH_WHEN_CLAUSE))
    }
}

/// Like `parse_angular_text_expression`, but bumps the expression token using Angular context
/// so the next token is positioned in Angular context (e.g., at `;` or `)`).
fn parse_angular_expression_into_angular_context(p: &mut HtmlParser) -> ParsedSyntax {
    if p.at_ts(token_set![T![')'], T![;], T!['}'], EOF]) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(HTML_LITERAL, HtmlLexContext::Angular);
    Present(m.complete(p, HTML_TEXT_EXPRESSION))
}

fn parse_angular_defer_hydrate_clause(p: &mut HtmlParser) -> ParsedSyntax {
    if !p.at(T![hydrate]) {
        return Absent;
    }

    let next = p.lookahead(|p| {
        p.bump_with_context(T![hydrate], HtmlLexContext::Angular);
        p.cur()
    });

    let m = p.start();
    p.bump_with_context(T![hydrate], HtmlLexContext::Angular);

    match next {
        T![on] => {
            p.bump_with_context(
                T![on],
                HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
            );
            parse_angular_expression_into_angular_context(p)
                .or_add_diagnostic(p, |p, range| expected_expression(p, range));
            Present(m.complete(p, ANGULAR_DEFER_HYDRATE_ON_CLAUSE))
        }
        T![when] => {
            p.bump_with_context(
                T![when],
                HtmlLexContext::restricted_expression(RestrictedExpressionStopAt::Semicolon),
            );
            parse_angular_expression_into_angular_context(p)
                .or_add_diagnostic(p, |p, range| expected_expression(p, range));
            Present(m.complete(p, ANGULAR_DEFER_HYDRATE_WHEN_CLAUSE))
        }
        T![never] => {
            p.bump_with_context(T![never], HtmlLexContext::Angular);
            Present(m.complete(p, ANGULAR_DEFER_HYDRATE_NEVER_CLAUSE))
        }
        _ => {
            p.error(expected_angular_name(p, p.cur_range()));
            Present(m.complete(p, ANGULAR_DEFER_HYDRATE_NEVER_CLAUSE))
        }
    }
}

#[derive(Debug)]
struct AngularLoadingParameterClauseList;

impl ParseSeparatedList for AngularLoadingParameterClauseList {
    type Kind = HtmlSyntaxKind;
    type Parser<'source> = HtmlParser<'source>;
    const LIST_KIND: Self::Kind = ANGULAR_LOADING_PARAMETER_CLAUSE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![after] => parse_angular_after_time_clause(p),
            T![minimum] => parse_angular_minimum_time_clause(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')']) || p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(HTML_BOGUS, token_set![T![;], T![')']]),
            expected_child,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![;]
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        p.expect_with_context(T![;], HtmlLexContext::Angular)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}
