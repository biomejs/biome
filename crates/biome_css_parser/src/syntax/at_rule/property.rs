use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::TokenSet;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::token_set;
use biome_parser::{Parser, SyntaxFeature, parsed_syntax::ParsedSyntax};

use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_block;
use crate::syntax::parse_error::{expected_dashed_identifier, scss_only_syntax_error};
use crate::syntax::scss::{
    is_at_scss_interpolated_dashed_identifier, parse_scss_interpolated_dashed_identifier,
};
use crate::syntax::{CssSyntaxFeatures, parse_dashed_identifier};

#[inline]
pub(crate) fn is_at_property_at_rule(p: &mut CssParser) -> bool {
    p.at(T![property])
}

#[inline]
pub(crate) fn parse_property_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_property_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    parse_property_at_rule_declarator(p).ok();
    parse_declaration_block(p);

    Present(m.complete(p, CSS_PROPERTY_AT_RULE))
}

#[inline]
pub(crate) fn parse_property_at_rule_declarator(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_property_at_rule(p) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![property]);

    let decl_kind = if parse_property_name(p)
        .or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS, PROPERTY_RECOVERY_SET)
                .enable_recovery_on_line_break(),
            expected_dashed_identifier,
        )
        .is_ok()
    {
        CSS_PROPERTY_AT_RULE_DECLARATOR
    } else {
        CSS_BOGUS
    };

    Present(m.complete(p, decl_kind))
}

/// Parses a plain or SCSS-interpolated custom property name.
///
/// ```scss
/// @property --#{$name} {}
/// @property --theme-#{$name} {}
/// ```
#[inline]
fn parse_property_name(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_interpolated_dashed_identifier(p) {
        CssSyntaxFeatures::Scss.parse_exclusive_syntax(
            p,
            parse_scss_interpolated_dashed_identifier,
            |p, m| scss_only_syntax_error(p, "SCSS interpolated dashed identifiers", m.range(p)),
        )
    } else {
        parse_dashed_identifier(p)
    }
}

const PROPERTY_RECOVERY_SET: TokenSet<CssSyntaxKind> = token_set!(T!['{']);
