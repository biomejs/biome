use crate::{services::aria::Aria, JsRuleAction};
use biome_analyze::{
    context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{
    jsx_attribute, jsx_attribute_initializer_clause, jsx_attribute_list, jsx_ident, jsx_name,
    jsx_string, jsx_string_literal, token,
};
use biome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsxAttribute, AnyJsxAttributeName, AnyJsxAttributeValue,
    JsxAttributeList, T,
};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt, TriviaPieceKind};

declare_lint_rule! {
    /// Enforce that `tabIndex` is assigned to non-interactive HTML elements with `aria-activedescendant`.
    ///
    /// `aria-activedescendant` is used to manage to focus within a [composite widget](https://www.w3.org/TR/wai-aria/#composite).
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
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div aria-activedescendant={someID} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div aria-activedescendant={someID} tabIndex={0} />
    /// ```
    ///
    /// ```jsx
    /// <input aria-activedescendant={someID} />
    /// ```
    ///
    pub UseAriaActivedescendantWithTabindex {
        version: "1.3.0",
        name: "useAriaActivedescendantWithTabindex",
        language: "jsx",
        sources: &[RuleSource::EslintJsxA11y("aria-activedescendant-has-tabindex")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseAriaActivedescendantWithTabindex {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_element()
            && ctx.aria_roles().is_not_interactive_element(node)
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
                "Enforce elements with aria-activedescendant are tabbable."
            )
            .note(
                "aria-activedescendant is used to manage focus within a composite widget.\nThe element with the attribute aria-activedescendant retains the active document focus."
            ).note(
                "Add the tabIndex attribute to the element with a value greater than or equal to -1."
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let descendant_attribute = node.find_attribute_by_name("aria-activedescendant")?;

        let old_attribute_list = descendant_attribute
            .syntax()
            .ancestors()
            .find_map(JsxAttributeList::cast)?;

        let new_attribute = jsx_attribute(AnyJsxAttributeName::JsxName(jsx_name(
            jsx_ident("tabIndex").with_leading_trivia([(TriviaPieceKind::Whitespace, " ")]),
        )))
        .with_initializer(jsx_attribute_initializer_clause(
            token(T![=]),
            AnyJsxAttributeValue::JsxString(jsx_string(jsx_string_literal("0"))),
        ))
        .build();

        let mut new_attribute_list: Vec<_> = old_attribute_list.iter().collect();
        new_attribute_list.push(AnyJsxAttribute::JsxAttribute(new_attribute));

        mutation.replace_node(old_attribute_list, jsx_attribute_list(new_attribute_list));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Add the "<Emphasis>"tabIndex"</Emphasis>" attribute." }.to_owned(),
            mutation,
        ))
    }
}
