use crate::parser::TailwindParser;
use crate::syntax::parse_error::*;
use crate::syntax::value::parse_value;
use crate::syntax::variant::VariantList;
use crate::token_source::TailwindLexContext;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, parse_recovery::ParseRecoveryTokenSet, token_set};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

mod parse_error;
mod value;
mod variant;

pub fn parse_root(p: &mut TailwindParser) {
    let m = p.start();

    if p.at(UNICODE_BOM) {
        p.eat(UNICODE_BOM);
    }
    CandidateList.parse_list(p);

    m.complete(p, TW_ROOT);
}

#[derive(Default)]
struct CandidateList;

impl ParseSeparatedList for CandidateList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = TW_CANDIDATE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_full_candidate(p)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        WHITESPACE
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE, NEWLINE, EOF]),
            expected_candidate,
        )
    }
}

fn parse_full_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    VariantList.parse_list(p);

    let candidate = parse_arbitrary_candidate(p)
        .or_else(|| parse_functional_or_static_candidate(p))
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE, NEWLINE, EOF]),
            expected_candidate,
        );

    match candidate {
        Ok(_) => {}
        Err(_) => {
            m.abandon(p);
            p.rewind(checkpoint);
            return Absent;
        }
    }

    if p.at(T![!]) {
        p.bump(T![!]);
    }

    Present(m.complete(p, TW_FULL_CANDIDATE))
}

fn parse_functional_or_static_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(TW_BASE) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();

    p.bump(TW_BASE);
    if p.at(T![:]) {
        // Oops, this is a Variant!
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    if !p.at(T![-]) {
        return Present(m.complete(p, TW_STATIC_CANDIDATE));
    }

    p.bump(DASH);
    match parse_value(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TW_BOGUS_VALUE, token_set![WHITESPACE, NEWLINE, T![!], EOF]),
        expected_value,
    ) {
        Ok(_) => {}
        Err(_) => {
            m.abandon(p);
            p.rewind(checkpoint);
            return Absent;
        }
    }

    if p.at(T![:]) {
        // Oops, this is a Variant!
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    if p.at(T![/]) {
        parse_modifier(p).or_add_diagnostic(p, expected_modifier);
    }

    Present(m.complete(p, TW_FUNCTIONAL_CANDIDATE))
}

fn parse_arbitrary_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(T!['[']) {
        return Absent;
    }

    let checkpoint = p.checkpoint();
    let m = p.start();
    if !p.expect_with_context(T!['['], TailwindLexContext::ArbitraryCandidate) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect_with_context(TW_PROPERTY, TailwindLexContext::ArbitraryCandidate) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect_with_context(T![:], TailwindLexContext::ArbitraryCandidate) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect_with_context(TW_VALUE, TailwindLexContext::ArbitraryCandidate) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect(T![']']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    if !p.at(T![/]) {
        return Present(m.complete(p, TW_ARBITRARY_CANDIDATE));
    }

    if p.at(T![/]) {
        parse_modifier(p).or_add_diagnostic(p, expected_modifier);
    }

    Present(m.complete(p, TW_ARBITRARY_CANDIDATE))
}

fn parse_modifier(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    if !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }
    match parse_value(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(
            TW_BOGUS_MODIFIER,
            token_set![WHITESPACE, NEWLINE, T![!], EOF],
        ),
        expected_value,
    ) {
        Ok(_) => {}
        Err(_) => {
            m.abandon(p);
            return Absent;
        }
    }

    Present(m.complete(p, TW_MODIFIER))
}
