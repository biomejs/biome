use crate::lexer::{CssLexContext, CssStringQuote};
use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;
use crate::syntax::parse_error::expected_string;
use crate::syntax::scss::expression::{
    parse_scss_inner_expression_in_string_until, parse_scss_interpolation_prefix,
};
use crate::syntax::scss::{expected_scss_expression, is_at_scss_interpolation};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS, SCSS_INTERPOLATED_STRING, SCSS_INTERPOLATED_STRING_PART_LIST, SCSS_INTERPOLATION,
    SCSS_STRING_CONTENT_LITERAL, SCSS_STRING_QUOTE, SCSS_STRING_TEXT,
};
use biome_css_syntax::{CssSyntaxKind, T, TextRange};
use biome_parser::Parser;
use biome_parser::SyntaxFeature;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::token_set;

const SCSS_INTERPOLATION_END_TOKEN_SET: biome_parser::TokenSet<CssSyntaxKind> =
    token_set![T!['}'], T![;]];

#[inline]
pub(crate) fn is_at_scss_interpolated_string(p: &mut CssParser) -> bool {
    CssSyntaxFeatures::Scss.is_supported(p) && p.at(SCSS_STRING_QUOTE)
}

/// Parses an SCSS string with embedded interpolations such as
/// `"foo #{$bar} baz"` or `'#{$name}.scss'`.
///
/// Docs: https://sass-lang.com/documentation/interpolation/
#[inline]
pub(crate) fn parse_scss_interpolated_string(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolated_string(p) {
        return Absent;
    }

    let Some(quote) = current_scss_string_quote(p) else {
        return Absent;
    };

    let context = CssLexContext::ScssString(quote);
    let m = p.start();

    p.bump_with_context(SCSS_STRING_QUOTE, context);
    ScssInterpolatedStringPartList::new(quote).parse_list(p);

    if p.at(SCSS_STRING_QUOTE) {
        p.bump_with_context(SCSS_STRING_QUOTE, CssLexContext::Regular);
    }

    Present(m.complete(p, SCSS_INTERPOLATED_STRING))
}

#[inline]
fn current_scss_string_quote(p: &CssParser) -> Option<CssStringQuote> {
    p.cur_text()
        .as_bytes()
        .first()
        .and_then(|byte| CssStringQuote::try_from(*byte).ok())
}

struct ScssInterpolatedStringPartList {
    quote: CssStringQuote,
}

impl ScssInterpolatedStringPartList {
    #[inline]
    fn new(quote: CssStringQuote) -> Self {
        Self { quote }
    }
}

impl ParseNodeList for ScssInterpolatedStringPartList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_INTERPOLATED_STRING_PART_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_scss_interpolation(p) {
            parse_scss_string_interpolation(p, self.quote)
        } else {
            parse_scss_string_text(p, self.quote)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(SCSS_STRING_QUOTE)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, SCSS_INTERPOLATED_STRING_PART_LIST_RECOVERY_SET),
            expected_scss_string_part,
        )
    }
}

#[inline]
fn is_at_scss_string_text(p: &mut CssParser) -> bool {
    p.at(SCSS_STRING_CONTENT_LITERAL)
}

/// Parses one text chunk inside an SCSS interpolated string.
///
/// The next token is lexed with `CssLexContext::ScssString(quote)` so the lexer
/// stays in the outer string context after this chunk.
#[inline]
fn parse_scss_string_text(p: &mut CssParser, quote: CssStringQuote) -> ParsedSyntax {
    if !is_at_scss_string_text(p) {
        return Absent;
    }

    let m = p.start();
    let kind = p.cur();
    p.bump_with_context(kind, CssLexContext::ScssString(quote));
    Present(m.complete(p, SCSS_STRING_TEXT))
}

/// Parses a `#{...}` interpolation inside an SCSS quoted string.
///
/// While parsing the inner expression, this enables recovery for the outer
/// string quote. If the outer quote is reached before a closing `}`, this
/// helper emits the missing-`}` diagnostic and leaves that quote for the
/// surrounding string parser to consume.
#[inline]
fn parse_scss_string_interpolation(p: &mut CssParser, quote: CssStringQuote) -> ParsedSyntax {
    let Some(m) = parse_scss_interpolation_prefix(p) else {
        return Absent;
    };

    p.with_scss_string_interpolation_recovery(quote, |p| {
        parse_scss_inner_expression_in_string_until(p, SCSS_INTERPOLATION_END_TOKEN_SET)
            .or_add_diagnostic(p, expected_scss_expression);
    });

    if p.at(SCSS_STRING_QUOTE) {
        p.error(expected_interpolation_end_before_closing_quote(p));
        return Present(m.complete(p, SCSS_INTERPOLATION));
    }

    p.expect_with_context(T!['}'], CssLexContext::ScssString(quote));
    Present(m.complete(p, SCSS_INTERPOLATION))
}

const SCSS_INTERPOLATED_STRING_PART_LIST_RECOVERY_SET: biome_parser::TokenSet<CssSyntaxKind> =
    token_set![T![#], SCSS_STRING_QUOTE];

fn expected_scss_string_part(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expected_string(p, range).with_hint("Expected string text or interpolation.")
}

fn expected_interpolation_end_before_closing_quote(p: &CssParser) -> ParseDiagnostic {
    ParseDiagnostic::new("Expected `}` before closing quote", p.cur_range())
        .with_hint("Close the interpolation before ending the string.")
}
