use super::module_clauses::{expected_scss_module_member, parse_scss_module_member_list};
use super::use_at_rule::parse_scss_with_clause;
use crate::parser::CssParser;
use crate::syntax::parse_error::{expected_identifier, expected_string};
use crate::syntax::{parse_regular_identifier, parse_string};
use biome_css_syntax::CssSyntaxKind::{
    SCSS_FORWARD_AS_CLAUSE, SCSS_FORWARD_AT_RULE, SCSS_HIDE_CLAUSE, SCSS_SHOW_CLAUSE,
};
use biome_css_syntax::T;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

/// Parses the SCSS `@forward` at-rule.
///
/// # Example
///
/// ```scss
/// @forward "theme" show $color;
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/
#[inline]
pub(crate) fn parse_scss_forward_at_rule(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_forward_at_rule(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![forward]);
    parse_string(p).or_add_diagnostic(p, expected_string);
    // The `as` clause is optional in the grammar.
    parse_scss_forward_as_clause(p).ok();
    // The visibility clause is optional in the grammar.
    parse_scss_forward_visibility_clause(p).ok();
    // The `with` clause is optional in the grammar.
    parse_scss_with_clause(p).ok();
    p.expect(T![;]);

    Present(m.complete(p, SCSS_FORWARD_AT_RULE))
}

#[inline]
fn is_at_scss_forward_at_rule(p: &mut CssParser) -> bool {
    p.at(T![forward])
}

/// Parses the optional SCSS `as <prefix>-*` clause inside `@forward`.
///
/// # Example
///
/// ```scss
/// @forward "theme" as theme-*;
///                  ^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/
#[inline]
fn parse_scss_forward_as_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_forward_as_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![as]);
    let prefix = parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    let invalid_prefix = prefix
        .as_ref()
        .filter(|prefix| !prefix.text(p).ends_with('-'));

    if let Some(prefix) = invalid_prefix {
        p.error(
            p.err_builder(
                "Expected the `@forward` prefix to end with `-` before `*`.",
                prefix.range(p).cover(p.cur_range()),
            )
            .with_hint("Write the clause as `as prefix-*` without spaces."),
        );
    }

    // Consume a stray `-` in invalid forms like `theme - *` or `as -*`.
    p.eat(T![-]);

    p.expect(T![*]);

    Present(m.complete(p, SCSS_FORWARD_AS_CLAUSE))
}

#[inline]
fn is_at_scss_forward_as_clause(p: &mut CssParser) -> bool {
    p.at(T![as])
}

#[inline]
fn parse_scss_forward_visibility_clause(p: &mut CssParser) -> ParsedSyntax {
    if is_at_scss_show_clause(p) {
        parse_scss_show_clause(p)
    } else if is_at_scss_hide_clause(p) {
        parse_scss_hide_clause(p)
    } else {
        Absent
    }
}

/// Parses the optional SCSS `show` clause inside `@forward`.
///
/// # Example
///
/// ```scss
/// @forward "theme" show $color, mixin-name;
///                  ^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/
#[inline]
fn parse_scss_show_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_show_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![show]);
    parse_scss_module_member_list(p).or_add_diagnostic(p, expected_scss_module_member);

    Present(m.complete(p, SCSS_SHOW_CLAUSE))
}

#[inline]
fn is_at_scss_show_clause(p: &mut CssParser) -> bool {
    p.at(T![show])
}

/// Parses the optional SCSS `hide` clause inside `@forward`.
///
/// # Example
///
/// ```scss
/// @forward "theme" hide $color, mixin-name;
///                  ^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
///
/// Docs: https://sass-lang.com/documentation/at-rules/forward/
#[inline]
fn parse_scss_hide_clause(p: &mut CssParser) -> ParsedSyntax {
    if !is_at_scss_hide_clause(p) {
        return Absent;
    }

    let m = p.start();

    p.bump(T![hide]);
    parse_scss_module_member_list(p).or_add_diagnostic(p, expected_scss_module_member);

    Present(m.complete(p, SCSS_HIDE_CLAUSE))
}

#[inline]
fn is_at_scss_hide_clause(p: &mut CssParser) -> bool {
    p.at(T![hide])
}
