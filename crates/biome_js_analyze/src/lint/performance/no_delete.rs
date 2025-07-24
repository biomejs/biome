use crate::JsRuleAction;
use crate::services::semantic::Semantic;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsExpression, JsComputedMemberExpressionFields,
    JsStaticMemberExpression, JsStaticMemberExpressionFields, JsUnaryExpression, JsUnaryOperator,
    T, global_identifier, is_node_imported_by_specifiers,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_delete::NoDeleteOptions;

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
    /// The rule isn't applied to `process.env`, because the `delete` operator is the recommended way by Node.js to remove environment variables from the process.
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
    /// ```js
    /// delete process.env.TEST_ENV;
    ///```
    ///
    /// ```js
    /// import { env } from "node:process";
    /// delete env.TEST_ENV;
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
    type Query = Semantic<JsUnaryExpression>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoDeleteOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
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
                if is_check_env_process(&static_member_expression, model)
                    || is_global_process(&static_member_expression, model)
                {
                    return None;
                }

                if let AnyJsExpression::JsStaticMemberExpression(static_expression) =
                    static_member_expression.object().ok()?
                {
                    let name = static_expression.member().ok()?;
                    let name = name.as_js_name()?;
                    if name.to_trimmed_text().text() == "dataset" {
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

/// Checks if static member expression is `process.`
fn is_global_process(
    static_member_expression: &JsStaticMemberExpression,
    model: &SemanticModel,
) -> bool {
    static_member_expression
        .identifier()
        .is_some_and(|identifier| {
            let is_global_identifier =
                global_identifier(&AnyJsExpression::JsIdentifierExpression(identifier.clone()));

            if let Some((identifier, value)) = is_global_identifier {
                if value.text() == "process" {
                    if model.binding(&identifier).is_none() {
                        return true;
                    }
                }
            }

            false
        })
}

/// Checks if the current static member expression is called `env` and it's imported from
/// `env` on `node:env`
fn is_check_env_process(
    static_member_expression: &JsStaticMemberExpression,
    model: &SemanticModel,
) -> bool {
    static_member_expression
        .identifier()
        .is_some_and(|identifier| {
            let Ok(identifier) = identifier.name() else {
                return false;
            };
            let Ok(member_name) = identifier.value_token() else {
                return false;
            };
            if member_name.text_trimmed() == "env" {
                if let Some(binding) = model.binding(&identifier) {
                    let syntax = binding.syntax();
                    return is_node_imported_by_specifiers(syntax, &["process", "node:process"])
                        .ok()
                        .unwrap_or_default();
                }
            }

            false
        })
}
