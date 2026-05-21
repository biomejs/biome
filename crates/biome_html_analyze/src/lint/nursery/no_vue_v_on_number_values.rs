use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, VueModifierList};
use biome_rowan::TextRange;
use biome_rule_options::no_vue_v_on_number_values::NoVueVOnNumberValuesOptions;

declare_lint_rule! {
    /// Disallow deprecated number modifiers on Vue `v-on` directives.
    ///
    /// Vue 3 no longer supports using key code numbers as event modifiers.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <input v-on:keyup.13="submit" />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <input @keyup.13="submit" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <input v-on:keyup.enter="submit" />
    /// ```
    ///
    /// ```vue
    /// <input @keyup.enter="submit" />
    /// ```
    ///
    pub NoVueVOnNumberValues {
        version: "2.4.15",
        name: "noVueVOnNumberValues",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-v-on-number-modifiers").same()],
    }
}

impl Rule for NoVueVOnNumberValues {
    type Query = Ast<AnyVueDirective>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoVueVOnNumberValuesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            AnyVueDirective::VueDirective(dir) => {
                if dir.name_token().ok()?.text_trimmed() != "v-on" {
                    return None;
                }
                find_number_modifier(&dir.modifiers())
            }
            AnyVueDirective::VueVOnShorthandDirective(dir) => find_number_modifier(&dir.modifiers()),
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Number modifiers are deprecated on Vue v-on directives."
                },
            )
            .note(markup! {
                "Vue 3 no longer supports key code modifiers, so this event modifier has no effect."
            })
            .note(markup! {
                "Use a named key modifier, such as " <Emphasis>"enter"</Emphasis> ", or handle the key code inside the event handler."
            }),
        )
    }
}

fn find_number_modifier(modifiers: &VueModifierList) -> Option<TextRange> {
    for modifier in modifiers {
        let modifier_token = modifier.modifier_token().ok()?;
        if modifier_token.text_trimmed().chars().all(|c| c.is_ascii_digit()) {
            return Some(modifier_token.text_trimmed_range());
        }
    }
    None
}
