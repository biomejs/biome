use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, JsIdentifierExpression};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Prefer using `globalThis` instead of `window`, `self`, or `global`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// window.location.href;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// global.setTimeout(() => {}, 100);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// self.postMessage('hello');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// globalThis.location.href;
    /// ```
    ///
    /// ```js
    /// globalThis.setTimeout(() => {}, 100);
    /// ```
    ///
    /// ```js
    /// globalThis.postMessage('hello');
    /// ```
    ///
    pub UseGlobalThis {
        version: "next",
        name: "useGlobalThis",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-global-this").inspired()],
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

#[derive(Debug)]
pub enum GlobalObject {
    Window,
    Self_,
    Global,
}

impl GlobalObject {
    pub fn as_str(&self) -> &'static str {
        match self {
            GlobalObject::Window => "window",
            GlobalObject::Self_ => "self",
            GlobalObject::Global => "global",
        }
    }
}

impl Rule for UseGlobalThis {
    type Query = Semantic<JsIdentifierExpression>;
    type State = GlobalObject;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?;

        match name.value_token().ok()?.text() {
            "window" | "self" | "global" => {
                // Check if this is a reference to the global object
                // We need to ensure it's not a local variable shadowing the global
                let model = ctx.model();
                let reference = node.name().ok()?;

                // If the identifier is bound to a local declaration, don't flag it
                if model.binding(&reference).is_some() {
                    return None;
                }

                // Check if it's one of the global objects we want to replace
                match name.value_token().ok()?.text() {
                    "window" => Some(GlobalObject::Window),
                    "self" => Some(GlobalObject::Self_),
                    "global" => Some(GlobalObject::Global),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let global_obj_name = state.as_str();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use "<Emphasis>"globalThis"</Emphasis>" instead of "<Emphasis>{ global_obj_name }</Emphasis>"."
                },
            )
            .note(markup! {
                "Use "<Emphasis>"globalThis"</Emphasis>" for consistent global object access across all environments."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // Create a new identifier expression with 'globalThis'
        let global_this_ident = make::js_reference_identifier(make::ident("globalThis"));
        let global_this_expr = make::js_identifier_expression(global_this_ident);

        // Replace the old identifier with 'globalThis'
        mutation.replace_node_discard_trivia(
            AnyJsExpression::from(node.clone()),
            AnyJsExpression::from(global_this_expr),
        );

        Some(JsRuleAction::new(
            ActionCategory::QuickFix("nursery/useGlobalThis".to_string().into()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"globalThis"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}