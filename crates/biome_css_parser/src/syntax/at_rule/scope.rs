use crate::parser::CssParser;
use crate::syntax::at_rule::parse_error::expected_any_scope_range;
use crate::syntax::block::parse_conditional_block;
use crate::syntax::selector::SelectorList;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_scope_at_rule(p: &mut CssParser) -> bool {
    p.at(T![scope])
}

#[inline]
pub(crate) fn parse_scope_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scope_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![scope]);

    parse_any_scope_range(p).ok(); // it's optional

    parse_conditional_block(p);

    Present(m.complete(p, CSS_SCOPE_AT_RULE))
}

const SCOPE_RANGE_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set![T!['{']];

#[inline]
pub(crate) fn parse_any_scope_range(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scope_range_start_or_interval(p) {
        parse_scope_range_start_or_interval(p)
    } else if is_at_scope_range_end(p) {
        parse_scope_range_end(p)
    } else {
        Absent
    }
}

#[inline]
pub(crate) fn is_at_scope_range_start_or_interval(p: &mut CssParser) -> bool {
    is_at_scope_edge(p)
}

#[inline]
pub(crate) fn parse_scope_range_start_or_interval(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scope_range_start_or_interval(p) {
        return Absent;
    }

    let m = p.start();

    if parse_or_recover_scope_edge(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_SCOPE_RANGE));
    }

    let kind = if p.eat(T![to]) {
        if parse_or_recover_scope_edge(p).is_err() {
            return Present(m.complete(p, CSS_BOGUS_SCOPE_RANGE));
        }
        CSS_SCOPE_RANGE_INTERVAL
    } else {
        CSS_SCOPE_RANGE_START
    };

    Present(m.complete(p, kind))
}

#[inline]
pub(crate) fn is_at_scope_range_end(p: &mut CssParser) -> bool {
    p.at(T![to])
}

#[inline]
pub(crate) fn parse_scope_range_end(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scope_range_end(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![to]);

    if parse_or_recover_scope_edge(p).is_err() {
        return Present(m.complete(p, CSS_BOGUS_SCOPE_RANGE));
    }

    Present(m.complete(p, CSS_SCOPE_RANGE_END))
}

const SCOPE_EDGE_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    SCOPE_RANGE_RECOVERY_SET.union(token_set![T!['{']]);
#[inline]
pub(crate) fn parse_or_recover_scope_edge(p: &mut CssParser) -> RecoveryResult {
    parse_scope_edge(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(CSS_BOGUS, SCOPE_EDGE_RECOVERY_SET)
            .enable_recovery_on_line_break(),
        expected_any_scope_range,
    )
}
#[inline]
pub(crate) fn is_at_scope_edge(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
pub(crate) fn parse_scope_edge(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scope_edge(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    SelectorList::default()
        .with_end_kind_ts(SCOPE_EDGE_SELECTOR_LIST_END_SET)
        .parse_list(p);
    p.expect(T![')']);

    Present(m.complete(p, CSS_SCOPE_EDGE))
}

const SCOPE_EDGE_SELECTOR_LIST_END_SET: TokenSet<CssSyntaxKind> = token_set![T![')'], T!['{']];
