use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunction, AnyJsMemberExpression, AnyJsParameter,
    JsAssignmentOperator, JsCallArguments, JsCallExpression, JsFormalParameter, JsLogicalOperator,
    JsNewExpression, JsParameters,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::use_promise_reject_errors::UsePromiseRejectErrorsOptions;
use smallvec::{SmallVec, smallvec};

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require Error objects as Promise rejection reasons.
    ///
    /// Error objects capture a stack trace that helps locate the cause of a rejection.
    /// Rejecting with a string or another non-Error value loses this information.
    ///
    /// This rule checks `Promise.reject()` and calls to the second parameter of a
    /// `new Promise()` executor. Values that could be errors, such as function calls
    /// and unknown variables, are allowed without inspecting their types.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Promise.reject("Request failed");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Promise((resolve, reject) => reject(42));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// Promise.reject();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Promise.reject(new Error("Request failed"));
    /// new Promise((resolve, reject) => reject(new TypeError("Invalid value")));
    /// Promise.reject(getError());
    /// ```
    pub UsePromiseRejectErrors {
        version: "next",
        name: "usePromiseRejectErrors",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-promise-reject-errors").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for UsePromiseRejectErrors {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UsePromiseRejectErrorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if !is_reject_call(call, ctx.model())? {
            return None;
        }
        let arguments = call.arguments().ok()?;
        if let Some(argument) = arguments.args().iter().next() {
            match argument.ok()? {
                AnyJsCallArgument::AnyJsExpression(expression) => {
                    if could_be_error(expression, ctx.model())? {
                        return None;
                    }
                }
                AnyJsCallArgument::JsSpread(_) => return None,
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! { "This Promise rejection reason is not an "<Emphasis>"Error"</Emphasis>" object." },
            )
            .note(markup! { "Error objects capture a stack trace that helps locate the cause of a rejection." })
            .note(markup! { "Pass an "<Emphasis>"Error"</Emphasis>" object to preserve debugging information." }),
        )
    }
}

fn is_reject_call(call: &JsCallExpression, model: &SemanticModel) -> Option<bool> {
    let callee = call.callee().ok()?.omit_parentheses();
    if let Some(member) = AnyJsMemberExpression::cast_ref(callee.syntax()) {
        return Some(
            member.member_name()?.text() == "reject"
                && is_global_promise(member.object().ok()?, model)?,
        );
    }

    let reference = callee.as_js_identifier_expression()?.name().ok()?;
    let binding = model.binding(&reference)?;
    let parameter = JsFormalParameter::cast(binding.syntax().parent()?)?;
    let parameters = JsParameters::cast(parameter.syntax().parent()?.parent()?)?;
    let second = parameters
        .items()
        .iter()
        .filter(|parameter| !matches!(parameter, Ok(AnyJsParameter::TsThisParameter(_))))
        .nth(1)?
        .ok()?;
    let second = JsFormalParameter::cast_ref(second.syntax())?;
    if second.initializer().is_some()
        || second.binding().ok()?.syntax().text_trimmed() != binding.syntax().text_trimmed()
    {
        return Some(false);
    }

    let function = parameters.parent::<AnyJsFunction>()?;
    let parent = AnyJsExpression::cast_ref(function.syntax())?
        .outer_expression()?
        .syntax()
        .parent()?;
    let arguments = JsCallArguments::cast(parent.parent()?)?;
    let constructor = arguments.parent::<JsNewExpression>()?;
    let first = arguments.args().iter().next()?.ok()?;
    let first = first.as_any_js_expression()?.clone().omit_parentheses();
    Some(
        first.syntax() == function.syntax()
            && is_global_promise(constructor.callee().ok()?, model)?,
    )
}

fn is_global_promise(expression: AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    let expression = expression.omit_parentheses();
    let reference = expression.as_js_identifier_expression()?.name().ok()?;
    Some(
        reference.value_token().ok()?.text_trimmed() == "Promise"
            && model.binding(&reference).is_none(),
    )
}

fn could_be_error(expression: AnyJsExpression, model: &SemanticModel) -> Option<bool> {
    let mut pending: SmallVec<[AnyJsExpression; 1]> = smallvec![expression];
    while let Some(expression) = pending.pop() {
        match expression.omit_parentheses() {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let reference = identifier.name().ok()?;
                if !reference.is_undefined() || model.binding(&reference).is_some() {
                    return Some(true);
                }
            }
            AnyJsExpression::JsCallExpression(_)
            | AnyJsExpression::JsNewExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_)
            | AnyJsExpression::JsAwaitExpression(_)
            | AnyJsExpression::JsYieldExpression(_) => return Some(true),
            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    return Some(true);
                }
            }
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                match assignment.operator().ok()? {
                    JsAssignmentOperator::Assign | JsAssignmentOperator::LogicalAndAssign => {
                        pending.push(assignment.right().ok()?);
                    }
                    JsAssignmentOperator::LogicalOrAssign
                    | JsAssignmentOperator::NullishCoalescingAssign => return Some(true),
                    _ => {}
                }
            }
            AnyJsExpression::JsSequenceExpression(sequence) => pending.push(sequence.right().ok()?),
            AnyJsExpression::JsLogicalExpression(logical) => {
                pending.push(logical.right().ok()?);
                if logical.operator().ok()? != JsLogicalOperator::LogicalAnd {
                    pending.push(logical.left().ok()?);
                }
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                pending.push(conditional.alternate().ok()?);
                pending.push(conditional.consequent().ok()?);
            }
            AnyJsExpression::TsAsExpression(expression) => {
                pending.push(expression.expression().ok()?)
            }
            AnyJsExpression::TsSatisfiesExpression(expression) => {
                pending.push(expression.expression().ok()?)
            }
            AnyJsExpression::TsTypeAssertionExpression(expression) => {
                pending.push(expression.expression().ok()?)
            }
            AnyJsExpression::TsNonNullAssertionExpression(expression) => {
                pending.push(expression.expression().ok()?)
            }
            AnyJsExpression::JsBogusExpression(_) => return None,
            _ => {}
        }
    }
    Some(false)
}
