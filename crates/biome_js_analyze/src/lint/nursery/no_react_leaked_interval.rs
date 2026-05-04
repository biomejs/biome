use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsVariableDeclarator};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_react_leaked_interval::NoReactLeakedIntervalOptions;

use crate::{
    react::{effect_callback, is_effect_call},
    typescript::unwrap_typescript_expression,
};

declare_lint_rule! {
    /// Disallow forgetting to clear `setInterval` within `useEffect`.
    ///
    /// This rule detects `setInterval` calls within `useEffect` hooks that don't have a corresponding
    /// `clearInterval` call in the cleanup function. Forgetting to clear an interval can lead to memory
    /// leaks and unexpected behavior when components unmount or dependencies change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const intervalId = setInterval(() => {
    ///       console.log("Hello");
    ///     }, 1000);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const intervalId = setInterval(() => {
    ///       console.log("Hello");
    ///     }, 1000);
    ///     return () => clearInterval(intervalId);
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedInterval {
        version: "next",
        name: "noReactLeakedInterval",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactXyz("web-api-no-leaked-interval").same(), RuleSource::EslintReactWebApi("no-leaked-interval").same()],
    }
}

impl Rule for NoReactLeakedInterval {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedIntervalOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let effect_call = ctx.query();
        if !is_effect_call(effect_call) {
            return None;
        }

        let callback = effect_callback(effect_call)?;
        let callback_node = callback.syntax();

        for call in callback_node
            .descendants()
            .filter_map(JsCallExpression::cast)
        {
            if !is_set_interval_call(&call) {
                continue;
            }

            // Check if it's a bare setInterval call (not assigned to a variable)
            if !call
                .syntax()
                .ancestors()
                .any(|ancestor| JsVariableDeclarator::can_cast(ancestor.kind()))
            {
                // Bare setInterval call without assignment - needs clearInterval
                if !has_clear_interval_in_cleanup(callback_node, None) {
                    return Some(call.range());
                }

            // setInterval assigned to a variable - check if that variable is cleared
            } else if let Some(var_name) = get_assigned_variable_name(&call)
                && !has_clear_interval_in_cleanup(callback_node, Some(&var_name))
            {
                return Some(call.range());
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "A "<Emphasis>"setInterval"</Emphasis>" created in an effect must be cleared in the cleanup function."
            },
        )
        .note(markup! {
            "Not clearing the interval can cause memory leaks and trigger state updates on an unmounted component."
        })
        .note(markup! {
            "Return a cleanup function from the effect that calls "<Emphasis>"clearInterval"</Emphasis>" with the interval ID."
        }))
    }
}

fn is_set_interval_call(call: &JsCallExpression) -> bool {
    let Ok(callee) = call.callee() else {
        return false;
    };

    // Handle normal `setInterval(...)` and `window.setInterval(...)` calls
    if let Some(name) = callee.get_callee_member_name() {
        return name.text_trimmed() == "setInterval";
    }

    // Handle `(setInterval as any)(...)` — strip parentheses and TS type assertions
    let inner = unwrap_typescript_expression(callee.omit_parentheses());
    if let Some(name) = inner.get_callee_member_name() {
        return name.text_trimmed() == "setInterval";
    }

    false
}

fn get_assigned_variable_name(call: &JsCallExpression) -> Option<String> {
    for ancestor in call.syntax().ancestors() {
        if let Some(decl) = JsVariableDeclarator::cast_ref(&ancestor)
            && let Ok(binding) = decl.id()
        {
            let binding_text = binding.to_trimmed_string();

            if !binding_text.is_empty() {
                return Some(binding_text);
            }
        }
    }
    None
}

fn has_clear_interval_in_cleanup(
    callback_node: &biome_js_syntax::JsSyntaxNode,
    var_name: Option<&str>,
) -> bool {
    for call in callback_node
        .descendants()
        .filter_map(JsCallExpression::cast)
    {
        let Ok(callee) = call.callee() else {
            continue;
        };

        let Some(name) = callee.get_callee_member_name() else {
            continue;
        };

        if name.text_trimmed() != "clearInterval" {
            continue;
        }

        // If no var_name specified, any clearInterval is good enough (for bare setInterval)
        if var_name.is_none() {
            return true;
        }

        // Check if the clearInterval argument matches the variable name
        if let Ok(args) = call.arguments() {
            let [arg] = args.get_arguments_by_index([0]);
            if let Some(arg_option) = arg
                && let Some(expr) = arg_option.as_any_js_expression()
            {
                let inner = unwrap_typescript_expression(expr.clone());
                let arg_text = inner.to_trimmed_string();
                if arg_text == var_name.unwrap_or("") {
                    return true;
                }
            }
        }
    }

    false
}
