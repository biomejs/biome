use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_access_key::NoAccessKeyOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce that the `accesskey` attribute is not used on any HTML element.
    ///
    /// The `accesskey` assigns a keyboard shortcut to the current element. However, the `accesskey` value
    /// can conflict with keyboard commands used by screen readers and keyboard-only users, which leads to
    /// inconsistent keyboard actions across applications. To avoid accessibility complications,
    /// this rule suggests users remove the `accesskey` attribute on elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input type="submit" accesskey="s" value="Submit" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <a href="https://webaim.org/" accesskey="w">WebAIM.org</a>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <button accesskey="n">Next</button>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WebAIM: Keyboard Accessibility - Accesskey](https://webaim.org/techniques/keyboard/accesskey#spec)
    /// - [MDN `accesskey` documentation](https://developer.mozilla.org/docs/Web/HTML/Global_attributes/accesskey)
    ///
    pub NoAccessKey {
        version: "next",
        name: "noAccessKey",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-access-key").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAccessKey {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAccessKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        if is_accesskey_attribute(node) {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Avoid the "<Emphasis>"accesskey"</Emphasis>" attribute to reduce inconsistencies between \
                    keyboard shortcuts and screen reader keyboard commands."
                },
            ).note(
                markup! {
                    "Assigning keyboard shortcuts using the "<Emphasis>"accesskey"</Emphasis>" attribute leads to \
                    inconsistent keyboard actions across applications."
                },
            )
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"accesskey"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

fn is_accesskey_attribute(node: &HtmlAttribute) -> bool {
    node.name().is_ok_and(|name| {
        name.value_token()
            .is_ok_and(|value_token| value_token.text_trimmed() == "accesskey")
    })
}
