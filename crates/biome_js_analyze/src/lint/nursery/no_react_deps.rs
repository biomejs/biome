use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow usage of dependency arrays in `createEffect` and `createMemo`.
    ///
    /// In Solid, `createEffect` and `createMemo` track dependencies automatically, it's no need to add dependency arrays.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal()]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, [signal]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createEffect } from "solid-js";
    /// const deps = [signal];
    /// createEffect(() => {
    ///   console.log(signal());
    /// }, deps)
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a(), b()]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), [a, b()]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const deps = [a, b];
    /// const value = createMemo(() => computeExpensiveValue(a(), b()), deps);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { createMemo } from "solid-js";
    /// const deps = [a, b];
    /// const memoFn = () => computeExpensiveValue(a(), b());
    /// const value = createMemo(memoFn, deps);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect(() => {
    ///    console.log(signal());
    /// });
    /// ```
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return prev + 1;
    /// }, 0);
    /// ```
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return (prev || 0) + 1;
    /// });
    /// ```
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// createEffect((prev) => {
    ///   console.log(signal());
    ///   return prev ? prev + 1 : 1;
    /// }, undefined);
    /// ```
    ///
    /// ```js
    /// import { createMemo } from "solid-js";
    /// const value = createMemo(() => computeExpensiveValue(a(), b()));
    /// ```
    ///
    /// ```js
    /// import { createMemo } from "solid-js";
    /// const sum = createMemo((prev) => input() + prev, 0);
    /// ```
    ///
    /// ```js
    /// import { createEffect } from "solid-js";
    /// const args = [
    ///   () => {
    ///     console.log(signal());
    ///   },
    ///   [signal()],
    /// ];
    /// createEffect(...args);
    /// ```
    pub NoReactDeps {
        version: "next",
        name: "noReactDeps",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("no-react-deps")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for NoReactDeps {
    type Query = Ast<JsCallExpression>;
    type State = (String, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let ident = callee.as_js_identifier_expression()?.name().ok()?;
        let callee_name = ident.value_token().ok()?;
        let callee_name = callee_name.text_trimmed();

        if callee_name != "createEffect" && callee_name != "createMemo" {
            return None;
        }

        let arguments = node.arguments().ok()?.args();
        let len = arguments.len();
        let mut iter = arguments.into_iter();

        let has_spread = iter.all(|arg| arg.is_ok_and(|arg| arg.as_js_spread().is_some()));

        if len == 2 && !has_spread {
            let first_argument = iter.next()?.ok()?;
            let first_argument = first_argument.as_any_js_expression()?;

            let is_first_arg_function_type =
                first_argument.as_js_arrow_function_expression().is_some()
                    || first_argument.as_js_function_expression().is_some();

            let first_arg_parameter_len = match first_argument {
                AnyJsExpression::JsArrowFunctionExpression(node) => node.parameters().ok()?.len(),
                AnyJsExpression::JsFunctionExpression(node) => {
                    node.parameters().ok()?.items().len()
                }
                _ => 0,
            };

            let second_argument = iter.next()?.ok()?;
            let second_argument = second_argument.as_any_js_expression()?;
            let is_second_arg_array_type = second_argument.as_js_array_expression().is_some();

            if is_first_arg_function_type
                && first_arg_parameter_len == 0
                && is_second_arg_array_type
            {
                return Some((callee_name.into(), second_argument.range()));
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (callee_name, range) = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "In Solid, "<Emphasis>{callee_name}</Emphasis>" doesn't accept a dependency array because it automatically tracks its dependencies."
                },
            )
            .note(markup! {
                "Please just remove the dependency array parameter here."
            })
            .note(markup! {
                "If you really need to override the list of dependencies, use \
                "<Hyperlink href="https://docs.solidjs.com/reference/reactive-utilities/on-util#on">"on"</Hyperlink>"."
            }),
        )
    }
}
