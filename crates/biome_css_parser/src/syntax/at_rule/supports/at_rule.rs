use crate::lexer::CssLexContext;
use crate::parser::CssParser;
use crate::syntax::parse_error::expected_identifier;
use crate::syntax::value::function::is_nth_at_adjacent_l_paren;
use crate::syntax::{is_nth_at_identifier, parse_custom_identifier_with_keywords};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{T, decode_css_identifier};
use biome_parser::parse_recovery::ParseRecoveryTokenSet;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::token_set;

#[inline]
pub(super) fn is_at_supports_at_rule_function(p: &mut CssParser) -> bool {
    is_nth_at_identifier(p, 0)
        && decode_css_identifier(p.cur_text()).eq_ignore_ascii_case("at-rule")
        && is_nth_at_adjacent_l_paren(p, 1)
}

/// Parses an [at-rule feature query](https://drafts.csswg.org/css-conditional-5/#typedef-supports-at-rule-fn)
/// or its balanced general-enclosed fallback without evaluating support.
///
/// ```css
/// @supports at-rule(@function) { .card { padding: --double(8px); } }
/// @supports at-rule(@ property) { .card { padding: 12px; } }
/// ```
pub(super) fn parse_supports_at_rule_function(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_supports_at_rule_function(p) {
        return Absent;
    }

    // CSS at-keywords require physical adjacency, including across comments.
    let is_feature = p.nth_at(2, T![@])
        && p.source_mut().is_nth_source_tight(3)
        && ((is_nth_at_identifier(p, 3) && p.nth_at(4, T![')']))
            || (p.nth_at(3, T![-])
                && p.nth_at(4, T![-])
                && p.source_mut().is_nth_source_tight(4)
                && p.nth_at(5, T![')'])));
    let m = p.start();
    parse_custom_identifier_with_keywords(p, CssLexContext::Regular, true)
        .or_add_diagnostic(p, expected_identifier);
    let fallback_end = if is_feature {
        None
    } else {
        p.source_mut().supports_at_rule_fallback_end()
    };
    let lex_context = match fallback_end {
        Some(end) if usize::from(p.cur_range().end()) < end => {
            CssLexContext::SupportsAtRuleFallback { end }
        }
        _ => CssLexContext::Regular,
    };
    p.expect_with_context(T!['('], lex_context);

    let kind = if is_feature {
        p.expect_with_context(T![@], CssLexContext::SupportsAtRuleName);
        parse_custom_identifier_with_keywords(p, CssLexContext::Regular, true)
            .or_add_diagnostic(p, expected_identifier);
        CSS_SUPPORTS_FEATURE_AT_RULE
    } else {
        let value = p.start();
        if fallback_end.is_some() {
            p.eat(CSS_SUPPORTS_AT_RULE_FALLBACK_LITERAL);
        } else {
            ParseRecoveryTokenSet::new(
                CSS_BOGUS,
                token_set![T!['{'], T!['}'], T![')'], T![']'], T![;]],
            )
            .enable_recovery_on_line_break()
            .recover(p)
            .ok();
            if p.at(T![')']) {
                p.error(p.err_builder("Expected valid tokens in `at-rule()`.", p.cur_range()));
            }
        }
        value.complete(p, CSS_SUPPORTS_AT_RULE_FALLBACK_VALUE);
        CSS_SUPPORTS_AT_RULE_FALLBACK
    };
    p.expect(T![')']);
    Present(m.complete(p, kind))
}
