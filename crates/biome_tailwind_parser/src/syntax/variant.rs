use crate::parser::TailwindParser;
use crate::syntax::css_value::parse_css_generic_component_value_list;
use crate::syntax::parse_error::*;
use crate::token_source::TailwindLexContext;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, parse_recovery::ParseRecoveryTokenSet, token_set};
use biome_parser::{ParserProgress, prelude::*};
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

#[derive(Default)]
pub(crate) struct VariantList;

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

pub(crate) fn parse_variant(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at(T![-]) {
        // variants can't start with a negative sign
        return Absent;
    }
    if p.at(T!['[']) {
        return parse_arbitrary_variant(p);
    }

    parse_variant_expression(p)
}

fn parse_arbitrary_variant(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();
    p.expect_with_context(T!['['], TailwindLexContext::ArbitraryVariant);
    p.expect_with_context(TW_SELECTOR, TailwindLexContext::ArbitraryVariant);
    p.expect(T![']']);

    if !p.at(COLON) {
        // if we don't reach a colon, we haven't actually parsed a variant.
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_ARBITRARY_VARIANT))
}

fn parse_variant_expression(p: &mut TailwindParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let m = p.start();

    p.source_mut()
        .re_lex_current_in_context(TailwindLexContext::VariantSegment);

    let segments = p.start();
    parse_any_variant_segment(p).or_add_diagnostic(p, expected_value);

    while p.at(T![-]) {
        p.bump_with_context(T![-], TailwindLexContext::VariantSegment);
        parse_any_variant_segment(p).or_add_diagnostic(p, expected_value);
    }

    segments.complete(p, TW_VARIANT_SEGMENT_LIST);

    if !p.at(COLON) {
        // if we don't reach a colon, we haven't actually parsed a variant.
        m.abandon(p);
        p.rewind(checkpoint);
        return Absent;
    }

    Present(m.complete(p, TW_VARIANT_EXPRESSION))
}

fn parse_any_variant_segment(p: &mut TailwindParser) -> ParsedSyntax {
    if p.at(T!['[']) {
        return parse_arbitrary_variant_segment(p);
    }

    if p.at(T!['(']) {
        return parse_css_variable_variant_segment(p);
    }

    parse_named_variant_segment(p)
}

fn parse_named_variant_segment(p: &mut TailwindParser) -> ParsedSyntax {
    if !p.at(TW_VARIANT_SEGMENT) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(TW_VARIANT_SEGMENT, TailwindLexContext::VariantSegment);

    Present(m.complete(p, TW_NAMED_VARIANT_SEGMENT))
}

fn parse_arbitrary_variant_segment(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.expect_with_context(T!['['], TailwindLexContext::CssValue);
    if !parse_css_generic_component_value_list(p) {
        p.error(expected_value(p, p.cur_range()));
    }
    p.expect(T![']']);
    Present(m.complete(p, TW_ARBITRARY_VARIANT_SEGMENT))
}

fn parse_css_variable_variant_segment(p: &mut TailwindParser) -> ParsedSyntax {
    let m = p.start();
    p.expect(T!['(']);
    p.expect(TW_VALUE);
    p.expect(T![')']);
    Present(m.complete(p, TW_CSS_VARIABLE_VARIANT_SEGMENT))
}
