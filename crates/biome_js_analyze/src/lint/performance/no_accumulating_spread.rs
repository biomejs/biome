use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsFunction, AnyJsMemberExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsFormalParameter, JsParameterList, JsParameters, JsSpread,
};
use biome_rowan::{AstNode, AstSeparatedList};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow the use of spread (`...`) syntax on accumulators.
    ///
    /// Spread syntax allows an iterable to be expanded into its individual elements.
    ///
    /// Spread syntax should be avoided on accumulators (like those in `.reduce`)
    /// because it causes a time complexity of `O(n^2)` instead of `O(n)`.
    ///
    /// Source: https://prateeksurana.me/blog/why-using-object-spread-with-reduce-bad-idea/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => [...acc, val], []);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {return [...acc, val];}, []);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => ({...acc, [val]: val}), {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {acc.push(val); return acc}, []);
    /// ```
    ///
    pub NoAccumulatingSpread {
        version: "1.0.0",
        name: "noAccumulatingSpread",
        language: "js",
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoAccumulatingSpread {
    type Query = Semantic<JsSpread>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        is_known_accumulator(node, model)?.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the use of spread (`...`) syntax on accumulators."
                },
            )
            .note(markup! {
                "Spread syntax should be avoided on accumulators (like those in `.reduce`) because it causes a time complexity of `O(n^2)`."
            })
            .note(markup! {
                "Consider methods such as .splice or .push instead."
            }),
        )
    }
}

fn is_known_accumulator(node: &JsSpread, model: &SemanticModel) -> Option<bool> {
    let reference = node
        .argument()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;
    let parameter = model
        .binding(&reference)
        .and_then(|declaration| declaration.syntax().parent())
        .and_then(JsFormalParameter::cast)?;
    let function = parameter
        .parent::<JsParameterList>()
        .and_then(|list| list.parent::<JsParameters>())
        .and_then(|parameters| parameters.parent::<AnyJsFunction>())?;

    // Known accumulators need at least 2 arguments and no more than 4. (accumulator, value, index, array)
    let param_count = function
        .parameters()
        .ok()?
        .as_js_parameters()?
        .items()
        .iter()
        .count();
    if !(2..=4).contains(&param_count) {
        return None;
    }

    let call_expression = function
        .parent::<JsCallArgumentList>()
        .and_then(|arguments| arguments.parent::<JsCallArguments>())
        .and_then(|arguments| arguments.parent::<JsCallExpression>())?;

    // The accumulator function should be a part of a call expression. This call expression should
    // have no more than 2 arguments. (callback, initialValue)
    let arg_count = call_expression.arguments().ok()?.args().iter().count();
    if arg_count > 2 {
        return None;
    }

    let callee = call_expression.callee().ok()?;
    let member_expression = AnyJsMemberExpression::cast(callee.into_syntax())?;

    // We only care about `.reduce` and `.reduceRight`.
    let member_name = member_expression.member_name()?;
    if !matches!(member_name.text(), "reduce" | "reduceRight") {
        return None;
    }

    // Finally check that the spread references the first parameter.
    Some(parameter.syntax().index() == 0)
}
