use crate::parser::CssParser;
use crate::syntax::block::{parse_declaration_block, parse_rule_block};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;

/// Checks if the current token in the parser is a `@starting-style` at-rule.
///
/// This function verifies if the current token matches the `@starting-style` rule,
/// which is a custom at-rule used for specific parsing scenarios.
#[inline]
pub(crate) fn is_at_starting_style_at_rule(p: &mut CssParser) -> bool {
    p.at(T![starting_style])
}

/// Parses a `@starting-style` at-rule in a CSS stylesheet.
///
/// This function handles the parsing of a `@starting-style` at-rule, which is defined in the
/// CSS Transitions Level 2 specification. It starts by confirming the presence of such a rule and then
/// processes the content depending on whether the parser is currently inside a nesting block or at the root level.
/// It employs different parsing strategies for declarations or rules based on the parser state `is_nesting_block`.
///
/// Specification: [CSS Transitions Level 2 - @starting-style](https://drafts.csswg.org/css-transitions-2/#at-ruledef-starting-style)
/// # Examples
/// Basic usage in a CSS stylesheet:
///
/// ```css
/// // At the root level of a stylesheet
/// @starting-style {
///     /* rulesets */
/// }
///
/// // Inside a selector
/// selector {
///   @starting-style {
///     /* declarations */
///   }
/// }
/// ```
#[inline]
pub(crate) fn parse_starting_style_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_starting_style_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![starting_style]);

    if p.state().is_nesting_block {
        parse_declaration_block(p);
    } else {
        parse_rule_block(p);
    };

    Present(m.complete(p, CSS_STARTING_STYLE_AT_RULE))
}
