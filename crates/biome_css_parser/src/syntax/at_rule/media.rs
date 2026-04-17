use super::parse_error::expected_media_query;
use crate::parser::CssParser;
use crate::syntax::at_rule::error::{AnyInParensChainParseRecovery, AnyInParensParseRecovery};
use crate::syntax::at_rule::feature::{expected_any_query_feature, parse_any_query_feature};
use crate::syntax::block::parse_conditional_block;
use crate::syntax::util::skip_possible_tailwind_syntax;
use crate::syntax::{
    is_at_identifier, is_at_metavariable, is_nth_at_identifier, parse_metavariable,
    parse_regular_identifier,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::diagnostic::expected_any;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_rowan::TextRange;

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

    parse_media_at_rule_declarator(p, T!['{']).ok();
    parse_conditional_block(p);

    Present(m.complete(p, CSS_MEDIA_AT_RULE))
}

#[inline]
pub(crate) fn parse_media_at_rule_declarator(
    p: &mut CssParser,
    end_kind: CssSyntaxKind,
) -> ParsedSyntax {
    if !is_at_media_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![media]);
    MediaQueryList::new(end_kind).parse_list(p);

    Present(m.complete(p, CSS_MEDIA_AT_RULE_DECLARATOR))
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
        let parsed = parse_any_media_query(p);
        if parsed.is_present() {
            skip_possible_tailwind_syntax(p);
        }
        parsed
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
pub(crate) fn is_at_any_media_query(p: &mut CssParser) -> bool {
    is_at_media_type_query(p) || is_at_metavariable(p) || is_at_any_media_condition(p)
}

#[inline]
pub(crate) fn parse_any_media_query(p: &mut CssParser) -> ParsedSyntax {
    if is_at_media_type_query(p) {
        parse_any_media_type_query(p)
    } else if is_at_metavariable(p) {
        parse_metavariable(p)
    } else if is_at_any_media_condition(p) {
        let m = p.start();
        // Guarded by `is_at_any_media_condition` above.
        parse_any_media_condition(p).ok();
        Present(m.complete(p, CSS_MEDIA_CONDITION_QUERY))
    } else {
        Absent
    }
}

#[inline]
pub(crate) fn is_at_any_media_condition(p: &mut CssParser) -> bool {
    is_at_media_not_condition(p) || is_at_any_media_in_parens(p)
}

/// Parses a media condition, including chained `and` / `or` forms.
///
/// Docs: https://drafts.csswg.org/mediaqueries-5/#typedef-media-condition
#[inline]
pub(crate) fn parse_any_media_condition(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_media_condition(p) {
        return Absent;
    }

    if is_at_media_not_condition(p) {
        parse_media_not_condition(p)
    } else {
        parse_any_media_in_parens(p).map(|lhs| match p.cur() {
            T![and] => parse_media_and_condition(p, lhs),
            T![or] => parse_media_or_condition(p, lhs),
            _ => lhs,
        })
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
        parse_any_media_type_condition(p).or_add_diagnostic(p, expected_any_media_condition);
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
    // Guarded by `is_at_media_type_query` above.
    parse_media_type(p).ok();

    Present(m.complete(p, CSS_MEDIA_TYPE_QUERY))
}

#[inline]
fn parse_media_type(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();

    // Guarded by `is_at_identifier` above.
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
        parse_any_media_in_parens(p).map(|lhs| parse_media_and_condition(p, lhs))
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
    parse_any_media_in_parens(p)
        .or_recover(p, &AnyInParensParseRecovery, expected_any_media_in_parens)
        .ok();

    Present(m.complete(p, CSS_MEDIA_NOT_CONDITION))
}

/// Parses a left-associated media `and` condition chain after the first
/// parenthesized operand has already been parsed.
#[inline]
fn parse_media_and_condition(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![and]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![and]);

    let recovery_result = parse_any_media_in_parens(p)
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![and]).with_stop_kind(T![')']),
            expected_any_media_in_parens,
        )
        .map(|rhs| parse_media_and_condition(p, rhs));

    if recovery_result.is_err() && p.at(T![and]) {
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_media_and_condition(p, rhs);
    }

    m.complete(p, CSS_MEDIA_AND_CONDITION)
}

/// Parses a left-associated media `or` condition chain after the first
/// parenthesized operand has already been parsed.
#[inline]
fn parse_media_or_condition(p: &mut CssParser, lhs: CompletedMarker) -> CompletedMarker {
    if !p.at(T![or]) {
        return lhs;
    }

    let m = lhs.precede(p);
    p.bump(T![or]);

    let recovery_result = parse_any_media_in_parens(p)
        .or_recover(
            p,
            &AnyInParensChainParseRecovery::new(T![or]).with_stop_kind(T![')']),
            expected_any_media_in_parens,
        )
        .map(|rhs| parse_media_or_condition(p, rhs));

    if recovery_result.is_err() && p.at(T![or]) {
        let m = p.start();
        let rhs = m.complete(p, CSS_BOGUS);
        parse_media_or_condition(p, rhs);
    }

    m.complete(p, CSS_MEDIA_OR_CONDITION)
}

#[inline]
fn is_at_any_media_in_parens(p: &mut CssParser) -> bool {
    p.at(T!['('])
}

/// Parses a parenthesized media query branch.
///
/// This helper disambiguates between nested media conditions such as
/// `(not (color))` and media features such as `(width <= 500px)`.
#[inline]
pub(crate) fn parse_any_media_in_parens(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_any_media_in_parens(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);

    let kind = if is_at_any_media_condition(p) {
        parse_any_media_condition(p)
            .or_recover(p, &AnyInParensParseRecovery, expected_any_media_condition)
            .ok();
        CSS_MEDIA_CONDITION_IN_PARENS
    } else {
        parse_any_query_feature(p)
            .or_recover(p, &AnyInParensParseRecovery, expected_any_query_feature)
            .ok();
        CSS_MEDIA_FEATURE_IN_PARENS
    };

    p.expect(T![')']);

    Present(m.complete(p, kind))
}

fn expected_any_media_condition(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["media condition", "parenthesized media query"], range, p)
}

fn expected_any_media_in_parens(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["media condition", "query feature"], range, p)
}
