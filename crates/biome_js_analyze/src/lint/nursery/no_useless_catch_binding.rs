use crate::JsRuleAction;
use crate::lint::correctness::no_unused_variables::is_unused;
use crate::services::semantic::Semantic;
use biome_analyze::{FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsCatchDeclaration, binding_ext::AnyJsIdentifierBinding};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_useless_catch_binding::NoUselessCatchBindingOptions;

declare_lint_rule! {
    /// Disallow unused catch bindings.
    ///
    /// This rule enforces removing unnecessary catch bindings in accordance with ECMAScript 2019.
    /// See also: [Optional catch binding](https://tc39.es/proposal-optional-catch-binding)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///     // Do something
    /// } catch (unused) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch (used) {
    ///     console.error(used);
    /// }
    /// ```
    ///
    /// ```js
    /// try {
    ///     // Do something
    /// } catch {}
    /// ```
    ///
    pub NoUselessCatchBinding {
        version: "next",
        name: "noUselessCatchBinding",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoUselessCatchBinding {
    type Query = Semantic<JsCatchDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessCatchBindingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let catch_declaration = ctx.query();
        let catch_binding = catch_declaration.binding().ok()?;
        let catch_binding_ident = catch_binding
            .as_any_js_binding()?
            .as_js_identifier_binding()?;
        let catch_binding_any_ident = AnyJsIdentifierBinding::from(catch_binding_ident.clone());
        if !is_unused(ctx.model(), &catch_binding_any_ident) {
            return None;
        }
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let catch_declaration = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                catch_declaration.range(),
                markup! {
                    "This "<Emphasis>"catch binding"</Emphasis>" is unused."
                },
            )
            .note(markup! {
                "Since ECMAScript 2019, catch bindings are optional; you can omit the catch binding if you don't need it."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        let catch_declaration = ctx.query();
        mutation.remove_node(catch_declaration.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the catch binding." }.to_owned(),
            mutation,
        ))
    }
}
