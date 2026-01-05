use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::VueDirective;
use biome_rowan::BatchMutationExt;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_valid_v_pre::UseVueValidVPreOptions;

declare_lint_rule! {
    /// Enforce valid `v-pre` Vue directives.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// This rule reports `v-pre` directives in the following cases:
    ///
    /// The directive has an argument so it is invalid.
    /// ```vue,expect_diagnostic
    /// <div v-pre:arg></div>
    /// ```
    ///
    /// The directive has a modifier so it is invalid.
    /// ```vue,expect_diagnostic
    /// <div v-pre.mod></div>
    /// ```
    ///
    /// The directive has a value so it is invalid.
    /// ```vue,expect_diagnostic
    /// <div v-pre="value"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div v-pre></div>
    /// ```
    ///
    pub UseVueValidVPre {
        version: "2.3.11",
        name: "useVueValidVPre",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("valid-v-pre").same()],
        fix_kind: FixKind::Unsafe,
    }
}

pub enum ViolationKind {
    Argument(TextRange),
    Modifier(TextRange),
    Value(TextRange),
}

impl Rule for UseVueValidVPre {
    type Query = Ast<VueDirective>;
    type State = ViolationKind;
    type Signals = Option<Self::State>;
    type Options = UseVueValidVPreOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let vue_directive = ctx.query();
        if vue_directive.name_token().ok()?.text_trimmed() != "v-pre" {
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
                    "The v-pre directive must not have an argument."
                },
            )
            .note(markup! {
                "Use v-pre without arguments, e.g. " <Emphasis>"v-pre"</Emphasis> "."
            }),
            ViolationKind::Modifier(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-pre directive does not support modifiers."
                },
            )
            .note(markup! {
                "Remove the modifier; v-pre is a stand-alone control directive."
            }),
            ViolationKind::Value(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The v-pre directive must not have a value."
                },
            )
            .note(markup! {
                "v-pre is a boolean-like directive and should be used without a value."
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
