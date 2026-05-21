use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{
    AnyHtmlAttributeInitializer, AnyHtmlElement, AnyHtmlTagName, AnyVueDirective,
    AnyVueVForBinding, HtmlAttributeInitializerClause, HtmlElement, VueDirective,
    VueVForIdentifierBinding, VueVForValue,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, TokenText};
use biome_rule_options::use_vue_valid_v_for::UseVueValidVForOptions;
use biome_unicode_table::{is_js_id_continue, is_js_id_start};

declare_lint_rule! {
    /// Enforces valid `v-for` directives in Vue templates.
    ///
    /// This rule reports `v-for` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-for:aaa="item in items"></div>`
    /// - The directive has a modifier. E.g. `<div v-for.bbb="item in items"></div>`
    /// - The directive does not have a value. E.g. `<div v-for></div>`
    /// - The second or third aliases are empty or are not simple identifiers.
    /// - A custom component rendered with `v-for` is missing `v-bind:key`.
    /// - The `v-bind:key` expression does not use any variable introduced by the `v-for` directive.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-for:aaa="item in items"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-for="(item, { key }) in items"></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <MyItem v-for="item in items"></MyItem>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-for="item in items" :key="foo"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-for="item in items"></div>
    /// ```
    ///
    /// ```vue
    /// <MyItem v-for="item in items" :key="item.id" />
    /// ```
    ///
    /// ```vue
    /// <template v-for="item in items">
    ///     <div :key="item.id"></div>
    /// </template>
    /// ```
    ///
    /// Related rules:
    /// - [`useVueVForKey`](https://biomejs.dev/linter/rules/use-vue-v-for-key/)
    ///
    pub UseVueValidVFor {
        version: "2.4.15",
        name: "useVueValidVFor",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-for").same()],
    }
}

#[derive(Debug)]
pub enum ViolationKind {
    UnexpectedArgument(TextRange),
    UnexpectedModifier(TextRange),
    MissingValue,
    InvalidSecondaryAlias(TextRange),
    MissingKey(TextRange),
    KeyDoesNotUseIterationVariables(TextRange),
}

impl Rule for UseVueValidVFor {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVForOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let directive = ctx.query();
        if directive.name_token().ok()?.text_trimmed() != "v-for" {
            return None;
        }

        if let Some(arg) = directive.arg() {
            return Some(ViolationKind::UnexpectedArgument(arg.range()));
        }

        if let Some(first_modifier) = directive.modifiers().iter().next() {
            return Some(ViolationKind::UnexpectedModifier(first_modifier.range()));
        }

        let Some(initializer) = directive.initializer() else {
            return Some(ViolationKind::MissingValue);
        };

        let v_for_value = initializer.value().ok()?.as_vue_v_for_value().cloned()?;

        if is_missing_v_for_value(&v_for_value) {
            return Some(ViolationKind::MissingValue);
        }

        if let Some(violation) = validate_secondary_bindings(&v_for_value) {
            return Some(violation);
        }

        if v_for_value.operator().is_err() || v_for_value.expression().is_err() {
            return None;
        }

        let element = enclosing_tag_element(directive)?;

        check_key_requirement(&element, &v_for_value)
            .or_else(|| check_template_child_keys(directive, &element, &v_for_value))
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let directive = ctx.query();
        Some(match state {
            ViolationKind::UnexpectedArgument(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-for directive does not accept an argument."
                },
            )
            .note(markup! {
                "v-for only accepts the special list-rendering syntax as its value."
            })
            .note(markup! {
                "Remove the argument and use a value such as "<Emphasis>"v-for=\"item in items\""</Emphasis>"."
            }),

            ViolationKind::UnexpectedModifier(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-for directive does not support modifiers."
                },
            )
            .note(markup! {
                "Modifiers change the meaning of other Vue directives, but v-for has a fixed syntax."
            })
            .note(markup! {
                "Remove the modifier and keep only the iteration expression."
            }),

            ViolationKind::MissingValue => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "The v-for directive requires a value."
                },
            )
            .note(markup! {
                "Without an iteration expression, Vue cannot determine which items to render."
            })
            .note(markup! {
                "Provide a value such as "<Emphasis>"v-for=\"item in items\""</Emphasis>"."
            }),

            ViolationKind::InvalidSecondaryAlias(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The second and third v-for aliases must be identifiers."
                },
            )
            .note(markup! {
                "Only the main item alias may use destructuring. The optional key and index aliases must be simple names."
            })
            .note(markup! {
                "Replace this alias with an identifier such as "<Emphasis>"index"</Emphasis>"."
            }),

            ViolationKind::MissingKey(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Custom components rendered with v-for require a v-bind:key directive."
                },
            )
            .note(markup! {
                "Vue relies on keys to keep component instances stable across list updates."
            })
            .note(markup! {
                "Add a key that uses one of the iteration variables, such as "<Emphasis>":key=\"item.id\""</Emphasis>"."
            }),

            ViolationKind::KeyDoesNotUseIterationVariables(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This v-bind:key directive does not use any variables from the v-for directive."
                },
            )
            .note(markup! {
                "Keys that are unrelated to the current iteration can cause Vue to reuse the wrong element or component instance."
            })
            .note(markup! {
                "Reference one of the variables introduced by the v-for directive in the key expression."
            }),
        })
    }
}

fn is_missing_v_for_value(value: &VueVForValue) -> bool {
    value.binding().is_err() && value.operator().is_err() && value.expression().is_err()
}

fn validate_secondary_bindings(value: &VueVForValue) -> Option<ViolationKind> {
    let AnyVueVForBinding::VueVForTupleBinding(tuple) = value.binding().ok()? else {
        return None;
    };

    validate_secondary_binding(tuple.second().as_ref())
        .or_else(|| validate_secondary_binding(tuple.third().as_ref()))
}

fn validate_secondary_binding(
    tuple_element: Option<&biome_html_syntax::VueVForTupleElement>,
) -> Option<ViolationKind> {
    let tuple_element = tuple_element?;
    let binding = tuple_element.binding().ok()?;

    if matches!(binding, AnyVueVForBinding::VueVForIdentifierBinding(_)) {
        None
    } else {
        Some(ViolationKind::InvalidSecondaryAlias(binding.range()))
    }
}

/// "Iteration bindings" are the bindings introduced by the v-for directive.
fn collect_iteration_bindings(
    value: &VueVForValue,
) -> impl Iterator<Item = TokenText> + '_ {
    value
        .binding()
        .ok()
        .into_iter()
        .flat_map(|binding| binding.syntax().descendants())
        .filter_map(VueVForIdentifierBinding::cast)
        .filter_map(|binding| binding.name_token().ok())
        .map(|token| token.token_text_trimmed())
}

fn enclosing_tag_element(directive: &VueDirective) -> Option<AnyHtmlTagElement> {
    directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyHtmlTagElement::cast)
}

fn check_key_requirement(
    element: &AnyHtmlTagElement,
    iteration_value: &VueVForValue,
) -> Option<ViolationKind> {
    if let Some(key_directive) = element.find_vue_binding("key") {
        if key_directive_uses_iteration_variables(&key_directive, iteration_value)? {
            return None;
        }

        return Some(ViolationKind::KeyDoesNotUseIterationVariables(
            key_directive.range(),
        ));
    }

    if is_custom_component(element) {
        return Some(ViolationKind::MissingKey(element.range()));
    }

    None
}

/// Handle special case for `<template v-for>`
///
/// Vue does not require keys on elements inside a `<template v-for>` as long as they do not use any variables from the `v-for` directive. This function checks for that case and only reports missing keys if necessary.
fn check_template_child_keys(
    directive: &VueDirective,
    element: &AnyHtmlTagElement,
    iteration_value: &VueVForValue,
) -> Option<ViolationKind> {
    let tag_name = element.tag_name()?;
    if tag_name.text() != "template" {
        return None;
    }

    if element.find_vue_binding("key").is_some() {
        return None;
    }

    let container = directive
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(HtmlElement::cast)?;

    for child in container.children() {
        let child = match child {
            AnyHtmlElement::HtmlElement(element) => {
                let Some(opening_element) = element.opening_element().ok() else {
                    continue;
                };
                AnyHtmlTagElement::HtmlOpeningElement(opening_element)
            }
            AnyHtmlElement::HtmlSelfClosingElement(element) => {
                AnyHtmlTagElement::HtmlSelfClosingElement(element)
            }
            _ => continue,
        };

        let Some(uses_parent_binding) = child_uses_parent_binding_in_v_for(&child, iteration_value)
        else {
            continue;
        };

        if uses_parent_binding {
            continue;
        }

        if let Some(violation) = check_key_requirement(&child, iteration_value) {
            return Some(violation);
        }
    }

    None
}

fn child_uses_parent_binding_in_v_for(
    element: &AnyHtmlTagElement,
    iteration_value: &VueVForValue,
) -> Option<bool> {
    let Some(v_for) = find_v_for_directive(element) else {
        return Some(false);
    };
    let value = v_for
        .initializer()?
        .value()
        .ok()?
        .as_vue_v_for_value()
        .cloned()?;
    let expression = value.expression().ok()?;
    let text = expression.string_value()?;

    Some(uses_iteration_variables(text.text(), iteration_value))
}

fn find_v_for_directive(element: &AnyHtmlTagElement) -> Option<VueDirective> {
    for attr in element.attributes() {
        let Some(directive) = attr
            .as_any_vue_directive()
            .and_then(|directive| directive.as_vue_directive())
        else {
            continue;
        };

        if directive
            .name_token()
            .is_ok_and(|token| token.text_trimmed() == "v-for")
        {
            return Some(directive.clone());
        }
    }

    None
}

fn key_directive_uses_iteration_variables(
    directive: &AnyVueDirective,
    iteration_value: &VueVForValue,
) -> Option<bool> {
    let initializer = match directive {
        AnyVueDirective::VueDirective(directive) => directive.initializer(),
        AnyVueDirective::VueVBindShorthandDirective(directive) => directive.initializer(),
        _ => None,
    };

    let expression = initializer.and_then(key_expression)?;

    Some(uses_iteration_variables(expression.text(), iteration_value))
}

fn key_expression(initializer: HtmlAttributeInitializerClause) -> Option<biome_rowan::Text> {
    match initializer.value().ok()? {
        AnyHtmlAttributeInitializer::HtmlString(value) => {
            value.inner_string_text().ok().map(Into::into)
        }
        _ => None,
    }
}

fn uses_iteration_variables(expression: &str, iteration_value: &VueVForValue) -> bool {
    collect_iteration_bindings(iteration_value)
        .any(|binding| contains_identifier(expression, binding.text()))
}

fn contains_identifier(expression: &str, expected: &str) -> bool {
    // This exists because we don't have introspection into the key expression from this rule currently.
    let mut chars = expression.char_indices().peekable();

    while let Some((start, current)) = chars.next() {
        if !is_js_id_start(current) {
            continue;
        }

        let mut end = start + current.len_utf8();
        while let Some((index, current)) = chars.peek() {
            if !{
                let character = *current;
                is_js_id_continue(character)
            } {
                break;
            }
            end = *index + current.len_utf8();
            chars.next();
        }

        if &expression[start..end] == expected {
            return true;
        }
    }

    false
}

fn is_custom_component(element: &AnyHtmlTagElement) -> bool {
    let is_component_name = matches!(
        element.name().ok(),
        Some(AnyHtmlTagName::HtmlComponentName(_) | AnyHtmlTagName::HtmlMemberName(_))
    );

    is_component_name || element.find_attribute_or_vue_binding("is").is_some()
}
