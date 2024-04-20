use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{self, token_decorated_with_space};
use biome_js_syntax::{
    global_identifier, AnyJsExpression, JsCallExpression, JsNewExpression, JsSyntaxKind,
};
use biome_rowan::{chain_trivia_pieces, declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce the use of new for all builtins, except String, Number, Boolean, Symbol and BigInt.
    ///
    /// They work the same, but new should be preferred for consistency with other constructors.
    /// Enforces the use of new for following builtins:
    ///
    /// - Object
    /// - Array
    /// - ArrayBuffer
    /// - BigInt64Array
    /// - BigUint64Array
    /// - DataView
    /// - Date
    /// - Error
    /// - Float32Array
    /// - Float64Array
    /// - Function
    /// - Int8Array
    /// - Int16Array
    /// - Int32Array
    /// - Map
    /// - WeakMap
    /// - Set
    /// - WeakSet
    /// - Promise
    /// - RegExp
    /// - Uint8Array
    /// - Uint16Array
    /// - Uint32Array
    /// - Uint8ClampedArray
    /// - SharedArrayBuffer
    /// - Proxy
    /// - WeakRef
    /// - FinalizationRegistry
    ///
    /// Disallows the use of new for following builtins.
    ///
    /// - String
    /// - Number
    /// - Boolean
    /// - Symbol
    /// - BigInt
    ///
    /// > These should not use new as that would create object wrappers for the primitive values, which is not what you want. However, without new they can be useful for coercing a value to that type.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const list = Array(10);
    /// const now = Date();
    /// const map = Map([
    ///   ['foo', 'bar']
    /// ]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const list = new Array(10);
    /// const now = new Date();
    /// const map = new Map([
    ///  ['foo', 'bar']
    /// ]);
    /// ```
    ///
    pub UseNewForBuiltins {
        version: "next",
        name: "useNewForBuiltins",
        sources: &[RuleSource::EslintUnicorn("new-for-builtins")],
        recommended: false,
    }
}

const BUILTINS_REQUIRING_NEW: &[&str] = &[
    "Object",
    "Array",
    "ArrayBuffer",
    "BigInt64Array",
    "BigUint64Array",
    "DataView",
    "Date",
    "Error",
    "Float32Array",
    "Float64Array",
    "Function",
    "Int8Array",
    "Int16Array",
    "Int32Array",
    "Map",
    "WeakMap",
    "Set",
    "WeakSet",
    "Promise",
    "RegExp",
    "Uint8Array",
    "Uint16Array",
    "Uint32Array",
    "Uint8ClampedArray",
    "SharedArrayBuffer",
    "Proxy",
    "WeakRef",
    "FinalizationRegistry",
];

const BUILTINS_NOT_REQUIRING_NEW: &[&str] = &["String", "Number", "Boolean", "Symbol", "BigInt"];

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

pub struct UseNewForBuiltinsState {
    name: String,
    creation_rule: BuiltinCreationRule,
}

declare_node_union! {
    pub JsNewOrCallExpression = JsNewExpression | JsCallExpression
}

impl Rule for UseNewForBuiltins {
    type Query = Semantic<JsNewOrCallExpression>;
    type State = UseNewForBuiltinsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let (callee, creation_rule) = extract_callee_and_rule(node)?;

        let (reference, name) = global_identifier(&callee.omit_parentheses())?;
        let name_text = name.text();

        if creation_rule.forbidden_builtins_list().contains(&name_text) {
            return ctx
                .model()
                .binding(&reference)
                .is_none()
                .then_some(UseNewForBuiltinsState {
                    name: name_text.to_string(),
                    creation_rule: creation_rule,
                });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node: &JsNewOrCallExpression = ctx.query();

        let name = &state.name;

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

        match node {
            JsNewOrCallExpression::JsNewExpression(node) => action_remove_new(ctx, node),
            JsNewOrCallExpression::JsCallExpression(_) => None,
        }
    }
}

fn extract_callee_and_rule(
    node: &JsNewOrCallExpression,
) -> Option<(AnyJsExpression, BuiltinCreationRule)> {
    match node {
        JsNewOrCallExpression::JsNewExpression(node) => {
            let callee = node.callee().ok()?;

            Some((callee, BuiltinCreationRule::MustNotUseNew))
        }
        JsNewOrCallExpression::JsCallExpression(node) => {
            let callee: AnyJsExpression = node.callee().ok()?;

            Some((callee, BuiltinCreationRule::MustUseNew))
        }
    }
}

fn convert_new_expression_to_call_expression(expr: &JsNewExpression) -> Option<JsCallExpression> {
    let new_token = expr.new_token().ok()?;
    let mut callee = expr.callee().ok()?;
    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        callee = callee.prepend_trivia_pieces(chain_trivia_pieces(
            new_token.leading_trivia().pieces(),
            new_token.trailing_trivia().pieces(),
        ))?;
    }
    Some(make::js_call_expression(callee, expr.arguments()?).build())
}

fn action_remove_new(
    ctx: &RuleContext<UseNewForBuiltins>,
    node: &JsNewExpression,
) -> Option<JsRuleAction> {
    let call_expression = convert_new_expression_to_call_expression(node)?;
    let mut mutation = ctx.root().begin();
    mutation.replace_node::<AnyJsExpression>(node.clone().into(), call_expression.into());
    Some(JsRuleAction {
        category: ActionCategory::QuickFix,
        applicability: Applicability::MaybeIncorrect,
        message: markup! { "Remove "<Emphasis>"new"</Emphasis>"." }.to_owned(),
        mutation,
    })
}

fn _convert_call_expression_to_new_expression(expr: &JsCallExpression) -> Option<JsNewExpression> {
    let new_token = token_decorated_with_space(JsSyntaxKind::NEW_KW);

    let mut callee = expr.callee().ok()?;
    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        callee = callee.prepend_trivia_pieces(chain_trivia_pieces(
            new_token.leading_trivia().pieces(),
            new_token.trailing_trivia().pieces(),
        ))?;
    }

    // TODO. make::js_new_expression currently does not accept arguments.
    // To fix that js.ungram needs to be updated, but that breaks a lot of rules.
    Some(make::js_new_expression(new_token, callee).build())
}

fn _action_add_new(
    ctx: &RuleContext<UseNewForBuiltins>,
    node: &JsCallExpression,
) -> Option<JsRuleAction> {
    let new_expression = _convert_call_expression_to_new_expression(node)?;
    let mut mutation = ctx.root().begin();
    mutation.replace_node::<AnyJsExpression>(node.clone().into(), new_expression.into());
    Some(JsRuleAction {
        category: ActionCategory::QuickFix,
        applicability: Applicability::MaybeIncorrect,
        message: markup! { "Add "<Emphasis>"new"</Emphasis>"." }.to_owned(),
        mutation,
    })
}
