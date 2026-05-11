use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::JsxAttribute;
use biome_js_syntax::JsxAttributeInitializerClause;
use biome_js_syntax::JsxAttributeList;
use biome_js_syntax::JsxElement;
use biome_js_syntax::JsxExpressionAttributeValue;
use biome_js_syntax::JsSyntaxKind;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_anchor_content::UseAnchorContentOptions;

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
    /// The following is valid because `<a>` is used as a JSX attribute value on a custom
    /// component. The rule is suppressed for any such prop on a custom component, as the
    /// component may render the anchor as a content wrapper whose children supply the link text.
    ///
    /// ```jsx
    /// <Button render={<a href="/home" aria-label="Home" />}>Home</Button>
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
        sources: &[RuleSource::EslintJsxA11y("anchor-has-content").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseAnchorContent {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseAnchorContentOptions;

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

            if is_jsx_attribute_anchor(node) {
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
        || node.find_attribute_by_name("innerHTML").is_some()
        || node
            .find_attribute_by_name("children")
            .is_some_and(|attribute| {
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .is_none_or(|attribute| !attribute.is_falsy())
            })
        || node.has_spread_prop()
}

/// Returns true when the `<a>` element is the value of a JSX attribute on a custom component.
///
/// A custom component may use the anchor as a content wrapper, injecting its own children into
/// it, so the final DOM can contain both the anchor's attributes and visible text — making the
/// lint check a false positive.
///
/// Handles self-closing (`<a />`), open/close (`<a></a>`), and parenthesized
/// (`render={(<a />)}`) forms. Native HTML elements are not exempted.
fn is_jsx_attribute_anchor(node: &AnyJsxElement) -> bool {
    for ancestor in node.syntax().ancestors().skip(1) {
        if let Some(attr_value) = JsxExpressionAttributeValue::cast(ancestor.clone()) {
            return is_component_attribute(&attr_value).unwrap_or(false);
        }
        match ancestor.kind() {
            // Walk up through transparent wrapper nodes:
            // - JsxElement wraps JsxOpeningElement
            // - JsxTagExpression wraps JSX elements used as JS expressions
            // - JsParenthesizedExpression for render={(<a />)}
            JsSyntaxKind::JSX_ELEMENT
            | JsSyntaxKind::JSX_TAG_EXPRESSION
            | JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {}
            _ => return false,
        }
    }
    false
}

/// Returns `Some(true)` when `attr_value` is an attribute of a custom JSX component (i.e. an
/// uppercase or member-expression name), `Some(false)` for native HTML elements, and `None`
/// when the surrounding tree is malformed.
fn is_component_attribute(attr_value: &JsxExpressionAttributeValue) -> Option<bool> {
    let initializer = JsxAttributeInitializerClause::cast(attr_value.syntax().parent()?)?;
    let attribute = JsxAttribute::cast(initializer.syntax().parent()?)?;
    let element = attribute
        .parent::<JsxAttributeList>()
        .and_then(|list| list.parent::<AnyJsxElement>())?;
    Some(element.is_custom_component())
}
