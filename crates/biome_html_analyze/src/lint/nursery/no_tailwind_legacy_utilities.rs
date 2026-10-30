use crate::HtmlRuleAction;
use crate::tailwind::{apply_fixed_class_string, host_range};
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlAttribute;
use biome_project_layout::ProjectLayout;
use biome_rowan::BatchMutationExt;
use biome_rule_options::no_tailwind_legacy_utilities::NoTailwindLegacyUtilitiesOptions;
use biome_tailwind_logic::no_tailwind_legacy_utilities::{LegacyUtility, legacy_utilities};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow Tailwind CSS utility classes that Tailwind CSS only keeps for backward compatibility.
    ///
    /// Tailwind CSS renamed several utilities, but still accepts some of the old names
    /// so that existing projects keep working.
    /// This rule reports those old names and suggests the current ones.
    /// It checks the following classes, including their variants and negative values:
    ///
    /// | Legacy class | Replacement |
    /// | --- | --- |
    /// | `bg-gradient-to-*` | `bg-linear-to-*` |
    /// | `bg-left-top`, `bg-right-top`, `bg-left-bottom`, `bg-right-bottom` | `bg-top-left`, `bg-top-right`, `bg-bottom-left`, `bg-bottom-right` |
    /// | `object-left-top`, `object-right-top`, `object-left-bottom`, `object-right-bottom` | `object-top-left`, `object-top-right`, `object-bottom-left`, `object-bottom-right` |
    /// | `max-w-screen-*` | `max-w-(--breakpoint-*)` |
    /// | `overflow-ellipsis` | `text-ellipsis` |
    /// | `decoration-slice`, `decoration-clone` | `box-decoration-slice`, `box-decoration-clone` |
    /// | `flex-grow`, `flex-grow-*` | `grow`, `grow-*` |
    /// | `flex-shrink`, `flex-shrink-*` | `shrink`, `shrink-*` |
    /// | `order-none` | `order-0` |
    /// | `break-words` | `wrap-break-word` |
    /// | `start-*` | `inset-s-*` |
    /// | `end-*` | `inset-e-*` |
    ///
    /// Some replacements were added after Tailwind CSS v4.0:
    /// `bg-top-left`, `object-top-left`, and `wrap-break-word` in v4.1,
    /// and `inset-s-*` and `inset-e-*` in v4.2.
    /// When the `tailwindcss` version range in the closest `package.json` allows a version
    /// that doesn't have the replacement yet, this rule doesn't report the legacy class.
    /// For example, `"tailwindcss": "^4.1.0"` can install v4.1, so `start-4` isn't reported.
    ///
    /// This rule doesn't read your Tailwind CSS theme. It only reports `start-*` and `end-*` classes
    /// whose values work without a theme, such as `start-4`, `start-1/2`, `start-px`, or `start-[3px]`,
    /// and `max-w-screen-*` classes that use a default breakpoint (`sm`, `md`, `lg`, `xl`, or `2xl`).
    /// This way, custom classes such as `start-date` aren't reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="flex-grow"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="hover:bg-gradient-to-r"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="grow hover:bg-linear-to-r text-ellipsis inset-s-4"></div>
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    pub NoTailwindLegacyUtilities {
        version: "2.6.0",
        name: "noTailwindLegacyUtilities",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        sources: &[RuleSource::EslintBetterTailwindcss("no-deprecated-classes").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoTailwindLegacyUtilities {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = LegacyUtility;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindLegacyUtilitiesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return Box::default();
        }
        let mut utilities = legacy_utilities(&ctx.query().tailwind_root().candidates());
        // Looking up the manifest clones it, so skip that for the common case of
        // no legacy classes.
        if !utilities.is_empty()
            && let Some((_, manifest)) = ctx
                .get_service::<Arc<ProjectLayout>>()
                .and_then(|layout| layout.find_node_manifest_for_path(ctx.file_path()))
        {
            utilities.retain(|utility| utility.is_replacement_available(&manifest));
        }
        utilities.into_iter().collect()
    }

    fn diagnostic(ctx: &RuleContext<Self>, utility: &Self::State) -> Option<RuleDiagnostic> {
        let (original, replacement) = utility.texts(&ctx.query().tailwind_root())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), utility.range()?)?,
                markup! {
                    "The Tailwind CSS class "<Emphasis>{original}</Emphasis>" is a legacy name."
                },
            )
            .note(markup! {
                "Tailwind CSS renamed this utility and only keeps the old name for backward compatibility."
            })
            .note(markup! {
                "Use "<Emphasis>{replacement}</Emphasis>" instead."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, utility: &Self::State) -> Option<HtmlRuleAction> {
        let fixed = utility.fixed_class_string(&ctx.query().tailwind_root())?;
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(&mut mutation, ctx.query().node(), &fixed)?;
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the current class name." }.to_owned(),
            mutation,
        ))
    }
}
