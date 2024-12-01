use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{JsSyntaxToken, TsModuleDeclaration, T};
use biome_rowan::BatchMutationExt;

use crate::JsRuleAction;

declare_lint_rule! {
    /// Require using the `namespace` keyword over the `module` keyword to declare TypeScript namespaces.
    ///
    /// TypeScript historically allowed a code organization called _namespace_.
    /// [_ECMAScript modules_ are preferred](https://www.typescriptlang.org/docs/handbook/2/modules.html#typescript-namespaces) (`import` / `export`).
    ///
    /// For projects still using _namespaces_, it's preferred to use the `namespace` keyword instead of the `module` keyword.
    /// The `module` keyword is deprecated to avoid any confusion with the _ECMAScript modules_ which are often called _modules_.
    ///
    /// Note that TypeScript `module` declarations to describe external APIs (`declare module "foo" {}`) are still allowed.
    ///
    /// See also: https://www.typescriptlang.org/docs/handbook/namespaces-and-modules.html
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// module Example {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// namespace Example {}
    /// ```
    ///
    /// ```ts
    /// declare module "foo" {}
    /// ```
    ///
    pub UseNamespaceKeyword {
        version: "1.0.0",
        name: "useNamespaceKeyword",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("prefer-namespace-keyword")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseNamespaceKeyword {
    type Query = Ast<TsModuleDeclaration>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ts_module = ctx.query();
        let token = ts_module.module_or_namespace().ok()?;
        ts_module.is_module().ok()?.then_some(token)
    }

    fn diagnostic(_: &RuleContext<Self>, module_token: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            module_token.text_trimmed_range(),
            markup! {
                "Use the "<Emphasis>"namespace"</Emphasis>" keyword instead of the outdated "<Emphasis>"module"</Emphasis>" keyword."
            },
        ).note(markup! {
            "The "<Emphasis>"module"</Emphasis>" keyword is deprecated to avoid any confusion with the "<Emphasis>"ECMAScript modules"</Emphasis>" which are often called "<Emphasis>"modules"</Emphasis>"."
        }))
    }

    fn action(ctx: &RuleContext<Self>, module_token: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.replace_token_transfer_trivia(module_token.clone(), make::token(T![namespace]));
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Use "<Emphasis>"namespace"</Emphasis>" instead."}.to_owned(),
            mutation,
        ))
    }
}
