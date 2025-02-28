use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsComputedMemberExpressionFields,
    JsStaticMemberExpressionFields, JsUnaryExpression, JsUnaryOperator, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow the use of the `delete` operator.
    ///
    /// The `delete` operator enables the removal of a property from an object.
    ///
    /// The `delete` operator should be avoided because it [can prevent some optimizations of _JavaScript_ engines](https://webkit.org/blog/10298/inline-caching-delete/).
    /// Moreover, it can lead to unexpected results.
    /// For instance, deleting an array element [does not change the length of the array](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/delete#deleting_array_elements).
    ///
    /// The only legitimate use of `delete` is on an object that behaves like a _map_.
    /// To allow this pattern, this rule does not report `delete` on computed properties that are not literal values.
    /// Consider using [Map](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map) instead of an object.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const arr = [1, 2, 3];
    /// delete arr[0];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const obj = {a: {b: {c: 123}}};
    /// delete obj.a.b.c;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = new Set([1,2,3]);
    /// foo.delete(1);
    ///```
    ///
    /// ```js
    /// const map = Object.create(null);
    /// const key = "key"
    /// map[key] = "value"
    /// delete map[key];
    ///```
    ///
    /// ```js
    /// let x = 5;
    /// delete f(); // uncovered by this rule.
    ///```
    ///
    pub NoDelete {
        version: "1.0.0",
        name: "noDelete",
        language: "js",
        severity: Severity::Warning,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoDelete {
    type Query = Ast<JsUnaryExpression>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let op = node.operator().ok()?;
        if op != JsUnaryOperator::Delete {
            return None;
        }
        let argument = node.argument().ok()?;

        let should_report = if let Some(computed) = argument.as_js_computed_member_expression() {
            // `delete record[x]` is allowed, but if `x` is a literal value.
            computed
                .member()
                .ok()?
                .as_any_js_literal_expression()
                .is_some()
        } else {
            let static_member_expression = argument.as_js_static_member_expression();
            if let Some(static_member_expression) = static_member_expression {
                if let AnyJsExpression::JsStaticMemberExpression(static_expression) =
                    static_member_expression.object().ok()?
                {
                    let name = static_expression.member().ok()?;
                    let name = name.as_js_name()?;
                    if name.to_trimmed_string() == "dataset" {
                        return None;
                    }
                }
                true
            } else {
                // if `argument` is not a computed or static member,
                // then `delete` has either no effect or an undefined behavior.
                // This should be rejected by another rule.
                false
            }
        };
        should_report.then_some(argument)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Avoid the "<Emphasis>"delete"</Emphasis>" operator which can impact performance."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, argument: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let assignment = to_assignment(argument).ok()?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::from(node.clone()),
            AnyJsExpression::from(make::js_assignment_expression(
                AnyJsAssignmentPattern::AnyJsAssignment(assignment),
                make::token_decorated_with_space(T![=]),
                AnyJsExpression::from(make::js_identifier_expression(
                    make::js_reference_identifier(make::ident("undefined")),
                )),
            )),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use an "<Emphasis>"undefined"</Emphasis>" assignment instead." }.to_owned(),
            mutation,
        ))
    }
}

fn to_assignment(expr: &AnyJsExpression) -> Result<AnyJsAssignment, ()> {
    match expr {
        AnyJsExpression::JsStaticMemberExpression(expr) if !expr.is_optional_chain() => {
            let JsStaticMemberExpressionFields {
                object,
                operator_token,
                member,
            } = expr.as_fields();
            Ok(AnyJsAssignment::from(make::js_static_member_assignment(
                object.map_err(drop)?,
                operator_token.map_err(drop)?,
                member.map_err(drop)?,
            )))
        }
        AnyJsExpression::JsComputedMemberExpression(expr) if !expr.is_optional_chain() => {
            let JsComputedMemberExpressionFields {
                object,
                optional_chain_token: _,
                l_brack_token,
                member,
                r_brack_token,
            } = expr.as_fields();
            Ok(AnyJsAssignment::from(make::js_computed_member_assignment(
                object.map_err(drop)?,
                l_brack_token.map_err(drop)?,
                member.map_err(drop)?,
                r_brack_token.map_err(drop)?,
            )))
        }
        _ => Err(()),
    }
}
