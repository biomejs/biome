use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::ReferencesExtensions;
use biome_js_syntax::{JsIdentifierBinding, AnyJsBindingPattern, AnyJsBinding, JsVariableDeclaration, JsVariableDeclarationClause, JsVariableDeclarator, JsVariableDeclaratorList, TsDeclareStatement};
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
    /// ```
    ///
    pub NoUnassignedVariables {
        version: "next",
        name: "noUnassignedVariables",
        language: "js",
        sources: &[RuleSource::Eslint("no-unassigned-vars")],
        recommended: false,
    }
}

impl Rule for NoUnassignedVariables {
    type Query = Semantic<JsVariableDeclarator>;
    type State = JsIdentifierBinding;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();
        let Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(id))) =
            declarator.id()
        else {
            return None;
        };
        let declaration = declarator
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?;
        if declaration.is_const() || declarator.initializer().is_some() {
            return None;
        }
        if declaration.parent::<JsVariableDeclarationClause>().is_some_and(|clause| clause.parent::<TsDeclareStatement>().is_some()) {
            return None;
        }
        let model = ctx.model();

        if id
            .all_writes(model)
            .next()
            .is_some()
        {
            return None;
        }
        if id
           .all_reads(model)
           .next()
           .is_some()
        {
            return Some(id);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        // let state = state.to_trimmed_text();
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
