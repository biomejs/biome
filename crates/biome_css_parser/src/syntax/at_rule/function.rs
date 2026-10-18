use crate::lexer::{CssCustomPropertyCommentMode, CssLexContext};
use crate::syntax::at_rule::parse_error::{
    expected_any_type, expected_function_parameter, expected_parameter_default_value,
};
use crate::syntax::block::parse_declaration_or_at_rule_list_block;
use crate::syntax::parse_error::expected_dashed_identifier;
use crate::syntax::property::parse_function_parameter_default_components;
use crate::syntax::scss::{
    is_at_scss_interpolated_dashed_identifier, parse_scss_interpolated_dashed_identifier,
};
use crate::syntax::value::r#type::{
    is_at_syntax_single_component, is_at_syntax_type, is_at_type_function,
    parse_any_syntax_component, parse_type_function,
};
use crate::syntax::{is_at_dashed_identifier, is_at_identifier, try_parse};
use crate::{parser::CssParser, syntax::parse_dashed_identifier};
use biome_css_syntax::{
    CssSyntaxKind::{self, *},
    T, TextSize,
};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecovery, ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::token_set;
use biome_parser::{
    Parser,
    parsed_syntax::ParsedSyntax::{self, Present},
    prelude::ParsedSyntax::Absent,
};

/// Checks if the current token in the parser is a `@function` at-rule.
#[inline]
pub(crate) fn is_at_function_at_rule(p: &mut CssParser) -> bool {
    p.at(T![function])
}

/// For more detailed information on the CSS `@function` syntax, refer to the
/// [CSS Mixins Module](https://drafts.csswg.org/css-mixins-1/#function-rule) specification.
///
/// # Examples
/// Basic usage in CSS:
///
/// ```css
/// @function --transparent(--color, --alpha) {}
/// ```
#[inline]
pub(crate) fn parse_function_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_function_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    parse_function_at_rule_declarator(p).ok();
    parse_declaration_or_at_rule_list_block(p);

    Present(m.complete(p, CSS_FUNCTION_AT_RULE))
}

#[inline]
pub(crate) fn parse_function_at_rule_declarator(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_function_at_rule(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![function]);

    parse_function_name(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set![T!['('], T!['{']]),
            expected_dashed_identifier,
        )
        .ok();

    let opening = p.cur_range().start();
    let has_l_paren = p.expect(T!['(']);
    CssFunctionParameterList::parse_with_body_recovery(p, has_l_paren.then_some(opening));
    p.expect(T![')']);

    parse_returns_statement(p).ok();

    Present(m.complete(p, CSS_FUNCTION_AT_RULE_DECLARATOR))
}

#[inline]
fn parse_function_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolated_dashed_identifier(p) {
        parse_scss_interpolated_dashed_identifier(p)
    } else {
        parse_dashed_identifier(p)
    }
}

#[inline]
fn parse_function_parameter(p: &mut CssParser, is_recovering_at_body: bool) -> ParsedSyntax {
    // expect identifier and not just dashed identifier
    // to allow better recovery on invalid parameter names
    if !is_at_identifier(p) {
        return Absent;
    }

    let m = p.start();

    parse_dashed_identifier(p)
        .or_recover(
            p,
            &CssFunctionParameterNameParseRecovery,
            expected_dashed_identifier,
        )
        .ok();

    parse_any_type(p).ok();
    parse_function_parameter_default_value(p, is_recovering_at_body).ok();

    Present(m.complete(p, CSS_FUNCTION_PARAMETER))
}

#[inline]
fn parse_any_type(p: &mut CssParser) -> ParsedSyntax {
    if is_at_type_function(p) {
        return parse_type_function(p);
    }

    if is_at_syntax_single_component(p) {
        return parse_any_syntax_component(p);
    }

    Absent
}

#[inline]
fn is_at_function_parameter_default_value(p: &mut CssParser) -> bool {
    p.at(T![:])
}

/// Leaves parameter separators outside the default and nested commas inside it.
///
/// ```css
/// @function --shadow(--value: 0 1px rgb(0, 0, 0), --extra: 2px) {}
/// ```
#[inline]
fn parse_function_parameter_default_value(
    p: &mut CssParser,
    is_recovering_at_body: bool,
) -> ParsedSyntax {
    if !is_at_function_parameter_default_value(p) {
        return Absent;
    }

    let m = p.start();

    p.bump_with_context(
        T![:],
        CssLexContext::CustomPropertyValue(CssCustomPropertyCommentMode::ScssLineComments),
    );
    let components = parse_function_parameter_default_components(p, is_recovering_at_body);
    if components.range(p).is_empty() {
        p.error(expected_parameter_default_value(p, p.cur_range()));
    }

    Present(m.complete(p, CSS_FUNCTION_PARAMETER_DEFAULT_VALUE))
}

#[inline]
fn is_at_returns_statement(p: &mut CssParser) -> bool {
    p.at(T![returns])
}

#[inline]
fn parse_returns_statement(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_returns_statement(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![returns]);

    parse_any_type(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, token_set![T!['{']]),
            expected_any_type,
        )
        .ok();

    Present(m.complete(p, CSS_RETURNS_STATEMENT))
}

struct CssFunctionParameterListParseRecovery;

impl ParseRecovery for CssFunctionParameterListParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS_FUNCTION_PARAMETER;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![,]) || p.at(T![')']) || p.at(T![returns]) || p.at(T!['{'])
    }
}

#[derive(Default)]
struct CssFunctionParameterList {
    is_recovering_at_body: bool,
    has_structural_error: bool,
}

impl CssFunctionParameterList {
    /// Keeps braced defaults, but leaves the body outside a malformed header.
    ///
    /// ```css
    /// @function --value(--x: {a:b}) { result: var(--x); }
    /// @function --value(--x: red { result: red; }
    /// ```
    fn parse_with_body_recovery(p: &mut CssParser, opening: Option<TextSize>) {
        if let Some(opening) = opening
            && !p
                .state()
                .unclosed_function_default_parens
                .contains(&opening)
        {
            let has_encountered_if_function = p.state().encountered_if_function;
            let is_speculative = p.is_speculative_parsing();
            let result = try_parse(p, |p| {
                let mut parameters = Self::default();
                parameters.parse_list(p);
                // A following selector's `)` must not close a malformed header.
                if p.at(T![')']) && !parameters.has_structural_error {
                    Ok(())
                } else {
                    Err(())
                }
            });
            p.state_mut().encountered_if_function = has_encountered_if_function
                || (result.is_ok() && is_speculative && p.state().encountered_if_function);
            if result.is_ok() {
                return;
            }
        }

        Self {
            is_recovering_at_body: true,
            ..Self::default()
        }
        .parse_list(p);
    }
}

impl ParseSeparatedList for CssFunctionParameterList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = CSS_FUNCTION_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        self.has_structural_error |= !p.at(T![,]) && !is_at_dashed_identifier(p);
        parse_function_parameter(p, self.is_recovering_at_body)
    }

    fn diagnose_missing_element(&mut self, p: &mut Self::Parser<'_>) {
        p.error(expected_function_parameter(p, p.cur_range()));
    }

    fn expect_separator(&mut self, p: &mut Self::Parser<'_>) -> bool {
        let is_separator_present = p.expect(T![,]);
        self.has_structural_error |= !is_separator_present;
        is_separator_present
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T![')'], T![returns], T!['{']])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover(
            p,
            &CssFunctionParameterListParseRecovery,
            expected_function_parameter,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

struct CssFunctionParameterNameParseRecovery;

impl ParseRecovery for CssFunctionParameterNameParseRecovery {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const RECOVERED_KIND: Self::Kind = CSS_BOGUS;

    fn is_at_recovered(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at_ts(token_set![T![,], T![')'], T![:]]) || is_at_type_function(p) || is_at_syntax_type(p)
    }
}

#[cfg(test)]
mod tests {
    use super::parse_function_at_rule_declarator;
    use crate::{CssParserOptions, parser::CssParser, syntax::parse_root};
    use biome_languages::CssFileSource;
    use biome_parser::Parser;

    #[test]
    fn valid_function_defaults_do_not_rewind() {
        for source_type in [CssFileSource::css(), CssFileSource::scss()] {
            for name in ["--f", "--f ", "--#{$name}", "--#{$name} "] {
                if source_type == CssFileSource::css() && name.contains('#') {
                    continue;
                }
                let source =
                    format!("function {name}(--x: {{a; b !important}} @raw(value), --y: [c, d])\n")
                        .repeat(32);
                let mut parser = CssParser::new_with_metavariables(
                    &source,
                    source_type,
                    CssParserOptions::default(),
                    &[],
                );
                for _ in 0..32 {
                    assert!(parse_function_at_rule_declarator(&mut parser).is_present());
                }
                assert!(
                    parser.context().diagnostics().is_empty(),
                    "{source_type:?}: {name}"
                );
                assert_eq!(parser.source().rewinds, 0, "{source_type:?}: {name}");
            }
        }
    }

    #[test]
    fn malformed_function_defaults_have_linear_token_work() {
        let mut failures = Vec::new();
        for source_type in [CssFileSource::css(), CssFileSource::scss()] {
            for name in ["--f", "--f ", "--#{$name}", "--#{$name} "] {
                if source_type == CssFileSource::css() && name.contains('#') {
                    continue;
                }
                for suffix in [
                    "",
                    "\"unterminated",
                    "] @function --good(--x:red) {result:red;}",
                    "} @function --good(--x:red) {result:red;}",
                ] {
                    for count in [32, 64, 128, 1024] {
                        let mut source =
                            format!("@function {name}(--x:red {{result:red;}}\n").repeat(count);
                        source.push_str(suffix);
                        let mut parser = CssParser::new_with_metavariables(
                            &source,
                            source_type,
                            CssParserOptions::default(),
                            &[],
                        );
                        parse_root(&mut parser);
                        let steps = parser.source().token_steps;
                        println!(
                            "{source_type:?}: {name:?}: {suffix:?}: {count} headers, {steps} token steps"
                        );
                        if steps > count * 100 {
                            failures.push((source_type, name, suffix, count, steps));
                        }
                    }
                }
            }
        }
        assert!(failures.is_empty(), "repeated suffix scans: {failures:#?}");
    }

    #[test]
    fn malformed_function_defaults_without_opening_have_linear_token_work() {
        for source_type in [CssFileSource::css(), CssFileSource::scss()] {
            for count in [32, 64, 128] {
                let source = "@function --f --x:red {result:red;}\n".repeat(count);
                let mut parser = CssParser::new_with_metavariables(
                    &source,
                    source_type,
                    CssParserOptions::default(),
                    &[],
                );
                parse_root(&mut parser);
                assert!(
                    parser.source().token_steps <= count * 100,
                    "{source_type:?}: {count} headers, {} token steps",
                    parser.source().token_steps
                );
            }
        }
    }
}
