use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// The scope prop should be used only on `<th>` elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div scope={scope} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div scope="col" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <th scope={scope}></th>
    /// ```
    ///
    /// ```jsx
    /// <th scope="col"></th>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.3.1](https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships)
    /// - [WCAG 4.1.1](https://www.w3.org/WAI/WCAG21/Understanding/parsing)
    ///
    pub NoHeaderScope {
        version: "1.0.0",
        name: "noHeaderScope",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("scope")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoHeaderScope {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.is_element()
            && element.name_value_token().ok()?.text_trimmed() != "th"
            && element.has_truthy_attribute("scope")
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let element = ctx.query();
        let scope_node = element.find_attribute_by_name("scope")?;

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            scope_node.range(),
            markup! {"Avoid using the "<Emphasis>"scope"</Emphasis>" attribute on elements other than "<Emphasis>"th"</Emphasis>" elements."}
                .to_owned(),
        ).note(markup!{
            "The "<Emphasis>"scope"</Emphasis>" attribute is used to associate a data cell with its corresponding header cell in a data table,
            so it should be placed on "<Emphasis>"th"</Emphasis>" elements to provide accessibility to screen readers."
        }).note(markup!{
            "Follow the links for more information,
            "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/info-and-relationships">"WCAG 1.3.1"</Hyperlink>"
            "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/parsing">"WCAG 4.1.1"</Hyperlink>""
        });

        Some(diagnostic)
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let element = ctx.query();
        let scope_node = element.find_attribute_by_name("scope")?;

        let mut mutation = ctx.root().begin();
        mutation.remove_node(scope_node);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"scope"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
