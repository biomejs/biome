use crate::parser::TailwindParser;
use crate::syntax::css_value::parse_css_generic_component_value_list;
use crate::syntax::parse_error::*;
use crate::syntax::value::parse_value;
use crate::syntax::variant::VariantList;
use crate::token_source::TailwindLexContext;
use biome_parser::Parser;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, RecoveryError, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_rowan::TextRange;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_unicode_table::{Dispatch::WHS, lookup_byte};

mod css_value;
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

impl ParseNodeList for CandidateList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = TW_CANDIDATE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_full_candidate(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &CandidateRecovery, expected_candidate)
    }
}

/// Recovers a malformed class into one bogus candidate that ends at the
/// next whitespace gap. Whitespace between classes is trivia, so a
/// token-set recovery on `WHITESPACE` never stops and one bad class would
/// swallow every class after it.
struct CandidateRecovery;

/// Recovers a malformed value or modifier inside a class into a bogus node
/// that ends at the next whitespace gap or at a trailing `!`. Unlike
/// [CandidateRecovery] it takes nothing when the parser already sits past
/// a gap: the piece is simply missing, and the next class starts there.
struct PieceRecovery(TailwindSyntaxKind);

impl ParseRecovery for PieceRecovery {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const RECOVERED_KIND: Self::Kind = TW_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.source().had_trivia_before() || p.at(T![!])
    }

    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        if p.at(EOF) {
            return Err(RecoveryError::Eof);
        }
        if self.is_at_recovered(p) {
            return Err(RecoveryError::AlreadyRecovered);
        }
        if p.is_speculative_parsing() {
            return Err(RecoveryError::RecoveryDisabled);
        }

        let m = p.start();
        while !(p.at(EOF) || self.is_at_recovered(p)) {
            p.bump_any();
        }
        Ok(m.complete(p, self.0))
    }
}

impl ParseRecovery for CandidateRecovery {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const RECOVERED_KIND: Self::Kind = TW_BOGUS_CANDIDATE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.source().had_trivia_before()
    }

    fn recover(&self, p: &mut Self::Parser<'_>) -> RecoveryResult {
        if p.at(EOF) {
            return Err(RecoveryError::Eof);
        }
        if p.is_speculative_parsing() {
            return Err(RecoveryError::RecoveryDisabled);
        }

        // The offending token itself may sit after a gap; always take it,
        // then run to the next gap.
        let m = p.start();
        p.bump_any();
        while !(p.at(EOF) || self.is_at_recovered(p)) {
            p.bump_any();
        }
        Ok(m.complete(p, Self::RECOVERED_KIND))
    }
}

fn parse_full_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let checkpoint_position = p.source().position();
    let m = p.start();

    if class_chunk_has_colon(p) {
        VariantList.parse_list(p);
    } else {
        // Every variant ends in a `:`, so a class without one can't start
        // with variants; complete the empty list directly instead of
        // parsing segments only to rewind.
        let variants = p.start();
        variants.complete(p, TW_VARIANT_LIST);
    }

    // Tailwind's legacy important spelling puts the `!` right before the
    // utility, after the variants and before the sign (`hover:!flex`,
    // `!-m-4`).
    let legacy_important = p.at(T![!]);
    if legacy_important {
        p.bump(T![!]);
    }

    if p.at(T![-]) {
        p.bump_with_context(T![-], TailwindLexContext::SawNegative);
    }

    // Variants, the legacy `!`, and the sign are all glued to the
    // utility. A gap after them (`hover: flex`) ends the class early:
    // what was consumed is a bogus candidate and the next class starts
    // after the gap.
    if p.source().position() > checkpoint_position && p.source().had_trivia_before() {
        let end = p.last_end().unwrap_or(checkpoint_position);
        p.error(expected_candidate(
            p,
            TextRange::new(checkpoint_position, end),
        ));
        return Present(m.complete(p, TW_BOGUS_CANDIDATE));
    }

    let candidate = parse_arbitrary_candidate(p)
        .or_else(|| parse_functional_or_static_candidate(p))
        .or_recover(p, &CandidateRecovery, expected_candidate);

    match candidate {
        Ok(_) => {}
        Err(_) => {
            m.abandon(p);
            p.rewind(checkpoint);
            return Absent;
        }
    }

    // The trailing `!` must be glued to the utility; whitespace before it
    // means the next class starts with the legacy `!` instead.
    if p.at(T![!]) && !p.source().had_trivia_before() {
        if legacy_important {
            p.error(duplicate_important(p, p.cur_range()));
        }
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
    let pos = p.source().position();
    if p.at(T![:]) {
        // Oops, this is a Variant!
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    if !p.at(T![-]) {
        // A modifier can glue straight onto a bare name
        // (`@container/sidebar` names the container); whitespace before
        // the `/` means the next class starts instead.
        if p.at(T![/]) && !p.source().had_trivia_before() {
            parse_modifier(p).or_add_diagnostic(p, expected_modifier);
            if p.at(T![:]) {
                // A `:` after the modifier means this was a (malformed)
                // variant, not a candidate; rewinding lets the whole
                // token recover as one bogus candidate.
                m.abandon(p);
                p.rewind(checkpoint);
                return Absent;
            }
        }
        return Present(m.complete(p, TW_STATIC_CANDIDATE));
    }
    if p.source().had_trivia_before() {
        // Whitespace is not allowed in tailwind candidates
        // Theres whitespace between these tokens, so it can't be a functional candidate
        return Present(m.complete(p, TW_STATIC_CANDIDATE));
    }
    if let Some(last_trivia) = p.source().trivia_list.last()
        && pos < last_trivia.text_range().start()
    {
        // Whitespace is not allowed in tailwind candidates
        // Theres whitespace between these tokens, so it can't be a functional candidate
        return Present(m.complete(p, TW_STATIC_CANDIDATE));
    }

    p.expect(T![-]);
    match parse_value(p).or_recover(p, &PieceRecovery(TW_BOGUS_VALUE), expected_value) {
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
        if p.at(T![:]) {
            // A `:` after the modifier means this was a (malformed)
            // variant, not a candidate; rewinding lets the whole token
            // recover as one bogus candidate.
            m.abandon(p);
            p.rewind(checkpoint);
            return Absent;
        }
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
    if !p.expect_with_context(T![:], TailwindLexContext::CssValue) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !parse_css_generic_component_value_list(p) {
        p.error(expected_value(p, p.cur_range()));
    }
    if !p.expect(T![']']) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    if !p.at(T![/]) {
        return Present(m.complete(p, TW_ARBITRARY_CANDIDATE));
    }

    parse_modifier(p).or_add_diagnostic(p, expected_modifier);
    if p.at(T![:]) {
        // A `:` after the modifier means this was a (malformed) variant,
        // not a candidate; rewinding lets the whole token recover as one
        // bogus candidate.
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_ARBITRARY_CANDIDATE))
}

/// Whether the class chunk at the current position contains a `:` before
/// the next whitespace.
///
/// Both variant forms end in a `:` (`parse_variant_expression` and
/// `parse_arbitrary_variant` rewind without one), so a chunk without a
/// colon can never begin with variants. The scan stops on the same bytes
/// the lexer classifies as whitespace, keeping the chunk boundary in sync
/// with tokenization.
fn class_chunk_has_colon(p: &TailwindParser) -> bool {
    let text = p.source().text().as_bytes();
    let start = usize::from(p.source().position());
    text[start..]
        .iter()
        .take_while(|&&byte| !matches!(lookup_byte(byte), WHS))
        .any(|&byte| byte == b':')
}

pub(crate) fn parse_modifier(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    let slash = p.cur_range();
    if !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }
    if p.source().had_trivia_before() {
        // `bg-red-500/ flex`: the value is missing and the next class
        // starts after the gap.
        p.error(expected_value(p, TextRange::empty(slash.end())));
        return Present(m.complete(p, TW_MODIFIER));
    }
    match parse_value(p).or_recover(p, &PieceRecovery(TW_BOGUS_MODIFIER), expected_value) {
        Ok(_) => {}
        Err(_) => {
            m.abandon(p);
            return Absent;
        }
    }

    Present(m.complete(p, TW_MODIFIER))
}
