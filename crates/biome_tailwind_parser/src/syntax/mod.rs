use crate::parser::TailwindParser;
use crate::syntax::css_value::parse_css_generic_component_value_list;
use crate::syntax::parse_error::*;
use crate::syntax::value::parse_value;
use crate::syntax::variant::VariantList;
use crate::token_source::TailwindLexContext;
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, parse_recovery::ParseRecoveryTokenSet, token_set};
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
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE])
                .enable_recovery_on_line_break(),
            expected_candidate,
        )
    }
}

fn parse_full_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
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
    let legacy_important = p.eat(T![!]);

    if p.at(T![-]) {
        p.bump_with_context(T![-], TailwindLexContext::SawNegative);
    }

    let candidate = parse_arbitrary_candidate(p)
        .or_else(|| parse_functional_or_static_candidate(p))
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE])
                .enable_recovery_on_line_break(),
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
    match parse_value(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TW_BOGUS_VALUE, token_set![WHITESPACE, T![!]])
            .enable_recovery_on_line_break(),
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
    if !p.expect(T![/]) {
        m.abandon(p);
        return Absent;
    }
    match parse_value(p).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TW_BOGUS_MODIFIER, token_set![WHITESPACE, NEWLINE, T![!]]),
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
