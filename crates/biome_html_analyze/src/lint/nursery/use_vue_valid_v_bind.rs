use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyVueDirective, VueModifierList};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_bind::UseVueValidVBindOptions;

declare_lint_rule! {
    /// Forbids `v-bind` directives with missing arguments or invalid modifiers.
    ///
    /// This rule reports v-bind directives in the following cases:
    /// - The directive does not have an argument. E.g. `<div v-bind></div>`
    /// - The directive does not have a value. E.g. `<div v-bind:aaa></div>`
    /// - The directive has invalid modifiers. E.g. `<div v-bind:aaa.bbb="ccc"></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <Foo v-bind />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-bind></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <Foo v-bind:foo="foo" />
    /// ```
    ///
    pub UseVueValidVBind {
        version: "2.3.6",
        name: "useVueValidVBind",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-bind").same()],
    }
}

const VALID_MODIFIERS: &[&str] = &["prop", "camel", "sync", "attr"];

pub enum ViolationKind {
    MissingValue,
    MissingArgument,
    InvalidModifier(TextRange),
}

impl Rule for UseVueValidVBind {
    type Query = Ast<AnyVueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            AnyVueDirective::VueDirective(vue_directive) => {
                if vue_directive.name_token().ok()?.text_trimmed() != "v-bind" {
                    return None;
                }

                if vue_directive.initializer().is_none() {
                    return Some(ViolationKind::MissingValue);
                }

                if vue_directive.arg().is_none() {
                    return Some(ViolationKind::MissingArgument);
                }

                if let Some(invalid_range) = find_invalid_modifiers(&vue_directive.modifiers()) {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                None
            }
            AnyVueDirective::VueVBindShorthandDirective(dir) => {
                // missing argument would be caught by the parser

                if dir.initializer().is_none() {
                    return Some(ViolationKind::MissingValue);
                }

                if let Some(invalid_range) = find_invalid_modifiers(&dir.modifiers()) {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            match state {
                ViolationKind::MissingValue => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "This v-bind directive is missing a value."
                    },
                )
                .note(markup! {
                        "v-bind directives require a value."
                }).note(markup! {
                        "Add a value to the directive, e.g. "<Emphasis>"v-bind:foo=\"bar\""</Emphasis>"."
                }),
                ViolationKind::MissingArgument => RuleDiagnostic::new(
                    rule_category!(),
                    ctx.query().range(),
                    markup! {
                        "This v-bind directive is missing an argument."
                    },
                )
                .note(markup! {
                        "v-bind directives require an argument to specify which attribute to bind to."
                }).note(markup! {
                        "For example, use " <Emphasis>"v-bind:foo"</Emphasis> " to bind to the " <Emphasis>"foo"</Emphasis> " attribute."
                }),
                ViolationKind::InvalidModifier(invalid_range) =>
                    RuleDiagnostic::new(
                        rule_category!(),
                        invalid_range,
                        markup! {
                            "This v-bind directive has an invalid modifier."
                        },
                    )
                    .note(markup! {
                            "Only the following modifiers are allowed on v-bind directives: "<Emphasis>"prop"</Emphasis>", "<Emphasis>"camel"</Emphasis>", "<Emphasis>"sync"</Emphasis>", and "<Emphasis>"attr"</Emphasis>"."
                    }).note(markup! {
                            "Remove or correct the invalid modifier."
                    }),
            }
        )
    }
}

fn find_invalid_modifiers(modifiers: &VueModifierList) -> Option<TextRange> {
    for modifier in modifiers {
        if !VALID_MODIFIERS.contains(&modifier.modifier_token().ok()?.text()) {
            return Some(modifier.range());
        }
    }
    None
}
