mod interpolation;
mod list;
mod map;
mod operand;
mod precedence;
mod regular_expression_operand;

use biome_css_syntax::{CssSyntaxKind, T};
use biome_parser::{Parser, TokenSet, token_set};

use crate::parser::CssParser;
use crate::syntax::FunctionCallContext;

use super::is_at_scss_variable_modifier;

pub(crate) use interpolation::{
    is_at_scss_interpolation, is_nth_at_scss_interpolation, parse_scss_interpolation_prefix,
    parse_scss_interpolation_with_context, parse_scss_regular_interpolation,
    parse_scss_selector_interpolation,
};
pub(crate) use list::{
    complete_empty_scss_expression, complete_scss_expression_from_item,
    parse_required_scss_value_until, parse_scss_expression, parse_scss_expression_from_head,
    parse_scss_expression_in_args_until, parse_scss_expression_in_variable_value_until,
    parse_scss_expression_until, parse_scss_inner_expression_in_string_until,
    parse_scss_optional_value_until, parse_scss_optional_value_until_with_boundary,
};
pub(crate) use precedence::{
    SCSS_UNARY_OPERATOR_TOKEN_SET, is_at_scss_binary_operator, is_at_scss_unary_operator,
};

/// Carries the caller-specific rules for parsing ambiguous SCSS expressions.
///
/// SCSS values reuse the same core parser in declaration, argument, and map
/// contexts, but each context differs on whether empty values, keyword
/// arguments, or `...` are legal.
#[derive(Clone, Copy)]
pub(super) struct ScssExpressionOptions {
    function_call_context: FunctionCallContext,
    end_ts: TokenSet<CssSyntaxKind>,
    boundary: Option<fn(&mut CssParser) -> bool>,
    allows_empty_value: bool,
    allows_keyword_arguments: bool,
    allows_ellipsis: bool,
    stops_before_variable_modifiers: bool,
    stops_at_string_quote: bool,
}

impl ScssExpressionOptions {
    pub(super) fn value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            function_call_context: FunctionCallContext::SourceTight,
            end_ts,
            boundary: None,
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
            stops_at_string_quote: false,
        }
    }

    pub(super) fn optional_value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            function_call_context: FunctionCallContext::SourceTight,
            end_ts,
            boundary: None,
            allows_empty_value: true,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
            stops_at_string_quote: false,
        }
    }

    pub(super) fn args(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            function_call_context: FunctionCallContext::SourceTight,
            end_ts,
            boundary: None,
            allows_empty_value: false,
            allows_keyword_arguments: true,
            allows_ellipsis: true,
            stops_before_variable_modifiers: false,
            stops_at_string_quote: false,
        }
    }

    pub(super) fn variable_value(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            function_call_context: FunctionCallContext::SourceTight,
            end_ts,
            boundary: None,
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: true,
            stops_at_string_quote: false,
        }
    }

    pub(super) fn value_in_string(end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            function_call_context: FunctionCallContext::SourceTight,
            stops_at_string_quote: true,
            ..Self::value(end_ts)
        }
    }

    /// Adds a caller-owned boundary for the outer expression.
    pub(super) fn with_boundary(mut self, boundary: fn(&mut CssParser) -> bool) -> Self {
        self.boundary = Some(boundary);
        self
    }

    /// Changes the delimiter set for a nested expression and clears the outer boundary.
    ///
    /// A parenthesized list item stops at `)`, but keeps the parent expression
    /// context so `@include foo(a (...))` remains strict inside nested values.
    /// Caller-owned semicolons and closing braces remain recovery boundaries
    /// when a nested closing parenthesis is missing.
    pub(super) fn with_end_ts(self, mut end_ts: TokenSet<CssSyntaxKind>) -> Self {
        for boundary in [T![;], T!['}']] {
            if self.end_ts.contains(boundary) {
                end_ts = end_ts.union(TokenSet::singleton(boundary));
            }
        }

        Self {
            end_ts,
            boundary: None,
            ..self
        }
    }

    /// Changes the delimiter set and requires expression content.
    ///
    /// Map pairs are nested required expressions: `(foo:)` must diagnose even
    /// when the outer declaration value may be empty.
    pub(super) fn with_required_end_ts(self, end_ts: TokenSet<CssSyntaxKind>) -> Self {
        Self {
            allows_empty_value: false,
            allows_keyword_arguments: false,
            allows_ellipsis: false,
            stops_before_variable_modifiers: false,
            ..self.with_end_ts(end_ts)
        }
    }

    /// Returns how this expression context treats `ident (` function heads.
    pub(super) const fn function_call_context(self) -> FunctionCallContext {
        self.function_call_context
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
        || options.boundary.is_some_and(|boundary| boundary(p))
        || (options.stops_before_variable_modifiers && is_at_scss_variable_modifier(p))
}
