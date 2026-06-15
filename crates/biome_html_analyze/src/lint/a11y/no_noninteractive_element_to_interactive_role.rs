use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_aria_metadata::AriaRole;
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{HtmlSyntaxKind, T};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, BatchMutationExt, TextRange, TokenText};
use biome_rule_options::no_noninteractive_element_to_interactive_role::NoNoninteractiveElementToInteractiveRoleOptions;

use crate::{Aria, HtmlRuleAction};

declare_lint_rule! {
    /// Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
    ///
    /// Non-interactive HTML elements indicate _content_ and _containers_ in the user interface.
    /// Non-interactive elements include `<main>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// Interactive HTML elements indicate _controls_ in the user interface.
    /// Interactive elements include `<a href>`, `<area>`, `<button>`, `<input>`, `<select>`, `<textarea>`.
    ///
    /// [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert a non-interactive element to an interactive element.
    /// Interactive ARIA roles include `button`, `link`, `checkbox`, `menuitem`, `menuitemcheckbox`, `menuitemradio`, `option`, `radio`, `searchbox`, `switch` and `textbox`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 role="button">Some text</h1>
    /// ```
    ///
    /// ### Valid
    ///
    ///
    /// ```jsx
    /// <span role="button">Some text</span>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ### Resources
    ///
    /// - [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro)
    /// - [WAI-ARIA Authoring Practices Guide - Design Patterns and Widgets](https://www.w3.org/TR/wai-aria-practices-1.1/#aria_ex)
    /// - [Fundamental Keyboard Navigation Conventions](https://www.w3.org/TR/wai-aria-practices-1.1/#kbd_generalnav)
    /// - [Mozilla Developer Network - ARIA Techniques](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_button_role#Keyboard_and_focus)
    ///
    pub NoNoninteractiveElementToInteractiveRole {
        version: "2.5.0",
        name: "noNoninteractiveElementToInteractiveRole",
        language: "html",
        sources: &[RuleSource::EslintJsxA11y("no-noninteractive-element-to-interactive-role").inspired()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoNoninteractiveElementToInteractiveRole {
    type Query = Aria<AnyHtmlTagElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = NoNoninteractiveElementToInteractiveRoleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_custom_component() {
            return None;
        }

        let tag_kind = node.tag_name_kind();

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_static_value = role_attribute
            .initializer()?
            .value()
            .ok()?
            .as_static_value()?;
        let role_attribute_value = role_attribute_static_value.text();

        // Exception: role `treeitem` is allowed on `<li>`
        // Reason: `treeitem` has the superclass role `listitem`, which means it is made to be used on `<li>`
        // Ref: https://w3c.github.io/aria/#treeitem
        // Ref: https://www.w3.org/WAI/ARIA/apg/patterns/treeview/examples/treeview-1a/
        if tag_kind == Some(T![li]) && role_attribute_value == "treeitem" {
            return None;
        }

        if ctx.aria_roles().is_not_interactive_element(node)
            && AriaRole::from_roles(role_attribute_value).is_some_and(|role| role.is_interactive())
        {
            // <div> and <span> are considered neither interactive nor non-interactive, depending on the presence or absence of the role attribute.
            // We don't report <div> and <span> here, because we cannot determine whether they are interactive or non-interactive.
            if tag_kind.is_some_and(|kind| ROLE_SENSITIVE_ELEMENTS.contains(kind)) {
                return None;
            }

            return Some(RuleState {
                attribute_range: role_attribute.range(),
                element_name: node.tag_name()?,
            });
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let element_name = state.element_name.text();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "The HTML element "<Emphasis>{{element_name}}</Emphasis>" is non-interactive and should not have an interactive role."
            },
        ).note(
            markup!{
                "Replace "<Emphasis>{{element_name}}</Emphasis>" with a div or a span."
            }
        ))
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let role_attribute = node.find_attribute_by_name("role")?;

        let mut mutation = ctx.root().begin();
        mutation.remove_node(role_attribute);
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}

/// `<div>` and `<span>` are neither interactive nor non-interactive without a
/// role, so they are not reported here.
const ROLE_SENSITIVE_ELEMENTS: TokenSet<HtmlSyntaxKind> = token_set!(T![div], T![span]);

pub struct RuleState {
    attribute_range: TextRange,
    element_name: TokenText,
}
