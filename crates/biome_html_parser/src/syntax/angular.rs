use crate::parser::HtmlParser;
use crate::syntax::AttrInitializerContext;
use crate::syntax::parse_attribute_initializer;
use crate::syntax::parse_error::expected_angular_name;
use crate::syntax::parse_error::expected_expression;
use crate::token_source::HtmlReLexContext;
use crate::token_source::RestrictedExpressionStopAt;
use crate::token_source::{HtmlFramework, HtmlLexContext};
use biome_html_syntax::HtmlSyntaxKind;
use biome_html_syntax::HtmlSyntaxKind::*;
use biome_html_syntax::T;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

pub const ANGULAR_KEYWORDS: TokenSet<HtmlSyntaxKind> = token_set!(T![let]);

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
    let block = p.start();
    p.bump_with_context(T![@], HtmlLexContext::Angular);

    match p.cur() {
        T![let] => {
            parse_let_block(p, block).ok();
        }
        _ => {
            p.error(p.err_builder(
                "Expected `if`, `for`, `switch`, `let`, or `defer`",
                p.cur_range(),
            ));
            block.complete(p, HTML_BOGUS_TEXT_EXPRESSION);
        }
    }

    Present(m.complete(p, ANY_ANGULAR_BLOCK))
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
