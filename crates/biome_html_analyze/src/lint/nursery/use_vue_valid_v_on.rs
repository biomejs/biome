use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, VueModifierList};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_on::UseVueValidVOnOptions;
use std::{collections::HashSet, sync::LazyLock};

declare_lint_rule! {
    /// Enforce valid `v-on` directives with proper arguments, modifiers, and handlers.
    ///
    /// This rule reports v-on directives in the following cases:
    /// - The directive does not have an event name. E.g. `<div v-on="foo"></div>`
    /// - The directive has invalid modifiers. E.g. `<div v-on:click.bogus="foo"></div>`
    /// - The directive is missing a handler expression. E.g. `<div v-on:click></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <Foo v-on />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <Foo v-on:click="foo" />
    /// ```
    ///
    pub UseVueValidVOn {
        version: "2.3.6",
        name: "useVueValidVOn",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-on").same()],
    }
}

static VALID_MODIFIERS_LIST: &[&str] = &[
    "stop", "prevent", "capture", "self", "ctrl", "shift", "alt", "meta", "native", "once", "left",
    "right", "middle", "passive", "esc", "tab", "enter", "space", "up", "down", "delete", "exact",
];

static VALID_MODIFIERS: LazyLock<HashSet<&'static str>> =
    LazyLock::new(|| VALID_MODIFIERS_LIST.iter().copied().collect());

pub enum ViolationKind {
    MissingEventName,
    InvalidModifier(TextRange),
    MissingHandler,
}

impl Rule for UseVueValidVOn {
    type Query = Ast<AnyVueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVOnOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let options = ctx.options();

        match node {
            AnyVueDirective::VueDirective(vue_directive) => {
                if vue_directive.name_token().ok()?.text_trimmed() != "v-on" {
                    return None;
                }

                // Check for missing event name
                if vue_directive.arg().is_none() {
                    return Some(ViolationKind::MissingEventName);
                }

                // Check for invalid modifiers
                if let Some(invalid_range) =
                    find_invalid_modifiers(&vue_directive.modifiers(), options.modifiers.as_ref())
                {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                // Check for missing handler
                if vue_directive.initializer().is_none() {
                    return Some(ViolationKind::MissingHandler);
                }

                None
            }
            AnyVueDirective::VueVOnShorthandDirective(dir) => {
                // Shorthand always has an argument (parser enforces this)

                // Check for invalid modifiers
                if let Some(invalid_range) =
                    find_invalid_modifiers(&dir.modifiers(), options.modifiers.as_ref())
                {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                // Check for missing handler
                if dir.initializer().is_none() {
                    return Some(ViolationKind::MissingHandler);
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            ViolationKind::MissingEventName => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The v-on directive is missing an event name."
                },
            )
            .note(markup! {
                    "Provide an event name after the colon, e.g. v-on:click=\"handler\"."
            }),
            ViolationKind::InvalidModifier(invalid_range) => RuleDiagnostic::new(
                rule_category!(),
                invalid_range,
                markup! {
                    "Invalid v-on modifier."
                },
            )
            .note(markup! {
                    "Remove or correct the invalid modifier."
            })
            .footer_list(
                markup! {
                        "Allowed modifiers:"
                },
                VALID_MODIFIERS_LIST.iter().copied().chain(
                    ctx.options()
                        .modifiers
                        .as_deref()
                        .unwrap_or(&[])
                        .iter()
                        .map(|s| s.as_str()),
                ),
            ),
            ViolationKind::MissingHandler => RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "The v-on directive for this event is missing a handler expression."
                },
            )
            .note(markup! {
                    "Add a handler, e.g. v-on:click=\"onClick\"."
            }),
        })
    }
}

fn find_invalid_modifiers(
    modifiers: &VueModifierList,
    additional_allowed_modifiers: Option<&Vec<String>>,
) -> Option<TextRange> {
    for modifier in modifiers {
        if let Ok(modifier_token) = modifier.modifier_token() {
            let modifier_text = modifier_token.text();
            if VALID_MODIFIERS.contains(modifier_text) {
                continue;
            }
            if let Some(additional) = additional_allowed_modifiers
                && additional.iter().any(|s| s.as_str() == modifier_text)
            {
                continue;
            }
            return Some(modifier.range());
        }
    }
    None
}
