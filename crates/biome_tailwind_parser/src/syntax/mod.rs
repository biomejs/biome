use crate::parser::TailwindParser;
use crate::syntax::parse_error::*;
use crate::syntax::value::parse_value;
use crate::token_source::TailwindLexContext;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{
    Parser, parse_lists::ParseNodeList, parse_recovery::ParseRecoveryTokenSet, token_set,
};
use biome_parser::{ParserProgress, prelude::*};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

mod parse_error;
mod value;

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
        parse_candidate(p)
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
            &ParseRecoveryTokenSet::new(TW_BOGUS_CANDIDATE, token_set![WHITESPACE, NEWLINE, EOF]),
            expected_candidate,
        )
    }
}

fn parse_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();

    VariantList.parse_list(p);

    parse_functional_or_static_candidate(p).or_add_diagnostic(p, expected_candidate);

    if p.at(T![!]) {
        p.bump(T![!]);
    }

    Present(m.complete(p, TW_CANDIDATE))
}

fn parse_functional_or_static_candidate(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(TW_BASE) {
        return Absent;
    }

    let m = p.start();

    p.bump(TW_BASE);
    if p.at(T![:]) {
        // Oops, this is a Variant!
        m.abandon(p);
        return Absent;
    }

    if !p.at(T![-]) {
        return Present(m.complete(p, TW_STATIC_CANDIDATE));
    }

    p.bump(DASH);
    parse_value(p).or_add_diagnostic(p, expected_value);

    if p.at(T![:]) {
        // Oops, this is a Variant!
        m.abandon(p);
        return Absent;
    }

    if p.at(T![/]) {
        parse_modifier(p).or_add_diagnostic(p, expected_modifier);
    }

    Present(m.complete(p, TW_FUNCTIONAL_CANDIDATE))
}

fn parse_modifier(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T![/]);
    parse_value(p).or_add_diagnostic(p, expected_value);

    Present(m.complete(p, TW_MODIFIER))
}

#[derive(Default)]
struct VariantList;

impl ParseSeparatedList for VariantList {
    type Kind = TailwindSyntaxKind;
    type Parser<'source> = TailwindParser<'source>;
    const LIST_KIND: Self::Kind = TW_VARIANT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_variant(p)
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![:]
    }

    fn is_at_list_end(&self, _p: &mut Self::Parser<'_>) -> bool {
        false
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
            &ParseRecoveryTokenSet::new(TW_BOGUS_VARIANT, token_set![WHITESPACE, T![:]]),
            expected_variant,
        )
    }

    /// We need to have a custom implementation for this because we need to use the presence of a variant to know if we are at the end of the list.
    fn parse_list(&mut self, p: &mut Self::Parser<'_>) -> CompletedMarker {
        let elements = self.start_list(p);
        let mut progress = ParserProgress::default();
        let mut first = true;
        loop {
            if (self.allow_empty() || !first)
                && (p.at(<Self::Parser<'_> as Parser>::Kind::EOF) || self.is_at_list_end(p))
            {
                break;
            }

            if first {
                first = false;
            } else {
                self.expect_separator(p);
            }

            progress.assert_progressing(p);

            let parsed_element = self.parse_element(p);
            if parsed_element.is_absent() {
                // stop if we don't have a variant
                break;
            }
            if self.recover(p, parsed_element).is_err() {
                break;
            }
        }
        self.finish_list(p, elements)
    }
}

fn parse_variant(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at(T!['[']) {
        return parse_arbitrary_variant(p);
    }

    parse_static_or_functional_variant(p)
}

fn parse_arbitrary_variant(p: &mut TailwindParser) -> ParsedSyntax {
    let parser_chk = p.context().checkpoint();
    let source_chk = p.source().checkpoint();
    let m = p.start();
    p.expect_with_context(T!['['], TailwindLexContext::ArbitraryVariant);
    p.expect_with_context(TW_SELECTOR, TailwindLexContext::ArbitraryVariant);
    p.expect(T![']']);

    if !p.at(COLON) {
        // if we don't reach a colon, we haven't actually parsed a variant.
        m.abandon(p);
        p.context_mut().rewind(parser_chk);
        p.source_mut().rewind(source_chk);
        return Absent;
    }

    Present(m.complete(p, TW_ARBITRARY_VARIANT))
}

fn parse_static_or_functional_variant(p: &mut TailwindParser) -> ParsedSyntax {
    let parser_chk = p.context().checkpoint();
    let source_chk = p.source().checkpoint();
    let m = p.start();
    p.bump(TW_BASE);

    if !p.at(T![-]) {
        if !p.at(COLON) {
            // if we don't reach a colon, we haven't actually parsed a variant.
            m.abandon(p);
            p.context_mut().rewind(parser_chk);
            p.source_mut().rewind(source_chk);
            return Absent;
        }
        return Present(m.complete(p, TW_STATIC_VARIANT));
    }

    p.expect(T![-]);

    parse_value(p).or_add_diagnostic(p, expected_value);

    if !p.at(COLON) {
        // if we don't reach a colon, we haven't actually parsed a variant.
        m.abandon(p);
        p.context_mut().rewind(parser_chk);
        p.source_mut().rewind(source_chk);
        return Absent;
    }

    Present(m.complete(p, TW_FUNCTIONAL_VARIANT))
}
