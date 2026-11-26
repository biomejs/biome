use crate::HtmlRuleAction;
use crate::tailwind::{apply_fixed_class_string, host_range};
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::no_tailwind_legacy_important::NoTailwindLegacyImportantOptions;
use biome_tailwind_logic::no_tailwind_legacy_important::{
    TrailingImportantClass, fixed_class_string, legacy_important_candidates,
};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_syntax::TwFullCandidate;

declare_lint_rule! {
    /// Disallow the leading `!` important modifier in Tailwind CSS classes.
    ///
    /// The `!` modifier makes all the styles of a class `!important`.
    /// In Tailwind CSS v4, the `!` goes at the end of the class name, as in `flex!` or `hover:bg-red-500!`.
    /// Tailwind CSS v3 wrote it at the start of the utility name instead, as in `!flex` or `hover:!bg-red-500`.
    /// Tailwind CSS v4 still accepts the old form for backward compatibility, but it's deprecated.
    ///
    /// This rule reports classes that put the `!` at the start, and moves it to the end.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="!flex"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="hover:!bg-red-500"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="flex! hover:bg-red-500! -m-4!"></div>
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    pub NoTailwindLegacyImportant {
        version: "next",
        name: "noTailwindLegacyImportant",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        sources: &[
            RuleSource::EslintBetterTailwindcss("enforce-consistent-important-position").same(),
            RuleSource::EslintTailwindcss("important-modifier-suffix").same(),
        ],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoTailwindLegacyImportant {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = TwFullCandidate;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindLegacyImportantOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Box::default();
        }
        legacy_important_candidates(&ctx.query().tailwind_root().candidates())
            .into_iter()
            .collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, candidate: &Self::State) -> Option<RuleDiagnostic> {
        let class = candidate.syntax().text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), candidate.range())?,
                markup! {
                    "The Tailwind CSS class "<Emphasis>{format_args!("{class}")}</Emphasis>" puts the important modifier "<Emphasis>"!"</Emphasis>" before the utility name."
                },
            )
            .note(markup! {
                "Tailwind CSS v4 expects the "<Emphasis>"!"</Emphasis>" at the end of the class name, and only accepts it before the utility name for backward compatibility."
            })
            .note(markup! {
                "Use "<Emphasis>{TrailingImportantClass(candidate)}</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, candidate: &Self::State) -> Option<HtmlRuleAction> {
        let fixed = fixed_class_string(&ctx.query().tailwind_root(), candidate)?;
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(&mut mutation, ctx.query().node(), &fixed)?;
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Move the "<Emphasis>"!"</Emphasis>" to the end of the class name." }
                .to_owned(),
            mutation,
        ))
    }
}
