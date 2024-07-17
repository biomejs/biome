use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, TextRange, T};
use biome_parser::diagnostic::{expect_one_of, ParseDiagnostic};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::ToDiagnostic;
use biome_parser::{parsed_syntax::ParsedSyntax, token_set, Parser};

use crate::parser::CssParser;
use crate::syntax::parse_error::{expected_component_value, expected_identifier};
use crate::syntax::{
    is_at_identifier, is_at_string, is_nth_at_identifier, parse_regular_identifier, parse_string,
};

/// Checks if the current token in the parser is a `@value` at-rule.
///
/// This function verifies if the current token matches the `@value` rule.
#[inline]
pub(crate) fn is_at_value_at_rule(p: &mut CssParser) -> bool {
    p.at(T![value])
}

/// Parses a `@value` at-rule in a CSS stylesheet.
/// For details, see [CSS Modules Values](https://github.com/css-modules/postcss-modules-values).
/// ```css
/// @value my-color #f00;
/// @value my-size 10px;
/// @value primary, secondary from colors;
/// @value small as bp-small, medium, large as bp-large from "./breakpoints.css";
/// ```
/// This function identifies and parses these `@value` rules within CSS stylesheets.
#[inline]
pub(crate) fn parse_value_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_value_at_rule(p) {
        return Absent;
    }

    if p.options().is_css_modules_disabled() {
        // @value at-rule is not a standard CSS feature.
        // Provide a hint on how to enable parsing of @value at-rules.
        p.error(value_at_rule_not_allowed(p, p.cur_range()));

        // Skip the entire rule to avoid parsing errors.
        // Skip until the next semicolon.
        while !p.eat(T![;]) {
            p.bump_any();
        }

        return Absent;
    }

    let m = p.start();

    p.bump(T![value]);

    if is_at_value_at_rule_declaration_clause(p) {
        parse_value_at_rule_declaration_clause(p).ok();
    } else if is_at_value_at_rule_import_clause(p) {
        parse_value_at_rule_import_clause(p).ok();
    } else {
        p.error(expected_at_rule_declaration_clause(p, p.cur_range()));
    }

    p.expect(T![;]);

    Present(m.complete(p, CSS_VALUE_AT_RULE))
}

/// Checks if the current parser position is at a value at-rule import clause.
fn is_at_value_at_rule_import_clause(p: &mut CssParser) -> bool {
    is_at_identifier(p)
}

/// Parses a value at-rule import clause from the CSS parser.
/// ```css
/// @value red, green from "./colors.css" ;
/// @value my-size 10px;
/// @value primary, secondary from colors;
/// @value small as bp-small, medium, large as bp-large from "./breakpoints.css";
/// ```
fn parse_value_at_rule_import_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_value_at_rule_import_clause(p) {
        return Absent;
    }

    let m = p.start();

    ValueAtRuleImportSpecifierList.parse_list(p);
    p.expect(T![from]);
    parse_value_at_rule_import_source(p).or_add_diagnostic(p, expected_import_source);

    Present(m.complete(p, CSS_VALUE_AT_RULE_IMPORT_CLAUSE))
}

struct ValueAtRuleImportSpecifierList;

impl ParseSeparatedList for ValueAtRuleImportSpecifierList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_VALUE_AT_RULE_IMPORT_SPECIFIER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_value_at_rule_import_specifier(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![from])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ValueAtRuleImportSpecifierListParseRecovery,
            expected_import_specifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

struct ValueAtRuleImportSpecifierListParseRecovery;

impl ParseRecovery for ValueAtRuleImportSpecifierListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        // , is a separator, from is an end of a list
        p.at_ts(token_set![T![,], T![from]])
    }
}

/// Parses a value at-rule import specifier from the CSS parser.
fn parse_value_at_rule_import_specifier(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_regular_identifier(p).ok();

    let kind = if p.eat(T![as]) {
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
        CSS_VALUE_AT_RULE_NAMED_IMPORT_SPECIFIER
    } else {
        CSS_VALUE_AT_RULE_IMPORT_SPECIFIER
    };

    Present(m.complete(p, kind))
}

/// Checks if the current parser position is at a value at-rule import source.
fn is_at_value_at_rule_import_source(p: &mut CssParser) -> bool {
    is_at_identifier(p) || is_at_string(p)
}

/// Parses a value at-rule import source from the CSS parser.
fn parse_value_at_rule_import_source(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_value_at_rule_import_source(p) {
        return Absent;
    }

    if is_at_identifier(p) {
        parse_regular_identifier(p)
    } else {
        parse_string(p)
    }
}

/// Checks if the current parser position is at a value at-rule declaration clause.
fn is_at_value_at_rule_declaration_clause(p: &mut CssParser) -> bool {
    is_nth_at_value_at_rule_generic_property(p, 0)
}

/// Parses a value at-rule declaration clause from the CSS parser.
fn parse_value_at_rule_declaration_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_value_at_rule_declaration_clause(p) {
        return Absent;
    }

    let m = p.start();
    ValueAtRulePropertyList.parse_list(p);
    Present(m.complete(p, CSS_VALUE_AT_RULE_DECLARATION_CLAUSE))
}

pub(crate) struct ValueAtRulePropertyList;

impl ParseSeparatedList for ValueAtRulePropertyList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_VALUE_AT_RULE_PROPERTY_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_value_at_rule_generic_property(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &ValueAtRulePropertyListParseRecovery,
            expected_component_value,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// Checks if the parser is at a generic property of a value at-rule at the nth position.
fn is_nth_at_value_at_rule_generic_property(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_identifier(p, n) && p.nth_at(n + 1, T![:])
}

/// Parses a generic property of a value at-rule from the CSS parser.
/// A generic property can be any sequence of tokens, which is why we parse it as an unformed tree.
/// This approach allows for flexibility, as the exact structure of generic properties may not be well-defined.
/// By parsing the property in this manner, we ensure that we can handle a wide variety of cases without
/// requiring specific grammar rules for each possible property structure.
///
/// If we want to extend the parsing with a specific grammar in the future, we can implement speculative parsing.
/// Speculative parsing would attempt to parse the property according to the new grammar rules.
/// If an error occurs during this process, we can fall back to the current implementation, which treats
/// the property as an unformed tree. This approach ensures backward compatibility and allows for incremental
/// improvements to the parser without breaking existing functionality.
fn parse_value_at_rule_generic_property(p: &mut CssParser) -> ParsedSyntax {
    if !is_nth_at_value_at_rule_generic_property(p, 0) {
        return Absent;
    }

    let m = p.start();
    parse_regular_identifier(p).ok();

    p.expect(T![:]);

    {
        let m = p.start();

        // Skip all tokens until the end of the property value or the next property.
        // EOF indicates the end of the file.
        // `;` indicates the end of the list.
        // `,` is the separator before the next property.
        while !(p.at(EOF)
            || ValueAtRulePropertyList.is_at_list_end(p)
            || p.at(T![,]) && is_nth_at_value_at_rule_generic_property(p, 1))
        {
            p.bump_any();
        }
        m.complete(p, CSS_VALUE_AT_RULE_GENERIC_VALUE);
    }

    Present(m.complete(p, CSS_VALUE_AT_RULE_GENERIC_PROPERTY))
}

/// Structure for recovering from parsing errors in a value at-rule property list.
struct ValueAtRulePropertyListParseRecovery;

impl ParseRecovery for ValueAtRulePropertyListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_PROPERTY;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;]) || is_at_identifier(p) || p.has_nth_preceding_line_break(0)
    }
}

/// Creates a diagnostic for an expected import source.
fn expected_import_source(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["identifier", "string"], range).into_diagnostic(p)
}

/// Creates a diagnostic for an expected import specifier.
fn expected_import_specifier(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(&["identifier", "<identifier> as <identifier>"], range).into_diagnostic(p)
}

/// Generates a parse diagnostic for an expected at-rule declaration clause.
/// This function returns a diagnostic error indicating that an at-rule declaration clause
/// or import clause was expected at the given range in the CSS parser.
fn expected_at_rule_declaration_clause(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    expect_one_of(
        &["declaration at rule clause", "import at rule clause"],
        range,
    )
    .into_diagnostic(p)
}

/// Generates a parse diagnostic for when the @value at-rule is not allowed.
///
/// This function returns an error diagnostic indicating that the @value at-rule
/// is not a standard CSS feature. It also provides a hint on how to enable
/// parsing of @value at-rules by setting the `css.parser.cssModules` option to `true`
/// in the configuration file.
pub(crate) fn value_at_rule_not_allowed(p: &CssParser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        "@value at-rule is not a standard CSS feature.",
        range,
    )
        .with_hint(
            "You can enable @value at-rule parsing by setting the `css.parser.cssModules` option to `true` in your configuration file.",
        )
}
