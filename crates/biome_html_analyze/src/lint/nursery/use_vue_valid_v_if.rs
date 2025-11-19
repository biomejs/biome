use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, HtmlElement, VueDirective};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_vue_valid_v_if::UseVueValidVIfOptions;

declare_lint_rule! {
    /// Enforces valid `v-if` usage for Vue templates.
    ///
    /// This rule reports `v-if` directives in following cases:
    /// - The directive has an argument. E.g. `<div v-if:aaa="foo"></div>`
    /// - The directive has a modifier. E.g. `<div v-if.bbb="foo"></div>`
    /// - The directive does not have an attribute value. E.g. `<div v-if></div>`
    /// - The same element also has `v-else` or `v-else-if`. E.g. `<div v-if="foo" v-else></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if:aaa="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if.bbb="foo"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="foo" v-else></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="foo" v-else-if="bar"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-if="ok"></div>
    /// ```
    ///
    /// ```vue
    /// <div v-if="a < b"></div>
    /// ```
    ///
    /// ```vue
    /// <div v-if="a"></div>
    /// <div v-else-if="b"></div>
    /// <div v-else></div>
    /// ```
    ///
    pub UseVueValidVIf {
        version: "2.3.6",
        name: "useVueValidVIf",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-if").same()],
    }
}

#[derive(Debug)]
pub enum ViolationKind {
    HasArgument(TextRange),
    HasModifier(TextRange),
    MissingValue,
    ConflictsWithElse(TextRange),
}

impl Rule for UseVueValidVIf {
    type Query = Ast<AnyVueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyVueDirective::VueDirective(vue_directive) => {
                // Check if this is a v-if directive
                if vue_directive.name_token().ok()?.text_trimmed() != "v-if" {
                    return None;
                }

                // Check for argument (v-if should not have arguments)
                if let Some(arg) = vue_directive.arg() {
                    return Some(ViolationKind::HasArgument(arg.range()));
                }

                // Check for modifiers (v-if should not have modifiers)
                if vue_directive.modifiers().len() > 0 {
                    return Some(ViolationKind::HasModifier(
                        vue_directive.modifiers().range(),
                    ));
                }

                // Check for missing value
                if vue_directive.initializer().is_none() {
                    return Some(ViolationKind::MissingValue);
                }

                // Check for conflicts with v-else or v-else-if on the same element
                if let Some(element) = find_containing_element(vue_directive)
                    && let Some(conflict_range) =
                        find_conflicting_else_directives(&element, vue_directive)
                {
                    return Some(ViolationKind::ConflictsWithElse(conflict_range));
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            ViolationKind::HasArgument(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "v-if cannot have an argument."
                },
            )
            .note(markup! {
                "Remove the argument. v-if takes a value expression only."
            }),

            ViolationKind::HasModifier(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "v-if cannot have modifiers."
                },
            )
            .note(markup! {
                "Remove the modifier. v-if takes a value expression only."
            }),

            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "v-if requires a value expression."
                },
            )
            .note(markup! {
                "Provide a boolean expression, e.g. v-if=\"condition\"."
            }),

            ViolationKind::ConflictsWithElse(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "v-if cannot be used on an element that also has v-else or v-else-if."
                },
            )
            .note(markup! {
                "Place v-if on one element, followed by sibling elements with v-else-if and/or v-else."
            }),
        })
    }
}

/// Find containing HTML element for a Vue directive
fn find_containing_element(directive: &VueDirective) -> Option<HtmlElement> {
    directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(HtmlElement::cast)
}

/// Find conflicting v-else or v-else-if directives on the same element
fn find_conflicting_else_directives(
    element: &HtmlElement,
    v_if_directive: &VueDirective,
) -> Option<TextRange> {
    let opening_element = element.opening_element().ok()?;

    for attribute in opening_element.attributes() {
        if let Some(AnyVueDirective::VueDirective(directive)) = attribute.as_any_vue_directive() {
            // Skip the v-if directive we're currently checking
            if directive.syntax() == v_if_directive.syntax() {
                continue;
            }

            // Check for v-else or v-else-if
            if let Ok(name_token) = directive.name_token() {
                let name = name_token.text();
                if name == "v-else" || name == "v-else-if" {
                    return Some(directive.range());
                }
            }
        }
    }

    None
}
