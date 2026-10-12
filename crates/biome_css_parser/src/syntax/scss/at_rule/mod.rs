mod at_root_at_rule;
mod container;
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
mod media;
mod mixin_at_rule;
mod module_clauses;
mod parameter;
mod query_feature;
mod return_at_rule;
mod supports;
mod use_at_rule;
mod warn;
mod while_at_rule;

use crate::parser::CssParser;
use crate::syntax::scss::{expected_scss_expression, parse_scss_expression_until};
use biome_css_syntax::CssSyntaxKind::EOF;
use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;
use biome_parser::{CompletedMarker, TokenSet, token_set};

#[derive(Clone, Copy)]
pub(crate) struct ScssBlockParser {
    parse_block: for<'source> fn(&mut CssParser<'source>) -> CompletedMarker,
    additional_header_end_ts: TokenSet<CssSyntaxKind>,
}

impl ScssBlockParser {
    pub(crate) const fn new(
        parse_block: for<'source> fn(&mut CssParser<'source>) -> CompletedMarker,
    ) -> Self {
        Self {
            parse_block,
            additional_header_end_ts: TokenSet::EMPTY,
        }
    }

    pub(crate) const fn with_additional_header_end_ts(
        mut self,
        additional_header_end_ts: TokenSet<CssSyntaxKind>,
    ) -> Self {
        self.additional_header_end_ts = additional_header_end_ts;
        self
    }

    pub(crate) const fn header_end_ts(
        self,
        end_ts: TokenSet<CssSyntaxKind>,
    ) -> TokenSet<CssSyntaxKind> {
        end_ts.union(self.additional_header_end_ts)
    }

    pub(crate) fn parse(self, p: &mut CssParser) -> CompletedMarker {
        (self.parse_block)(p)
    }
}

pub(crate) use at_root_at_rule::parse_scss_at_root_at_rule;
pub(crate) use container::{
    is_at_scss_container_interpolated_query, is_at_scss_container_name_or_query,
    parse_scss_container_interpolated_query, parse_scss_container_name_or_query,
};
pub(crate) use content_at_rule::parse_scss_content_at_rule;
pub(crate) use debug::parse_scss_debug_at_rule;
pub(crate) use each_at_rule::{parse_scss_each_at_rule, parse_scss_each_at_rule_with_block};
pub(crate) use else_clause::parse_bogus_scss_else_at_rule;
pub(crate) use error::parse_scss_error_at_rule;
pub(crate) use extend_at_rule::parse_scss_extend_at_rule;
pub(crate) use for_at_rule::{parse_scss_for_at_rule, parse_scss_for_at_rule_with_block};
pub(crate) use forward_at_rule::parse_scss_forward_at_rule;
pub(crate) use function_at_rule::parse_scss_function_at_rule;
pub(crate) use if_at_rule::{parse_scss_if_at_rule, parse_scss_if_at_rule_with_block};
pub(crate) use import_at_rule::parse_scss_import_at_rule;
pub(crate) use include_at_rule::parse_scss_include_at_rule;
pub(crate) use keyframes::{
    is_at_scss_keyframes_name, is_at_scss_keyframes_selector, parse_scss_keyframes_name,
    parse_scss_keyframes_selector,
};
pub(crate) use media::{
    is_at_scss_interpolated_media_in_parens, is_at_scss_media_condition, is_at_scss_media_query,
    parse_scss_interpolated_media_in_parens, parse_scss_media_condition,
    parse_scss_media_condition_from_query, parse_scss_media_query,
};
pub(crate) use mixin_at_rule::parse_scss_mixin_at_rule;
pub(crate) use query_feature::parse_scss_interpolated_query_feature;
pub(crate) use return_at_rule::parse_scss_return_at_rule;
pub(crate) use supports::{
    is_at_scss_supports_interpolated_condition, parse_scss_supports_interpolated_condition,
};
pub(crate) use use_at_rule::parse_scss_use_at_rule;
pub(crate) use warn::parse_scss_warn_at_rule;
pub(crate) use while_at_rule::{parse_scss_while_at_rule, parse_scss_while_at_rule_with_block};

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
    expect_scss_semicolon_at_rule(p);

    Present(m.complete(p, kind))
}

/// Expects a semicolon after a blockless SCSS at-rule.
///
/// Examples:
/// ```scss
/// @use "theme"
/// @mixin x { @include button }
/// ```
#[inline]
pub(crate) fn expect_scss_semicolon_at_rule(p: &mut CssParser) {
    // Dart Sass allows omitting the final semicolon only at the end of the
    // current block or file, not before the next statement.
    if p.eat(T![;]) || p.at(T!['}']) || p.at(EOF) {
        return;
    }

    p.expect(T![;]);
}
