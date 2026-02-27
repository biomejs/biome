use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::SyntaxFeature;
use biome_parser::token_set;

use crate::parser::CssParser;
use crate::syntax::CssSyntaxFeatures;

/// Skips possible Tailwind CSS specific syntax in the `@import` rule that we don't know how to handle yet.
///
/// See: https://github.com/biomejs/biome/issues/7920
pub(crate) fn skip_possible_tailwind_syntax(p: &mut CssParser) {
    if CssSyntaxFeatures::Tailwind.is_supported(p)
        && p.at_ts(token_set![IDENT, T![source], T![theme], T![important]])
    {
        if p.cur_text() == "prefix" || p.cur_text() == "source" || p.cur_text() == "theme" {
            p.parse_as_skipped_trivia_tokens(skip_tailwind_function_clause)
        } else if p.cur_text() == "important" {
            p.parse_as_skipped_trivia_tokens(|p| p.bump_any());
        }
    }
}

fn skip_tailwind_function_clause(p: &mut CssParser) {
    while !(p.at(EOF) || p.at(T![')'])) {
        p.bump_any();
    }
    if p.at(T![')']) {
        p.bump_any(); // consume ')'
    }
}
