mod at_root_at_rule;
mod content_at_rule;
mod debug;
mod each_at_rule;
mod else_clause;
mod error;
mod extend_at_rule;
mod for_at_rule;
mod forward_at_rule;
mod function_at_rule;
mod if_at_rule;
mod import_at_rule;
mod include_at_rule;
mod keyframes;
mod mixin_at_rule;
mod module_clauses;
mod parameter;
mod query_feature;
mod return_at_rule;
mod use_at_rule;
mod warn;
mod while_at_rule;

use crate::parser::CssParser;
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{TokenSet, token_set};

pub(crate) use at_root_at_rule::parse_scss_at_root_at_rule;
pub(crate) use content_at_rule::parse_scss_content_at_rule;
pub(crate) use debug::parse_scss_debug_at_rule;
pub(crate) use each_at_rule::parse_scss_each_at_rule;
pub(crate) use else_clause::parse_bogus_scss_else_at_rule;
pub(crate) use error::parse_scss_error_at_rule;
pub(crate) use extend_at_rule::parse_scss_extend_at_rule;
pub(crate) use for_at_rule::parse_scss_for_at_rule;
pub(crate) use forward_at_rule::parse_scss_forward_at_rule;
pub(crate) use function_at_rule::parse_scss_function_at_rule;
pub(crate) use if_at_rule::parse_scss_if_at_rule;
pub(crate) use import_at_rule::parse_scss_import_at_rule;
pub(crate) use include_at_rule::parse_scss_include_at_rule;
pub(crate) use keyframes::{is_at_scss_keyframes_selector, parse_scss_keyframes_selector};
pub(crate) use mixin_at_rule::parse_scss_mixin_at_rule;
pub(crate) use query_feature::parse_scss_interpolated_query_feature;
pub(crate) use return_at_rule::parse_scss_return_at_rule;
pub(crate) use use_at_rule::parse_scss_use_at_rule;
pub(crate) use warn::parse_scss_warn_at_rule;
pub(crate) use while_at_rule::parse_scss_while_at_rule;

const SCSS_STATEMENT_AT_RULE_VALUE_END_SET: TokenSet<CssSyntaxKind> = token_set![T![;], T!['}']];

#[inline]
pub(super) fn parse_scss_expression_at_rule(
    p: &mut CssParser,
    keyword: CssSyntaxKind,
    kind: CssSyntaxKind,
) -> ParsedSyntax {
    if !p.at(keyword) {
        return Absent;
    }

    let m = p.start();

    p.bump(keyword);
    parse_scss_expression_until(p, SCSS_STATEMENT_AT_RULE_VALUE_END_SET)
        .or_add_diagnostic(p, expected_scss_expression);
    expect_scss_statement_at_rule_end(p);

    Present(m.complete(p, kind))
}

/// Accepts `;` or a closing block boundary for SCSS statement at-rules.
///
/// Examples:
/// ```scss
/// @mixin x { @content; }
/// @mixin x { @content }
/// ```
#[inline]
pub(super) fn expect_scss_statement_at_rule_end(p: &mut CssParser) {
    if p.eat(T![;]) || p.at(T!['}']) {
        return;
    }

    p.expect(T![;]);
}
