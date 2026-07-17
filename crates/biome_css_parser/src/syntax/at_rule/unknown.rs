use crate::parser::CssParser;
use crate::syntax::block::parse_declaration_or_rule_list_block;
use crate::syntax::scss::{
    is_at_scss_interpolation, parse_scss_interpolation_or_identifier,
    parse_scss_regular_interpolation,
};
use crate::syntax::{CssSyntaxFeatures, is_at_identifier, parse_regular_identifier};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::parsed_syntax::ParsedSyntax::Present;
use biome_parser::prelude::ParsedSyntax::Absent;
use biome_parser::prelude::*;
use biome_parser::{CompletedMarker, Marker, ParserProgress};

#[inline]
pub(crate) fn is_at_unknown_at_rule(p: &mut CssParser) -> bool {
    is_at_identifier(p)
}

/// Parses an unknown CSS at-rule, including Sass plain-CSS passthroughs.
///
/// Examples:
/// ```scss
/// @unknown #{$value};
/// @#{$rule-name} #{$value};
/// @unknown #{meta.inspect($value)} { color: red; }
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/css/
#[inline]
pub(crate) fn parse_unknown_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_unknown_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    // Guarded by `is_at_unknown_at_rule`.
    parse_regular_identifier(p).ok();
    parse_unknown_at_rule_components(p);

    complete_unknown_at_rule(p, m)
}

/// Parses an unknown SCSS at-rule with an interpolated name.
///
/// Example:
/// ```scss
/// @#{$rule-name} #{$value};
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/css/
#[inline]
pub(crate) fn parse_scss_interpolated_unknown_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_interpolation(p) {
        return Absent;
    }

    let m = p.start();

    // Guarded by `is_at_scss_interpolation`.
    parse_scss_interpolation_or_identifier(p).ok();
    parse_scss_unknown_at_rule_components(p);

    complete_unknown_at_rule(p, m)
}

#[inline]
fn complete_unknown_at_rule(p: &mut CssParser, m: Marker) -> ParsedSyntax {
    let kind = if p.at(T!['{']) {
        parse_declaration_or_rule_list_block(p);
        CSS_UNKNOWN_BLOCK_AT_RULE
    } else {
        p.expect(T![;]);
        CSS_UNKNOWN_VALUE_AT_RULE
    };

    Present(m.complete(p, kind))
}

/// Parses the generic at-rule prelude before `;` or a top-level block.
///
/// Example: `@unknown fn({ width: 300px }) #{$query} {}` keeps the function
/// body balanced while the final `{` starts the at-rule block.
#[inline]
fn parse_unknown_at_rule_components(p: &mut CssParser) -> CompletedMarker {
    parse_unknown_at_rule_components_with(p, consume_unknown_at_rule_component)
}

#[inline]
fn parse_scss_unknown_at_rule_components(p: &mut CssParser) -> CompletedMarker {
    parse_unknown_at_rule_components_with(p, consume_scss_unknown_at_rule_component)
}

#[inline]
fn parse_unknown_at_rule_components_with(
    p: &mut CssParser,
    mut parse_component: impl FnMut(&mut CssParser) -> bool,
) -> CompletedMarker {
    let m = p.start();
    let mut progress = ParserProgress::default();
    let mut balance = UnknownAtRuleComponentBalance::default();

    while !balance.is_at_end(p) {
        progress.assert_progressing(p);
        if !parse_component(p) {
            balance.bump(p);
        }
    }

    m.complete(p, CSS_UNKNOWN_AT_RULE_COMPONENT_LIST)
}

#[inline]
fn consume_unknown_at_rule_component(p: &mut CssParser) -> bool {
    if CssSyntaxFeatures::Scss.is_supported(p) && is_at_scss_interpolation(p) {
        // `@unknown #{$value};`: keep interpolation structured inside the
        // otherwise token-shaped generic prelude.
        parse_scss_regular_interpolation(p).ok();
        true
    } else {
        false
    }
}

#[inline]
fn consume_scss_unknown_at_rule_component(p: &mut CssParser) -> bool {
    if is_at_scss_interpolation(p) {
        // `@#{$name} #{$value};`: this parser is called from the SCSS-exclusive
        // at-rule name entrypoint, so interpolation stays structured even when
        // SCSS is unsupported and the caller will report the exclusive syntax.
        parse_scss_regular_interpolation(p).ok();
        true
    } else {
        false
    }
}

/// Tracks nested delimiters inside an unknown at-rule prelude.
///
/// Example: in `@unknown fn({ width: 300px }) #{$query} {}`, the inner
/// `{ width: 300px }` stays in the prelude and the top-level `{` starts the
/// at-rule block.
#[derive(Default)]
struct UnknownAtRuleComponentBalance {
    paren_depth: usize,
    bracket_depth: usize,
    curly_depth: usize,
}

impl UnknownAtRuleComponentBalance {
    #[inline]
    fn is_at_end(&self, p: &mut CssParser) -> bool {
        if p.at(EOF) {
            return true;
        }

        if !self.is_at_top_level() {
            return false;
        }

        // `@unknown #{x} {}`: the top-level block starts after the prelude.
        p.at(T!['{'])
            // `@unknown #{x};`: a semicolon ends a value at-rule.
            || p.at(T![;])
            // `@mixin x { @unknown #{x} }`: the parent block closes a
            // semicolonless value at-rule.
            || p.at(T!['}'])
            // `@unknown #{x}\n@media {}`: a new at-rule starts recovery for a
            // missing semicolon.
            || p.at(T![@]) && p.has_preceding_line_break()
    }

    #[inline]
    fn is_at_top_level(&self) -> bool {
        self.paren_depth == 0 && self.bracket_depth == 0 && self.curly_depth == 0
    }

    #[inline]
    fn bump(&mut self, p: &mut CssParser) {
        match p.cur() {
            T!['('] => self.paren_depth += 1,
            T![')'] => self.paren_depth = self.paren_depth.saturating_sub(1),
            T!['['] => self.bracket_depth += 1,
            T![']'] => self.bracket_depth = self.bracket_depth.saturating_sub(1),
            T!['{'] => self.curly_depth += 1,
            T!['}'] => self.curly_depth = self.curly_depth.saturating_sub(1),
            _ => {}
        }

        p.bump_any();
    }
}
