use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_factory::make;
use biome_html_syntax::{AnyVueDirective, VueModifier, VueModifierList};
use biome_rowan::{AstNode, AstNodeList, BatchMutationExt};
use biome_rule_options::no_vue_deprecated_v_bind_sync::NoVueDeprecatedVBindSyncOptions;

use crate::HtmlRuleAction;

declare_lint_rule! {
    /// Disallow the `.sync` modifier on Vue `v-bind` directives.
    ///
    /// In Vue 2, adding `.sync` to a `v-bind` directive let a child component update the
    /// value it received from its parent. Vue 3 removed the `.sync` modifier. The value is
    /// still passed to the child component, but changes made by the child are no longer
    /// sent back to the parent.
    ///
    /// Use `v-model` with an argument instead. For example, replace `:title.sync="pageTitle"`
    /// with `v-model:title="pageTitle"`. If `.sync` is used on a whole object, as in
    /// `v-bind.sync="page"`, use a separate `v-model` for each property of the object.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <MyComponent v-bind:title.sync="pageTitle" />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <MyComponent :title.sync="pageTitle" />
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <MyComponent v-bind.sync="page" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <MyComponent v-model:title="pageTitle" />
    /// <MyComponent :title="pageTitle" />
    /// <MyComponent v-bind="page" />
    /// ```
    ///
    /// ## References
    ///
    /// - [Vue 3 migration guide: `v-model`](https://v3-migration.vuejs.org/breaking-changes/v-model.html)
    pub NoVueDeprecatedVBindSync {
        version: "next",
        name: "noVueDeprecatedVBindSync",
        language: "html",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-deprecated-v-bind-sync").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoVueDeprecatedVBindSync {
    type Query = Ast<AnyVueDirective>;
    /// The `.sync` modifier of the reported directive.
    type State = VueModifier;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedVBindSyncOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        match ctx.query() {
            AnyVueDirective::VueDirective(directive) if directive.is_binding() => {
                find_sync_modifier(&directive.modifiers())
            }
            AnyVueDirective::VueVBindShorthandDirective(directive) => {
                find_sync_modifier(&directive.modifiers())
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, modifier: &Self::State) -> Option<RuleDiagnostic> {
        let has_argument = match ctx.query() {
            AnyVueDirective::VueDirective(directive) => directive.arg(),
            AnyVueDirective::VueVBindShorthandDirective(directive) => directive.arg().ok(),
            _ => None,
        }
        .is_some_and(|arg| arg.arg().is_some());
        let advice = if has_argument {
            markup! {
                "Use "<Emphasis>"v-model"</Emphasis>" with the same argument instead, e.g. "<Emphasis>"v-model:title=\"pageTitle\""</Emphasis>"."
            }
        } else {
            markup! {
                "Use a separate "<Emphasis>"v-model"</Emphasis>" for each property of the object instead, e.g. "<Emphasis>"v-model:title=\"page.title\""</Emphasis>"."
            }
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                modifier.range(),
                markup! {
                    "The "<Emphasis>".sync"</Emphasis>" modifier on "<Emphasis>"v-bind"</Emphasis>" is deprecated."
                },
            )
            .note(markup! {
                "Vue 3 removed this modifier, so changes made by the child component are no longer sent back to this value."
            })
            .note(advice),
        )
    }

    fn action(ctx: &RuleContext<Self>, _modifier: &Self::State) -> Option<HtmlRuleAction> {
        let node = ctx.query();
        let (arg, modifiers, initializer) = match node {
            AnyVueDirective::VueDirective(directive) => (
                directive.arg()?,
                directive.modifiers(),
                directive.initializer()?,
            ),
            AnyVueDirective::VueVBindShorthandDirective(directive) => (
                directive.arg().ok()?,
                directive.modifiers(),
                directive.initializer()?,
            ),
            _ => return None,
        };
        // `v-model` has no equivalent for the argument-less form (`v-bind.sync="obj"`)
        // or for the other `v-bind` modifiers.
        if arg.arg().is_none() || modifiers.len() != 1 {
            return None;
        }

        // In the shorthand form, the `:` is the first token and carries the attribute's
        // leading whitespace. `replace_node` moves that whitespace onto `v-model`, so it must
        // not stay on the `:` as well.
        let new_directive =
            make::vue_directive(make::ident("v-model"), make::vue_modifier_list([]))
                .with_arg(arg.trim_leading_trivia()?)
                .with_initializer(initializer)
                .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node.clone(), AnyVueDirective::VueDirective(new_directive));
        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"v-model"</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

fn find_sync_modifier(modifiers: &VueModifierList) -> Option<VueModifier> {
    modifiers.iter().find(|modifier| {
        modifier
            .modifier_token()
            .is_ok_and(|token| token.text_trimmed() == "sync")
    })
}
