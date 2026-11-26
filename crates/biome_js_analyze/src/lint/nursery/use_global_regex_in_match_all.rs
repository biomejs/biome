use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, JsCallArguments,
    JsCallExpression, JsIdentifierExpression, JsRegexLiteralExpression, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_global_regex_in_match_all::UseGlobalRegexInMatchAllOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require regular expressions passed to `matchAll()` to have the `g` flag.
    ///
    /// `String#matchAll()` finds every match of a regular expression in a string.
    /// It only works with regular expressions that have the `g` (global) flag, such as `/pattern/g`.
    /// When it receives a regular expression without this flag, it throws a `TypeError` instead of returning the matches.
    ///
    /// This rule checks regular expression literals, such as `/pattern/`, and regular expressions created with
    /// `new RegExp()` or `RegExp()` when their flags are written as a string.
    /// It also follows `const` variables to the regular expression they hold.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// text.matchAll(/pattern/);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// text.matchAll(new RegExp("pattern", "i"));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const pattern = /pattern/u;
    /// text.matchAll(pattern);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// text.matchAll(/pattern/g);
    /// text.matchAll(new RegExp("pattern", "gi"));
    /// text.matchAll("pattern");
    /// ```
    ///
    pub UseGlobalRegexInMatchAll {
        version: "2.6.7",
        name: "useGlobalRegexInMatchAll",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Oxc("bad-match-all-arg").same()],
    }
}

impl Rule for UseGlobalRegexInMatchAll {
    type Query = Semantic<JsCallExpression>;
    /// The regular expression without the `g` flag. It is either the argument of `matchAll()`
    /// or the value of the `const` variable passed as that argument.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = UseGlobalRegexInMatchAllOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        if callee.member_name()?.text() != "matchAll" {
            return None;
        }
        let argument = call.arguments().ok()?.args().first()?.ok()?;
        find_non_global_regex(argument.as_any_js_expression()?, ctx.model())
    }

    fn diagnostic(ctx: &RuleContext<Self>, regex: &Self::State) -> Option<RuleDiagnostic> {
        let argument = ctx.query().arguments().ok()?.args().first()?.ok()?;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            argument.range(),
            markup! {
                "The regular expression passed to "<Emphasis>"matchAll()"</Emphasis>" is missing the "<Emphasis>"g"</Emphasis>" flag."
            },
        );
        let diagnostic = if argument
            .as_any_js_expression()
            .and_then(AnyJsExpression::inner_expression)
            .is_some_and(|argument| argument.range() == regex.range())
        {
            diagnostic
        } else {
            diagnostic.detail(
                regex.range(),
                markup! { "The regular expression is created here." },
            )
        };
        Some(
            diagnostic
                .note(markup! {
                    <Emphasis>"matchAll()"</Emphasis>" throws a "<Emphasis>"TypeError"</Emphasis>" when it receives a regular expression without the "<Emphasis>"g"</Emphasis>" (global) flag."
                })
                .note(markup! {
                    "Add the "<Emphasis>"g"</Emphasis>" flag to the regular expression."
                }),
        )
    }
}

/// Returns the regular expression that `expression` evaluates to when it is known to have no `g` flag.
///
/// The expression can be a regular expression literal, a call to the global `RegExp` with or
/// without `new`, or a `const` variable whose value is one of these. Returns `None` when the
/// value or its flags can't be determined, or when the chain of `const` variables is longer
/// than [`MAX_VARIABLE_HOPS`].
fn find_non_global_regex(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<AnyJsExpression> {
    let mut expression = expression.inner_expression()?;
    for _ in 0..=MAX_VARIABLE_HOPS {
        let has_global_flag = match &expression {
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsRegexLiteralExpression(regex),
            ) => has_global_flag(regex)?,
            AnyJsExpression::JsNewExpression(new_expression) => regexp_call_has_global_flag(
                &new_expression.callee().ok()?,
                &new_expression.arguments()?,
                model,
            )?,
            AnyJsExpression::JsCallExpression(call) => {
                regexp_call_has_global_flag(&call.callee().ok()?, &call.arguments().ok()?, model)?
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                expression = const_value(identifier, model)?.inner_expression()?;
                continue;
            }
            _ => return None,
        };
        return (!has_global_flag).then_some(expression);
    }
    None
}

/// Returns whether a call to `callee` with `arguments` creates a regular expression with the
/// `g` flag, or `None` if `callee` isn't the global `RegExp` or the flags can't be determined.
fn regexp_call_has_global_flag(
    callee: &AnyJsExpression,
    arguments: &JsCallArguments,
    model: &SemanticModel,
) -> Option<bool> {
    if !is_global_regexp(callee, model) {
        return None;
    }
    let mut arguments = arguments.args().iter();
    let pattern = arguments
        .next()
        .map(|pattern| pattern.ok()?.as_any_js_expression().cloned());
    if let Some(flags) = arguments.next() {
        // A spread pattern can provide the flags too, as in `new RegExp(...["a", "g"], "i")`.
        pattern.flatten()?;
        let flags = flags.ok()?.as_any_js_expression()?.inner_expression()?.as_static_value()?;
        return Some(flags.as_string_constant()?.contains('g'));
    }
    let Some(pattern) = pattern else {
        // `RegExp()` creates an empty regular expression without flags.
        return Some(false);
    };
    // Without a flags argument, `RegExp` copies the flags of a regular expression pattern,
    // so `new RegExp(/a/g)` has the `g` flag. Other patterns of unknown value could be
    // regular expressions too.
    match pattern?.inner_expression()? {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsRegexLiteralExpression(regex),
        ) => has_global_flag(&regex),
        pattern => pattern.as_static_value().map(|_| false),
    }
}

fn has_global_flag(regex: &JsRegexLiteralExpression) -> Option<bool> {
    let (_, flags) = regex.decompose().ok()?;
    Some(flags.text().contains('g'))
}

/// Returns `true` if `callee` refers to the global `RegExp`, either directly or through
/// `globalThis` or `window`.
fn is_global_regexp(callee: &AnyJsExpression, model: &SemanticModel) -> bool {
    callee
        .clone()
        .omit_parentheses()
        .as_any_global_identifier_expression()
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, name)| {
            name.text() == "RegExp" && model.binding(&reference).is_none()
        })
}

/// Returns the value assigned to the variable that `identifier` reads,
/// if that variable is declared with `const`.
fn const_value(identifier: &JsIdentifierExpression, model: &SemanticModel) -> Option<AnyJsExpression> {
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) =
        model.binding(&identifier.name().ok()?)?.tree().declaration()?
    else {
        return None;
    };
    // A variable from a destructuring pattern, such as `source` in `const { source } = /a/`,
    // holds part of the value, not the value itself.
    if !declarator.declaration()?.is_const()
        || !matches!(declarator.id(), Ok(AnyJsBindingPattern::AnyJsBinding(_)))
    {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

/// The maximum number of `const` variables followed to find a regular expression,
/// as in `const a = /x/; const b = a; text.matchAll(b);`.
const MAX_VARIABLE_HOPS: usize = 8;
