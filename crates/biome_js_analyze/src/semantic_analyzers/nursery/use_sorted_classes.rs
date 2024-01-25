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
use lazy_static::lazy_static;

use crate::JsRuleAction;

pub use self::options::UtilityClassSortingOptions;
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
    ///
    /// :::caution
    /// ## Important notes
    ///
    /// This rule is a work in progress, and is only partially implemented. Progress is being tracked in the following GitHub issue: https://github.com/biomejs/biome/issues/1274
    ///
    /// Currently, utility class sorting is **not part of the formatter**, and is implemented as a linter rule instead, with an automatic fix. The fix is, at this stage, classified as unsafe. This means that **it won't be applied automatically** as part of IDE actions such as "fix on save".
    ///
    /// We appreciate any feedback on this rule, and encourage you to try it out and report any issues you find.
    ///
    /// **Please read this entire documentation page before reporting an issue.**
    ///
    /// Notably, keep in mind that the following features are not supported yet:
    ///
    /// - Variant sorting.
    /// - Custom utilitites and variants (such as ones introduced by Tailwind CSS plugins). Only the default Tailwind CSS configuration is supported.
    /// - Options such as `prefix` and `separator`.
    /// - Tagged template literals.
    /// - Object properties (e.g. in `clsx` calls).
    ///
    /// Please don't report issues about these features.
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
    /// ```js,ignore
    /// clsx("px-2 foo p-4 bar", {
    ///     "block mx-4": condition,
    /// });
    /// ```
    ///
    /// Tagged template literals are also supported, for example:
    ///
    /// ```js,ignore
    /// tw`px-2`;
    /// tw.div`px-2`;
    /// ```
    ///
    /// :::caution
    /// Tagged template literal support has not been implemented yet.
    /// :::
    ///
    /// ### Sort-related
    ///
    /// :::caution
    /// At the moment, this rule does not support customizing the sort options. Instead, the default Tailwind CSS configuration is hard-coded.
    /// :::
    ///
    /// ## Differences with [Prettier](https://github.com/tailwindlabs/prettier-plugin-tailwindcss)
    ///
    /// The main key difference is that Tailwind CSS and its Prettier plugin read and execute the `tailwind.config.js` JavaScript file, which Biome can't do. Instead, Biome implements a simpler version of the configuration. The trade-offs are explained below.
    ///
    /// ### Values are not known
    ///
    /// The rule has no knowledge of values such as colors, font sizes, or spacing values, which are normally defined in a configuration file like `tailwind.config.js`. Instead, the rule matches utilities that support values in a simpler way: if they start with a known utility prefix, such as `px-` or `text-`, they're considered valid.
    ///
    /// This has two implications:
    ///
    /// - False positives: classes can be wrongly recognized as utilities even though their values are incorrect. For example, if there's a `px-` utility defined in the configuration, it will match all of the following classes: `px-2`, `px-1337`, `px-[not-actually-valid]`, `px-literally-anything`.
    /// - No distinction between different utilities that share the same prefix: for example, `text-red-500` and `text-lg` are both interpreted as the same type of utility by this rule, even though the former refers to a color and the latter to a font size. This results in all utilities that share the same prefix being sorted together, regardless of their actual values.
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
    /// This is a deliberate decision. We're unsure about this behavior, and would appreciate feedback on it. If this is a problem for you, please share a detailed explanation of your use case in [the GitHub issue](https://github.com/biomejs/biome/issues/1274).
    ///
    pub(crate) UseSortedClasses {
        version: "next",
        name: "useSortedClasses",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

lazy_static! {
    static ref SORT_CONFIG: SortConfig = SortConfig::new(
        get_utilities_preset(&UseSortedClassesPreset::default()),
        Vec::new(),
    );
}

impl Rule for UseSortedClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = UtilityClassSortingOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();

        if node.should_visit(options).is_some() {
            if let Some(value) = node.value() {
                let sorted_value = sort_class_name(&value, &SORT_CONFIG);
                if value.text() != sorted_value {
                    return Some(sorted_value);
                }
            }
        }
        None
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
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let replacement = js_string_literal_expression(js_string_literal(state));
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let replacement = jsx_string(js_string_literal(state));
                mutation.replace_node(jsx_string_node.clone(), replacement);
            }
            AnyClassStringLike::JsTemplateChunkElement(_) => return None,
        };

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Sort the classes."
            }
            .to_owned(),
            mutation,
        })
    }
}
