use crate::{
    lint::correctness::no_invalid_builtin_instantiation::convert_new_expression_to_call_expression,
    services::semantic::Semantic, JsRuleAction,
};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsExpression, JsNewOrCallExpression,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::lint::style::use_throw_new_error::convert_call_expression_to_new_expression;

declare_lint_rule! {
    /// Enforce the use of `new` for all builtins, except `String`, `Number` and `Boolean`.
    ///
    /// `new Builtin()` and `Builtin()` work the same, but `new` should be preferred for consistency with other constructors.
    /// Enforces the use of new for following builtins:
    ///
    /// - AggregateError
    /// - Array
    /// - Date
    /// - Error
    /// - EvalError
    /// - Object
    /// - Promise
    /// - RangeError
    /// - ReferenceError
    /// - RegExp
    /// - SyntaxError
    /// - TypeError
    /// - URIError
    ///
    /// Disallows the use of `new` for following builtins:
    ///
    /// - Boolean
    /// - Number
    /// - String
    ///
    /// > These should not use `new` as that would create object wrappers for the primitive values, which is not what you want.
    /// > However, without `new` they can be useful for coercing a value to that type.
    ///
    /// Note that, builtins that require `new` to be instantiated and
    /// builtins that require no `new` to be instantiated (`Symbol` and `BigInt`) are covered by the
    /// [noInvalidBuiltinInstantiation](https://biomejs.dev/linter/rules/no-invalid-builtin-instantiation/) rule.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const text = new String(10);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const now = Date();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const text = String(10);
    /// ```
    ///
    /// ```js
    /// const now = new Date();
    /// ```
    pub UseConsistentBuiltinInstantiation {
        version: "1.7.2",
        name: "useConsistentBuiltinInstantiation",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-new-wrappers"),
            // FIXME: uncomment once we allow multiple rules to have the same source.
            //RuleSource::Eslint("no-new-native-nonconstructor"),
        ],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseConsistentBuiltinInstantiation {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseConsistentBuiltinInstantiationState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (callee, creation_rule) = extract_callee_and_rule(node)?;
        let (reference, name) = global_identifier(&callee.omit_parentheses())?;

        if creation_rule
            .forbidden_builtins_list()
            .binary_search(&name.text())
            .is_ok()
        {
            return ctx.model().binding(&reference).is_none().then_some(
                UseConsistentBuiltinInstantiationState {
                    name,
                    creation_rule,
                },
            );
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let name = state.name.text();

        let (use_this, instead_of) = match state.creation_rule {
            BuiltinCreationRule::MustUseNew => ("new ", ""),
            BuiltinCreationRule::MustNotUseNew => ("", "new "),
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>{use_this}{name}"()"</Emphasis>" instead of "<Emphasis>{instead_of}{name}"()"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        match node {
            JsNewOrCallExpression::JsNewExpression(node) => {
                let call_expression = convert_new_expression_to_call_expression(node)?;

                mutation
                    .replace_node::<AnyJsExpression>(node.clone().into(), call_expression.into());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
                    mutation,
                ))
            }
            JsNewOrCallExpression::JsCallExpression(node) => {
                let new_expression = convert_call_expression_to_new_expression(node)?;

                mutation
                    .replace_node::<AnyJsExpression>(node.clone().into(), new_expression.into());
                Some(JsRuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Add "<Emphasis>"new"</Emphasis>" keyword." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}

/// Sorted array of builtins that require new keyword.
const BUILTINS_REQUIRING_NEW: &[&str] = &[
    "AggregateError",
    "Array",
    "Date",
    "Error",
    "EvalError",
    "Object",
    "Promise",
    "RangeError",
    "ReferenceError",
    "RegExp",
    "SyntaxError",
    "TypeError",
    "URIError",
];

/// Sorted array of builtins that should not use new keyword.
const BUILTINS_NOT_REQUIRING_NEW: &[&str] = &["Boolean", "Number", "String"];

enum BuiltinCreationRule {
    MustUseNew,
    MustNotUseNew,
}

impl BuiltinCreationRule {
    fn forbidden_builtins_list(&self) -> &[&str] {
        match self {
            BuiltinCreationRule::MustUseNew => BUILTINS_REQUIRING_NEW,
            BuiltinCreationRule::MustNotUseNew => BUILTINS_NOT_REQUIRING_NEW,
        }
    }
}

pub struct UseConsistentBuiltinInstantiationState {
    name: StaticValue,
    creation_rule: BuiltinCreationRule,
}

fn extract_callee_and_rule(
    node: &JsNewOrCallExpression,
) -> Option<(AnyJsExpression, BuiltinCreationRule)> {
    let rule = match node {
        JsNewOrCallExpression::JsNewExpression(_) => BuiltinCreationRule::MustNotUseNew,
        JsNewOrCallExpression::JsCallExpression(_) => BuiltinCreationRule::MustUseNew,
    };
    let callee = node.callee().ok()?;

    Some((callee, rule))
}

#[test]
fn test_order() {
    for items in BUILTINS_REQUIRING_NEW.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in BUILTINS_NOT_REQUIRING_NEW.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
