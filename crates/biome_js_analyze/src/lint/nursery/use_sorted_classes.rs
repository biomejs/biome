mod any_class_string_like;
mod class_info;
mod class_lexer;
mod options;
mod presets;
mod sort;
mod sort_config;
mod tailwind_preset;

use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_rowan::{AstNode, BatchMutationExt};
use presets::get_config_preset;
use std::sync::LazyLock;

use crate::JsRuleAction;

pub use self::options::UtilityClassSortingOptions;
use self::{
    any_class_string_like::AnyClassStringLike, presets::UseSortedClassesPreset,
    sort::get_sort_class_name_range, sort::should_ignore_postfix, sort::should_ignore_prefix,
    sort::sort_class_name, sort_config::SortConfig,
};

declare_lint_rule! {
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
    /// - Screen variant sorting (e.g. `md:`, `max-lg:`). Only static, dynamic and arbitrary variants are supported.
    /// - Custom utilitites and variants (such as ones introduced by Tailwind CSS plugins). Only the default Tailwind CSS configuration is supported.
    /// - Options such as `prefix` and `separator`.
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
    /// ```jsx,expect_diagnostic
    /// <div class="hover:focus:m-2 foo hover:px-2 p-4">
    /// ```
    ///
    /// ## Options
    ///
    /// ### Code-related
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "attributes": ["classList"],
    ///         "functions": ["clsx", "cva", "tw", "tw.*"]
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
    /// ```js,expect_diagnostic,use_options
    /// clsx("px-2 foo p-4 bar", {
    ///     "some-css-class": condition,
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// clsx("some-css-class", {
    ///     "block mx-4": condition,
    /// });
    /// ```
    ///
    /// Tagged template literals are also supported, for example:
    ///
    /// ```js,use_options
    /// tw`px-2`;
    /// tw.div`px-2`;
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// tw`px-2 foo p-4 bar`;
    /// ```
    ///
    /// **Since v2.0.0**, tagged template literals like `` tw.div`...` `` are supported by setting `tw.*`:
    ///
    /// ```js,expect_diagnostic,use_options
    /// tw.div`px-2 foo p-4 bar`;
    /// ```
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
    /// - **False positives:** classes can be wrongly recognized as utilities even though their values are incorrect.
    ///   For example, if there's a `px-` utility defined in the configuration, it will match all of the following classes:
    ///   `px-2`, `px-1337`, `px-[not-actually-valid]`, `px-literally-anything`.
    ///
    /// - **No distinction between different utilities that share the same prefix:** for example,
    ///   `text-red-500` and `text-lg` are both interpreted as the same type of utility by this rule,
    ///    even though the former refers to a color and the latter to a font size. This results in all
    ///    utilities that share the same prefix being sorted together, regardless of their actual values.
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
    pub UseSortedClasses {
        version: "1.6.0",
        name: "useSortedClasses",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

static SORT_CONFIG: LazyLock<SortConfig> =
    LazyLock::new(|| SortConfig::new(&get_config_preset(&UseSortedClassesPreset::default())));

impl Rule for UseSortedClasses {
    type Query = Ast<AnyClassStringLike>;
    type State = String;
    type Signals = Option<Self::State>;
    type Options = Box<UtilityClassSortingOptions>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let options = ctx.options();
        let node = ctx.query();

        if node.should_visit(options)? {
            if let Some(value) = node.value() {
                // Check if the class should be ignored.
                let ignore_prefix = should_ignore_prefix(node);
                let ignore_postfix = should_ignore_postfix(node);
                let sorted_value =
                    sort_class_name(&value, &SORT_CONFIG, ignore_prefix, ignore_postfix);
                if sorted_value.is_empty() {
                    return None;
                }
                if value.text() != sorted_value {
                    return Some(sorted_value);
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        // Calculate the range offset to account for the ignored prefix and postfix.
        let sort_range = if let Some(value) = node.value() {
            let range = node.range();
            let ignore_prefix = should_ignore_prefix(node);
            let ignore_postfix = should_ignore_postfix(node);
            let real_sort_range =
                get_sort_class_name_range(&value, &range, ignore_prefix, ignore_postfix);
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
        match ctx.query() {
            AnyClassStringLike::JsStringLiteralExpression(string_literal) => {
                let replacement =
                    js_string_literal_expression(if ctx.as_preferred_quote().is_double() {
                        js_string_literal(state)
                    } else {
                        js_string_literal_single_quotes(state)
                    });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsLiteralMemberName(string_literal) => {
                let replacement = js_literal_member_name(if ctx.as_preferred_quote().is_double() {
                    js_string_literal(state)
                } else {
                    js_string_literal_single_quotes(state)
                });
                mutation.replace_node(string_literal.clone(), replacement);
            }
            AnyClassStringLike::JsxString(jsx_string_node) => {
                let replacement = jsx_string(if ctx.as_preferred_jsx_quote().is_double() {
                    js_string_literal(state)
                } else {
                    js_string_literal_single_quotes(state)
                });
                mutation.replace_node(jsx_string_node.clone(), replacement);
            }
            AnyClassStringLike::JsTemplateChunkElement(chunk) => {
                let replacement = js_template_chunk_element(js_template_chunk(state));
                mutation.replace_node(chunk.clone(), replacement);
            }
        };

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
