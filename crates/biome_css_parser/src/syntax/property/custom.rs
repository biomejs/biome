use crate::lexer::{CssCustomPropertyCommentMode, CssLexContext};
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_component_value;
use crate::syntax::scss::{
    is_at_scss_interpolated_string, is_at_scss_interpolation, parse_scss_interpolated_string,
    parse_scss_interpolation_inner_expression, parse_scss_interpolation_prefix,
};
use crate::syntax::value::dimension::{is_at_any_dimension, parse_any_dimension};
use crate::syntax::value::function::is_nth_at_source_tight_l_paren;
use crate::syntax::{is_at_identifier, parse_custom_identifier_with_keywords, parse_number};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::{RecoveryError, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{CompletedMarker, Parser, TokenSet, token_set};

const CUSTOM_PROPERTY_DELIMITER_SET: TokenSet<CssSyntaxKind> = token_set![
    CSS_DELIM_LITERAL,
    T![$],
    T![#],
    T![&],
    T![@],
    T![,],
    T![:],
    T![;],
    T![/],
    T![.],
    T![...],
    T![::],
    T![+],
    T![-],
    T![*],
    T![%],
    T![^],
    T![~],
    T![|],
    T![||],
    T![=],
    T![==],
    T![!=],
    T![<],
    T![<=],
    T![>],
    T![>=],
    T![|=],
    T![^=],
    T![*=],
    T!["$="],
    T![~=],
    T![!],
    CDO,
    CDC,
];
const CUSTOM_PROPERTY_BLOCK_END_SET: TokenSet<CssSyntaxKind> =
    token_set![T![')'], T![']'], T!['}'], EOF];
const CUSTOM_PROPERTY_BLOCK_START_SET: TokenSet<CssSyntaxKind> =
    token_set![T!['('], T!['['], T!['{']];

/// Parses an SCSS custom-property value without evaluating SassScript.
///
/// Example: `$gap + 1px` in `--space: $gap + 1px;`.
pub(crate) fn parse_custom_property_value(
    p: &mut CssParser,
    end_set: TokenSet<CssSyntaxKind>,
) -> CompletedMarker {
    parse_custom_property_value_with_mode(
        p,
        end_set,
        CssCustomPropertyCommentMode::PreserveDoubleSlash,
    )
}

/// Parses a raw custom-property value with the caller's `//` policy.
fn parse_custom_property_value_with_mode(
    p: &mut CssParser,
    end_set: TokenSet<CssSyntaxKind>,
    comment_mode: CssCustomPropertyCommentMode,
) -> CompletedMarker {
    let value = p.start();
    CustomPropertyComponentList::value(end_set, comment_mode).parse_list(p);
    value.complete(p, CSS_CUSTOM_PROPERTY_VALUE)
}

struct CustomPropertyComponentList {
    boundary: CustomPropertyListBoundary,
    comment_mode: CssCustomPropertyCommentMode,
}

/// Controls which tokens terminate a raw custom-property component list.
enum CustomPropertyListBoundary {
    Value(TokenSet<CssSyntaxKind>),
    Block,
}

impl CustomPropertyComponentList {
    fn value(end_set: TokenSet<CssSyntaxKind>, comment_mode: CssCustomPropertyCommentMode) -> Self {
        Self {
            boundary: CustomPropertyListBoundary::Value(end_set),
            comment_mode,
        }
    }

    fn block(comment_mode: CssCustomPropertyCommentMode) -> Self {
        Self {
            boundary: CustomPropertyListBoundary::Block,
            comment_mode,
        }
    }
}

impl ParseNodeList for CustomPropertyComponentList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_CUSTOM_PROPERTY_COMPONENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_custom_property_component(p, self.comment_mode)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        match self.boundary {
            CustomPropertyListBoundary::Value(end_set) => {
                if p.at(T![!]) {
                    p.source()
                        .is_at_final_custom_property_important(self.comment_mode)
                } else {
                    p.at_ts(end_set)
                }
            }
            CustomPropertyListBoundary::Block => p.at_ts(CUSTOM_PROPERTY_BLOCK_END_SET),
        }
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        if let Present(parsed) = parsed_element {
            return Ok(parsed);
        }

        if p.is_speculative_parsing() {
            p.error(expected_component_value(p, p.cur_range()));
            return Err(RecoveryError::RecoveryDisabled);
        }

        // Raw values accept every non-boundary token, so isolate the malformed
        // token without discarding subsequent components.
        let recovered = p.start();
        let kind = p.cur();
        p.bump_with_context(kind, CssLexContext::CustomPropertyValue(self.comment_mode));
        let recovered = recovered.complete(p, CSS_BOGUS_PROPERTY_VALUE);
        p.error(expected_component_value(p, recovered.range(p)));
        Ok(recovered)
    }
}

/// Parses one raw custom-property component.
///
/// Example: `{a: #{$value}}` is a braced component containing interpolation.
fn parse_custom_property_component(
    p: &mut CssParser,
    comment_mode: CssCustomPropertyCommentMode,
) -> ParsedSyntax {
    let context = CssLexContext::CustomPropertyValue(comment_mode);

    if is_at_scss_interpolation(p) {
        parse_custom_property_interpolation(p, context)
    } else if is_at_scss_interpolated_string(p) {
        parse_scss_interpolated_string(p, context)
    } else if p.at(CSS_STRING_LITERAL) {
        parse_custom_property_string(p, context)
    } else if is_at_custom_property_function(p) {
        parse_custom_property_function(p, comment_mode)
    } else if is_at_any_dimension(p) {
        parse_any_dimension(p, context)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_number(p, context)
    } else if p.at_ts(CUSTOM_PROPERTY_BLOCK_START_SET) {
        parse_custom_property_block(p, comment_mode)
    } else if is_at_identifier(p) {
        parse_custom_identifier_with_keywords(p, context, true)
    } else {
        parse_custom_property_delimiter(p, context)
    }
}

/// Returns whether a source-tight raw function such as `url(` starts here.
#[inline]
fn is_at_custom_property_function(p: &mut CssParser) -> bool {
    is_at_identifier(p) && is_nth_at_source_tight_l_paren(p, 1)
}

/// Parses a source-tight function with a preserved raw body.
///
/// Example: `function(rule){...}` in `--script: function(rule){...};`.
fn parse_custom_property_function(
    p: &mut CssParser,
    comment_mode: CssCustomPropertyCommentMode,
) -> ParsedSyntax {
    if !is_at_custom_property_function(p) {
        return Absent;
    }

    let function = p.start();
    let is_url_function = p.at(T![url]);
    let context = CssLexContext::CustomPropertyValue(comment_mode);

    parse_custom_identifier_with_keywords(p, context, true).ok();

    // Raw `url(...)` contents keep `//` as URL text; fallback function bodies
    // parse it as a Sass comment.
    let body_comment_mode = if comment_mode == CssCustomPropertyCommentMode::ScssLineComments
        && is_url_function
        && p.source().is_scss_raw_url_body()
    {
        CssCustomPropertyCommentMode::PreserveDoubleSlash
    } else {
        comment_mode
    };

    p.bump_with_context(
        T!['('],
        CssLexContext::CustomPropertyValue(body_comment_mode),
    );
    CustomPropertyComponentList::block(body_comment_mode).parse_list(p);
    p.expect_with_context(T![')'], context);
    Present(function.complete(p, CSS_CUSTOM_PROPERTY_FUNCTION))
}

/// Parses a balanced raw block such as `(a)`, `[a]`, or `{a}`.
fn parse_custom_property_block(
    p: &mut CssParser,
    comment_mode: CssCustomPropertyCommentMode,
) -> ParsedSyntax {
    let (open, close, kind) = match p.cur() {
        T!['('] => (T!['('], T![')'], CSS_CUSTOM_PROPERTY_PARENTHESIZED_BLOCK),
        T!['['] => (T!['['], T![']'], CSS_CUSTOM_PROPERTY_BRACKETED_BLOCK),
        T!['{'] => (T!['{'], T!['}'], CSS_CUSTOM_PROPERTY_BRACED_BLOCK),
        _ => return Absent,
    };

    let block = p.start();
    let context = CssLexContext::CustomPropertyValue(comment_mode);
    p.bump_with_context(open, context);
    CustomPropertyComponentList::block(comment_mode).parse_list(p);
    p.expect_with_context(close, context);
    Present(block.complete(p, kind))
}

/// Parses `#{$gap}` and resumes raw-value lexing after `}`.
fn parse_custom_property_interpolation(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let Some(interpolation) = parse_scss_interpolation_prefix(p) else {
        return Absent;
    };

    parse_scss_interpolation_inner_expression(p);
    p.expect_with_context(T!['}'], context);
    Present(interpolation.complete(p, SCSS_INTERPOLATION))
}

/// Parses a plain string such as `"$gap"` as raw value content.
#[inline]
fn parse_custom_property_string(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_STRING_LITERAL) {
        return Absent;
    }

    let string = p.start();
    p.bump_with_context(CSS_STRING_LITERAL, context);
    Present(string.complete(p, CSS_STRING))
}

/// Parses one raw delimiter such as `$` in `--space: $gap;`.
#[inline]
fn parse_custom_property_delimiter(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at_ts(CUSTOM_PROPERTY_DELIMITER_SET) {
        return Absent;
    }

    let delimiter = p.start();
    let kind = p.cur();
    p.bump_with_context(kind, context);

    Present(delimiter.complete(p, CSS_CUSTOM_PROPERTY_DELIMITER))
}
