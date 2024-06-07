use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::feature::parse_any_query_feature;
use crate::syntax::block::parse_conditional_block;
use crate::syntax::parse_error::expected_non_css_wide_keyword_identifier;
use crate::syntax::{is_at_declaration, parse_custom_identifier, parse_declaration};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_container_at_rule(p: &mut CssParser) -> bool {
    p.at(T![container])
}

#[inline]
pub(crate) fn parse_container_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![container]);

    if parse_custom_identifier(p, CssLexContext::Regular)
        .ok()
        .is_none()
    {
        // Because the name is optional, we have to indirectly check if it's
        // a CSS-wide keyword that can't be used. If it was required, we could
        // use `.or_recover` or `.or_add_diagnostic` here instead.
        if p.cur().is_css_wide_keyword() {
            p.err_and_bump(
                expected_non_css_wide_keyword_identifier(p, p.cur_range()),
                CSS_BOGUS,
            )
        }
    };

    parse_any_container_query(p).ok(); // TODO handle error
    parse_conditional_block(p);

    Present(m.complete(p, CSS_CONTAINER_AT_RULE))
}

#[inline]
fn parse_any_container_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_not_query(p) {
        parse_container_not_query(p)
    } else {
        let query_in_parens = parse_any_container_query_in_parens(p);

        match p.cur() {
            T![and] => {
                let m = query_in_parens.precede(p);
                p.bump(T![and]);
                parse_container_and_query(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_CONTAINER_AND_QUERY))
            }
            T![or] => {
                let m = query_in_parens.precede(p);
                p.bump(T![or]);
                parse_container_or_query(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_CONTAINER_OR_QUERY))
            }
            _ => query_in_parens,
        }
    }
}

#[inline]
fn parse_container_and_query(p: &mut CssParser) -> ParsedSyntax {
    let query_in_parens = parse_any_container_query_in_parens(p);

    if p.at(T![and]) {
        let m = query_in_parens.precede(p);
        p.bump(T![and]);
        parse_container_and_query(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_CONTAINER_AND_QUERY))
    } else {
        query_in_parens
    }
}

#[inline]
fn parse_container_or_query(p: &mut CssParser) -> ParsedSyntax {
    let query_in_parens = parse_any_container_query_in_parens(p);

    if p.at(T![or]) {
        let m = query_in_parens.precede(p);
        p.bump(T![or]);
        parse_container_or_query(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_CONTAINER_OR_QUERY))
    } else {
        query_in_parens
    }
}

#[inline]
fn is_at_container_not_query(p: &mut CssParser) -> bool {
    p.at(T![not])
}
#[inline]
fn parse_container_not_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_not_query(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_container_query_in_parens(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_CONTAINER_NOT_QUERY))
}
#[inline]
fn parse_any_container_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_query_in_parens(p) {
        parse_container_query_in_parens(p)
    } else if is_at_container_style_query_in_parens(p) {
        parse_container_style_query_in_parens(p)
    } else if is_at_container_size_feature_in_parens(p) {
        parse_container_size_feature_in_parens(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_container_query_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['(']) && (p.nth_at(1, T![not]) || p.nth_at(1, T!['(']))
}

#[inline]
fn parse_container_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_query_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_container_query(p).ok(); // TODO handle error
    p.bump(T![')']);

    Present(m.complete(p, CSS_CONTAINER_QUERY_IN_PARENS))
}

#[inline]
fn is_at_container_size_feature_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_container_size_feature_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_size_feature_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_any_query_feature(p).ok(); // TODO handle error
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_SIZE_FEATURE_IN_PARENS))
}

#[inline]
fn is_at_container_style_query_in_parens(p: &mut CssParser) -> bool {
    p.at(T![style])
}

#[inline]
fn parse_container_style_query_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_style_query_in_parens(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![style]);
    p.expect(T!['(']);
    parse_any_container_style_query(p).ok(); // TODO handle error
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_STYLE_QUERY_IN_PARENS))
}

#[inline]
fn parse_any_container_style_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_style_not_query(p) {
        parse_container_style_not_query(p)
    } else if is_at_declaration(p) {
        parse_declaration(p)
    } else {
        parse_any_container_style_combinable_query(p)
    }
}

#[inline]
fn parse_any_container_style_combinable_query(p: &mut CssParser) -> ParsedSyntax {
    let style_in_parens = parse_container_style_in_parens(p);

    match p.cur() {
        T![and] => {
            let m = style_in_parens.precede(p);
            p.bump(T![and]);
            parse_any_container_style_combinable_query(p).ok(); // TODO handle error
            Present(m.complete(p, CSS_CONTAINER_STYLE_AND_QUERY))
        }
        T![or] => {
            let m = style_in_parens.precede(p);
            p.bump(T![or]);
            parse_any_container_style_combinable_query(p).ok(); // TODO handle error
            Present(m.complete(p, CSS_CONTAINER_STYLE_OR_QUERY))
        }
        _ => style_in_parens,
    }
}

#[inline]
fn is_at_container_style_not_query(p: &mut CssParser) -> bool {
    p.at(T![not]) && p.nth_at(1, T!['('])
}

#[inline]
fn parse_container_style_not_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_container_style_not_query(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_container_style_in_parens(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_CONTAINER_STYLE_NOT_QUERY))
}

#[inline]
fn parse_container_style_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    parse_any_container_style_query(p).ok(); // TODO handle error
    p.expect(T![')']);
    Present(m.complete(p, CSS_CONTAINER_STYLE_IN_PARENS))
}
