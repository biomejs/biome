use super::parse_error::expected_media_query;
use crate::parser::CssParser;
use crate::syntax::at_rule::feature::parse_any_query_feature;
use crate::syntax::block::parse_conditional_block;
use crate::syntax::{
    is_at_identifier, is_at_metavariable, is_nth_at_identifier, parse_metavariable,
    parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

#[inline]
pub(crate) fn is_at_media_at_rule(p: &mut CssParser) -> bool {
    p.at(T![media])
}

#[inline]
pub(crate) fn parse_media_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_media_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![media]);

    MediaQueryList::new(T!['{']).parse_list(p);

    parse_conditional_block(p);

    Present(m.complete(p, CSS_MEDIA_AT_RULE))
}

pub(crate) struct MediaQueryList {
    end_kind: CssSyntaxKind,
}

impl MediaQueryList {
    pub(crate) fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

impl ParseSeparatedList for MediaQueryList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_MEDIA_QUERY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_media_query(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_MEDIA_QUERY, token_set!(T![,], T!['{'])),
            expected_media_query,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

#[inline]
fn parse_any_media_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_media_type_query(p) {
        parse_any_media_type_query(p)
    } else if is_at_metavariable(p) {
        parse_metavariable(p)
    } else if is_at_any_media_condition(p) {
        let m = p.start();
        parse_any_media_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_MEDIA_CONDITION_QUERY))
    } else {
        Absent
    }
}

#[inline]
fn is_at_any_media_condition(p: &mut CssParser) -> bool {
    is_at_media_not_condition(p) || is_at_any_media_in_parens(p)
}

#[inline]
fn parse_any_media_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_media_condition(p) {
        return Absent;
    }

    if is_at_media_not_condition(p) {
        parse_media_not_condition(p)
    } else {
        let media_in_parens = parse_any_media_in_parens(p);

        match p.cur() {
            T![and] => {
                let m = media_in_parens.precede(p);
                p.expect(T![and]); // TODO handle error
                parse_media_and_condition(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_MEDIA_AND_CONDITION))
            }
            T![or] => {
                let m = media_in_parens.precede(p);
                p.expect(T![or]); // TODO handle error
                parse_media_or_condition(p).ok(); // TODO handle error
                Present(m.complete(p, CSS_MEDIA_OR_CONDITION))
            }
            _ => media_in_parens,
        }
    }
}

const MODIFIER_TYPE_QUERY_SET: TokenSet<CssSyntaxKind> = token_set!(T![only], T![not]);

#[inline]
fn parse_any_media_type_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_media_type_query(p) {
        return Absent;
    }

    let media_type_query = parse_media_type_query(p);

    if p.at(T![and]) {
        let m = media_type_query.precede(p);
        p.bump(T![and]);
        parse_any_media_type_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_MEDIA_AND_TYPE_QUERY))
    } else {
        media_type_query
    }
}
#[inline]
fn is_at_media_type_query(p: &mut CssParser) -> bool {
    (p.at_ts(MODIFIER_TYPE_QUERY_SET) && is_nth_at_identifier(p, 1))
        || (is_at_identifier(p) && !p.at(T![not]))
}
#[inline]
fn parse_media_type_query(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_media_type_query(p) {
        return Absent;
    }

    let m = p.start();

    p.eat_ts(MODIFIER_TYPE_QUERY_SET);
    parse_media_type(p).ok();

    Present(m.complete(p, CSS_MEDIA_TYPE_QUERY))
}

#[inline]
fn parse_media_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();

    Present(m.complete(p, CSS_MEDIA_TYPE))
}

#[inline]
fn is_at_any_media_type_condition(p: &mut CssParser) -> bool {
    is_at_media_not_condition(p) || is_at_any_media_in_parens(p)
}
#[inline]
fn parse_any_media_type_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_media_type_condition(p) {
        return Absent;
    }

    if is_at_media_not_condition(p) {
        parse_media_not_condition(p)
    } else {
        parse_media_and_condition(p)
    }
}

#[inline]
fn is_at_media_not_condition(p: &mut CssParser) -> bool {
    p.at(T![not])
}
#[inline]
fn parse_media_not_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_media_not_condition(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![not]);
    parse_any_media_in_parens(p).ok(); // TODO handle error

    Present(m.complete(p, CSS_MEDIA_NOT_CONDITION))
}

#[inline]
fn parse_media_and_condition(p: &mut CssParser) -> ParsedSyntax {
    let media_in_parens = parse_any_media_in_parens(p);

    if p.at(T![and]) {
        let m = media_in_parens.precede(p);
        p.expect(T![and]); // TODO handle error
        parse_media_and_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_MEDIA_AND_CONDITION))
    } else {
        media_in_parens
    }
}

#[inline]
fn parse_media_or_condition(p: &mut CssParser) -> ParsedSyntax {
    let media_in_parens = parse_any_media_in_parens(p);

    if p.at(T![or]) {
        let m = media_in_parens.precede(p);
        p.expect(T![or]); // TODO handle error
        parse_media_or_condition(p).ok(); // TODO handle error
        Present(m.complete(p, CSS_MEDIA_OR_CONDITION))
    } else {
        media_in_parens
    }
}

#[inline]
fn is_at_any_media_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

#[inline]
fn parse_any_media_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_media_in_parens(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let kind = if is_at_any_media_condition(p) {
        parse_any_media_condition(p).ok(); //TODO handle error
        CSS_MEDIA_CONDITION_IN_PARENS
    } else {
        parse_any_query_feature(p).ok(); //TODO handle error
        CSS_MEDIA_FEATURE_IN_PARENS
    };

    p.expect(T![')']); //TODO handle error

    Present(m.complete(p, kind))
}
