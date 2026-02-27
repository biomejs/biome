use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{HtmlAttribute, HtmlElement, HtmlSelfClosingElement};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_autofocus::NoAutofocusOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Enforce that the `autofocus` attribute is not used on elements.
    ///
    /// Autofocusing elements can cause usability issues for sighted and non-sighted users, alike.
    /// However, the `autofocus` attribute is allowed on elements inside a `dialog` element or
    /// elements with a `popover` attribute, as these are modal contexts where autofocus is expected.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <input autofocus />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <input autofocus="true" />
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <textarea autofocus>content</textarea>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <input />
    /// ```
    ///
    /// ```html
    /// <div popover><input autofocus /></div>
    /// ```
    ///
    /// ```html
    /// <dialog><input autofocus /></dialog>
    /// ```
    ///
    /// ## Resources
    ///
    /// - [WHATWG HTML Standard, The autofocus attribute](https://html.spec.whatwg.org/multipage/interaction.html#attr-fe-autofocus)
    /// - [The accessibility of HTML 5 autofocus](https://brucelawson.co.uk/2009/the-accessibility-of-html-5-autofocus/)
    /// - [MDN Web Docs, HTMLElement: autofocus property](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/autofocus)
    ///
    pub NoAutofocus {
        version: "2.4.0",
        name: "noAutofocus",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-autofocus").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoAutofocus {
    type Query = Ast<HtmlAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoAutofocusOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();

        // Check if this is an autofocus attribute
        if !is_autofocus_attribute(node) {
            return None;
        }

        // Check if element is inside a dialog or has popover attribute in ancestors
        if is_inside_allowed_context(node).unwrap_or(false) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Avoid the "<Emphasis>"autofocus"</Emphasis>" attribute."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node.clone());
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"autofocus"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

/// Check if the attribute is an autofocus attribute
fn is_autofocus_attribute(node: &HtmlAttribute) -> bool {
    node.name().is_ok_and(|name| {
        name.value_token()
            .is_ok_and(|value_token| value_token.text_trimmed().eq_ignore_ascii_case("autofocus"))
    })
}

/// Check if the element is inside an allowed context (dialog or popover)
///
/// Note: We skip the first [HtmlElement] (the one containing the autofocus attribute)
/// because we only want to check if it's *inside* a dialog/popover, not if
/// it *is* the dialog/popover itself.
fn is_inside_allowed_context(attr: &HtmlAttribute) -> Option<bool> {
    let mut skip_first_element = true;

    // Walk up the ancestors to find if we're inside a dialog or popover
    for ancestor in attr.syntax().ancestors() {
        let Some(tag_element) = get_tag_element(&ancestor) else {
            continue;
        };

        if skip_first_element {
            skip_first_element = false;
            continue;
        }

        if is_dialog_or_popover(&tag_element) {
            return Some(true);
        }
    }

    Some(false)
}

/// Extract an [AnyHtmlTagElement] from an ancestor node
fn get_tag_element(node: &biome_html_syntax::HtmlSyntaxNode) -> Option<AnyHtmlTagElement> {
    HtmlElement::cast_ref(node)
        .and_then(|e| e.opening_element().ok())
        .map(AnyHtmlTagElement::from)
        .or_else(|| {
            HtmlSelfClosingElement::cast_ref(node).map(|e| AnyHtmlTagElement::from(e.clone()))
        })
}

/// Check if the tag element is a dialog or has popover attribute
fn is_dialog_or_popover(tag_element: &AnyHtmlTagElement) -> bool {
    let is_dialog = tag_element
        .name()
        .ok()
        .and_then(|n| n.token_text_trimmed())
        .is_some_and(|text| text.eq_ignore_ascii_case("dialog"));

    is_dialog || tag_element.find_attribute_by_name("popover").is_some()
}
