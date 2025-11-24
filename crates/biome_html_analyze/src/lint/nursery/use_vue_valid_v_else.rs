use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, HtmlElement, HtmlSelfClosingElement, VueDirective};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::use_vue_valid_v_else::UseVueValidVElseOptions;

declare_lint_rule! {
    /// Enforce valid usage of v-else.
    ///
    /// This rule reports v-else directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-if="foo"></div><div v-else:aaa></div>`
    /// - The directive has a modifier. E.g. `<div v-if="foo"></div><div v-else.bbb></div>`
    /// - The directive has an attribute value. E.g. `<div v-if="foo"></div><div v-else="bar"></div>`
    /// - The directive is on elements where the previous element doesn't have `v-if`/`v-else-if` directives. E.g. `<div v-else></div>`
    /// - The directive is on elements which have `v-if`/`v-else-if` directives. E.g. `<div v-if="foo" v-else></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-else:arg></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-else.mod></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-else="value"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-else></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="foo" v-else></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-if="foo"></div>
    /// <div v-else></div>
    /// ```
    ///
    /// ```vue
    /// <div v-if="foo"></div>
    /// <div v-else-if="bar"></div>
    /// <div v-else></div>
    /// ```
    ///
    pub UseVueValidVElse {
        version: "2.3.6",
        name: "useVueValidVElse",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-else").same()],
    }
}

pub enum ViolationKind {
    HasArgument(TextRange),
    HasModifier(TextRange),
    HasValue(TextRange),
    MissingPreviousIfOrElseIf,
    CombinedWithIfOrElseIf(TextRange),
}

declare_node_union! {
    pub AnyHtmlElement = HtmlElement | HtmlSelfClosingElement
}

impl Rule for UseVueValidVElse {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVElseOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let vue_directive = ctx.query();
        if vue_directive.name_token().ok()?.text_trimmed() != "v-else" {
            return None;
        }

        // Check for argument
        if let Some(arg) = vue_directive.arg() {
            return Some(ViolationKind::HasArgument(arg.range()));
        }

        // Check for modifiers
        let modifiers = vue_directive.modifiers();
        if let Some(modifier) = modifiers.into_iter().next() {
            return Some(ViolationKind::HasModifier(modifier.range()));
        }

        // Check for value
        if let Some(initializer) = vue_directive.initializer() {
            return Some(ViolationKind::HasValue(initializer.range()));
        }

        // Get parent element
        let parent_element = vue_directive
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(|ancestor| AnyHtmlElement::cast_ref(&ancestor))?;

        // Check if current element also has v-if or v-else-if
        if has_v_if_or_else_if_directives(&parent_element) {
            return Some(ViolationKind::CombinedWithIfOrElseIf(
                vue_directive.name_token().ok()?.text_range(),
            ));
        }

        // Check if previous sibling has v-if or v-else-if
        if !has_previous_sibling_with_v_if_or_else_if(&parent_element) {
            return Some(ViolationKind::MissingPreviousIfOrElseIf);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            match state {
                ViolationKind::HasArgument(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "v-else must not have an argument."
                    },
                )
                .note(markup! {
                    "Remove the argument; v-else is a stand-alone control directive."
                }),
                ViolationKind::HasModifier(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "v-else must not have modifiers."
                    },
                )
                .note(markup! {
                    "Remove the modifier; v-else is a stand-alone control directive."
                }),
                ViolationKind::HasValue(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "v-else must not have a value."
                    },
                )
                .note(markup! {
                    "Remove the value; v-else is a stand-alone control directive."
                }),
                ViolationKind::MissingPreviousIfOrElseIf => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "v-else requires a previous sibling element with v-if or v-else-if."
                    },
                )
                .note(markup! {
                    "Place v-else immediately after an element with v-if or v-else-if, within the same parent."
                }),
                ViolationKind::CombinedWithIfOrElseIf(range) => RuleDiagnostic::new(
                    rule_category!(),
                    range,
                    markup! {
                        "v-else cannot be used on the same element as v-if or v-else-if."
                    },
                )
                .note(markup! {
                    "Move v-else onto a separate element immediately following the v-if/v-else-if element."
                }),
            }
        )
    }
}

fn has_v_if_or_else_if_directives(element: &AnyHtmlElement) -> bool {
    // Check attributes for v-if or v-else-if directives

    let attribute_list = match element {
        AnyHtmlElement::HtmlElement(html_element) => {
            let Ok(opening_element) = html_element.opening_element() else {
                return false;
            };
            opening_element.attributes()
        }
        AnyHtmlElement::HtmlSelfClosingElement(self_closing) => self_closing.attributes(),
    };

    for attribute in attribute_list {
        if let Ok(AnyVueDirective::VueDirective(vue_dir)) =
            AnyVueDirective::try_cast(attribute.syntax().clone())
            && let Ok(name_token) = vue_dir.name_token()
        {
            let name = name_token.text();
            if name == "v-if" || name == "v-else-if" {
                return true;
            }
        }
    }

    false
}

fn has_previous_sibling_with_v_if_or_else_if(element: &AnyHtmlElement) -> bool {
    if let Some(sibling) = element
        .syntax()
        .prev_sibling()
        .and_then(|s| AnyHtmlElement::cast_ref(&s))
    {
        return has_v_if_or_else_if_directives(&sibling);
    }

    false
}
