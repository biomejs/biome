use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{HtmlAttributeList, HtmlElement, VueDirective};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_vue_valid_v_memo::UseVueValidVMemoOptions;

declare_lint_rule! {
    /// Enforce valid `v-memo` Vue directives.
    ///
    /// `v-memo` takes an array of values. Vue skips updating the element and its
    /// children until one of those values changes.
    ///
    /// This rule reports `v-memo` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-memo:aaa="[x]"></div>`
    /// - The directive has a modifier. E.g. `<div v-memo.bbb="[x]"></div>`
    /// - The directive has no value, or an empty value. E.g. `<div v-memo></div>`
    /// - The directive is on an element inside another element that has `v-for`. `v-memo` doesn't work there. To use `v-memo` with `v-for`, put both on the same element.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-memo:aaa="[x]"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-memo.bbb="[x]"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-memo></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///     <div v-for="item in items">
    ///         <div v-memo="[item.id]"></div>
    ///     </div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///     <div v-memo="[valueA, valueB]"></div>
    ///     <div v-memo="dependencies"></div>
    ///     <div v-for="item in items" :key="item.id" v-memo="[item.id === selected]"></div>
    /// </template>
    /// ```
    ///
    pub UseVueValidVMemo {
        version: "next",
        name: "useVueValidVMemo",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-memo").same()],
    }
}

pub enum ViolationKind {
    InsideVFor,
    UnexpectedArgument,
    UnexpectedModifiers,
    MissingValue,
}

impl Rule for UseVueValidVMemo {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidVMemoOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if !directive
            .name_token()
            .is_ok_and(|name| name.text_trimmed() == "v-memo")
        {
            return Box::default();
        }

        let mut violations = Vec::new();
        if is_inside_v_for(directive) {
            violations.push(ViolationKind::InsideVFor);
        }
        if directive.arg().is_some() {
            violations.push(ViolationKind::UnexpectedArgument);
        }
        if !directive.modifiers().is_empty() {
            violations.push(ViolationKind::UnexpectedModifiers);
        }

        let has_value = directive
            .initializer()
            .and_then(|initializer| {
                initializer
                    .value()
                    .ok()?
                    .as_html_string()?
                    .inner_string_text()
                    .ok()
            })
            .is_some_and(|text| !text.trim().is_empty());
        if !has_value {
            violations.push(ViolationKind::MissingValue);
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        Some(match state {
            ViolationKind::InsideVFor => RuleDiagnostic::new(
                rule_category!(),
                directive.name_token().ok()?.text_trimmed_range(),
                markup! {
                    "The "<Emphasis>"v-memo"</Emphasis>" directive doesn't work inside an element with "<Emphasis>"v-for"</Emphasis>"."
                },
            )
            .note(markup! {
                "To use "<Emphasis>"v-memo"</Emphasis>" with "<Emphasis>"v-for"</Emphasis>", put both directives on the same element."
            }),
            ViolationKind::UnexpectedArgument => RuleDiagnostic::new(
                rule_category!(),
                directive.arg()?.range(),
                markup! {
                    "The "<Emphasis>"v-memo"</Emphasis>" directive doesn't accept an argument."
                },
            )
            .note(markup! {
                "Remove the argument."
            }),
            ViolationKind::UnexpectedModifiers => RuleDiagnostic::new(
                rule_category!(),
                directive.modifiers().syntax().text_trimmed_range(),
                markup! {
                    "The "<Emphasis>"v-memo"</Emphasis>" directive doesn't support modifiers."
                },
            )
            .note(markup! {
                "Remove the modifiers."
            }),
            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-memo"</Emphasis>" directive is missing a value."
                },
            )
            .note(markup! {
                "Vue skips updating this element until one of the values in the "<Emphasis>"v-memo"</Emphasis>" array changes."
            })
            .note(markup! {
                "Add an array of these values, for example "<Emphasis>"v-memo=\"[item.id, selected]\""</Emphasis>"."
            }),
        })
    }
}

/// Returns `true` when an element around the element of `directive` has a
/// `v-for` directive.
fn is_inside_v_for(directive: &VueDirective) -> bool {
    let Some(tag) = directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyHtmlTagElement::cast)
    else {
        return false;
    };
    // The opening tag sits inside its element, so start above that element.
    let element = match &tag {
        AnyHtmlTagElement::HtmlOpeningElement(opening) => opening.syntax().parent(),
        AnyHtmlTagElement::HtmlSelfClosingElement(element) => Some(element.syntax().clone()),
    };
    element.is_some_and(|element| {
        element
            .ancestors()
            .skip(1)
            .filter_map(HtmlElement::cast)
            .any(|ancestor| {
                ancestor
                    .opening_element()
                    .is_ok_and(|opening| has_v_for(&opening.attributes()))
            })
    })
}

fn has_v_for(attributes: &HtmlAttributeList) -> bool {
    attributes.iter().any(|attribute| {
        attribute
            .as_any_vue_directive()
            .and_then(|directive| directive.as_vue_directive())
            .is_some_and(VueDirective::is_for)
    })
}
