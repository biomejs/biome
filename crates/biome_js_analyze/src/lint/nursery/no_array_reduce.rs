use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsStatement,
    JsArrayExpression, JsAssignmentExpression, JsAwaitExpression, JsBinaryExpression,
    JsCallExpression, JsClassExpression, JsInExpression, JsInstanceofExpression, JsLanguage,
    JsName, JsNewExpression, JsObjectExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsTemplateExpression, JsThisExpression, JsUnaryExpression,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, SyntaxKindSet};
use biome_rule_options::no_array_reduce::NoArrayReduceOptions;

use crate::{
    services::semantic::Semantic, utils::indexed_collection::is_known_non_indexed_collection,
};

declare_lint_rule! {
    /// Disallow the `reduce()` and `reduceRight()` array methods.
    ///
    /// The function passed to `reduce()` receives the result of its previous call along with
    /// the next item, and it has to return the result for the following call.
    /// This makes it hard to see what the code computes, especially when the result is an object
    /// or an array that is built up step by step.
    /// A `for...of` loop, or methods such as `map()` and `filter()`, usually express the same
    /// thing more clearly.
    ///
    /// This rule also reports `reduce()` and `reduceRight()` when they are borrowed from
    /// `Array.prototype` or `[]` with `call()` or `apply()`.
    ///
    /// Calls whose function only returns the result of a single arithmetic, comparison, or
    /// bitwise operation, such as `(total, item) => total + item`, are allowed, because they are
    /// short and easy to read. Logical operations, such as `(found, item) => found || item`, are
    /// still reported.
    ///
    /// This rule doesn't report calls when the value is known not to be an array,
    /// such as a `Map` or a `Set`, either because it is created with `new Map()`
    /// or because its type annotation says so.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const counts = words.reduce((counts, word) => {
    ///     counts[word] = (counts[word] ?? 0) + 1;
    ///     return counts;
    /// }, {});
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = array.reduceRight(reducer, initialValue);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = Array.prototype.reduce.call(array, reducer);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const result = [].reduce.apply(array, [reducer, initialValue]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const counts = {};
    /// for (const word of words) {
    ///     counts[word] = (counts[word] ?? 0) + 1;
    /// }
    ///
    /// const total = array.reduce((total, item) => total + item, 0);
    ///
    /// const product = array.reduce(function (product, item) {
    ///     return product * item;
    /// });
    /// ```
    pub NoArrayReduce {
        version: "2.6.10",
        name: "noArrayReduce",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-array-reduce").same()],
        recommended: false,
    }
}

impl Rule for NoArrayReduce {
    type Query = Semantic<JsCallExpression>;
    /// The name of the `reduce` or `reduceRight` method that is called.
    type State = JsName;
    type Signals = Option<Self::State>;
    type Options = NoArrayReduceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        // `value.reduce?.()` is skipped, because the optional call suggests that `value`
        // may not be an array.
        if call.is_optional() {
            return None;
        }
        let arguments = call.arguments().ok()?;
        if arguments
            .args()
            .iter()
            .any(|argument| matches!(argument, Ok(AnyJsCallArgument::JsSpread(_))))
        {
            return None;
        }
        let [first, second] = arguments.get_arguments_by_index([0, 1]);

        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_static_member_expression()?;
        let method = callee.member().ok()?.as_js_name()?.clone();
        let method_name = method.value_token().ok()?;
        let model = ctx.model();
        match method_name.text_trimmed() {
            // `array.reduce(callback)` and `array.reduce(callback, initialValue)`
            "reduce" | "reduceRight" => {
                let callback = first
                    .as_ref()
                    .and_then(AnyJsCallArgument::as_any_js_expression)?;
                if arguments.args().len() > 2
                    || is_not_function(callback)
                    || is_simple_operation(callback)
                    || is_known_non_indexed_collection(&callee.object().ok()?, model)
                {
                    return None;
                }
                Some(method)
            }
            // `Array.prototype.reduce.call(array, callback)` and `[].reduce.apply(array, arguments)`
            "call" | "apply" => {
                if callee.is_optional() {
                    return None;
                }
                let object = callee.object().ok()?.omit_parentheses();
                let borrowed = object.as_js_static_member_expression()?;
                if borrowed.is_optional() {
                    return None;
                }
                let borrowed_method = borrowed.member().ok()?.as_js_name()?.clone();
                let borrowed_name = borrowed_method.value_token().ok()?;
                if !matches!(borrowed_name.text_trimmed(), "reduce" | "reduceRight")
                    || !is_array_prototype(&borrowed.object().ok()?, model)
                {
                    return None;
                }
                if method_name.text_trimmed() == "call"
                    && second
                        .as_ref()
                        .and_then(AnyJsCallArgument::as_any_js_expression)
                        .is_some_and(is_not_function)
                {
                    return None;
                }
                Some(borrowed_method)
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let name = method.value_token().ok()?;
        let name = name.text_trimmed();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            method.range(),
            markup! {
                "Avoid calling "<Emphasis>{name}"()"</Emphasis>"."
            },
        )
        .note(markup! {
            "The function passed to "<Emphasis>{name}"()"</Emphasis>" has to pass its result along to the next call, which makes it hard to see what the code computes."
        });
        Some(if name == "reduceRight" {
            diagnostic.note(markup! {
                "Use a "<Emphasis>"for...of"</Emphasis>" loop instead. To go through the items from last to first, loop over "<Emphasis>"toReversed()"</Emphasis>"."
            })
        } else {
            diagnostic.note(markup! {
                "Use a "<Emphasis>"for...of"</Emphasis>" loop, or methods such as "<Emphasis>"map()"</Emphasis>" and "<Emphasis>"filter()"</Emphasis>", instead."
            })
        })
    }
}

/// Returns `true` if `expression` can't be a function, or most likely isn't one.
///
/// Calls are included, except for `bind()` calls, which return a function.
fn is_not_function(expression: &AnyJsExpression) -> bool {
    let expression = expression.clone().omit_parentheses();
    if NOT_FUNCTION_KINDS.matches(expression.syntax().kind()) {
        return true;
    }
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .is_ok_and(|reference| reference.is_undefined()),
        AnyJsExpression::JsCallExpression(call) => !is_bind_call(&call),
        _ => false,
    }
}

/// Returns `true` if `call` is a non-optional `value.bind()` call.
fn is_bind_call(call: &JsCallExpression) -> bool {
    !call.is_optional()
        && call.callee().is_ok_and(|callee| {
            callee
                .as_js_static_member_expression()
                .is_some_and(|member| {
                    !member.is_optional()
                        && member
                            .member()
                            .ok()
                            .and_then(|name| name.as_js_name()?.value_token().ok())
                            .is_some_and(|name| name.text_trimmed() == "bind")
                })
        })
}

/// Returns `true` if `callback` is a function whose body only returns the result of an operation
/// listed in [SIMPLE_OPERATION_KINDS], such as `(total, item) => total + item` or
/// `function (total, item) { return total * item; }`.
fn is_simple_operation(callback: &AnyJsExpression) -> bool {
    let body = match callback.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok(),
        AnyJsExpression::JsFunctionExpression(function) => {
            function.body().ok().map(AnyJsFunctionBody::JsFunctionBody)
        }
        _ => None,
    };
    let returned = match body {
        Some(AnyJsFunctionBody::AnyJsExpression(expression)) => Some(expression),
        Some(AnyJsFunctionBody::JsFunctionBody(body)) => {
            let statements = body.statements();
            if body.directives().is_empty() && statements.len() == 1 {
                match statements.first() {
                    Some(AnyJsStatement::JsReturnStatement(statement)) => statement.argument(),
                    _ => None,
                }
            } else {
                None
            }
        }
        None => None,
    };
    returned.is_some_and(|expression| {
        SIMPLE_OPERATION_KINDS.matches(expression.omit_parentheses().syntax().kind())
    })
}

/// Returns `true` if `expression` is `Array.prototype` or an empty array literal `[]`, where
/// `Array` refers to the global `Array` constructor.
fn is_array_prototype(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    match expression.clone().omit_parentheses() {
        AnyJsExpression::JsArrayExpression(array) => array.elements().is_empty(),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            !member.is_optional()
                && member
                    .member()
                    .ok()
                    .and_then(|name| name.as_js_name()?.value_token().ok())
                    .is_some_and(|name| name.text_trimmed() == "prototype")
                && member
                    .object()
                    .ok()
                    .and_then(|object| object.omit_parentheses().as_js_reference_identifier())
                    .is_some_and(|reference| {
                        reference.has_name("Array") && model.binding(&reference).is_none()
                    })
        }
        _ => false,
    }
}

/// Operations between two values, such as `a + b`, `a < b`, or `a in b`. Logical operations,
/// such as `a || b`, are not included.
const SIMPLE_OPERATION_KINDS: SyntaxKindSet<JsLanguage> = JsBinaryExpression::KIND_SET
    .union(JsInExpression::KIND_SET)
    .union(JsInstanceofExpression::KIND_SET);

/// Expressions whose value can't be a function, or most likely isn't one.
const NOT_FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = SIMPLE_OPERATION_KINDS
    .union(JsArrayExpression::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(AnyJsLiteralExpression::KIND_SET)
    .union(JsObjectExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsNewExpression::KIND_SET)
    .union(JsThisExpression::KIND_SET);
