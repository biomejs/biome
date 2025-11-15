use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, HtmlElement, HtmlSelfClosingElement, VueDirective};
use biome_rowan::{AstNode, AstNodeList, TextRange, declare_node_union};
use biome_rule_options::use_vue_valid_v_else_if::UseVueValidVElseIfOptions;

declare_lint_rule! {
    /// Enforce valid `v-else-if` directives.
    ///
    /// Biome flags these cases:
    /// - Has an argument: `<div v-else-if:arg="b"></div>`.
    /// - Has modifiers: `<div v-else-if.mod="b"></div>`.
    /// - Missing value: `<div v-else-if></div>`.
    /// - Not preceded by a sibling with `v-if`/`v-else-if`.
    /// - On the same element as `v-if` or `v-else`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="a"></div><div v-else-if:arg="b"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="a"></div><div v-else-if.mod="b"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="a"></div><div v-else-if></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-else-if="b"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-if="a" v-else-if="b"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-if="a"></div><div v-else-if="b"></div><div v-else></div>
    /// ```
    ///
    pub UseVueValidVElseIf {
        version: "2.3.6",
        name: "useVueValidVElseIf",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-else-if").same()],
    }
}

pub enum ViolationKind {
    HasArgument,
    HasModifier(TextRange),
    MissingValue,
    MissingPreviousConditional,
    ConflictsWithOtherDirective(TextRange),
}

declare_node_union! {
    pub AnyHtmlElement = HtmlElement | HtmlSelfClosingElement
}

impl Rule for UseVueValidVElseIf {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVElseIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let vue_directive = ctx.query();

        if vue_directive.name_token().ok()?.text_trimmed() != "v-else-if" {
            return None;
        }

        // Check for argument (should not exist)
        if vue_directive.arg().is_some() {
            return Some(ViolationKind::HasArgument);
        }

        // Check for modifiers (should not exist)
        if vue_directive.modifiers().len() > 0 {
            return Some(ViolationKind::HasModifier(
                vue_directive.modifiers().range(),
            ));
        }

        // Check for missing value
        if vue_directive.initializer().is_none() {
            return Some(ViolationKind::MissingValue);
        }

        // Check for conflicting directives on same element
        if let Some(conflict_range) = find_conflicting_directive(vue_directive) {
            return Some(ViolationKind::ConflictsWithOtherDirective(conflict_range));
        }

        // Find parent element
        let parent_element = vue_directive
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(|ancestor| AnyHtmlElement::cast_ref(&ancestor))?;

        // Check for previous conditional element sibling
        if !has_previous_sibling_with_v_if_or_else_if(&parent_element) {
            return Some(ViolationKind::MissingPreviousConditional);
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            match state {
                ViolationKind::HasArgument => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "The v-else-if directive cannot take an argument."
                    },
                )
                .note(markup! {
                    "Remove the argument from the v-else-if directive."
                })
                .note(markup! {
                    "For example, use " <Emphasis>"v-else-if=\"condition\""</Emphasis> " instead of " <Emphasis>"v-else-if:arg=\"condition\""</Emphasis>"."
                }),

                ViolationKind::HasModifier(modifier_range) => RuleDiagnostic::new(
                    rule_category!(),
                    modifier_range,
                    markup! {
                        "The v-else-if directive cannot have modifiers."
                    },
                )
                .note(markup! {
                    "Remove the modifier from the v-else-if directive."
                })
                .note(markup! {
                    "For example, use " <Emphasis>"v-else-if=\"condition\""</Emphasis> " instead of " <Emphasis>"v-else-if.mod=\"condition\""</Emphasis>"."
                }),

                ViolationKind::MissingValue => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "The v-else-if directive requires a conditional expression value."
                    },
                )
                .note(markup! {
                    "Provide an expression after the '=' sign."
                })
                .note(markup! {
                    "For example, use " <Emphasis>"v-else-if=\"condition\""</Emphasis> " instead of just " <Emphasis>"v-else-if"</Emphasis>"."
                }),

                ViolationKind::MissingPreviousConditional => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "The v-else-if directive must follow an element with v-if or v-else-if."
                    },
                )
                .note(markup! {
                    "Add a preceding element with v-if or v-else-if, or remove this directive."
                })
                .note(markup! {
                    "v-else-if directives are part of conditional chains and must follow another conditional directive."
                }),

                ViolationKind::ConflictsWithOtherDirective(conflict_range) => RuleDiagnostic::new(
                    rule_category!(),
                    conflict_range,
                    markup! {
                        "Cannot use v-else-if together with v-if or v-else on the same element."
                    },
                )
                .note(markup! {
                    "Split into separate elements or remove one of the conflicting directives."
                })
                .note(markup! {
                    "Each element should only have one conditional directive from the v-if/v-else-if/v-else family."
                }),
            }
        )
    }
}

fn find_conflicting_directive(vue_directive: &VueDirective) -> Option<TextRange> {
    // Use same logic as v-else rule: inspect attributes of parent element
    let parent_element = vue_directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(|ancestor| AnyHtmlElement::cast_ref(&ancestor))?;

    let attribute_list = match &parent_element {
        AnyHtmlElement::HtmlElement(html_element) => {
            let Ok(opening_element) = html_element.opening_element() else {
                return None;
            };
            opening_element.attributes()
        }
        AnyHtmlElement::HtmlSelfClosingElement(self_closing) => self_closing.attributes(),
    };

    for attribute in attribute_list {
        if let Ok(AnyVueDirective::VueDirective(other_dir)) =
            AnyVueDirective::try_cast(attribute.syntax().clone())
            && other_dir.syntax() != vue_directive.syntax()
            && let Ok(name_token) = other_dir.name_token()
        {
            let name = name_token.text();
            if name == "v-if" || name == "v-else" {
                return Some(name_token.text_range());
            }
        }
    }
    None
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

fn has_v_if_or_else_if_directives(element: &AnyHtmlElement) -> bool {
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
            if (name == "v-if" || name == "v-else-if") && is_valid_chain_directive(name, &vue_dir) {
                return true;
            }
        }
    }
    false
}

fn is_valid_chain_directive(name: &str, dir: &VueDirective) -> bool {
    // Common invalid cases: argument present, modifiers present, missing value,
    // or conflicting directives on same element (for v-else-if specifically).
    if dir.arg().is_some() {
        return false;
    }
    if dir.modifiers().into_iter().next().is_some() {
        return false;
    }
    if dir.initializer().is_none() {
        return false;
    }
    if name == "v-else-if" {
        // Reuse conflict logic: if element combines v-else-if with v-if or v-else treat as invalid predecessor.
        if find_conflicting_directive(dir).is_some() {
            return false;
        }
    }
    true
}
