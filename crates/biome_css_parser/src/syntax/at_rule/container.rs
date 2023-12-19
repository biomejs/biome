use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::{
    is_at_any_value, is_at_identifier, parse_any_value, parse_declaration,
    parse_or_recover_rule_block, parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
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

    parse_regular_identifier(p).ok();

    parse_any_container_query(p).ok(); // TODO handle error

    if parse_or_recover_rule_block(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    Present(m.complete(p, CSS_CONTAINER_AT_RULE))
}

#[inline]
fn parse_any_container_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_container_not_query(p) {
        parse_container_not_query(p)
    } else {
        parse_any_container_combinable_query(p)
    }
}

#[inline]
fn parse_any_container_combinable_query(p: &mut CssParser) -> ParsedSyntax {
    let query_in_parens = parse_any_container_query_in_parens(p);

    match p.cur() {
        T![and] => {
            let m = query_in_parens.precede(p);
            p.bump(T![and]);
            parse_any_container_combinable_query(p).ok(); // TODO handle error
            Present(m.complete(p, CSS_CONTAINER_AND_QUERY))
        }
        T![or] => {
            let m = query_in_parens.precede(p);
            p.bump(T![or]);
            parse_any_container_combinable_query(p).ok(); // TODO handle error
            Present(m.complete(p, CSS_CONTAINER_OR_QUERY))
        }
        _ => query_in_parens,
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
    parse_any_size_feature(p).ok(); // TODO handle error
    p.expect(T![')']);

    Present(m.complete(p, CSS_CONTAINER_SIZE_FEATURE_IN_PARENS))
}

#[inline]
fn parse_any_size_feature(p: &mut CssParser) -> ParsedSyntax {
    if is_at_size_feature_range(p) {
        parse_size_feature_range(p)
    } else if is_at_size_feature_plain(p) {
        parse_size_feature_plain(p)
    } else if is_at_size_feature_boolean(p) {
        parse_size_feature_boolean(p)
    } else if is_at_size_feature_range_interval(p) {
        parse_size_feature_range_interval(p)
    } else {
        Absent
    }
}

#[inline]
fn is_at_size_feature_boolean(p: &mut CssParser) -> bool {
    is_at_identifier(p)
}

#[inline]
fn parse_size_feature_boolean(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_size_feature_boolean(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();

    Present(m.complete(p, CSS_SIZE_FEATURE_BOOLEAN))
}

#[inline]
fn is_at_size_feature_range(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at_ts(1, SIZE_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
}

#[inline]
fn parse_size_feature_range(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_size_feature_range(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();
    parse_size_feature_range_comparison(p).ok();
    parse_any_size_feature_value(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_SIZE_FEATURE_RANGE))
}

#[inline]
fn is_at_size_feature_range_interval(p: &mut CssParser) -> bool {
    is_at_any_size_feature_value(p)
}

#[inline]
fn parse_size_feature_range_interval(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_size_feature_range_interval(p) {
        return Absent;
    }

    let m = p.start();

    parse_any_size_feature_value(p).ok();
    parse_size_feature_range_comparison(p).ok();
    parse_regular_identifier(p).ok(); // TODO handle error
    parse_size_feature_range_comparison(p).ok(); // TODO handle error
    parse_any_size_feature_value(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_SIZE_FEATURE_RANGE_INTERVAL))
}

const SIZE_FEATURE_RANGE_COMPARISON_OPERATOR_SET: TokenSet<CssSyntaxKind> =
    token_set![T![>], T![<], T![>=], T![<=]];

#[inline]
fn is_at_size_feature_range_comparison(p: &mut CssParser) -> bool {
    p.at_ts(SIZE_FEATURE_RANGE_COMPARISON_OPERATOR_SET)
}

#[inline]
fn parse_size_feature_range_comparison(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_size_feature_range_comparison(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_ts(SIZE_FEATURE_RANGE_COMPARISON_OPERATOR_SET);

    Present(m.complete(p, CSS_SIZE_FEATURE_RANGE_COMPARISON))
}

#[inline]
fn is_at_size_feature_plain(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.nth_at(1, T![:])
}

#[inline]
fn parse_size_feature_plain(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_size_feature_plain(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    p.bump(T![:]);
    parse_any_size_feature_value(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_SIZE_FEATURE_PLAIN))
}

#[inline]
pub(crate) fn is_at_any_size_feature_value(p: &mut CssParser) -> bool {
    is_at_any_value(p)
}

#[inline]
fn parse_any_size_feature_value(p: &mut CssParser) -> ParsedSyntax {
    parse_any_value(p)
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
    } else if is_at_identifier(p) {
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
