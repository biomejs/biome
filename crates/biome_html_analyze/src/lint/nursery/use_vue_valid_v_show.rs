use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{AnyHtmlAttributeInitializer, T, VueDirective, inner_string_text};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_vue_valid_v_show::UseVueValidVShowOptions;

declare_lint_rule! {
    /// Enforce valid `v-show` directives.
    ///
    /// The `v-show` directive shows or hides an element depending on whether its value is true or false.
    /// It takes only a value, and it must be placed on an element that appears on the page.
    ///
    /// This rule reports `v-show` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-show:aaa="foo"></div>`
    /// - The directive has a modifier. E.g. `<div v-show.bbb="foo"></div>`
    /// - The directive does not have a value. E.g. `<div v-show></div>`
    /// - The directive is on a `<template>` element. E.g. `<template v-show="foo"></template>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-show:aaa="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-show.bbb="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-show></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template v-show="foo"><div></div></template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-show="isVisible"></div>
    /// <template v-if="isVisible"><div></div></template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `v-show` directive](https://vuejs.org/api/built-in-directives.html#v-show)
    /// - [Conditional rendering with `v-show`](https://vuejs.org/guide/essentials/conditional.html#v-show)
    pub UseVueValidVShow {
        version: "next",
        name: "useVueValidVShow",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-show").same()],
    }
}

#[derive(Clone, Copy)]
pub enum ViolationKind {
    UnexpectedArgument,
    UnexpectedModifier,
    MissingValue,
    OnTemplate,
}

impl Rule for UseVueValidVShow {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidVShowOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if !directive.is_show() {
            return Box::default();
        }

        let mut violations = Vec::new();
        if directive.arg().is_some() {
            violations.push(ViolationKind::UnexpectedArgument);
        }
        if !directive.modifiers().is_empty() {
            violations.push(ViolationKind::UnexpectedModifier);
        }
        if is_missing_value(directive) {
            violations.push(ViolationKind::MissingValue);
        }
        if is_on_template(directive) {
            violations.push(ViolationKind::OnTemplate);
        }
        violations.into_boxed_slice()
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        Some(match state {
            ViolationKind::UnexpectedArgument => RuleDiagnostic::new(
                rule_category!(),
                directive.arg()?.range(),
                markup! {
                    "The "<Emphasis>"v-show"</Emphasis>" directive doesn't accept an argument."
                },
            )
            .note(markup! {
                <Emphasis>"v-show"</Emphasis>" only uses its value to decide whether to show the element, so the argument does nothing."
            })
            .note(markup! {
                "Remove the argument, like "<Emphasis>"v-show=\"isVisible\""</Emphasis>"."
            }),
            ViolationKind::UnexpectedModifier => RuleDiagnostic::new(
                rule_category!(),
                directive.modifiers().range(),
                markup! {
                    "The "<Emphasis>"v-show"</Emphasis>" directive doesn't accept modifiers."
                },
            )
            .note(markup! {
                <Emphasis>"v-show"</Emphasis>" has no modifiers, so this modifier does nothing."
            })
            .note(markup! {
                "Remove the modifier, like "<Emphasis>"v-show=\"isVisible\""</Emphasis>"."
            }),
            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-show"</Emphasis>" directive is missing a value."
                },
            )
            .note(markup! {
                <Emphasis>"v-show"</Emphasis>" needs a value to decide whether to show or hide the element."
            })
            .note(markup! {
                "Add a condition, like "<Emphasis>"v-show=\"isVisible\""</Emphasis>"."
            }),
            ViolationKind::OnTemplate => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-show"</Emphasis>" directive can't be used on a "<Emphasis>"<template>"</Emphasis>" element."
                },
            )
            .note(markup! {
                <Emphasis>"v-show"</Emphasis>" hides an element by changing its CSS "<Emphasis>"display"</Emphasis>" style. A "<Emphasis>"<template>"</Emphasis>" element doesn't appear on the page, so there is nothing for "<Emphasis>"v-show"</Emphasis>" to hide."
            })
            .note(markup! {
                "Use "<Emphasis>"v-if"</Emphasis>" on the "<Emphasis>"<template>"</Emphasis>" instead, or move "<Emphasis>"v-show"</Emphasis>" to an element inside it."
            }),
        })
    }
}

/// Returns `true` when the directive has no value, or its value is blank.
fn is_missing_value(directive: &VueDirective) -> bool {
    let Some(initializer) = directive.initializer() else {
        return true;
    };
    match initializer.value() {
        Ok(AnyHtmlAttributeInitializer::HtmlString(string)) => string
            .value_token()
            .is_ok_and(|token| inner_string_text(&token).trim().is_empty()),
        _ => false,
    }
}

fn is_on_template(directive: &VueDirective) -> bool {
    directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyHtmlTagElement::cast)
        .is_some_and(|element| element.tag_name_kind() == Some(T![template]))
}
