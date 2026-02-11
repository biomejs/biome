use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsFunction, JsCallExpression, JsSyntaxNode};
use biome_rowan::AstNode;
use biome_rule_options::no_nested_promises::NoNestedPromisesOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow nested `.then()` or `.catch()` promise calls.
    ///
    /// Nesting `.then()` or `.catch()` calls defeats the purpose of promises,
    /// which is to create a flat chain of asynchronous operations. Nested promise
    /// callbacks can make code harder to read and maintain.
    ///
    /// However, nesting is allowed when the nested callback references variables
    /// from the outer scope, as flattening would break the code in such cases.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// doThing().then(function() { return a.then() })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// doThing().then(() => b.catch())
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// doThing()
    ///   .then(a => getB(a)
    ///     .then(b => getC(b))
    ///   )
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // Simple returns
    /// doThing().then(function() { return 4 })
    /// doThing().then(() => 4)
    /// ```
    ///
    /// ```js
    /// // Chained promises (no nesting)
    /// doThing()
    ///   .then(a => getB(a))
    ///   .then(b => getC(b))
    /// ```
    ///
    /// ```js
    /// // Nested but references outer scope variable 'a'
    /// doThing()
    ///   .then(a => getB(a)
    ///     .then(b => getC(a, b))
    ///   )
    /// ```
    ///
    /// ```js
    /// // Promise.resolve/all are fine
    /// doThing().then(function() { return Promise.all([a,b,c]) })
    /// doThing().then(() => Promise.resolve(4))
    /// ```
    ///
    pub NoNestedPromises {
        version: "next",
        name: "noNestedPromises",
        language: "js",
        recommended: true,
        sources: &[RuleSource::EslintPromise("no-nesting").same()],
    }
}

impl Rule for NoNestedPromises {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoNestedPromisesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expr = ctx.query();
        let model = ctx.model();

        // Check if this is a .then() or .catch() call
        let member_name = call_expr
            .callee()
            .ok()?
            .as_js_static_member_expression()?
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?;
        let member_text = member_name.text_trimmed();

        // This rule intentionally does not check type information to know if this is actually a Promise, because that's what the source rule does.
        if member_text != "then" && member_text != "catch" {
            return None;
        }

        // Check if this call is inside a promise callback function
        let parent_promise_callback = find_parent_promise_callback(call_expr.syntax())?;

        // Check if the nested promise callback references parent scope variables
        if let Some(nested_callback) = get_nested_callback(call_expr)
            && references_outer_scope(&nested_callback, &parent_promise_callback, model)
        {
            // Allow nesting when the nested callback references outer scope variables
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        // Report the location of the then/catch member
        let member_name = ctx
            .query()
            .callee()
            .ok()?
            .as_js_static_member_expression()?
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member_name.text_range(),
                markup! {
                    "Avoid nesting promises."
                },
            )
            .note(markup! {
                "Nesting promises can lead to harder-to-read code because it creates multiple levels of indentation and makes the flow of asynchronous operations less clear."
            })
            .note(markup! {
                "Consider refactoring the code to use promise chaining (foo.then().then()) instead of nesting."
            }),
        )
    }
}

/// Find the parent promise callback function that contains this node
fn find_parent_promise_callback(node: &JsSyntaxNode) -> Option<AnyJsFunction> {
    let mut current = node.parent()?;

    loop {
        // Check if current node is a function
        if let Some(func) = AnyJsFunction::cast_ref(&current) {
            // Check if this function is a callback to .then() or .catch()
            if is_promise_callback(&func) {
                return Some(func);
            }
        }
        current = current.parent()?;
    }
}

/// Check if a function is a direct callback to .then() or .catch()
fn is_promise_callback(func: &AnyJsFunction) -> bool {
    // Walk up the tree to find a call expression
    // The structure is: Function -> JsCallArgument (or similar) -> ... -> JsCallExpression
    func.syntax()
        .ancestors()
        .skip(1) // skip self
        .find_map(|node| {
            JsCallExpression::cast_ref(&node)?
                .callee()
                .ok()?
                .as_js_static_member_expression()?
                .member()
                .ok()?
                .as_js_name()?
                .value_token()
                .ok()
        })
        .is_some_and(|token| {
            let text = token.text_trimmed();
            text == "then" || text == "catch"
        })
}

/// Get the nested callback function from a .then() or .catch() call
fn get_nested_callback(call_expr: &JsCallExpression) -> Option<AnyJsFunction> {
    let args = call_expr.arguments().ok()?;
    let first_arg = args.args().into_iter().next()?.ok()?;

    match first_arg {
        biome_js_syntax::AnyJsCallArgument::AnyJsExpression(expr) => match expr {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                Some(AnyJsFunction::JsArrowFunctionExpression(arrow))
            }
            AnyJsExpression::JsFunctionExpression(func) => {
                Some(AnyJsFunction::JsFunctionExpression(func))
            }
            _ => None,
        },
        _ => None,
    }
}

/// Check if the nested callback references any variables from the outer scope
/// (i.e., variables defined in the parent callback or its parent scopes)
fn references_outer_scope(
    nested_callback: &AnyJsFunction,
    parent_callback: &AnyJsFunction,
    model: &biome_js_semantic::SemanticModel,
) -> bool {
    // Walk through the nested callback and check if any references point to bindings
    // that are defined outside the nested callback
    for node in nested_callback.syntax().descendants() {
        if let Some(ident) = biome_js_syntax::JsReferenceIdentifier::cast_ref(&node) {
            // Check if this identifier has a binding in the semantic model
            if let Some(binding) = model.binding(&ident) {
                // Check if the binding is defined outside the nested callback
                // by checking if the binding's syntax node is NOT a descendant of the nested callback
                let binding_syntax = binding.syntax();

                // If the binding is not inside the nested callback, it's an outer scope reference
                if !is_descendant_of(binding_syntax, nested_callback.syntax()) {
                    // Further check: is it inside the parent callback?
                    // If yes, this is a closure over parent scope variable
                    if is_descendant_of(binding_syntax, parent_callback.syntax()) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

/// Check if a node is a descendant of another node
fn is_descendant_of(node: &JsSyntaxNode, potential_ancestor: &JsSyntaxNode) -> bool {
    let mut current = Some(node.clone());
    while let Some(node) = current {
        if node == *potential_ancestor {
            return true;
        }
        current = node.parent();
    }
    false
}
