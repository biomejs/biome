use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::element_ext::AnyHtmlTagElement;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnyHtmlElement, AnyHtmlTagName, AnyVueDirective,
    AnyVueVForBinding, AnyVueVForDestructuredBinding, HtmlAttributeInitializerClause, HtmlElement,
    HtmlOpeningElement, HtmlSelfClosingElement, VueDirective, VueDirectiveArgument, VueVForValue,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, TextRange, TokenText};
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
    pub UseVueValidVFor {
        version: "next",
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
    InvalidEmptyAlias,
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

        if v_for_value.binding().is_err() {
            return Some(ViolationKind::InvalidEmptyAlias);
        }

        if let Some(violation) = validate_secondary_aliases(&v_for_value) {
            return Some(violation);
        }

        if v_for_value.operator().is_err() || v_for_value.expression().is_err() {
            return None;
        }

        let aliases = collect_iteration_aliases(&v_for_value);
        let element = enclosing_tag_element(directive)?;

        check_key_requirement(&element, &aliases)
            .or_else(|| check_template_child_keys(directive, &element, &aliases))
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

            ViolationKind::InvalidEmptyAlias => RuleDiagnostic::new(
                rule_category!(),
                directive.range(),
                markup! {
                    "This v-for directive contains an empty alias."
                },
            )
            .note(markup! {
                "Every alias slot in a v-for directive must map to a real iteration variable."
            })
            .note(markup! {
                "Remove the empty slot or replace it with an identifier."
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

fn validate_secondary_aliases(value: &VueVForValue) -> Option<ViolationKind> {
    let AnyVueVForBinding::VueVForTupleBinding(tuple) = value.binding().ok()? else {
        return None;
    };

    validate_secondary_alias(tuple.second().as_ref())
        .or_else(|| validate_secondary_alias(tuple.third().as_ref()))
}

fn validate_secondary_alias(
    tuple_element: Option<&biome_html_syntax::VueVForTupleElement>,
) -> Option<ViolationKind> {
    let tuple_element = tuple_element?;
    let Ok(binding) = tuple_element.binding() else {
        return Some(ViolationKind::InvalidEmptyAlias);
    };

    if matches!(binding, AnyVueVForBinding::VueVForIdentifierBinding(_)) {
        None
    } else {
        Some(ViolationKind::InvalidSecondaryAlias(binding.range()))
    }
}

fn collect_iteration_aliases(value: &VueVForValue) -> Vec<TokenText> {
    let mut aliases = Vec::new();
    let mut pending = Vec::new();

    if let Ok(binding) = value.binding() {
        pending.push(binding);
    }

    while let Some(binding) = pending.pop() {
        match binding {
            AnyVueVForBinding::VueVForIdentifierBinding(binding) => {
                if let Ok(token) = binding.name_token() {
                    aliases.push(token.token_text_trimmed());
                }
            }
            AnyVueVForBinding::VueVForTupleBinding(binding) => {
                if let Some(third) = binding.third()
                    && let Ok(binding) = third.binding()
                {
                    pending.push(binding);
                }
                if let Some(second) = binding.second()
                    && let Ok(binding) = second.binding()
                {
                    pending.push(binding);
                }
                if let Ok(binding) = binding.value() {
                    pending.push(binding);
                }
            }
            AnyVueVForBinding::AnyVueVForDestructuredBinding(binding) => {
                let bindings = match binding {
                    AnyVueVForDestructuredBinding::VueVForArrayBinding(binding) => {
                        binding.bindings()
                    }
                    AnyVueVForDestructuredBinding::VueVForObjectBinding(binding) => {
                        binding.bindings()
                    }
                };

                for binding in bindings.iter().flatten() {
                    match binding {
                        biome_html_syntax::AnyVueVForBindingListElement::VueVForIdentifierBinding(binding) => {
                            if let Ok(token) = binding.name_token() {
                                aliases.push(token.token_text_trimmed());
                            }
                        }
                        biome_html_syntax::AnyVueVForBindingListElement::VueVForObjectPropertyBinding(binding) => {
                            if let Ok(binding) = binding.binding() {
                                pending.push(binding);
                            }
                        }
                        biome_html_syntax::AnyVueVForBindingListElement::VueVForRestBinding(binding) => {
                            if let Ok(binding) = binding.binding()
                                && let Ok(token) = binding.name_token()
                            {
                                aliases.push(token.token_text_trimmed());
                            }
                        }
                        biome_html_syntax::AnyVueVForBindingListElement::AnyVueVForDestructuredBinding(binding) => {
                            pending.push(AnyVueVForBinding::AnyVueVForDestructuredBinding(binding.clone()));
                        }
                    }
                }
            }
        }
    }

    aliases
}

fn enclosing_tag_element(directive: &VueDirective) -> Option<AnyHtmlTagElement> {
    directive.syntax().ancestors().skip(1).find_map(|ancestor| {
        HtmlOpeningElement::cast(ancestor.clone())
            .map(AnyHtmlTagElement::HtmlOpeningElement)
            .or_else(|| {
                HtmlSelfClosingElement::cast(ancestor)
                    .map(AnyHtmlTagElement::HtmlSelfClosingElement)
            })
    })
}

fn check_key_requirement(
    element: &AnyHtmlTagElement,
    aliases: &[TokenText],
) -> Option<ViolationKind> {
    if let Some(key_directive) = find_v_bind_key(element) {
        if key_directive_uses_iteration_variables(&key_directive, aliases)? {
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

fn check_template_child_keys(
    directive: &VueDirective,
    element: &AnyHtmlTagElement,
    aliases: &[TokenText],
) -> Option<ViolationKind> {
    let tag_name = element.tag_name()?;
    if tag_name.text() != "template" {
        return None;
    }

    if find_v_bind_key(element).is_some() {
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
                AnyHtmlTagElement::HtmlOpeningElement(element.opening_element().ok()?)
            }
            AnyHtmlElement::HtmlSelfClosingElement(element) => {
                AnyHtmlTagElement::HtmlSelfClosingElement(element)
            }
            _ => continue,
        };

        if child_uses_parent_alias_in_v_for(&child, aliases)? {
            continue;
        }

        if let Some(violation) = check_key_requirement(&child, aliases) {
            return Some(violation);
        }
    }

    None
}

fn child_uses_parent_alias_in_v_for(
    element: &AnyHtmlTagElement,
    aliases: &[TokenText],
) -> Option<bool> {
    let Some(v_for) = find_v_for_directive(element) else {
        return Some(false);
    };
    let value = v_for_value(&v_for)?;
    let expression = value.expression().ok()?;
    let text = expression.string_value()?;

    Some(uses_iteration_variables(text.text(), aliases))
}

fn find_v_for_directive(element: &AnyHtmlTagElement) -> Option<VueDirective> {
    for attr in element.attributes() {
        let directive = attr.as_any_vue_directive()?.as_vue_directive()?;

        if directive
            .name_token()
            .is_ok_and(|token| token.text_trimmed() == "v-for")
        {
            return Some(directive.clone());
        }
    }

    None
}

fn v_for_value(directive: &VueDirective) -> Option<VueVForValue> {
    directive
        .initializer()?
        .value()
        .ok()?
        .as_vue_v_for_value()
        .cloned()
}

fn find_v_bind_key(element: &AnyHtmlTagElement) -> Option<AnyVueDirective> {
    element
        .attributes()
        .iter()
        .find_map(|attribute| match attribute {
            AnyHtmlAttribute::AnyVueDirective(directive) => match directive {
                AnyVueDirective::VueDirective(directive)
                    if directive
                        .name_token()
                        .is_ok_and(|token| token.text_trimmed() == "v-bind")
                        && directive
                            .arg()
                            .is_some_and(|argument| is_key_argument(&argument)) =>
                {
                    Some(AnyVueDirective::VueDirective(directive))
                }
                AnyVueDirective::VueVBindShorthandDirective(directive)
                    if directive
                        .arg()
                        .is_ok_and(|argument| is_key_argument(&argument)) =>
                {
                    Some(AnyVueDirective::VueVBindShorthandDirective(directive))
                }
                _ => None,
            },
            _ => None,
        })
}

fn is_key_argument(argument: &VueDirectiveArgument) -> bool {
    argument
        .arg()
        .ok()
        .and_then(|argument| argument.as_vue_static_argument().cloned())
        .and_then(|argument| argument.name_token().ok())
        .is_some_and(|token| token.text_trimmed() == "key")
}

fn key_directive_uses_iteration_variables(
    directive: &AnyVueDirective,
    aliases: &[TokenText],
) -> Option<bool> {
    let initializer = match directive {
        AnyVueDirective::VueDirective(directive) => directive.initializer(),
        AnyVueDirective::VueVBindShorthandDirective(directive) => directive.initializer(),
        _ => None,
    };

    let expression = initializer.and_then(key_expression)?;

    Some(uses_iteration_variables(expression.text(), aliases))
}

fn key_expression(initializer: HtmlAttributeInitializerClause) -> Option<biome_rowan::Text> {
    match initializer.value().ok()? {
        AnyHtmlAttributeInitializer::HtmlString(value) => {
            value.inner_string_text().ok().map(Into::into)
        }
        AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(value) => {
            value.expression().ok()?.string_value()
        }
        AnyHtmlAttributeInitializer::VueVForValue(_) => None,
    }
}

fn uses_iteration_variables(expression: &str, aliases: &[TokenText]) -> bool {
    aliases
        .iter()
        .any(|alias| contains_identifier(expression, alias.text()))
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
