use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, global_identifier};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
use biome_rule_options::no_implied_eval::NoImpliedEvalOptions;

declare_lint_rule! {
    /// Disallow the use of `eval()`-like methods.
    ///
    /// The `eval()` function evaluates the passed string as a _JavaScript_ code.
    /// Calling `setTimeout`, `setInterval`, or `setImmediate` with a string argument
    /// is an implied `eval()` because the string is evaluated as code.
    ///
    /// Using implied `eval()` is considered a bad practice because:
    /// 1. It exposes your code to security risks and performance issues
    /// 2. The code is evaluated in the global scope rather than the local scope
    /// 3. It prevents the JavaScript engine from optimizing the code
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// setTimeout("alert('Hello world!');", 100);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// setInterval("alert('Hello world!');", 100);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// setImmediate("alert('Hello world!');");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.setTimeout("count = 5", 10);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.setInterval("foo = bar", 10);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// setTimeout(function() {
    ///     alert('Hello world!');
    /// }, 100);
    /// ```
    ///
    /// ```js
    /// setInterval(() => {
    ///     alert('Hello world!');
    /// }, 100);
    /// ```
    ///
    /// ```js
    /// // setTimeout is shadowed by a local variable
    /// function foo(setTimeout) {
    ///     setTimeout("alert('Hello world!');", 100);
    /// }
    /// ```
    ///
    /// ## Resources
    ///
    /// - [MDN setTimeout() documentation](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout#the_string_problem)
    /// - [MDN eval() documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_direct_eval!)
    ///
    pub NoImpliedEval {
        version: "next",
        name: "noImpliedEval",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-implied-eval").same(),
            RuleSource::EslintTypeScript("no-implied-eval").same(),
        ],
        recommended: false,
        severity: Severity::Error,
        issue_number: Some("8735"),
    }
}

const EVAL_LIKE_FUNCTIONS: &[&str] = &["setTimeout", "setInterval", "setImmediate"];

const GLOBAL_OBJECTS: &[&str] = &["window", "global", "globalThis"];

impl Rule for NoImpliedEval {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoImpliedEvalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let model = ctx.model();

        // Get the callee (function being called)
        let callee = call_expr.callee().ok()?;

        // Check if it's one of the eval-like functions
        if !is_eval_like_function(&callee, model) {
            return None;
        }

        // Check if the first argument is a string
        let args = call_expr.arguments().ok()?;
        let first_arg = args.args().first()?.ok()?;

        if is_string_argument(first_arg.as_any_js_expression()?) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let call_expr = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                call_expr.range(),
                markup! {
                    "Implied "<Emphasis>"eval()"</Emphasis>" is not allowed."
                },
            )
            .note(markup! {
                "Passing strings to functions like "<Emphasis>"setTimeout"</Emphasis>", "
                <Emphasis>"setInterval"</Emphasis>", or "<Emphasis>"setImmediate"</Emphasis>
                " is a form of implied "<Emphasis>"eval()"</Emphasis>" and can lead to security and performance issues."
            })
            .note(markup! {
                "Use a function instead of a string."
            }),
        )
    }
}

/// Checks if the callee is one of the eval-like functions
/// Handles patterns like:
/// - setTimeout (direct identifier)
/// - window.setTimeout (static member)
/// - `window["setTimeout"]` (computed member)
/// - globalThis.setTimeout (static member)
/// - (0, setTimeout) (sequence expression)
fn is_eval_like_function(
    callee: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    match callee {
        // Direct call: setTimeout(...)
        AnyJsExpression::JsIdentifierExpression(_) => {
            if let Some((reference, name)) = global_identifier(callee) {
                let name_text = name.text();
                // Check if it's a global binding and one of the eval-like functions
                return model.binding(&reference).is_none()
                    && EVAL_LIKE_FUNCTIONS.contains(&name_text);
            }
            false
        }

        // Member access: window.setTimeout(...) or globalThis.setTimeout(...)
        AnyJsExpression::JsStaticMemberExpression(member) => {
            if let (Ok(object), Ok(member_name)) = (member.object(), member.member())
                && let Some(js_name) = member_name.as_js_name()
                && let Ok(token) = js_name.value_token()
            {
                let name_text = token.text_trimmed();
                // Check if object is global and method is eval-like
                return is_global_object(&object, model)
                    && EVAL_LIKE_FUNCTIONS.contains(&name_text);
            }
            false
        }

        // Computed member: window["setTimeout"](...)
        AnyJsExpression::JsComputedMemberExpression(member) => {
            if let (Ok(object), Ok(member_expr)) = (member.object(), member.member())
                && is_global_object(&object, model)
                && let Some(static_value) = member_expr.as_static_value()
                && let Some(name_text) = static_value.as_string_constant()
            {
                return EVAL_LIKE_FUNCTIONS.contains(&name_text);
            }
            false
        }

        // Sequence expression: (0, setTimeout)(...)
        AnyJsExpression::JsSequenceExpression(sequence) => {
            // Get the last expression in the sequence
            if let Ok(right) = sequence.right() {
                return is_eval_like_function(&right, model);
            }
            false
        }

        // Parenthesized expression: may contain a sequence expression
        AnyJsExpression::JsParenthesizedExpression(paren) => {
            // Unwrap the parenthesized expression and check recursively
            if let Ok(inner) = paren.expression() {
                return is_eval_like_function(&inner, model);
            }
            false
        }

        _ => false,
    }
}

/// Checks if the expression is a global object (window, global, globalThis)
fn is_global_object(expr: &AnyJsExpression, model: &biome_js_semantic::SemanticModel) -> bool {
    // Handle direct identifiers: window, global, globalThis
    if let Some((reference, name)) = global_identifier(expr) {
        let name_text = name.text();

        // Check if it's one of the global objects and is actually global
        if GLOBAL_OBJECTS.contains(&name_text) && model.binding(&reference).is_none() {
            return true;
        }
    }

    // Handle chained access: window.window, globalThis.globalThis
    // Only recurse if the member name is also a global object
    if let AnyJsExpression::JsStaticMemberExpression(member) = expr
        && let Ok(object) = member.object()
        && let Ok(member_name) = member.member()
        && let Some(js_name) = member_name.as_js_name()
        && let Ok(token) = js_name.value_token()
    {
        let name_text = token.text_trimmed();
        // Only continue checking if the member is also a global object
        if GLOBAL_OBJECTS.contains(&name_text) {
            return is_global_object(&object, model);
        }
    }

    false
}

/// Checks if the argument is a string (literal, template, or concatenation)
fn is_string_argument(arg: &AnyJsExpression) -> bool {
    use biome_js_syntax::AnyJsExpression::*;

    match arg {
        // String literal: "code"
        AnyJsLiteralExpression(lit) => lit.as_js_string_literal_expression().is_some(),

        // Template literal: `code`
        JsTemplateExpression(template) => {
            // Only flag templates with no substitutions
            template
                .elements()
                .iter()
                .all(|element| element.as_js_template_chunk_element().is_some())
        }

        // Binary expression: "a" + "b"
        JsBinaryExpression(bin) => {
            if let Ok(operator) = bin.operator() {
                use biome_js_syntax::JsBinaryOperator::Plus;
                if matches!(operator, Plus) {
                    // Check if either operand is a string
                    if let (Ok(left), Ok(right)) = (bin.left(), bin.right()) {
                        return is_string_argument(&left) || is_string_argument(&right);
                    }
                }
            }
            false
        }

        _ => false,
    }
}
