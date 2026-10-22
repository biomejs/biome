use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, HtmlAttributeList, VueDirective, element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, AstNodeList};
use biome_rule_options::use_vue_valid_v_is::UseVueValidVIsOptions;

use crate::vue::is_native_element_name;

declare_lint_rule! {
    /// Enforce valid `v-is` Vue directives.
    ///
    /// The `v-is` directive tells Vue to render a component in place of a native HTML element,
    /// such as `<tr v-is="'my-row'">`. Its value is a JavaScript expression, so a fixed
    /// component name needs its own quotes inside the attribute value.
    ///
    /// This rule reports `v-is` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-is:aaa="'my-row'"></div>`
    /// - The directive has any modifiers. E.g. `<div v-is.bbb="'my-row'"></div>`
    /// - The directive does not have a value. E.g. `<div v-is></div>`
    /// - The directive is on a component instead of a native HTML element. E.g. `<MyComponent v-is="'my-row'"></MyComponent>`
    ///
    /// An element counts as a component when its name isn't a known HTML, SVG, or MathML
    /// element, for example `<MyComponent>`, `<my-component>`, or `<component>`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <tr v-is:row="'my-row'"></tr>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <tr v-is.lazy="'my-row'"></tr>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <tr v-is></tr>
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///   <MyComponent v-is="'my-row'"></MyComponent>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <tr v-is="'my-row'"></tr>
    ///   <div v-is="componentName"></div>
    /// </template>
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue `is` attribute](https://vuejs.org/api/built-in-special-attributes.html#is)
    pub UseVueValidVIs {
        version: "next",
        name: "useVueValidVIs",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-is").same()],
    }
}

#[derive(Clone, Copy)]
pub enum ViolationKind {
    UnexpectedArgument,
    UnexpectedModifier,
    MissingValue,
    OnComponent,
}

impl Rule for UseVueValidVIs {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Box<[Self::State]>;
    type Options = UseVueValidVIsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if directive
            .name_token()
            .ok()
            .is_none_or(|name| name.text_trimmed() != "v-is")
        {
            return Box::default();
        }

        let mut violations = Vec::new();
        if directive.arg().is_some() {
            violations.push(ViolationKind::UnexpectedArgument);
        }
        if !directive.modifiers().is_empty() {
            violations.push(ViolationKind::UnexpectedModifier);
        }
        if !has_value(directive) {
            violations.push(ViolationKind::MissingValue);
        }
        if owner_element(directive)
            .and_then(|element| element.name().ok())
            .is_some_and(|name| !is_native_element_name(&name))
        {
            violations.push(ViolationKind::OnComponent);
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
                    "The "<Emphasis>"v-is"</Emphasis>" directive does not accept an argument."
                },
            )
            .note(markup! {
                "Remove the argument. Pass the component name as the value instead, like "<Emphasis>"v-is=\"'my-row'\""</Emphasis>"."
            }),
            ViolationKind::UnexpectedModifier => {
                let modifiers = directive.modifiers();
                let range = modifiers.first()?.range().cover(modifiers.last()?.range());
                RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "The "<Emphasis>"v-is"</Emphasis>" directive does not support modifiers."
                    },
                )
                .note(markup! {
                    "Remove the modifiers. "<Emphasis>"v-is"</Emphasis>" doesn't accept any."
                })
            }
            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-is"</Emphasis>" directive is missing a value."
                },
            )
            .note(markup! {
                "The value tells Vue which component to render. For example, "<Emphasis>"v-is=\"'my-row'\""</Emphasis>" renders the "<Emphasis>"my-row"</Emphasis>" component."
            }),
            ViolationKind::OnComponent => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The "<Emphasis>"v-is"</Emphasis>" directive is used on a component."
                },
            )
            .note(markup! {
                <Emphasis>"v-is"</Emphasis>" is meant for native HTML elements, such as "<Emphasis>"<tr>"</Emphasis>" or "<Emphasis>"<div>"</Emphasis>". It tells Vue to render a component in place of the element."
            })
            .note(markup! {
                "Use the component tag directly, or use "<Emphasis>"<component :is=\"...\">"</Emphasis>" to choose the component while the app runs."
            }),
        })
    }
}

/// Returns `true` if the directive has a value that isn't empty or only whitespace.
///
/// A value that isn't a plain string, such as one the parser couldn't read, counts as
/// present so the rule doesn't report it as missing.
fn has_value(directive: &VueDirective) -> bool {
    let Some(initializer) = directive.initializer() else {
        return false;
    };
    match initializer.value() {
        Ok(AnyHtmlAttributeInitializer::HtmlString(string)) => string
            .inner_string_text()
            .is_ok_and(|text| !text.trim().is_empty()),
        _ => true,
    }
}

fn owner_element(directive: &VueDirective) -> Option<AnyHtmlTagElement> {
    directive
        .parent::<HtmlAttributeList>()?
        .parent::<AnyHtmlTagElement>()
}
