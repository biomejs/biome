use crate::JsRuleAction;
use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_syntax::{jsx_ext::AnyJsxElement, JsxAttribute};
use biome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce that autoFocus prop is not used on elements.
    ///
    /// Autofocusing elements can cause usability issues for sighted and non-sighted users, alike.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus="true" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={"false"} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={undefined} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input />
    ///```
    ///
    /// ```jsx
    /// <div />
    ///```
    ///
    /// ```jsx
    /// <button />
    ///```
    ///
    /// ```jsx
    /// // `autoFocus` prop in user created component is valid
    /// <MyComponent autoFocus={true} />
    ///```
    ///
    /// ## Resources
    ///
    /// - [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
    /// - [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)
    ///
    pub NoAutofocus {
        version: "1.0.0",
        name: "noAutofocus",
        source: RuleSource::EslintJsxA11y("no-autofocus"),
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAutofocus {
    type Query = Ast<AnyJsxElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_custom_component() {
            return None;
        }
        node.find_attribute_by_name("autoFocus")
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            attr.syntax().text_trimmed_range(),
            markup! {
                "Avoid the "<Emphasis>"autoFocus"</Emphasis>" attribute."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, attr: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        if attr.syntax().has_trailing_comments() {
            let prev_token = attr.syntax().first_token()?.prev_token()?;
            let new_token =
                prev_token.append_trivia_pieces(attr.syntax().last_trailing_trivia()?.pieces());
            mutation.replace_token_discard_trivia(prev_token, new_token);
        }
        mutation.remove_node(attr.clone());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"autoFocus"</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        })
    }
}
