use biome_analyze::{Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsFunction, AnyJsMemberExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsFormalParameter, JsParameterList, JsParameters, JsReferenceIdentifier, JsSpread,
    JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::no_accumulating_spread::NoAccumulatingSpreadOptions;

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
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => Object.assign(acc, val), []);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var a = ['a', 'b', 'c'];
    /// a.reduce((acc, val) => {return Object.assign(acc, val);}, []);
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
        severity: Severity::Warning,
    }
}

declare_node_union! {
    pub AnySpread = JsSpread | JsStaticMemberExpression
}

pub struct FoundSpread {
    range: TextRange,
    is_spread: bool,
}

impl FoundSpread {
    fn new(range: TextRange, is_spread: bool) -> Self {
        Self { range, is_spread }
    }
}

impl Rule for NoAccumulatingSpread {
    type Query = Semantic<AnySpread>;
    type State = FoundSpread;
    type Signals = Option<Self::State>;
    type Options = NoAccumulatingSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();

        match ctx.query() {
            AnySpread::JsSpread(node) => {
                handle_spread(node, model)?.then_some(FoundSpread::new(node.range(), true))
            }
            AnySpread::JsStaticMemberExpression(node) => {
                handle_object_assign(node, model)?.then_some(FoundSpread::new(node.range(), false))
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, found_spread: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                found_spread.range,
                match found_spread.is_spread {
                    true => markup! {
                        "Avoid the use of spread (`...`) syntax on accumulators."
                    },
                    false => markup! {
                        "Avoid the use of Object.assign on accumulators."
                    }
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

fn is_known_accumulator(reference: &JsReferenceIdentifier, model: &SemanticModel) -> Option<bool> {
    let parameter = model
        .binding(reference)
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

fn handle_spread(node: &JsSpread, model: &SemanticModel) -> Option<bool> {
    let reference = node
        .argument()
        .ok()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;

    is_known_accumulator(&reference, model)
}

// https://github.com/biomejs/biome/issues/5277
// Spread operators are just syntax for Object.assign
// Lets handle the rare cases where someone may use this
fn handle_object_assign(node: &JsStaticMemberExpression, model: &SemanticModel) -> Option<bool> {
    let object = node.object().ok()?;

    let object_name = object.as_js_identifier_expression()?.name().ok()?;
    if object_name.to_trimmed_text() != "Object" {
        return None;
    }
    let operator = node.member().ok()?;
    if operator.to_trimmed_text() != "assign" {
        return None;
    }

    let call_expression = node.parent::<JsCallExpression>()?;
    let arguments = call_expression.arguments().ok()?;
    let reference = arguments
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()?
        .as_js_identifier_expression()?
        .name()
        .ok()?;

    is_known_accumulator(&reference, model)
}
