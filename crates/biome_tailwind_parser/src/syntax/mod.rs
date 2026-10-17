use crate::lexer::simd::{Stop, is_whitespace};
use crate::parser::TailwindParser;
use crate::syntax::css_value::parse_css_generic_component_value_list;
use crate::syntax::parse_error::*;
use crate::syntax::value::parse_value;
use crate::syntax::variant::VariantList;
use crate::token_source::TailwindTokenSource;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{Parser, parse_recovery::ParseRecoveryTokenSet, token_set};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

mod css_value;
mod parse_error;
mod value;
mod variant;

pub(crate) use css_value::group_css_value;
#[cfg(test)]
pub(crate) use css_value::{group_css_unit, group_url_value};
#[cfg(test)]
pub(crate) use variant::{group_selector, group_variant_segment};

pub fn parse_root(p: &mut TailwindParser) {
    let m = p.start();

    if p.at(UNICODE_BOM) {
        p.eat(UNICODE_BOM);
    }
    p.eat(WHITESPACE);
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
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE]),
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
        p.bump_with_context(T![-], group_negative_base);
    }

    let candidate = parse_arbitrary_candidate(p)
        .or_else(|| parse_functional_or_static_candidate(p))
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE]),
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
    if p.at(T![!]) {
        if legacy_important {
            p.error(duplicate_important(p, p.cur_range()));
        }
        p.bump(T![!]);
    }

    Present(m.complete(p, TW_FULL_CANDIDATE))
}

/// Groups the token after the current one where the grammar doesn't pick a grouping, as after the
/// tokens that error recovery skips. It depends on the current token: a `[` starts a selector, a
/// `(` a value up to the next `)`, a `-` a value and a `/` a modifier.
#[inline(always)]
pub(crate) fn group_regular(source: &mut TailwindTokenSource) {
    match source.current() {
        T!['['] => group_bracketed(source),
        T!['('] => group_parenthesized(source),
        T![-] => group_value(source),
        T![/] => group_modifier(source),
        _ => group_class_token(source),
    }
}

/// Groups a token of a class outside of brackets and values: a basename, a variant segment name
/// that starts with `*`, or one byte.
fn group_class_token(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']' | b':' | b'-' | b'%' | b'!' | b'/')) => {
            source.take_punctuation(byte);
        }
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'@') => source.merge_base(),
        // `*` only begins a variant (`*:flex`). It's a segment name rather than an error token,
        // whose diagnostic would outlive the variant's regrouping.
        Some(b'*') => source.merge_until(Stop::Segment, TW_VARIANT_SEGMENT),
        Some(_) if source.at_byte_order_mark() => source.take(UNICODE_BOM, 3),
        Some(_) => source.take_unexpected(),
    }
}

/// Groups the token after a `[` that error recovery skips: a selector or one bracket.
fn group_bracketed(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']')) => source.take_punctuation(byte),
        Some(_) => source.merge_selector(),
    }
}

/// Groups the token after a `(` that error recovery skips: a value up to the next `)` or one
/// bracket.
fn group_parenthesized(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']')) => source.take_punctuation(byte),
        Some(_) => source.merge_until(Stop::ParenOrWhitespace, TW_VALUE),
    }
}

/// Groups the value after the `-` of a candidate: the atoms up to the next whitespace, `:`, `/`,
/// `!`, `]`, `)` or `%`, which may be none, or one bracket.
pub(crate) fn group_value(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']')) => source.take_punctuation(byte),
        Some(_) => source.merge_value(Stop::Value),
    }
}

/// Groups the modifier after a `/`: the atoms up to the next whitespace, `!`, `%` or `:`, or one
/// bracket. An empty modifier is an error token.
pub(crate) fn group_modifier(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']')) => source.take_punctuation(byte),
        Some(b'!' | b'%' | b':') => source.take(ERROR_TOKEN, 0),
        Some(_) => source.merge_value(Stop::Modifier),
    }
}

/// Groups the token after the `-` of a negative candidate: a basename, which here can't start
/// with `@` or `*`, or one byte.
pub(crate) fn group_negative_base(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']' | b':' | b'-' | b'%' | b'!' | b'/')) => {
            source.take_punctuation(byte);
        }
        Some(b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_') => source.merge_base(),
        Some(_) => source.take_unexpected(),
    }
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
        // A modifier can glue straight onto a bare name
        // (`@container/sidebar` names the container); whitespace before
        // the `/` means the next class starts instead.
        if p.at(T![/]) {
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
    p.expect_with_context(T![-], group_value);
    match parse_value(p, true).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TW_BOGUS_VALUE, token_set![WHITESPACE, T![!]]),
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
    if !p.expect_with_context(T!['['], group_property) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    // A property ends at a `:`, whitespace or the end of the source, which the regular tokens
    // group like the candidate does.
    if !p.expect(TW_PROPERTY) {
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }
    if !p.expect_with_context(T![:], group_css_value) {
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

/// Groups the token after the `[` of an arbitrary candidate: the property, which runs to the next
/// `:` or whitespace, or one byte.
pub(crate) fn group_property(source: &mut TailwindTokenSource) {
    match source.next_byte() {
        None => source.take(EOF, 0),
        Some(byte) if is_whitespace(byte) => source.take_whitespace(),
        Some(byte @ (b'(' | b')' | b'[' | b']' | b':')) => source.take_punctuation(byte),
        Some(_) => source.merge_until(Stop::ColonOrWhitespace, TW_PROPERTY),
    }
}

/// Whether the class chunk at the current position contains a `:` before
/// the next whitespace.
///
/// Both variant forms end in a `:` (`parse_variant_expression` and
/// `parse_arbitrary_variant` rewind without one), so a chunk without a
/// colon can never begin with variants.
fn class_chunk_has_colon(p: &TailwindParser) -> bool {
    p.source().has_colon_before_whitespace()
}

pub(crate) fn parse_modifier(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    if !p.expect_with_context(T![/], group_modifier) {
        m.abandon(p);
        return Absent;
    }
    match parse_value(p, false).or_recover_with_token_set(
        p,
        &ParseRecoveryTokenSet::new(TW_BOGUS_MODIFIER, token_set![WHITESPACE, T![!]]),
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
