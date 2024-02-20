mod at_rule;
mod block;
mod parse_error;
mod property;
mod selector;
mod value;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::parse_error::expected_any_rule;
use crate::syntax::property::{is_at_any_property, parse_any_property};
use crate::syntax::selector::is_nth_at_selector;
use crate::syntax::selector::relative_selector::{is_at_relative_selector, RelativeSelectorList};
use crate::syntax::selector::SelectorList;
use crate::syntax::value::function::BINARY_OPERATION_TOKEN;
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{token_set, Parser};
use value::dimension::{is_at_any_dimension, parse_any_dimension};
use value::function::{is_at_any_function, parse_any_function};

use self::parse_error::{expected_component_value, expected_declaration_item, expected_number};
pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    p.eat(UNICODE_BOM);

    RuleList::new(EOF).parse_list(p);

    m.complete(p, CSS_ROOT);
}

struct RuleList {
    end_kind: CssSyntaxKind,
}

impl RuleList {
    fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

#[inline]
pub(crate) fn is_at_rule_list_element(p: &mut CssParser) -> bool {
    is_at_at_rule(p) || is_at_qualified_rule(p)
}

struct RuleListParseRecovery;

impl ParseRecovery for RuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_RULE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        is_at_rule_list_element(p)
    }
}

impl ParseNodeList for RuleList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_RULE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_rule(p) {
            parse_at_rule(p)
        } else if is_at_qualified_rule(p) {
            parse_qualified_rule(p)
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &RuleListParseRecovery, expected_any_rule)
    }
}

#[inline]
pub(crate) fn is_at_qualified_rule(p: &mut CssParser) -> bool {
    is_nth_at_selector(p, 0)
}

#[inline]
pub(crate) fn parse_qualified_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_qualified_rule(p) {
        return Absent;
    }

    let m = p.start();

    SelectorList::default().parse_list(p);

    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, CSS_QUALIFIED_RULE))
}

/// Checks if the current position in the CSS parser is at the start of a nested qualified rule.
/// Nested qualified rules are determined by the presence of a relative selector, indicating the
/// start of a rule that is nested within another rule.
#[inline]
pub(crate) fn is_at_nested_qualified_rule(p: &mut CssParser) -> bool {
    is_at_relative_selector(p)
}

/// Parses a nested qualified rule from the current position in the CSS parser. If the current
/// position is identified as the start of a nested qualified rule, it proceeds to parse the rule.
/// This involves parsing the list of relative selectors and then parsing or recovering the declaration
/// or rule list block. The kind of rule parsed (nested qualified or bogus) is determined based on
/// the success of parsing the block.
#[inline]
pub(crate) fn parse_nested_qualified_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_nested_qualified_rule(p) {
        return Absent;
    }

    let m = p.start();

    RelativeSelectorList::new(T!['{']).parse_list(p);

    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, CSS_NESTED_QUALIFIED_RULE))
}

pub(crate) struct DeclarationList;

impl ParseNodeList for DeclarationList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_DECLARATION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_declaration_with_semicolon(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T!['}'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set!(T!['}'])),
            expected_declaration_item,
        )
    }
}

pub(crate) fn is_at_declaration(p: &mut CssParser) -> bool {
    is_at_any_property(p)
}
#[inline]
pub(crate) fn parse_declaration(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration(p) {
        return Absent;
    }

    let m = p.start();

    parse_any_property(p).ok();
    parse_declaration_important(p).ok();

    Present(m.complete(p, CSS_DECLARATION))
}

/// Parses a CSS declaration that may optionally end with a semicolon.
///
/// This function attempts to parse a single CSS declaration from the current position
/// of the parser. It handles the optional semicolon (';') at the end of the declaration,
/// adhering to CSS syntax rules where the semicolon is mandatory for all declarations
/// except the last one in a block. In the case of the last declaration before a closing
/// brace ('}'), the semicolon is optional. If the semicolon is omitted for declarations
/// that are not at the end, the parser will raise an error.
#[inline]
pub(crate) fn parse_declaration_with_semicolon(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration(p) {
        return Absent;
    }

    let m = p.start();

    parse_declaration(p).ok();

    // If the next token is a closing brace ('}'), the semicolon is optional.
    // Otherwise, a semicolon is expected and the parser will enforce its presence.
    // div { color: red; }
    // div { color: red }
    if !p.at(T!['}']) {
        if p.nth_at(1, T!['}']) {
            p.eat(T![;]);
        } else {
            p.expect(T![;]);
        }
    }

    Present(m.complete(p, CSS_DECLARATION_WITH_SEMICOLON))
}

#[inline]
fn is_at_declaration_important(p: &mut CssParser) -> bool {
    p.at(T![!]) && p.nth_at(1, T![important])
}

#[inline]
fn parse_declaration_important(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_declaration_important(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![!]);
    p.bump(T![important]);
    Present(m.complete(p, CSS_DECLARATION_IMPORTANT))
}

#[inline]
pub(crate) fn is_at_any_value(p: &mut CssParser) -> bool {
    is_at_any_function(p)
        || is_at_identifier(p)
        || p.at(CSS_STRING_LITERAL)
        || is_at_any_dimension(p)
        || p.at(CSS_NUMBER_LITERAL)
        || is_at_dashed_identifier(p)
        || is_at_ratio(p)
        || is_at_color(p)
}

#[inline]
pub(crate) fn parse_any_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        parse_any_function(p)
    } else if is_at_dashed_identifier(p) {
        parse_dashed_identifier(p)
    } else if is_at_identifier(p) {
        parse_regular_identifier(p)
    } else if p.at(CSS_STRING_LITERAL) {
        parse_string(p)
    } else if is_at_any_dimension(p) {
        parse_any_dimension(p)
    } else if is_at_ratio(p) {
        parse_ratio(p)
    } else if p.at(CSS_NUMBER_LITERAL) {
        parse_regular_number(p)
    } else if is_at_color(p) {
        parse_color(p)
    } else {
        Absent
    }
}

#[inline]
pub(crate) fn is_at_color(p: &mut CssParser) -> bool {
    p.at(T![#])
}
#[inline]
pub(crate) fn parse_color(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_color(p) {
        return Absent;
    }
    let m = p.start();
    p.bump_with_context(T![#], CssLexContext::Color);
    p.expect(CSS_COLOR_LITERAL);
    Present(m.complete(p, CSS_COLOR))
}

struct CssComponentValueList;
impl ParseNodeList for CssComponentValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_COMPONENT_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_any_value(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at_ts(BINARY_OPERATION_TOKEN)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set!(T![')'], T![;])),
            expected_component_value,
        )
    }
}

#[inline]
pub(crate) fn is_at_ratio(p: &mut CssParser) -> bool {
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![/])
}

#[inline]
pub(crate) fn parse_ratio(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_ratio(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_number(p).ok();
    p.eat(T![/]);
    parse_regular_number(p).or_add_diagnostic(p, expected_number);
    Present(m.complete(p, CSS_RATIO))
}

#[inline]
pub(crate) fn is_at_css_wide_keyword(p: &mut CssParser) -> bool {
    p.cur().is_css_wide_keyword()
}

#[inline]
pub(crate) fn is_at_identifier(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
}

#[inline]
pub(crate) fn is_nth_at_identifier(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T![ident]) || p.nth(n).is_contextual_keyword()
}

/// Parse any identifier using the Regular lexing context.
#[inline]
pub(crate) fn parse_regular_identifier(p: &mut CssParser) -> ParsedSyntax {
    parse_identifier(p, CssLexContext::Regular)
}

/// Parse any identifier as a general CssIdentifier. Regular identifiers are
/// case-insensitive, often used for property names, values, etc.
#[inline]
pub(crate) fn parse_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_IDENTIFIER);

    Present(identifier)
}

/// Custom identifiers are identifiers not defined by CSS itself. These _are_
/// case-sensitive, used for class names, ids, etc. Custom identifiers _may_
/// have the same value as an identifier defined by CSS (e.g, `color`, used as
/// a class name), however they _must not_ be any of the CSS-wide keywords.
///
/// Custom identifiers have the same syntax as general identifiers, so the
/// [is_at_identifier] function can be used to check for both while parsing.
///
/// Custom identifiers can also be used in places where the CSS grammar
/// specifies `<ident>` but also includes case-sensitivity, such as in
/// class and id selectors. In these cases, CSS wide keywords _are_ accepted,
/// and can be handled by calling `parse_custom_identifier_with_keywords` with
/// `allow_css_wide_keywords` as `true` to cast them as identifiers.
///
/// When recovering from a parse error here, use
/// [parse_error::expected_non_css_wide_keyword_identifier] to provide the user
/// with additional information about how the CSS-wide keywords are not allowed
/// as custom identifiers.
#[inline]
pub(crate) fn parse_custom_identifier(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    parse_custom_identifier_with_keywords(p, context, false)
}

/// See [parse_custom_identifier]. This function allows for overriding the
/// handling of CSS-wide keywords using the `allow_css_wide_keywords` parameter.
///
/// This function should only be needed in cases where the CSS specification
/// defines a token as `<ident>` _and also_ case-sensitive. Otherwise, either
/// `parse_identifier` or `parse_custom_identifier` should be sufficient.
#[inline]
pub(crate) fn parse_custom_identifier_with_keywords(
    p: &mut CssParser,
    context: CssLexContext,
    allow_css_wide_keywords: bool,
) -> ParsedSyntax {
    if !is_at_identifier(p) || (!allow_css_wide_keywords && is_at_css_wide_keyword(p)) {
        return Absent;
    }

    let m = p.start();
    p.bump_remap_with_context(T![ident], context);
    let identifier = m.complete(p, CSS_CUSTOM_IDENTIFIER);

    Present(identifier)
}

#[inline]
pub(crate) fn is_at_dashed_identifier(p: &mut CssParser) -> bool {
    is_at_identifier(p) && p.cur_text().starts_with("--")
}

/// Dashed identifiers are any identifiers that start with two dashes (`--`).
/// Case sensitive, these are guaranteed to never overlap with an identifier
/// defined by CSS.
#[inline]
pub(crate) fn parse_dashed_identifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_dashed_identifier(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![ident]);
    Present(m.complete(p, CSS_DASHED_IDENTIFIER))
}

#[inline]
pub(crate) fn parse_regular_number(p: &mut CssParser) -> ParsedSyntax {
    parse_number(p, CssLexContext::Regular)
}
#[inline]
pub(crate) fn parse_number(p: &mut CssParser, context: CssLexContext) -> ParsedSyntax {
    if !p.at(CSS_NUMBER_LITERAL) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(CSS_NUMBER_LITERAL, context);

    Present(m.complete(p, CSS_NUMBER))
}

#[inline]
pub(crate) fn parse_string(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_string(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(CSS_STRING_LITERAL);

    Present(m.complete(p, CSS_STRING))
}

fn is_at_string(p: &mut CssParser) -> bool {
    p.at(CSS_STRING_LITERAL)
}

/// Attempt to parse some input with the given parsing function. If parsing
/// succeeds, `Ok` is returned with the result of the parse and the state is
/// preserved. If parsing fails, this function rewinds the parser back to
/// where it was before attempting the parse and the `Err` value is returned.
#[must_use = "The result of try_parse contains information about whether the parse succeeded and should not be ignored"]
#[allow(dead_code)]
pub(crate) fn try_parse<T, E>(
    p: &mut CssParser,
    func: impl FnOnce(&mut CssParser) -> Result<T, E>,
) -> Result<T, E> {
    let checkpoint = p.checkpoint();
    let old_speculative_parsing = std::mem::replace(&mut p.state_mut().speculative_parsing, true);

    let res = func(p);
    p.state_mut().speculative_parsing = old_speculative_parsing;

    if res.is_err() {
        p.rewind(checkpoint);
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{parser::CssParser, CssParserOptions};
    use biome_css_syntax::{CssSyntaxKind, T};
    use biome_parser::prelude::ParsedSyntax::{Absent, Present};
    use biome_parser::Parser;

    use super::{parse_regular_identifier, parse_regular_number, try_parse};

    #[test]
    fn try_parse_rewinds_to_checkpoint() {
        let mut p = CssParser::new("width: blue;", CssParserOptions::default());

        let pre_try_range = p.cur_range();
        let result = try_parse(&mut p, |p| {
            // advance the parser within the attempt
            // parse `width`
            parse_regular_identifier(p).ok();
            // parse `:`
            p.expect(T![:]);

            // attempt to parse a number, but fail because the input has `blue`.
            match parse_regular_number(p) {
                Present(marker) => Ok(Present(marker)),
                Absent => Err(()),
            }
        });

        assert!(result.is_err());
        // The parser should've rewound back to the start.
        assert_eq!(p.cur_range(), pre_try_range);
        assert_eq!(p.cur_text(), "width");
    }

    #[test]
    fn try_parse_preserves_position_on_success() {
        let mut p = CssParser::new("width: 100;", CssParserOptions::default());

        let pre_try_range = p.cur_range();
        let result = try_parse(&mut p, |p| {
            // advance the parser within the attempt
            // parse `width`
            parse_regular_identifier(p).ok();
            // parse `:`
            p.expect(T![:]);

            // attempt to parse a number, and succeed because the input has `100`.
            match parse_regular_number(p) {
                Present(marker) => Ok(Present(marker)),
                Absent => Err(()),
            }
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap().kind(&p), Some(CssSyntaxKind::CSS_NUMBER));
        // The parser should not have rewound and is now at the semicolon
        assert_ne!(p.cur_range(), pre_try_range);
        assert_eq!(p.cur_text(), ";");
    }
}
