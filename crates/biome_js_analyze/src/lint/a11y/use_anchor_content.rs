use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_js_syntax::JsxElement;
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
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
    /// ### Valid
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
    pub UseAnchorContent {
        version: "1.0.0",
        name: "useAnchorContent",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("anchor-has-content")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseAnchorContent {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token().ok()?;

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
            AnyJsxElement::JsxOpeningElement(node) => node
                .parent::<JsxElement>()?
                .syntax()
                .text_range_with_trivia(),
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
        ).note(
            markup! {
                "Accessible content refers to digital content that is designed and structured in a way that makes it easy for people with disabilities to access, understand, and interact with using assistive technologies."
            }
        ).note(
            markup! {
                "Follow these links for more information,\n "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/link-purpose-in-context">"WCAG 2.4.4"</Hyperlink>"\n "<Hyperlink href="https://www.w3.org/WAI/WCAG21/Understanding/name-role-value">"WCAG 4.1.2"</Hyperlink>""
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        if node.has_truthy_attribute("aria-hidden") {
            let aria_hidden = node.find_attribute_by_name("aria-hidden")?;
            mutation.remove_node(aria_hidden);

            return Some(JsRuleAction::new(
                ctx.metadata().action_category(ctx.category(), ctx.group()),
                ctx.metadata().applicability(),
                 markup! { "Remove the "<Emphasis>"aria-hidden"</Emphasis>" attribute to allow the anchor element and its content visible to assistive technologies." }.to_owned(),
                mutation,
            ));
        }
        None
    }
}

/// check if the node has a valid anchor attribute
fn has_valid_anchor_content(node: &AnyJsxElement) -> bool {
    node.find_attribute_by_name("dangerouslySetInnerHTML")
        .is_some()
        || node
            .find_attribute_by_name("children")
            .is_some_and(|attribute| {
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .map_or(true, |attribute| !attribute.is_falsy())
            })
        || node.has_spread_prop()
}
