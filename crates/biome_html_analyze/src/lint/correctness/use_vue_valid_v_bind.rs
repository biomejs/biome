use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::{
    AnyVueDirective, AnyVueDirectiveArgument, VueDirectiveArgument, VueModifierList,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_bind::UseVueValidVBindOptions;

declare_lint_rule! {
    /// Forbids `v-bind` directives with missing values or invalid modifiers.
    ///
    /// This rule reports v-bind directives in the following cases:
    /// - The directive has neither a value nor a static argument from which to
    ///   derive one. E.g. `<div v-bind></div>` or `<div v-bind:[foo]></div>`.
    ///   `v-bind:foo` and `:foo` are accepted because they are valid Vue 3.4+
    ///   same-name shorthand for `:foo="foo"`.
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
    /// ```vue
    /// <Foo :foo />
    /// ```
    ///
    pub UseVueValidVBind {
        version: "2.3.6",
        name: "useVueValidVBind",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-bind").same()],
    }
}

const VALID_MODIFIERS: &[&str] = &["prop", "camel", "sync", "attr"];

pub enum ViolationKind {
    MissingValue,
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
                if !vue_directive.is_binding() {
                    return None;
                }

                if vue_directive.initializer().is_none()
                    && !vue_directive.arg().is_some_and(|arg| is_static_arg(&arg))
                {
                    return Some(ViolationKind::MissingValue);
                }

                if let Some(invalid_range) = find_invalid_modifiers(&vue_directive.modifiers()) {
                    return Some(ViolationKind::InvalidModifier(invalid_range));
                }

                None
            }
            AnyVueDirective::VueVBindShorthandDirective(dir) => {
                // missing argument would be caught by the parser

                if dir.initializer().is_none()
                    && !dir.arg().is_ok_and(|arg| is_static_arg(&arg))
                {
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

/// Returns `true` if the directive argument is a static identifier
/// (e.g. `foo` in `:foo`), as opposed to a dynamic expression
/// (e.g. `[foo]` in `:[foo]`). Only static arguments can be the
/// source of a Vue 3.4+ same-name shorthand binding.
fn is_static_arg(arg: &VueDirectiveArgument) -> bool {
    matches!(
        arg.arg().ok(),
        Some(AnyVueDirectiveArgument::VueStaticArgument(_))
    )
}
