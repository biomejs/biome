use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::JsVariableDeclaration;
use biome_rowan::{AstNode, BatchMutationExt, TextRange};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow initializing variables to undefined.
    ///
    /// A variable that is declared and not initialized to any value automatically gets the value of undefined.
    /// It’s considered a best practice to avoid initializing variables to undefined.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-undef-init
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = undefined;
    ///
    /// let b = undefined, c = 1, d = undefined;
    ///
    /// for (let i = 0; i < 100; i++) {
    /// 	let i = undefined;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var a = 1;
    /// class Foo {
    /// 	bar = undefined;
    /// }
    /// ```
    ///
    pub NoUselessUndefinedInitialization {
        version: "next",
        name: "noUselessUndefinedInitialization",
        sources: &[RuleSource::Eslint("no-undef-init")],
        fix_kind: FixKind::Safe,
        recommended: false,
    }
}

impl Rule for NoUselessUndefinedInitialization {
    type Query = Ast<JsVariableDeclaration>;
    type State = (String, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let let_or_var_kind = node.is_let() || node.is_var();

        let mut signals = vec![];

        if !let_or_var_kind {
            return signals;
        }

        for declarator in node.declarators() {
            let Ok(decl) = declarator else { continue };

            let Some(keyword) = decl
                .initializer()
                .map(|initializer| initializer.expression())
                .and_then(|expression| expression.ok())
                .and_then(|expression| expression.as_js_reference_identifier())
            else {
                continue;
            };

            if keyword.is_undefined() {
                let decl_range = decl.range();
                let Some(binding_name) = decl.id().ok().map(|id| id.text()) else {
                    continue;
                };
                signals.push((binding_name, decl_range));
            }
        }

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.1,
            markup! {
                "It's not necessary to initialize "<Emphasis>{state.0}</Emphasis>" to undefined."
            }).note("A variable that is declared and not initialized to any value automatically gets the value of undefined.")
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let declarators = ctx.query().declarators();

        let initializer = declarators
            .clone()
            .into_iter()
            .find(|el| {
                el.as_ref()
                    .ok()
                    .and_then(|element| element.id().ok())
                    .is_some_and(|id| id.text() == state.0)
            })
            .map(|decl| decl.ok())?
            .and_then(|declarator| declarator.initializer())?;

        let mut mutation = declarators.begin();
        mutation.remove_node(initializer);

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
            message: markup! { "Remove undefined initialization" }.to_owned(),
            mutation,
        })
    }
}
