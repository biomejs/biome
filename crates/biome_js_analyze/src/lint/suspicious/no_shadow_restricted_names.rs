use crate::globals::javascript::language::ES_BUILTIN;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow identifiers from shadowing restricted names.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function NaN() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let Set;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {	} catch(Object) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function Array() {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function test(JSON) {console.log(JSON)}
    /// ```
    pub NoShadowRestrictedNames {
        version: "1.0.0",
        name: "noShadowRestrictedNames",
        language: "js",
        sources: &[RuleSource::Eslint("no-shadow-restricted-names")],
        recommended: true,
        severity: Severity::Error,
    }
}

pub struct State {
    shadowed_name: String,
}

impl Rule for NoShadowRestrictedNames {
    type Query = Ast<JsIdentifierBinding>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        if ES_BUILTIN.contains(&name) {
            Some(State {
                shadowed_name: name.to_string(),
            })
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = ctx.query();

        let diag = RuleDiagnostic::new(rule_category!(),
            binding.syntax().text_trimmed_range(),
            markup! {
                "Do not shadow the global \"" {state.shadowed_name} "\" property."
            },
        )
        .note(
            markup! {"Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global."},
        );

        Some(diag)
    }
}
