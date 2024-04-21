use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, AnyJsExpression, JsCallExpression, JsNewExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};

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
    /// Following builtins are handled by [noInvalidBuiltin](https://biomejs.dev/linter/rules/no-invalid-new-builtin/):
    ///
    /// - String
    /// - Number
    /// - Boolean
    /// - Symbol
    /// - BigInt
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
    pub UseConsistentNewBuiltin {
        version: "next",
        name: "useConsistentNewBuiltin",
        sources: &[RuleSource::EslintUnicorn("new-for-builtins")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

/// Sorted array of builtins that require new keyword.
const BUILTINS_REQUIRING_NEW: &[&str] = &[
    "Array",
    "ArrayBuffer",
    "BigInt64Array",
    "BigUint64Array",
    "DataView",
    "Date",
    "Error",
    "FinalizationRegistry",
    "Float32Array",
    "Float64Array",
    "Function",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Map",
    "Object",
    "Promise",
    "Proxy",
    "RegExp",
    "Set",
    "SharedArrayBuffer",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "WeakMap",
    "WeakRef",
    "WeakSet",
];

impl Rule for UseConsistentNewBuiltin {
    type Query = Semantic<JsCallExpression>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let (reference, name) = global_identifier(&callee.omit_parentheses())?;
        let name_text = name.text();

        if BUILTINS_REQUIRING_NEW.binary_search(&name_text).is_ok() {
            return ctx
                .model()
                .binding(&reference)
                .is_none()
                .then_some(name_text.to_string());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use "<Emphasis>"new "{state}"()"</Emphasis>" instead of "<Emphasis>{state}"()"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let new_expression = convert_call_expression_to_new_expression(node)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia::<AnyJsExpression>(
            node.clone().into(),
            new_expression.into(),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Add "<Emphasis>"new"</Emphasis>"." }.to_owned(),
            mutation,
        })
    }
}

fn convert_call_expression_to_new_expression(expr: &JsCallExpression) -> Option<JsNewExpression> {
    let mut callee = expr.callee().ok()?;
    let leading_trivia_pieces = callee.syntax().first_leading_trivia()?.pieces();

    let new_token = make::token(JsSyntaxKind::NEW_KW)
        .with_leading_trivia_pieces(leading_trivia_pieces)
        .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);

    // Remove leading trivia from the callee.
    callee = callee.with_leading_trivia_pieces([])?;

    Some(
        make::js_new_expression(new_token.clone(), callee)
            .with_arguments(expr.arguments().ok()?)
            .build(),
    )
}

#[test]
fn test_order() {
    for items in BUILTINS_REQUIRING_NEW.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
