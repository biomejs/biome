use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::no_shadow_restricted_names::NoShadowRestrictedNamesOptions;

declare_lint_rule! {
    /// Disallow identifiers from shadowing restricted names.
    ///
    /// See also: [`noShadow`](http://biomejs.dev/linter/rules/no-shadow)
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
        sources: &[RuleSource::Eslint("no-shadow-restricted-names").same()],
        recommended: true,
        severity: Severity::Error,
    }
}

pub struct State {
    shadowed_name: TokenText,
}

impl Rule for NoShadowRestrictedNames {
    type Query = Ast<JsIdentifierBinding>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoShadowRestrictedNamesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let binding = ctx.query();
        let name = binding.name_token().ok()?;
        let name = name.text_trimmed();

        // should this also cover web/node js globals?
        if crate::globals::is_js_language_global(name) {
            Some(State {
                shadowed_name: binding.name_token().ok()?.token_text_trimmed(),
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
                "Do not shadow the global \"" {state.shadowed_name.text()} "\" property."
            },
        )
        .note(
            markup! {"Consider renaming this variable. It's easy to confuse the origin of variables when they're named after a known global."},
        );

        Some(diag)
    }
}
