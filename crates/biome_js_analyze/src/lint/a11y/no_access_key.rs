use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{jsx_ext::AnyJsxElement, JsxAttribute, JsxAttributeList};
use biome_rowan::{AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Enforce that the `accessKey` attribute is not used on any HTML element.
    ///
    /// The `accessKey` assigns a keyboard shortcut to the current element. However, the `accessKey` value
    /// can conflict with keyboard commands used by screen readers and keyboard-only users, which leads to
    /// inconsistent keyboard actions across applications. To avoid accessibility complications,
    /// this rule suggests users remove the `accessKey` attribute on elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input type="submit" accessKey="s" value="Submit" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="https://webaim.org/" accessKey="w">WebAIM.org</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button accessKey="n">Next</button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)
    /// - [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)
    ///
    pub NoAccessKey {
        version: "1.0.0",
        name: "noAccessKey",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("no-access-key")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAccessKey {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.name_value_token().ok()?.text_trimmed() != "accessKey" {
            return None;
        }

        let element = node
            .parent::<JsxAttributeList>()
            .and_then(|list| list.parent::<AnyJsxElement>())?;

        // We do not know if the `accessKey` prop is used for HTML elements
        // or for user-created React components
        if element.is_custom_component() {
            return None;
        }

        let attribute_value = node.initializer()?.value().ok()?;
        if attribute_value.is_value_null_or_undefined() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Avoid the "<Emphasis>"accessKey"</Emphasis>" attribute to reduce inconsistencies between \
                    keyboard shortcuts and screen reader keyboard comments."
                },
            ).note(
                markup! {
                    "Assigning keyboard shortcuts using the "<Emphasis>"accessKey"</Emphasis>" attribute leads to \
                    inconsistent keyboard actions across applications."
                },
            )
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"accessKey"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
