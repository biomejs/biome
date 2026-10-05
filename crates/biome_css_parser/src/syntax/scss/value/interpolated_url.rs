use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::scss::{is_at_scss_interpolation, parse_scss_interpolation_with_context};
use biome_css_syntax::CssSyntaxKind::{
    CSS_BOGUS, SCSS_INTERPOLATED_URL_VALUE, SCSS_INTERPOLATED_URL_VALUE_PART_LIST,
    SCSS_URL_CONTENT_LITERAL, SCSS_URL_TEXT,
};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::Parser;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{TokenSet, token_set};

const SCSS_INTERPOLATED_URL_VALUE_PART_RECOVERY_SET: TokenSet<CssSyntaxKind> =
    token_set![SCSS_URL_CONTENT_LITERAL, T![#], T![')']];

/// Returns whether `images/` or `#{$name}` starts an interpolated URL value.
#[inline]
fn is_at_scss_interpolated_url_value(p: &mut CssParser) -> bool {
    p.at(SCSS_URL_CONTENT_LITERAL) || is_at_scss_interpolation(p)
}

/// Parses a valid unquoted SCSS URL value containing interpolation.
///
/// ```scss
/// .logo {
///   background-image: url(images/#{$name}.png);
/// }
/// ```
#[inline]
pub(crate) fn parse_scss_interpolated_url_value(
    p: &mut CssParser,
    lex_context: CssLexContext,
) -> ParsedSyntax {
    if !is_at_scss_interpolated_url_value(p) {
        return Absent;
    }

    let value = p.start();
    ScssInterpolatedUrlValuePartList { lex_context }.parse_list(p);
    Present(value.complete(p, SCSS_INTERPOLATED_URL_VALUE))
}

/// Parses `images/`, `#{$name}`, and `.png` in `url(images/#{$name}.png)`.
struct ScssInterpolatedUrlValuePartList {
    lex_context: CssLexContext,
}

impl ParseNodeList for ScssInterpolatedUrlValuePartList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = SCSS_INTERPOLATED_URL_VALUE_PART_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_scss_interpolation(p) {
            parse_scss_interpolation_with_context(p, self.lex_context)
        } else {
            parse_scss_url_text(p, self.lex_context)
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, SCSS_INTERPOLATED_URL_VALUE_PART_RECOVERY_SET),
            expected_component_value,
        )
    }
}

/// Parses one text chunk such as `images/` in `url(images/#{$name}.png)`.
#[inline]
fn parse_scss_url_text(p: &mut CssParser, lex_context: CssLexContext) -> ParsedSyntax {
    if !p.at(SCSS_URL_CONTENT_LITERAL) {
        return Absent;
    }

    let text = p.start();
    p.bump_with_context(SCSS_URL_CONTENT_LITERAL, lex_context);
    Present(text.complete(p, SCSS_URL_TEXT))
}
