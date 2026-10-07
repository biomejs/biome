use crate::parser::CssParser;
use crate::state::IfFunctionKind;
use crate::syntax::css_modules::v_bind_not_allowed;
use crate::syntax::parse_error::{expected_identifier, scss_only_syntax_error};
use crate::syntax::scss::{
    is_at_scss_module_member_access, is_nth_at_scss_module_member_access, parse_scss_function_name,
};
use crate::syntax::value::attr::{is_at_attr_function, parse_attr_function};
use crate::syntax::value::r#if::{is_at_if_function, parse_if_function};
use crate::syntax::value::url::{is_at_url_function, parse_url_function_with_context};
use crate::syntax::{
    CssSyntaxFeatures, FunctionCallContext, ValueParsingContext, ValueParsingMode,
    is_nth_at_identifier, parse_regular_identifier, try_parse,
};
use biome_css_syntax::CssSyntaxKind::*;
use biome_css_syntax::{T, TextSize, decode_css_identifier};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::{Parser, SyntaxFeature};

use super::parameter::ParameterList;

/// Checks if the current position is at any function recognized by CSS-only value parsing.
#[inline]
pub(crate) fn is_at_any_css_function(p: &mut CssParser) -> bool {
    is_at_any_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
pub(crate) fn is_at_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> bool {
    (matches!(
        context.function_call_context(),
        FunctionCallContext::LooseRecovery
    ) || is_nth_at_adjacent_l_paren(p, 1))
        && (is_at_url_function(p)
            || is_at_if_function(p)
            || is_at_attr_function(p)
            || is_at_vue_v_bind_function(p))
        || is_at_function_with_context(p, context)
}

#[inline]
pub(crate) fn parse_any_function_with_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> ParsedSyntax {
    if !is_at_any_function_with_context(p, context) {
        return Absent;
    }

    if is_at_url_function(p) {
        parse_url_function_with_context(p, context)
    } else if is_at_if_function(p) {
        if context.is_full_scss_parsing_allowed() {
            parse_scss_if_function(p, context)
        } else {
            parse_if_function(p)
        }
    } else if is_at_attr_function(p) {
        parse_attr_function(p, context)
    } else if is_at_vue_v_bind_function(p) {
        CssSyntaxFeatures::CssModulesWithVue.parse_exclusive_syntax(
            p,
            |p| parse_function_with_context(p, context),
            |p, marker| v_bind_not_allowed(p, marker.range(p)),
        )
    } else {
        parse_function_with_context(p, context)
    }
}

#[inline]
pub(crate) fn is_at_css_if_function_in_context(
    p: &mut CssParser,
    context: ValueParsingContext,
) -> bool {
    if !is_at_if_function(p) {
        return false;
    }

    if !context.is_full_scss_parsing_allowed() {
        return true;
    }

    // CSS if() branches can only start with supported condition syntax:
    // `style(...)`, `media(...)`, `supports(...)`, `sass(...)`, `not`, `else`, or a
    // parenthesized boolean expression. Anything else remains a regular
    // function call in SCSS-aware mode, such as Sass `if($cond, a, b)`.
    p.nth_at(2, T![style])
        || p.nth_at(2, T![media])
        || p.nth_at(2, T![supports])
        || p.nth_at(2, T![sass])
        || p.nth_at(2, T![not])
        || p.nth_at(2, T![else])
        || p.nth_at(2, T!['('])
}

/// Parses an SCSS `if()` call, preferring complete modern branches over legacy
/// Sass arguments. If neither grammar succeeds, the grammar whose first parser
/// error occurs later owns recovery; ties prefer modern syntax.
///
/// Nested calls share their selected grammar until the outer call completes.
/// For `if(sass(if(sass(1), 2, 3)), 4, 5)`, an outer rewind revisits the inner
/// call without repeating its failed modern attempt.
fn parse_scss_if_function(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    let outermost = p.state().if_function_kinds.is_none();
    let start = p.cur_range().start();
    let cached = p
        .state_mut()
        .if_function_kinds
        .get_or_insert_with(Default::default)
        .get(&start)
        .copied();

    let function = match cached {
        Some(IfFunctionKind::Modern) => parse_if_function(p),
        Some(IfFunctionKind::Legacy) => parse_function_with_context(p, context),
        None => {
            let (function, kind) = match try_parse_function(p, parse_if_function) {
                Ok(function) => (function, IfFunctionKind::Modern),
                Err(modern_error) => {
                    match try_parse_function(p, |p| parse_function_with_context(p, context)) {
                        Ok(function) => (function, IfFunctionKind::Legacy),
                        // Recover with the grammar that accepted the longest prefix.
                        Err(legacy_error) if legacy_error > modern_error => (
                            parse_function_with_context(p, context),
                            IfFunctionKind::Legacy,
                        ),
                        Err(_) => (parse_if_function(p), IfFunctionKind::Modern),
                    }
                }
            };
            if let Some(kinds) = &mut p.state_mut().if_function_kinds {
                kinds.insert(start, kind);
            }
            function
        }
    };
    if outermost {
        p.state_mut().if_function_kinds = None;
    }
    function
}

/// Attempts a complete function with error recovery disabled.
///
/// Success requires a present node, a consumed closing parenthesis, and no new
/// parser diagnostics. Failure rewinds tokens, trivia, tree events, and
/// diagnostics, returning the first parser error's offset or the stopping
/// position if the first diagnostic has no span. Callers compare these offsets to
/// select recovery when both grammars fail.
///
/// Restores the surrounding speculative mode and preserves its incoming
/// declaration-recovery signal. Only an accepted attempt inside an already
/// speculative context can add to that signal.
fn try_parse_function(
    p: &mut CssParser,
    parse: impl FnOnce(&mut CssParser) -> ParsedSyntax,
) -> Result<ParsedSyntax, TextSize> {
    let encountered = p.state().encountered_if_function;
    let speculative = p.is_speculative_parsing();
    let result = try_parse(p, |p| {
        let diagnostics = p.context().diagnostics().len();
        let function = parse(p);
        if function.is_present()
            && p.last() == Some(T![')'])
            && p.context().diagnostics().len() == diagnostics
        {
            Ok(function)
        } else {
            Err(p
                .context()
                .diagnostics()
                .get(diagnostics)
                .and_then(|diagnostic| diagnostic.span())
                .map_or(p.cur_range().start(), |range| range.start()))
        }
    });
    p.state_mut().encountered_if_function =
        encountered || (result.is_ok() && speculative && p.state().encountered_if_function);
    result
}

#[inline]
fn is_at_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> bool {
    is_nth_at_function_with_context(p, 0, context) && !is_at_url_function(p)
}

#[inline]
pub(crate) fn is_nth_at_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(
        p,
        n,
        ValueParsingContext::new(p, ValueParsingMode::ScssAware),
    )
}

/// Checks if the `n`th token starts a simple function head in CSS-only mode.
#[inline]
pub(crate) fn is_nth_at_css_function(p: &mut CssParser, n: usize) -> bool {
    is_nth_at_function_with_context(p, n, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn is_nth_at_function_with_context(
    p: &mut CssParser,
    n: usize,
    context: ValueParsingContext,
) -> bool {
    let is_function_paren = match context.function_call_context() {
        FunctionCallContext::LooseRecovery => p.nth_at(n + 1, T!['(']),
        FunctionCallContext::SourceTight => is_nth_at_adjacent_l_paren(p, n + 1),
    };

    is_nth_at_identifier(p, n) && is_function_paren
        || (context.is_scss_qualified_function_recovery_allowed()
            // Sass module calls are always source-tight: `math.pow(...)`.
            && is_nth_at_scss_module_member_access(p, n)
            && is_nth_at_adjacent_l_paren(p, n + 3))
}

/// Returns whether the `n`th non-trivia token is `(` directly attached to
/// the preceding token, with no intervening whitespace or comments.
///
/// `n == 0` checks the current token against the last consumed token.
/// Whitespace inside an identifier escape remains part of that token.
///
/// Sass parses `fn(...)` as a call, but `fn (...)` and `fn/**/(...)` as
/// an identifier followed by a parenthesized value.
#[inline]
pub(crate) fn is_nth_at_adjacent_l_paren(p: &mut CssParser, n: usize) -> bool {
    p.nth_at(n, T!['('])
        && if n == 0 {
            p.last_end() == Some(p.cur_range().start())
        } else {
            p.source_mut().is_nth_source_tight(n)
        }
}

#[inline]
fn is_at_vue_v_bind_function(p: &mut CssParser) -> bool {
    if !is_nth_at_css_function(p, 0) {
        return false;
    }

    p.cur_text() == "v-bind"
}

#[inline]
pub(crate) fn parse_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::ScssAware))
}

/// Parses a simple function using CSS-only branches.
#[inline]
pub(crate) fn parse_css_function(p: &mut CssParser) -> ParsedSyntax {
    parse_function_with_context(p, ValueParsingContext::new(p, ValueParsingMode::CssOnly))
}

#[inline]
fn parse_function_with_context(p: &mut CssParser, context: ValueParsingContext) -> ParsedSyntax {
    if !is_at_function_with_context(p, context) {
        return Absent;
    }

    // CSS keywords ignore case, but the legacy Sass builtin requires lowercase `if`.
    let is_legacy_if = context.is_full_scss_parsing_allowed()
        && is_at_if_function(p)
        && decode_css_identifier(p.cur_text()) == "if";
    let m = p.start();

    if is_legacy_if {
        p.bump(T![if]);
    } else if context.is_scss_qualified_function_recovery_allowed()
        && is_at_scss_module_member_access(p)
    {
        CssSyntaxFeatures::Scss
            .parse_exclusive_syntax(p, parse_scss_function_name, |p, marker| {
                scss_only_syntax_error(p, "SCSS qualified function names", marker.range(p))
            })
            .or_add_diagnostic(p, expected_identifier);
    } else {
        parse_regular_identifier(p).or_add_diagnostic(p, expected_identifier);
    }
    p.bump(T!['(']);
    ParameterList::new(context).parse_list(p);
    p.expect(T![')']);

    Present(m.complete(
        p,
        if is_legacy_if {
            SCSS_LEGACY_IF_FUNCTION
        } else {
            CSS_FUNCTION
        },
    ))
}
