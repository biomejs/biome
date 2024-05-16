use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsPropertyModifier, JsPropertyClassMember, JsRegexLiteralExpression};
use biome_rowan::{AstNode, AstNodeList};

use crate::services::control_flow::AnyJsControlFlowRoot;

declare_rule! {
    /// Require all regex literals to be declared at the top level.
    ///
    /// This rule is useful to avoid performance issues when using regex literals inside functions called many times (hot paths). Regex literals create a new RegExp object when they are evaluated. (See https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp) By declaring them at the top level, this overhead can be avoided.
    ///
    /// It's important to note that this rule is not recommended for all cases. Placing regex literals at the top level can hurt startup times. In browser contexts, this can result in longer page loads.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function foo(someString) {
    ///     return /[a-Z]*/.test(someString)
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const REGEX = /[a-Z]*/;
    ///
    /// function foo(someString) {
    ///     return REGEX.test(someString)
    /// }
    /// ```
    ///
    pub UseTopLevelRegex {
        version: "next",
        name: "useTopLevelRegex",
        language: "js",
        recommended: false,
    }
}

impl Rule for UseTopLevelRegex {
    type Query = Ast<JsRegexLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let regex = ctx.query();
        let found_all_allowed = regex.syntax().ancestors().all(|node| {
            if let Some(node) = AnyJsControlFlowRoot::cast_ref(&node) {
                matches!(
                    node,
                    AnyJsControlFlowRoot::JsStaticInitializationBlockClassMember(_)
                        | AnyJsControlFlowRoot::TsModuleDeclaration(_)
                        | AnyJsControlFlowRoot::JsModule(_)
                        | AnyJsControlFlowRoot::JsScript(_)
                )
            } else if let Some(node) = JsPropertyClassMember::cast_ref(&node) {
                node.modifiers()
                    .iter()
                    .any(|modifier| matches!(modifier, AnyJsPropertyModifier::JsStaticModifier(_)))
            } else {
                true
            }
        });
        if found_all_allowed {
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This regex literal is not defined in the top level scope. This can lead to performance issues if this function is called frequently."
                },
            )
            .note(markup! {
                "Move the regex literal outside of this scope, and place it at the top level of this module, as a constant."
            }),
        )
    }
}
