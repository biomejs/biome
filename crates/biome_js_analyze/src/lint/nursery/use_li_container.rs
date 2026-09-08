use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsxChildList, JsxElement, jsx_ext::AnyJsxElement};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_li_container::UseLiContainerOptions;

declare_lint_rule! {
    /// Require `<li>` elements with an HTML element parent to be children of `<ul>`, `<ol>`, or `<menu>`.
    ///
    /// List items need a list container to define their relationship to the other items.
    /// Placing a list item outside a list container produces invalid HTML.
    ///
    /// List items without an HTML element parent are ignored, including standalone items,
    /// fragments, expressions, and component children. Their container may be supplied
    /// where they are rendered.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div><li>Item</li></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <ul><div><li /></div></ul>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <ul><li>Item</li></ul>
    ///     <ol><li>Item</li></ol>
    ///     <menu><li /></menu>
    /// </>
    /// ```
    ///
    /// ```jsx
    /// function Item() {
    ///     return <li>Item rendered in a list elsewhere</li>;
    /// }
    /// ```
    ///
    /// ```jsx
    /// <List><li>Component child</li></List>
    /// ```
    ///
    pub UseLiContainer {
        version: "next",
        name: "useLiContainer",
        language: "jsx",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::HtmlEslint("require-li-container").inspired()],
    }
}

const LIST_CONTAINERS: [&str; 3] = ["ul", "ol", "menu"];

impl Rule for UseLiContainer {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseLiContainerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        if name.text_trimmed() != "li" {
            return None;
        }

        let element = match node {
            AnyJsxElement::JsxOpeningElement(opening) => opening.syntax().parent()?,
            AnyJsxElement::JsxSelfClosingElement(element) => element.syntax().clone(),
        };
        let siblings = JsxChildList::cast(element.parent()?)?;
        let parent = siblings.parent::<JsxElement>()?;
        let name = parent
            .opening_element()
            .ok()?
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?;
        if LIST_CONTAINERS.contains(&name.text_trimmed()) {
            return None;
        }

        Some(node.syntax().text_trimmed_range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                *range,
                markup! {
                    "This "<Emphasis>"<li>"</Emphasis>" element is outside a list container."
                },
            )
            .note("List items need a list container to form a valid HTML list.")
            .note(markup! {
                "Make this element a direct child of "<Emphasis>"<ul>"</Emphasis>", "<Emphasis>"<ol>"</Emphasis>", or "<Emphasis>"<menu>"</Emphasis>"."
            }),
        )
    }
}
