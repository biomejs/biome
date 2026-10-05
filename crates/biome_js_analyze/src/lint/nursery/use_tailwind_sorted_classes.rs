mod sort;

use self::sort::{get_sort_class_name_range, get_template_literal_space_context, sort_classes};
use crate::JsRuleAction;
use crate::tailwind::{AnyTailwindClassString, apply_fixed_class_string};
use biome_analyze::{FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_tailwind_sorted_classes::UseTailwindSortedClassesOptions;
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_sorted_classes::TailwindDesignSystem;
use std::sync::Arc;

declare_lint_rule! {
    /// Enforce the sorting of Tailwind CSS classes.
    ///
    /// Classes are sorted in the same order as the official [Tailwind CSS Prettier plugin](https://github.com/tailwindlabs/prettier-plugin-tailwindcss),
    /// which is the order in which Tailwind CSS writes their CSS. Classes that Tailwind CSS
    /// doesn't know, such as your own CSS classes, come first and keep their original order.
    ///
    /// :::caution
    /// ## Important notes
    ///
    /// This rule is a work in progress. Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/1274
    ///
    /// Class sorting is **not part of the formatter**. It's a lint rule with a fix that is classified as unsafe, which means that **it won't be applied automatically** as part of IDE actions such as "fix on save".
    ///
    /// To make the rule aware of your own theme values, utilities, and variants, point the [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet) option at the CSS file that holds your Tailwind CSS configuration. Without it, the rule only knows the default configuration, and treats classes that use your own theme values, utilities, or variants like classes that Tailwind CSS doesn't know.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="px-2 foo p-4 bar" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="hover:focus:m-2 foo hover:px-2 p-4" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class="foo bar p-4 px-2" />;
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    /// ## Differences with [Prettier](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
    ///
    /// ### Whitespace is collapsed
    ///
    /// The Tailwind CSS Prettier plugin preserves all original whitespace. This rule, however, collapses all whitespace (including newlines) into single spaces.
    ///
    /// This is a deliberate decision. We're unsure about this behavior, and would appreciate feedback on it. If this is a problem for you, please share a detailed explanation of your use case in [the GitHub issue](https://github.com/biomejs/biome/issues/1274).
    ///
    pub UseTailwindSortedClasses {
        version: "1.6.0",
        name: "useTailwindSortedClasses",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Tailwind],
        fix_kind: FixKind::Unsafe,
        issue_number: Some("1274"),
    }
}

impl Rule for UseTailwindSortedClasses {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = Box<str>;
    type Signals = Option<Self::State>;
    type Options = UseTailwindSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let query = ctx.query();
        let node = query.node();
        let value = node.value()?;
        let template_ctx = get_template_literal_space_context(node);
        let design: &TailwindDesignSystem = match ctx.get_service::<Arc<TailwindDesignSystem>>() {
            Some(design) => design,
            None => TailwindDesignSystem::default_ref(),
        };
        let sorted_value = sort_classes(query, &value, template_ctx.as_ref(), design)?;
        if sorted_value.is_empty() || value.text() == sorted_value {
            return None;
        }
        Some(sorted_value.into())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query().node();

        // Calculate the range offset to account for the ignored prefix and postfix.
        let sort_range = if let Some(value) = node.value() {
            let range = node.range();
            let template_ctx = get_template_literal_space_context(node);
            let real_sort_range = get_sort_class_name_range(&value, &range, &template_ctx);
            real_sort_range.unwrap_or(range)
        } else {
            node.range()
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            sort_range,
            "These CSS classes should be sorted.",
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(
            &mut mutation,
            ctx.query().node(),
            state,
            ctx.preferred_quote(),
            ctx.preferred_jsx_quote(),
        );

        Some(JsRuleAction::new(
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
