use crate::parser::CssParser;
use crate::syntax::scss::{
    add_scss_variable_member_function_name_diagnostic, is_at_scss_interpolated_function_or_value,
    is_at_scss_interpolated_string, is_at_scss_module_member_access,
    is_at_scss_parent_selector_value, is_at_scss_variable, parse_scss_function_call_from_name,
    parse_scss_interpolated_function_or_value, parse_scss_interpolated_string,
    parse_scss_module_member_access, parse_scss_parent_selector_value, parse_scss_variable,
};
use crate::syntax::{FunctionCallContext, ValueParsingContext};
use biome_css_syntax::T;
use biome_parser::Parser;
use biome_parser::prelude::ParsedSyntax;
use biome_parser::prelude::ParsedSyntax::{Absent, Present};

#[inline]
pub(crate) fn is_at_any_scss_value(p: &mut CssParser) -> bool {
    is_at_scss_variable(p)
        || is_at_scss_module_member_access(p)
        || is_at_scss_interpolated_function_or_value(p)
        || is_at_scss_parent_selector_value(p)
        || is_at_scss_interpolated_string(p)
}

/// Parses one SCSS-only value form.
///
/// This covers the SCSS-specific value families that do not belong to the
/// shared CSS value parser, including variables, module-member access values,
/// parent selectors, interpolated strings, and interpolation-led
/// function-or-value forms.
///
/// Examples:
/// ```scss
/// $value
/// module.$value
/// &-suffix
/// "#{$name}"
/// foo#{1 + 1}(arg)
/// ```
///
/// Docs:
/// - https://sass-lang.com/documentation/syntax/structure
/// - https://sass-lang.com/documentation/modules
/// - https://sass-lang.com/documentation/interpolation
#[inline]
pub(crate) fn parse_any_scss_value_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_scss_value(p) {
        return Absent;
    }

    if is_at_scss_variable(p) {
        parse_scss_variable(p)
    } else if is_at_scss_module_member_access(p) {
        let has_dollar_member = p.nth_at(2, T![$]);
        // Parse the qualified head once, then wrap it as a function only when
        // the already parsed head is immediately followed by `(`.
        let name = match parse_scss_module_member_access(p) {
            Present(name) => name,
            Absent => return Absent,
        };

        if is_at_scss_module_function_call_paren(p, context) {
            // `module.$name(` still recovers as a call, but `$name` is not a
            // valid module function member.
            let name =
                add_scss_variable_member_function_name_diagnostic(p, has_dollar_member, name);
            parse_scss_function_call_from_name(p, name)
        } else {
            Present(name)
        }
    } else if is_at_scss_interpolated_function_or_value(p) {
        parse_scss_interpolated_function_or_value(p)
    } else if is_at_scss_parent_selector_value(p) {
        parse_scss_parent_selector_value(p)
    } else {
        parse_scss_interpolated_string(p)
    }
}

#[inline]
fn is_at_scss_module_function_call_paren(p: &mut CssParser, context: ValueParsingContext) -> bool {
    // `module.fn(...)` is a Sass call. In loose declaration recovery,
    // `module.fn (...)` can still recover as a call.
    p.at(T!['('])
        && match context.function_call_context() {
            FunctionCallContext::LooseRecovery => true,
            FunctionCallContext::SourceTight => {
                !p.has_preceding_whitespace() && !p.has_preceding_line_break()
            }
        }
}
