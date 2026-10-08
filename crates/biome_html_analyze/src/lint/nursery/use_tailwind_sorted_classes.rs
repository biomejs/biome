use biome_analyze::{FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_html_syntax::HtmlAttribute;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_tailwind_sorted_classes::UseTailwindSortedClassesOptions;
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_sorted_classes::{TailwindDesignSystem, sort_class_list};
use std::sync::Arc;

use crate::HtmlRuleAction;
use crate::tailwind::apply_fixed_class_string;

declare_lint_rule! {
    /// Enforce the sorting of Tailwind CSS classes.
    ///
    /// Classes are sorted in the same order as the official [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss),
    /// which is the order in which Tailwind CSS writes their CSS. Classes that Tailwind CSS
    /// doesn't know, such as your own CSS classes, come first and keep their original order.
    ///
    /// To make the rule aware of your own theme values, utilities, and variants, point the
    /// [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet)
    /// option at the CSS file that holds your Tailwind CSS configuration. Without it, the rule
    /// only knows the default configuration, and treats classes that use your own theme values,
    /// utilities, or variants like classes that Tailwind CSS doesn't know.
    ///
    /// Biome scans your project when this rule is enabled, so that it can read the stylesheet
    /// and the files it imports. The scan happens even if `tailwind.stylesheet` isn't set,
    /// although the rule then only uses the default configuration.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <div class="px-2 foo p-4 bar"></div>
    /// ```
    ///
    /// ```html,expect_diagnostic
    /// <div class="hover:focus:m-2 foo hover:px-2 p-4"></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <div class="bar foo p-4 px-2"></div>
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    pub UseTailwindSortedClasses {
        version: "2.5.0",
        name: "useTailwindSortedClasses",
        language: "html",
        recommended: false,
        domains: &[RuleDomain::Tailwind, RuleDomain::Project],
        fix_kind: FixKind::Unsafe,
        issue_number: Some("9181"),
    }
}

impl Rule for UseTailwindSortedClasses {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = Box<str>;
    type Signals = Option<Self::State>;
    type Options = UseTailwindSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();
        if query.tailwind_has_errors() {
            return None;
        }
        let value = query.node().html_string()?.inner_string_text().ok()?;
        let design: &TailwindDesignSystem = match ctx.get_service::<Arc<TailwindDesignSystem>>() {
            Some(design) => design,
            None => TailwindDesignSystem::default_ref(),
        };
        let sorted_value = sort_class_list(&query.tailwind_root(), design);
        if sorted_value.is_empty() || value.text() == sorted_value {
            return None;
        }
        Some(sorted_value.into())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().node().html_string()?.range(),
            "These CSS classes should be sorted.",
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<HtmlRuleAction> {
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(&mut mutation, ctx.query().node(), state)?;

        Some(HtmlRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Sort the classes."
            }
            .to_owned(),
            mutation,
        ))
    }
}
