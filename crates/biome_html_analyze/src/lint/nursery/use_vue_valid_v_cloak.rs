use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::VueDirective;
use biome_rowan::BatchMutationExt;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_cloak::UseVueValidVCloakOptions;

declare_lint_rule! {
    /// Enforce valid `v-cloak` Vue directives.
    ///
    /// This rule reports `v-cloak` directives in the following cases:
    /// - The directive has an argument. E.g. `<div v-cloak:aaa></div>`
    /// - The directive has any modifiers. E.g. `<div v-cloak.bbb></div>`
    /// - The directive has an attribute value. E.g. `<div v-cloak="foo"></div>`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-cloak:arg></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-cloak.mod></div>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <div v-cloak="value"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-cloak></div>
    /// ```
    ///
    pub UseVueValidVCloak {
        version: "2.3.11",
        name: "useVueValidVCloak",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-cloak").same()],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum ViolationKind {
    Argument(TextRange),
    Modifier(TextRange),
    Value(TextRange),
}

impl Rule for UseVueValidVCloak {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVCloakOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let vue_directive = ctx.query();
        if vue_directive.name_token().ok()?.text_trimmed() != "v-cloak" {
            return None;
        }

        if let Some(arg) = vue_directive.arg() {
            return Some(ViolationKind::Argument(arg.range()));
        }

        if let Some(modifier) = vue_directive.modifiers().into_iter().next() {
            return Some(ViolationKind::Modifier(modifier.range()));
        }

        if let Some(initializer) = vue_directive.initializer() {
            return Some(ViolationKind::Value(initializer.range()));
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(match state {
            ViolationKind::Argument(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-cloak directive must not have an argument."
                },
            )
            .note(markup! {
                "Use v-cloak without arguments, e.g. " <Emphasis>"v-cloak"</Emphasis> "."
            }),
            ViolationKind::Modifier(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-cloak directive does not support modifiers."
                },
            )
            .note(markup! {
                "Remove the modifier; v-cloak is a stand-alone control directive."
            }),
            ViolationKind::Value(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-cloak directive must not have a value."
                },
            )
            .note(markup! {
                "v-cloak is a boolean-like directive and should be used without a value."
            }),
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<crate::HtmlRuleAction> {
        let directive = ctx.query();
        let mut mutation = BatchMutationExt::begin(ctx.root());

        match state {
            ViolationKind::Argument(_range) => {
                if let Some(arg) = directive.arg() {
                    mutation.remove_node(arg);
                }
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the argument." }.to_owned(),
                    mutation,
                ))
            }
            ViolationKind::Modifier(_range) => {
                if let Some(first) = directive.modifiers().into_iter().next() {
                    mutation.remove_node(first);
                }
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the modifier." }.to_owned(),
                    mutation,
                ))
            }
            ViolationKind::Value(_range) => {
                if let Some(initializer) = directive.initializer() {
                    mutation.remove_node(initializer);
                }
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Remove the value." }.to_owned(),
                    mutation,
                ))
            }
        }
    }
}
