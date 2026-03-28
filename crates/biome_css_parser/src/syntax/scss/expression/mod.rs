mod interpolation;
mod list;
mod map;
mod precedence;
mod primary;

use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::{Parser, TokenSet, token_set};

use super::is_at_scss_variable_modifier_start;

pub(crate) use interpolation::{
    ScssInterpolationMode, is_at_scss_interpolation, is_nth_at_scss_interpolation,
    parse_scss_interpolation_with_mode,
};
pub(crate) use list::{
    complete_empty_scss_expression, parse_required_scss_value_until, parse_scss_expression,
    parse_scss_expression_in_args_until, parse_scss_expression_in_variable_value_until,
    parse_scss_expression_until, parse_scss_optional_value_until,
};
pub(crate) use precedence::SCSS_UNARY_OPERATOR_TOKEN_SET;

/// Carries the caller-specific rules for parsing ambiguous SCSS expressions.
///
/// SCSS values reuse the same core parser in declaration, argument, and map
/// contexts, but each context differs on whether empty values, keyword
/// arguments, or `...` are legal.
#[derive(Clone, Copy)]
pub(super) struct ScssExpressionOptions {
    end_ts: TokenSet<CssSyntaxKind>,
    allows_empty_value: bool,
    allows_keyword_arguments: bool,
    allows_ellipsis: bool,
    stops_before_variable_modifiers: bool,
}

impl ScssExpressionOptions {
    pub(super) fn value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
        }
    }

    pub(super) fn optional_value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_empty_value: true,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
        }
    }

    pub(super) fn args(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_empty_value: false,
            allows_keyword_arguments: true,
            allows_ellipsis: true,
            stops_before_variable_modifiers: false,
        }
    }

    pub(super) fn variable_value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            end_ts,
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: true,
        }
    }

    pub(super) fn comma_separates_list(self) -> bool {
        !self.end_ts.contains(T![,])
    }

    pub(super) fn recovery_end_ts(self) -> TokenSet<CssSyntaxKind> {
        if self.comma_separates_list() {
            self.end_ts.union(token_set![T![,]])
        } else {
            self.end_ts
        }
    }
}

#[inline]
pub(super) fn is_at_scss_expression_end(
    p: &mut crate::parser::CssParser,
    options: ScssExpressionOptions,
) -> bool {
    p.at_ts(options.end_ts)
        || p.at(T![')'])
        || (options.stops_before_variable_modifiers && is_at_scss_variable_modifier_start(p))
}
