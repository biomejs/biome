use crate::aria_services::Aria;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

declare_rule! {
    /// Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.
    ///
    /// `aria-activedescendant` is used to manage to focus within a [composite widget].
    /// The element with the attribute `aria-activedescendant` retains the active document focus.
    ///
    /// It indicates which of its child elements has a secondary focus by assigning the ID of that
    /// element to the value of `aria-activedescendant`. This pattern is used to build a widget
    /// like a search typeahead select list. The search input box retains document focus
    /// so that the user can type in the input. If the down arrow key is pressed and
    /// a search suggestion is highlighted, the ID of the suggestion element will be applied
    /// as the value of `aria-activedescendant` on the input element.
    ///
    /// Because an element with `aria-activedescendant` must be tabbable,
    /// it must either have an inherent tabIndex of zero or declare a tabIndex attribute.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-activedescendant-has-tabindex.md
    ///
    /// [Composite widget](https://www.w3.org/TR/wai-aria/#composite)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div aria-activedescendant={someID} />
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <div aria-activedescendant={someID} tabIndex={0} />
    /// ```
    ///
    /// ```jsx
    /// <input aria-activedescendant={someID} />
    /// ```
    ///
    pub(crate) UseAriaActivedescendantWithTabindex {
        version: "next",
        name: "useAriaActivedescendantWithTabindex",
        recommended: false,
    }
}

impl Rule for UseAriaActivedescendantWithTabindex {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let attributes = ctx.extract_attributes(&node.attributes());

        if node.is_element()
            && aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes)
            && node
                .find_attribute_by_name("aria-activedescendant")
                .is_some()
            && node.find_attribute_by_name("tabIndex").is_none()
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Enforce elements with aria-activedescendant are tabbable."
                },
            )
            .note(markup! {
                "Add the tabIndex attribute to the element with a value greater than or equal to -1."
            }),
        )
    }
}
