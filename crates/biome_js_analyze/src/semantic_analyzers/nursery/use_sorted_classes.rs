mod any_class_string_like;
mod class_info;
mod class_lexer;
mod options;
mod presets;
mod sort;
mod sort_config;

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{js_string_literal, js_string_literal_expression, jsx_string};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

pub use self::options::UseSortedClassesOptions;
use self::{
    any_class_string_like::AnyClassStringLike,
    presets::{get_utilities_preset, UseSortedClassesPreset},
    sort::sort_class_name,
    sort_config::SortConfig,
};

declare_rule! {
    /// Enforce the sorting of CSS utility classes.
    ///
    /// This rule implements the same sorting algorithm as [Tailwind CSS](https://tailwindcss.com/blog/automatic-class-sorting-with-prettier#how-classes-are-sorted), but supports any utility class framework including [UnoCSS](https://unocss.dev/).
    ///
    /// It is analogous to [`prettier-plugin-tailwindcss`](https://github.com/tailwindlabs/prettier-plugin-tailwindcss).
    ///
    /// NOTE: this rule is only partially implemented. Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/1274
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class="px-2 foo p-4 bar" />;
    /// ```
    ///
    /// ## Options
    ///
    /// ### Code-related
    ///
    /// ```json
    /// {
    ///     "options": {
    ///         "attributes": ["classList"],
    ///         "functions": ["clsx", "cva", "tw"]
    ///     }
    /// }
    /// ```
    ///
    /// #### attributes
    ///
    /// Classes in the `class` and `className` JSX attributes are always sorted. Use this option to add more attributes that should be sorted.
    ///
    /// #### functions
    ///
    /// If specified, strings in the indicated functions will be sorted. This is useful when working with libraries like [`clsx`](https://github.com/lukeed/clsx) or [`cva`](https://cva.style/).
    ///
    /// Tagged template literals are also supported, for example:
    ///
    /// ```js
    /// tw`px-2`;
    /// tw.div`px-2`;
    /// ```
    ///
    /// NOTE: tagged template literal support has not been implemented yet.
    ///
    /// ### Sort-related
    ///
    /// NOTE: at the moment, this rule does not support customizing the sort options. Instead, the default Tailwind CSS configuration is hard-coded.
    ///
    /// ## Differences with [Prettier](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
    ///
    /// The main key difference is that Tailwind CSS and its Prettier plugin read the `tailwind.config.js` file, which Biome can't access. Instead, Biome implements a simpler version of the configuration. The trade-offs are explained below.
    ///
    /// ### Values are not known
    ///
    /// The rule has no knowledge of values such as colors, font sizes, or spacing values, which are normally defined in a configuration file like `tailwind.config.js`. Instead, the rule matches utilities that support values in a simpler way: if they start with a known utility prefix, such as `px-` or `text-`, they're considered valid.
    ///
    /// This can result in false positives, i.e. classes that are wrongly recognized as utilities even though their values are incorrect. For example, if there's a `px-` utility defined in the configuration, it will match all of the following classes: `px-2`, `px-1337`, `px-[not-actually-valid]`, `px-literally-anything`.
    ///
    /// ### Custom additions must be specified
    ///
    /// The built-in Tailwind CSS preset (enabled by default) contains the set of utilities and variants that are available with the default configuration. More utilities and variants can be added through Tailwind CSS plugins. In Biome, these need to be manually specified in the Biome configuration file in order to "extend" the preset.
    ///
    /// ### Presets can't be modified
    ///
    /// In Tailwind CSS, core plugins (which provide the default utilities and variants) can be disabled. In Biome, however, there is no way to disable parts of a preset: it's all or nothing. A work-around is to, instead of using a preset, manually specify all utilities and variants in the Biome configuration file.
    ///
    /// ### Whitespace is collapsed
    ///
    /// The Tailwind CSS Prettier plugin preserves all original whitespace. This rule, however, collapses all whitespace (including newlines) into single spaces.
    ///
    pub(crate) UseSortedClasses {
        version: "next",
        name: "useSortedClasses",
        recommended: false,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = UseSortedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        // TODO: unsure if options are needed here. The sort config should ideally be created once
        // from the options and then reused for all queries.
        // let options = &ctx.options();
        // TODO: the sort config should already exist at this point, and be generated from the options,
        // including the preset and extended options as well.
        let sort_config = SortConfig::new(
            get_utilities_preset(&UseSortedClassesPreset::default()),
            Vec::new(),
        );

        let value = ctx.query().value()?;
        let sorted_value = sort_class_name(&value, &sort_config);
        if value.text() != sorted_value {
            Some(sorted_value)
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            "These CSS classes should be sorted.",
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        match ctx.query() {
            // TODO: make this DRYer
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let replacement = js_string_literal_expression(js_string_literal(state));
                mutation.replace_node(string_literal.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "Sort the classes."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let replacement = jsx_string(js_string_literal(state));
                mutation.replace_node(jsx_string_node.clone(), replacement);
                Some(JsRuleAction {
                    category: ActionCategory::QuickFix,
                    applicability: Applicability::Always,
                    message: markup! {
                        "Sort the classes."
                    }
                    .to_owned(),
                    mutation,
                })
            }
            _ => None,
        }
    }
}
