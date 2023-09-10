use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_rowan::AstNode;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::JsxElement;

declare_rule! {
    /// Enforce that anchors have content and that the content is accessible to screen readers.
    ///
    /// Accessible means the content is not hidden using the `aria-hidden` attribute.
    /// Refer to the references to learn about why this is important.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a></a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a>    </a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a aria-hidden>content</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a><span aria-hidden="true">content</span></a>
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <a>content</a>
    /// ```
    ///
    /// ```jsx
    /// function html() {
    ///     return { __html: "foo" }
    /// }
    /// <a dangerouslySetInnerHTML={html()} />
    /// ```
    ///
    /// ```jsx
    /// <a><TextWrapper aria-hidden={true} />content</a>
    /// ```
    ///
    /// ```jsx
    /// <a><div aria-hidden="true"></div>content</a>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.4](https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub(crate) UseAnchorContent {
        version: "1.0.0",
        name: "useAnchorContent",
        recommended: true,
    }
}

impl Rule for UseAnchorContent {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token()?;

        if name.text_trimmed() == "a" {
            if node.has_truthy_attribute("aria-hidden") {
                return Some(());
            }

            if has_valid_anchor_content(node) {
                return None;
            }

            match node {
                AnyJsxElement::JsxOpeningElement(opening_element) => {
                    if !opening_element.has_accessible_child() {
                        return Some(());
                    }
                }
                AnyJsxElement::JsxSelfClosingElement(_) => return Some(()),
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            AnyJsxElement::JsxOpeningElement(node) => {
                node.parent::<JsxElement>()?.syntax().text_range()
            }
            AnyJsxElement::JsxSelfClosingElement(node) => node.syntax().text_trimmed_range(),
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"`a`"</Emphasis>" elements."
            }
        ).note(
            markup! {
                "All links on a page should have content that is accessible to screen readers."
            }
        ))
    }
}

/// check if the node has a valid anchor attribute
fn has_valid_anchor_content(node: &AnyJsxElement) -> bool {
    node.find_attribute_by_name("dangerouslySetInnerHTML")
        .is_some()
        || node
            .find_attribute_by_name("children")
            .map_or(false, |attribute| {
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .map_or(true, |attribute| !attribute.is_falsy())
            })
        || node.has_spread_prop()
}
