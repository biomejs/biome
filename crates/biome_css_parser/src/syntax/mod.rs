mod at_rule;
mod block;
mod css_modules;
mod declaration;
mod parse_error;
mod property;
mod scss;
mod selector;
mod util;
mod value;

use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::at_rule::{is_at_at_rule, parse_at_rule};
use crate::syntax::block::{DeclarationOrRuleList, parse_declaration_or_rule_list_block};
use crate::syntax::parse_error::{
    expected_any_rule, expected_component_value, expected_non_css_wide_keyword_identifier,
    inconsistent_scss_bracketed_list_separators, scss_only_syntax_error, tailwind_disabled,
};
use crate::syntax::property::color::{is_at_color, parse_color};
use crate::syntax::property::unicode_range::{is_at_unicode_range, parse_unicode_range};
use crate::syntax::scss::{
    is_at_scss_declaration, is_at_scss_identifier, is_at_scss_parent_selector_value,
    is_at_scss_qualified_name, parse_scss_declaration, parse_scss_identifier,
    parse_scss_parent_selector_value, parse_scss_qualified_name,
};
use crate::syntax::selector::SelectorList;
use crate::syntax::selector::is_nth_at_selector;
use crate::syntax::selector::relative_selector::{RelativeSelectorList, is_at_relative_selector};
use crate::syntax::value::function::{
    BINARY_OPERATION_TOKEN, parse_tailwind_value_theme_reference,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, EmbeddingKind, T};
use biome_parser::parse_lists::{ParseNodeList, ParseSeparatedList};
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature, token_set};
use value::dimension::{is_at_any_dimension, parse_any_dimension};
use value::function::{is_at_any_function, parse_any_function};

pub(crate) enum CssSyntaxFeatures {
    /// Enable support for SCSS-specific syntax.
    Scss,
    /// Enable support for Tailwind CSS directives and syntax.
    Tailwind,

    /// Enable support for CSS Modules syntax.
    CssModules,

    /// Enable support for CSS Modules syntax plus parsing of pseudo selectors for `:slotted`, `:deep`, and the `v-bind()` function.
    CssModulesWithVue,
}

pub(crate) use declaration::{
    DeclarationList, is_at_any_declaration, is_at_any_declaration_with_semicolon,
    is_at_declaration, parse_any_declaration_with_semicolon, parse_declaration,
};

impl SyntaxFeature for CssSyntaxFeatures {
    type Parser<'source> = CssParser<'source>;

    fn is_supported(&self, p: &Self::Parser<'_>) -> bool {
        match self {
            Self::Scss => p.source_type.is_scss(),
            Self::Tailwind => p.options().is_tailwind_directives_enabled(),
            Self::CssModules => p.options().is_css_modules_enabled(),
            Self::CssModulesWithVue => p.options().is_css_modules_vue_enabled(),
        }
    }
}

pub(crate) fn parse_root(p: &mut CssParser) {
    let m = p.start();
    match p.source_type.as_embedding_kind() {
        EmbeddingKind::Styled => {
            DeclarationOrRuleList::new(EOF).parse_list(p);

            m.complete(p, CSS_SNIPPET_ROOT);
        }
        EmbeddingKind::None | EmbeddingKind::Html(_) => {
            p.eat(UNICODE_BOM);

            RootItemList.parse_list(p);

            m.complete(p, CSS_ROOT);
        }
    }
}

struct RootItemList;

#[inline]
pub(crate) fn is_at_root_item_list_element(p: &mut CssParser) -> bool {
    is_at_at_rule(p) || is_at_scss_declaration(p) || is_at_qualified_rule(p)
}

struct RootItemListParseRecovery;

impl ParseRecovery for RootItemListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_RULE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || is_at_root_item_list_element(p)
    }
}

impl ParseNodeList for RootItemList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_ROOT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_at_rule(p) {
            parse_at_rule(p)
        } else if is_at_scss_declaration(p) {
            CssSyntaxFeatures::Scss.parse_exclusive_syntax(
                p,
                parse_scss_declaration,
                |p, marker| {
                    scss_only_syntax_error(p, "SCSS variable declarations", marker.range(p))
                },
            )
        } else if is_at_qualified_rule(p) {
            parse_qualified_rule(p)
        } else {
            Absent
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(p, &RootItemListParseRecovery, expected_any_rule)
    }
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

struct RuleListParseRecovery {
    end_kind: CssSyntaxKind,
}

impl RuleListParseRecovery {
    fn new(end_kind: CssSyntaxKind) -> Self {
        Self { end_kind }
    }
}

impl ParseRecovery for RuleListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_RULE;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(self.end_kind) || is_at_rule_list_element(p)
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
        parsed_element.or_recover(
            p,
            &RuleListParseRecovery::new(self.end_kind),
            expected_any_rule,
        )
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
    parse_nested_qualified_rule_with_selector_recovery(p, false).map_or(Absent, |(rule, _)| rule)
}

/// Speculatively parses a nested qualified rule without selector recovery.
///
/// This is used to disambiguate SCSS nesting declarations from nested qualified rules.
/// The parse is considered successful only when the selector is strict and the parsed
/// block is complete.
#[inline]
pub(crate) fn try_parse_nested_qualified_rule_without_selector_recovery(
    p: &mut CssParser,
    end_kind: CssSyntaxKind,
) -> Result<ParsedSyntax, ()> {
    try_parse(p, |p| {
        let Some((rule, block_kind)) = parse_nested_qualified_rule_with_selector_recovery(p, true)
        else {
            return Err(());
        };

        if block_kind != CSS_DECLARATION_OR_RULE_BLOCK
            || p.last().is_none_or(|kind| kind != end_kind)
        {
            return Err(());
        }

        Ok(rule)
    })
}

#[inline]
fn parse_nested_qualified_rule_with_selector_recovery(
    p: &mut CssParser,
    disable_selector_recovery: bool,
) -> Option<(ParsedSyntax, CssSyntaxKind)> {
    if !is_at_nested_qualified_rule(p) {
        return None;
    }

    let m = p.start();

    if disable_selector_recovery {
        RelativeSelectorList::new(T!['{'])
            .disable_recovery()
            .parse_list(p);

        // In strict mode, reject selectors that don't reach the opening brace.
        if !p.at(T!['{']) {
            m.abandon(p);
            return None;
        }
    } else {
        RelativeSelectorList::new(T!['{']).parse_list(p);
    }

    let block = parse_declaration_or_rule_list_block(p);
    let block_kind = block.kind(p);

    Some((
        Present(m.complete(p, CSS_NESTED_QUALIFIED_RULE)),
        block_kind,
    ))
}

#[inline]
fn is_at_metavariable(p: &mut CssParser) -> bool {
    p.at(GRIT_METAVARIABLE)
}

#[inline]
fn is_nth_at_metavariable(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, GRIT_METAVARIABLE)
}

#[inline]
fn parse_metavariable(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_metavariable(p) {
        return Absent;
    }
    let m = p.start();
    p.bump(GRIT_METAVARIABLE);
    Present(m.complete(p, CSS_METAVARIABLE))
}

#[inline]
pub(crate) fn is_at_any_value(p: &mut CssParser) -> bool {
    is_at_any_function(p)
        || is_at_scss_identifier(p)
        || is_at_scss_qualified_name(p)
        || is_at_scss_parent_selector_value(p)
        || is_at_identifier(p)
        || p.at(CSS_STRING_LITERAL)
        || is_at_any_dimension(p)
        || p.at(CSS_NUMBER_LITERAL)
        || is_at_dashed_identifier(p)
        || is_at_ratio(p)
        || is_at_color(p)
        || is_at_bracketed_value(p)
        || is_at_metavariable(p)
}

#[inline]
pub(crate) fn parse_any_value(p: &mut CssParser) -> ParsedSyntax {
    if is_at_any_function(p) {
        parse_any_function(p)
    } else if is_at_scss_identifier(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(p, parse_scss_identifier, |p, m| {
            scss_only_syntax_error(p, "SCSS variables", m.range(p))
        })
    } else if is_at_scss_qualified_name(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(p, parse_scss_qualified_name, |p, m| {
            scss_only_syntax_error(p, "SCSS qualified names", m.range(p))
        })
    } else if is_at_scss_parent_selector_value(p) {
        parse_scss_parent_selector_value(p)
    } else if is_at_dashed_identifier(p) {
        if p.nth_at(1, T![-]) && p.nth_at(2, T![*]) {
            CssSyntaxFeatures::Tailwind.parse_exclusive_syntax(
                p,
                parse_tailwind_value_theme_reference,
                |p, m| tailwind_disabled(p, m.range(p)),
            )
        } else {
            parse_dashed_identifier(p)
        }
    } else if is_at_unicode_range(p) {
        parse_unicode_range(p)
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
    } else if is_at_bracketed_value(p) {
        parse_bracketed_value(p)
    } else if is_at_metavariable(p) {
        parse_metavariable(p)
    } else {
        Absent
    }
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
    p.at(CSS_NUMBER_LITERAL) && p.nth_at(1, T![/]) && p.nth_at(2, CSS_NUMBER_LITERAL)
}

#[inline]
pub(crate) fn parse_ratio(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_ratio(p) {
        return Absent;
    }
    let m = p.start();
    parse_regular_number(p).ok();
    p.bump(T![/]);
    parse_regular_number(p).ok();
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

#[inline]
pub(crate) fn is_at_string(p: &mut CssParser) -> bool {
    p.at(CSS_STRING_LITERAL)
}

/// Checks if the parser is currently at the start of a bracketed value.
#[inline]
pub(crate) fn is_at_bracketed_value(p: &mut CssParser) -> bool {
    p.at(T!['['])
}

/// Parses a bracketed value from the current position in the CSS parser.
///
/// This function parses a list of values enclosed in square brackets, commonly used in CSS properties
/// like `grid-template-areas` where the value is a list of identifiers representing grid areas.
/// For details on the syntax of bracketed values,
/// see the [CSS Syntax specification](https://drafts.csswg.org/css-grid/#named-lines)
#[inline]
pub(crate) fn parse_bracketed_value(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_bracketed_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['[']);
    BracketedValueList::default().parse_list(p);
    p.expect(T![']']);

    Present(m.complete(p, CSS_BRACKETED_VALUE))
}

/// The list parser for bracketed values.
///
/// This parser is responsible for parsing a list of identifiers inside a bracketed value.
#[derive(Default)]
pub(crate) struct BracketedValueList {
    separator: Option<BracketedValueSeparator>,
    mixed_separators_reported: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum BracketedValueSeparator {
    Comma,
    Slash,
}

impl BracketedValueSeparator {
    fn from_current_token(p: &mut CssParser) -> Option<Self> {
        if p.at(T![,]) {
            Some(Self::Comma)
        } else if p.at(T![/]) {
            Some(Self::Slash)
        } else {
            None
        }
    }

    fn bump(self, p: &mut CssParser) {
        match self {
            Self::Comma => p.bump(T![,]),
            Self::Slash => p.bump(T![/]),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Comma => "`,`",
            Self::Slash => "`/`",
        }
    }
}

impl ParseNodeList for BracketedValueList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_BRACKETED_VALUE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if p.at(T![*]) {
            return CssSyntaxFeatures::Tailwind.parse_exclusive_syntax(
                p,
                |p| {
                    let m = p.start();
                    p.bump_remap(T![ident]);
                    Present(m.complete(p, CSS_CUSTOM_IDENTIFIER))
                },
                |p, m| tailwind_disabled(p, m.range(p)),
            );
        }

        if let Some(separator) = BracketedValueSeparator::from_current_token(p) {
            return self.parse_scss_bracketed_value_delimiter(p, separator);
        }

        parse_custom_identifier(p, CssLexContext::Regular)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![']'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &BracketedValueListRecovery,
            expected_non_css_wide_keyword_identifier,
        )
    }
}

impl BracketedValueList {
    fn parse_scss_bracketed_value_delimiter(
        &mut self,
        p: &mut CssParser,
        separator: BracketedValueSeparator,
    ) -> ParsedSyntax {
        // Preserve explicit separators inside bracketed values so Sass list
        // separators survive parsing (e.g. `[a, b, c]`).
        // Example: `$list: [a, b, c];`
        // Docs: https://sass-lang.com/documentation/values/lists
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            |p| {
                let m = p.start();
                self.report_mixed_separator_if_needed(p, separator);
                separator.bump(p);
                Present(m.complete(p, CSS_GENERIC_DELIMITER))
            },
            |p, m| scss_only_syntax_error(p, "Sass list separators", m.range(p)),
        )
    }

    fn report_mixed_separator_if_needed(
        &mut self,
        p: &mut CssParser,
        separator: BracketedValueSeparator,
    ) {
        let Some(previous_separator) = self.separator else {
            self.separator = Some(separator);
            return;
        };

        if previous_separator != separator && !self.mixed_separators_reported {
            p.error(inconsistent_scss_bracketed_list_separators(
                p,
                previous_separator.as_str(),
                separator.as_str(),
                p.cur_range(),
            ));
            self.mixed_separators_reported = true;
        }
    }
}

/// Recovery strategy for bracketed value lists.
///
/// This recovery strategy handles the recovery process when parsing bracketed value lists.
struct BracketedValueListRecovery;

impl ParseRecovery for BracketedValueListRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_CUSTOM_IDENTIFIER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // If the next token is the end of the list or the next element, we're at a recovery point.
        p.at(T![']'])
            || is_at_identifier(p)
            || (CssSyntaxFeatures::Scss.is_supported(p) && (p.at(T![,]) || p.at(T![/])))
    }
}

/// Attempt to parse some input with the given parsing function. If parsing
/// succeeds, `Ok` is returned with the result of the parse and the state is
/// preserved. If parsing fails, this function rewinds the parser back to
/// where it was before attempting the parse and the `Err` value is returned.
#[must_use = "The result of try_parse contains information about whether the parse succeeded and should not be ignored"]
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
    use crate::{CssParserOptions, parser::CssParser};
    use biome_css_syntax::{CssFileSource, CssSyntaxKind, T};
    use biome_parser::Parser;
    use biome_parser::prelude::ParsedSyntax::{Absent, Present};

    use super::{parse_regular_identifier, parse_regular_number, try_parse};

    #[test]
    fn try_parse_rewinds_to_checkpoint() {
        let mut p = CssParser::new(
            "width: blue;",
            CssFileSource::css(),
            CssParserOptions::default(),
        );

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
        let mut p = CssParser::new(
            "width: 100;",
            CssFileSource::css(),
            CssParserOptions::default(),
        );

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
