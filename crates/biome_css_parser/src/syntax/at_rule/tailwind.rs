use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::block::{parse_declaration_or_rule_list_block, parse_rule_block};
use crate::syntax::parse_error::{expected_identifier, expected_selector, expected_string};
use crate::syntax::selector::parse_selector;
use crate::syntax::{is_at_identifier, parse_identifier, parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::{self, *};
use biome_css_syntax::T;
use biome_parser::parse_lists::ParseNodeList;
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

// @theme { --color-primary: #3b82f6; }
pub(crate) fn parse_theme_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![theme]) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![theme]);
    parse_regular_identifier(p).ok();
    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, TW_THEME_AT_RULE))
}

// @utility tab-4 { tab-size: 4; }
// @utility tab-* { tab-size: --value([integer]); }
pub(crate) fn parse_utility_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![utility]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![utility]);

    // Parse utility name - can be simple or functional
    if !is_at_identifier(p) {
        p.error(expected_identifier(p, p.cur_range()));
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    parse_utility_name(p).ok();

    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, TW_UTILITY_AT_RULE))
}

fn parse_utility_name(p: &mut CssParser) -> ParsedSyntax {
    // Check if this is a functional utility (ends with -*)
    if p.at(T![ident]) && p.nth_at(1, T![-]) && p.nth_at(2, T![*]) {
        // Functional utility: tab-*
        let m = p.start();

        parse_regular_identifier(p).ok();
        p.expect(T![-]);
        p.expect(T![*]);

        Present(m.complete(p, TW_FUNCTIONAL_UTILITY_NAME))
    } else {
        // Simple utility: center-flex
        parse_regular_identifier(p)
    }
}

// @variant dark { background: black; }
pub(crate) fn parse_variant_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![variant]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![variant]);

    if !is_at_identifier(p) {
        p.error(expected_identifier(p, p.cur_range()));
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    parse_regular_identifier(p).ok();

    parse_declaration_or_rule_list_block(p);

    Present(m.complete(p, TW_VARIANT_AT_RULE))
}

// @custom-variant theme-midnight (&:where([data-theme="midnight"] *));
pub(crate) fn parse_custom_variant_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![custom_variant]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![custom_variant]);

    if !is_at_identifier(p) {
        p.error(expected_identifier(p, p.cur_range()));
        return Present(m.complete(p, CSS_BOGUS_AT_RULE));
    }

    parse_regular_identifier(p).ok();

    if p.at(T!['(']) {
        // shorthand syntax
        // @custom-variant theme-midnight (&:where([data-theme="midnight"] *));
        parse_custom_variant_shorthand(p).ok();
    } else {
        // longhand syntax
        // @custom-variant theme-midnight {
        //   &:where([data-theme="midnight"] *) {
        //     @slot;
        //   }
        // }
        parse_rule_block(p);
    }

    Present(m.complete(p, TW_CUSTOM_VARIANT_AT_RULE))
}

fn parse_custom_variant_shorthand(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();

    p.bump(T!['(']);
    parse_selector(p).or_add_diagnostic(p, expected_selector);
    p.expect(T![')']);
    p.expect(T![;]);

    Present(m.complete(p, TW_CUSTOM_VARIANT_SHORTHAND))
}

// @apply text-lg font-bold;
pub(crate) fn parse_apply_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![apply]) {
        return Absent;
    }

    let m = p.start();
    p.bump_with_context(T![apply], CssLexContext::TailwindUtility);
    ApplyClassList.parse_list(p);
    p.expect(T![;]);

    Present(m.complete(p, TW_APPLY_AT_RULE))
}

struct ApplyClassList;

impl ParseNodeList for ApplyClassList {
    type Kind = CssSyntaxKind;
    type Parser<'source> = CssParser<'source>;
    const LIST_KIND: Self::Kind = TW_APPLY_CLASS_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_identifier(p, CssLexContext::TailwindUtility)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(T![;])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> biome_parser::parse_recovery::RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(CSS_BOGUS_CUSTOM_IDENTIFIER, token_set![T![;], EOF])
                .enable_recovery_on_line_break(),
            expected_identifier,
        )
    }
}

// @config "../../tailwind.config.js";
pub(crate) fn parse_config_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![config]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![config]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    p.expect(T![;]);

    Present(m.complete(p, TW_CONFIG_AT_RULE))
}

// @plugin "@tailwindcss/typography";
pub(crate) fn parse_plugin_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![plugin]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![plugin]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    p.expect(T![;]);

    Present(m.complete(p, TW_PLUGIN_AT_RULE))
}

// @source "../node_modules/@my-company/ui-lib";
pub(crate) fn parse_source_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![source]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![source]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    p.expect(T![;]);

    Present(m.complete(p, TW_SOURCE_AT_RULE))
}

// @reference "../../app.css";
pub(crate) fn parse_reference_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !p.at(T![reference]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![reference]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    p.expect(T![;]);

    Present(m.complete(p, TW_REFERENCE_AT_RULE))
}
