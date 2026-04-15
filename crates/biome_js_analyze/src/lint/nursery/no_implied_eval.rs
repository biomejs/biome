use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression::{self, *},
    AnyJsMemberExpression, JsBinaryOperator, JsNewOrCallExpression, global_identifier,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList};
use biome_rule_options::no_implied_eval::NoImpliedEvalOptions;

declare_lint_rule! {
    /// Disallow the use of `eval()`-like methods.
    ///
    /// The `eval()` function evaluates the passed string as a _JavaScript_ code.
    /// Calling `setTimeout`, `setInterval`, or `setImmediate` with a string argument,
    /// or using the global `Function` constructor, is an implied `eval()` because code
    /// is created from strings at runtime.
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
    /// ```js,expect_diagnostic
    /// Function("a", "b", "return a + b");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Function("a", "b", "return a + b");
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
    /// ```js
    /// function foo(Function) {
    ///     Function("a", "b", "return a + b");
    /// }
    /// ```
    ///
    /// ## Resources
    ///
    /// - [MDN setTimeout() documentation](https://developer.mozilla.org/en-US/docs/Web/API/setTimeout#the_string_problem)
    /// - [MDN eval() documentation](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_direct_eval!)
    ///
    pub NoImpliedEval {
        version: "2.4.10",
        name: "noImpliedEval",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-implied-eval").same(),
            RuleSource::Eslint("no-new-func").inspired(),
            RuleSource::EslintTypeScript("no-implied-eval").same(),
        ],
        recommended: true,
        severity: Severity::Error,
        issue_number: Some("8735"),
    }
}

const EVAL_LIKE_FUNCTIONS: &[&str] = &["setTimeout", "setInterval", "setImmediate"];

impl Rule for NoImpliedEval {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = NoImpliedEvalState;
    type Signals = Option<Self::State>;
    type Options = NoImpliedEvalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        let callee = node.callee().ok()?;

        if is_function_constructor_call(node, &callee, model) {
            return Some(NoImpliedEvalState::FunctionConstructor);
        }

        let JsNewOrCallExpression::JsCallExpression(call_expression) = node else {
            return None;
        };

        if !is_eval_like_function(&callee, model)? {
            return None;
        }

        let args = call_expression.arguments().ok()?;
        let first_arg = args.args().first()?.ok()?;

        if is_string_argument(first_arg.as_any_js_expression()?) {
            return Some(NoImpliedEvalState::StringArgument);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match state {
            NoImpliedEvalState::StringArgument => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
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
            ),
            NoImpliedEvalState::FunctionConstructor => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "Do not use the global "<Emphasis>"Function"</Emphasis>" constructor."
                    },
                )
                .note(markup! {
                    "It parses strings into executable code at runtime and has the same security and performance drawbacks as "<Emphasis>"eval()"</Emphasis>"."
                })
                .note(markup! {
                    "Use a function declaration, function expression, or arrow function instead."
                }),
            ),
        }
    }
}

pub enum NoImpliedEvalState {
    StringArgument,
    FunctionConstructor,
}

/// Checks if the callee is one of the eval-like functions
/// Handles patterns like:
/// - setTimeout (direct identifier)
/// - window.setTimeout (static member)
/// - `window["setTimeout"]` (computed member)
/// - globalThis.setTimeout (static member)
///
/// Uses the `global_identifier` utility which handles window/globalThis chains automatically
fn is_eval_like_function(
    callee: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> Option<bool> {
    let unwrapped = callee.clone().omit_parentheses();
    let (reference, name) = global_identifier(&unwrapped)?;

    Some(model.binding(&reference).is_none() && EVAL_LIKE_FUNCTIONS.contains(&name.text()))
}

/// Checks if the call is a call to the global `Function` constructor, either directly `new Function()` or via `apply`/`call`/`bind`.
fn is_function_constructor_call(
    node: &JsNewOrCallExpression,
    callee: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    match node {
        JsNewOrCallExpression::JsNewExpression(_) => is_global_function_constructor(callee, model),
        JsNewOrCallExpression::JsCallExpression(_) => {
            is_global_function_constructor(callee, model)
                || is_function_call_method(callee, model).unwrap_or(false)
        }
    }
}

fn is_global_function_constructor(
    expression: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    let Some((reference, name)) = global_identifier(&expression.clone()) else {
        return false;
    };

    name.text() == "Function" && model.binding(&reference).is_none()
}

fn is_function_call_method(
    callee: &AnyJsExpression,
    model: &biome_js_semantic::SemanticModel,
) -> Option<bool> {
    let callee = callee.clone().omit_parentheses();
    let member_expression = AnyJsMemberExpression::cast_ref(callee.syntax())?;
    let member_name = member_expression.member_name()?;

    if !matches!(member_name.text(), "apply" | "bind" | "call") {
        return Some(false);
    }

    let object = member_expression.object().ok()?;

    Some(is_global_function_constructor(&object, model))
}

/// Checks if the argument is a string (literal, template, or concatenation)
fn is_string_argument(arg: &AnyJsExpression) -> bool {
    let unwrapped = arg.clone().omit_parentheses();

    match unwrapped {
        AnyJsLiteralExpression(lit) => lit.as_js_string_literal_expression().is_some(),

        JsTemplateExpression(template) => {
            // Only flag templates with no substitutions
            template
                .elements()
                .iter()
                .all(|element| element.as_js_template_chunk_element().is_some())
        }

        JsBinaryExpression(bin) => {
            if let Ok(operator) = bin.operator()
                && matches!(operator, JsBinaryOperator::Plus)
                && let (Ok(left), Ok(right)) = (bin.left(), bin.right())
            {
                return is_string_argument(&left) || is_string_argument(&right);
            }
            false
        }

        _ => false,
    }
}
