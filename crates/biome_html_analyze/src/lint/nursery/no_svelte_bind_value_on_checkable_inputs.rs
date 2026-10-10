use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{
    AnyHtmlAttribute, AnyHtmlAttributeInitializer, AnySvelteBindingProperty, AnySvelteDirective,
    HtmlSyntaxKind, HtmlSyntaxToken, SvelteBindDirective, T, element_ext::AnyHtmlTagElement,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_svelte_bind_value_on_checkable_inputs::NoSvelteBindValueOnCheckableInputsOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow `bind:value` on checkbox and radio inputs.
    ///
    /// `bind:value` keeps a variable in sync with the `value` of an input. For most `<input>` types,
    /// `value` is what the user types or picks. Checkboxes and radio buttons are different: the
    /// user changes whether they are checked, and their `value` only describes what a checked
    /// input stands for. So `bind:value` on these inputs does not track the user's choice. On a
    /// checkbox, Svelte also throws an error during development.
    ///
    /// Use `bind:checked` to track whether a single checkbox is checked. Use `bind:group` to
    /// track which checkboxes or radio buttons in a group are selected.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <input type="checkbox" bind:value={accepted} />
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <input type="radio" bind:value={size} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <input type="checkbox" bind:checked={accepted} />
    /// <input type="checkbox" value="cheese" bind:group={toppings} />
    /// <input type="radio" value="small" bind:group={size} />
    /// <input type="text" bind:value={name} />
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte `bind:checked`](https://svelte.dev/docs/svelte/bind#input-bind:checked)
    /// - [Svelte `bind:group`](https://svelte.dev/docs/svelte/bind#input-bind:group)
    ///
    pub NoSvelteBindValueOnCheckableInputs {
        version: "next",
        name: "noSvelteBindValueOnCheckableInputs",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-bind-value-on-checkable-inputs").same()],
        fix_kind: FixKind::Unsafe,
    }
}

pub struct State {
    input_kind: CheckableInputKind,
    directive: SvelteBindDirective,
}

impl Rule for NoSvelteBindValueOnCheckableInputs {
    type Query = Ast<AnyHtmlTagElement>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoSvelteBindValueOnCheckableInputsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        if element.tag_name_kind() != Some(T![input]) {
            return None;
        }

        let input_type = element
            .find_attribute_by_name("type")?
            .as_html_attribute()?
            .html_string()?
            .inner_string_text()
            .ok()?;
        let input_kind = if input_type.eq_ignore_ascii_case("checkbox") {
            CheckableInputKind::Checkbox
        } else if input_type.eq_ignore_ascii_case("radio") {
            CheckableInputKind::Radio
        } else {
            return None;
        };

        let directive = element
            .attributes()
            .into_iter()
            .find_map(|attribute| match attribute {
                AnyHtmlAttribute::AnySvelteDirective(AnySvelteDirective::SvelteBindDirective(
                    directive,
                )) if is_binding_named(&directive, "value") => Some(directive),
                _ => None,
            })?;

        Some(State {
            input_kind,
            directive,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.directive.range(),
            markup! {
                <Emphasis>"bind:value"</Emphasis>" does not track whether this "{state.input_kind}" is checked."
            },
        )
        .note(markup! {
            "Users change whether a "{state.input_kind}" is checked, not its "<Emphasis>"value"</Emphasis>"."
        });

        Some(match state.input_kind {
            CheckableInputKind::Checkbox => diagnostic.note(markup! {
                "Use "<Emphasis>"bind:checked"</Emphasis>" to track whether this checkbox is checked, or "<Emphasis>"bind:group"</Emphasis>" to track the values of all checked checkboxes in a group."
            }),
            CheckableInputKind::Radio => diagnostic.note(markup! {
                "Use "<Emphasis>"bind:group"</Emphasis>" to track the value of the selected radio button in a group."
            }),
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let replacement = state.input_kind.replacement_binding();
        // Svelte treats `bind:checked` and `checked` as the same attribute name and rejects
        // elements that repeat a name, so renaming the binding would break those elements.
        let element = ctx.query();
        if element.find_attribute_by_name(replacement).is_some()
            || element
                .attributes()
                .into_iter()
                .any(|attribute| match attribute {
                    AnyHtmlAttribute::AnySvelteDirective(
                        AnySvelteDirective::SvelteBindDirective(directive),
                    ) => is_binding_named(&directive, replacement),
                    _ => false,
                })
        {
            return None;
        }

        let value = state.directive.value().ok()?;
        let name = value.property().ok()?;
        let name = name.as_svelte_name()?;
        let mut mutation = ctx.root().begin();

        if let Some(initializer) = value.initializer() {
            // Svelte rejects a getter and setter pair in `bind:group`; it only accepts a variable
            // or a property access.
            if matches!(state.input_kind, CheckableInputKind::Radio)
                && initializer
                    .as_svelte_bind_function_binding_initializer_clause()
                    .is_some()
            {
                return None;
            }
            mutation
                .replace_token_transfer_trivia(name.ident_token().ok()?, make::ident(replacement));
        } else {
            // `bind:value` is shorthand for `bind:value={value}`, so the variable name has to be
            // written out once the property is renamed.
            let initializer = make::html_attribute_initializer_clause(
                make::token(T![=]),
                AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(
                    make::html_attribute_single_text_expression(
                        make::token(T!['{']),
                        make::html_text_expression(HtmlSyntaxToken::new_detached(
                            HtmlSyntaxKind::HTML_LITERAL,
                            "value",
                            [],
                            [],
                        )),
                        make::token(T!['}']),
                    ),
                ),
            );
            let new_value = make::svelte_directive_value(
                value.colon_token().ok()?,
                AnySvelteBindingProperty::SvelteName(make::svelte_name(make::ident(replacement))),
                value.modifiers(),
            )
            .with_initializer(initializer.into())
            .build();
            mutation.replace_node(value, new_value);
        }

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"bind:"{replacement}</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Clone, Copy)]
enum CheckableInputKind {
    Checkbox,
    Radio,
}

impl CheckableInputKind {
    /// Returns the binding that tracks the state of this kind of input.
    const fn replacement_binding(self) -> &'static str {
        match self {
            Self::Checkbox => "checked",
            Self::Radio => "group",
        }
    }
}

impl biome_console::fmt::Display for CheckableInputKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Checkbox => "checkbox",
            Self::Radio => "radio button",
        })
    }
}

fn is_binding_named(directive: &SvelteBindDirective, name: &str) -> bool {
    directive
        .value()
        .ok()
        .and_then(|value| value.property().ok())
        .and_then(|property| property.as_svelte_name()?.ident_token().ok())
        .is_some_and(|token| token.text_trimmed() == name)
}
