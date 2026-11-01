use crate::JsRuleAction;
use crate::tailwind::{AnyTailwindClassString, apply_fixed_class_string, host_range};
use biome_analyze::{
    FixKind, Rule, RuleAction, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::BatchMutationExt;
use biome_rule_options::no_tailwind_restricted_classes::NoTailwindRestrictedClassesOptions;
use biome_tailwind_logic::no_tailwind_restricted_classes::{
    ClassStringText, RestrictedClass, restricted_classes,
};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_sorted_classes::TailwindDesignSystem;
use std::sync::Arc;

declare_lint_rule! {
    /// Disallow Tailwind CSS classes and variants that your project doesn't allow.
    ///
    /// Teams often agree to avoid some Tailwind CSS classes, for example raw colors
    /// that have a named replacement in their design system, or layout utilities
    /// such as `float-left` that they've replaced with flexbox or grid.
    /// This rule reports the classes and variants that you list in its options,
    /// and shows the message you configure for each of them.
    /// Without any classes or variants configured, this rule doesn't report anything.
    ///
    /// ## Examples
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "classes": {
    ///             "text-green-500": "Use the success color from the design system.",
    ///             "float-*": "Use flexbox or grid for layout."
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="text-green-500" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="md:float-left" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx,use_options
    /// <div className="text-success flex" />;
    /// ```
    ///
    /// ## How classes and variants are matched
    ///
    /// A class is matched without its variants, its `!`, or its modifier, which is the
    /// part after a `/`, such as `/50` in `bg-red-500/50`.
    /// So restricting `bg-red-500` also reports `hover:bg-red-500/50!`.
    /// To match the modifier as well, include it in the key, such as `bg-red-500/50`.
    /// A fraction such as `1/2` in `w-1/2` is part of the class, so restricting `w-1` doesn't report `w-1/2`.
    ///
    /// A key matches a negative value only when the key starts with `-`.
    /// So restricting `mt-4` doesn't report `-mt-4`.
    ///
    /// ## Wildcards
    ///
    /// In a key, `*` stands for a whole part of a class, not for any characters.
    /// A class such as `bg-red-500` has a utility name, `bg`, and a value, `red-500`.
    ///
    /// - `bg-*` matches every value of the `bg` utility, such as `bg-red-500` and `bg-[#fff]`.
    /// - `*-md` matches every utility whose value is `md`, such as `shadow-md` and `max-w-md`.
    /// - `[color:*]` matches every value of the `color` property written as an arbitrary property,
    ///   such as `[color:red]`.
    ///
    /// Since the value of `bg-red-500` is `red-500`, the key `bg-red-*` doesn't match it.
    /// To restrict a color in every utility, use `*-red-500`.
    ///
    /// In a variant key, `*` stands for one or more parts separated by `-`:
    ///
    /// - `group-*` matches variants that start with `group-`, such as `group-hover` and `group-focus`.
    /// - `*-hover` matches variants that end with `-hover`, such as `group-hover` and `peer-hover`.
    /// - `[*]` matches every arbitrary variant, such as `[&_p]`.
    ///
    /// Biome reports a configuration error for a `*` used in any other way, such as in `bg-*-500`.
    ///
    /// Biome splits a class into its utility name and value using the utilities that Tailwind CSS
    /// provides by default. When the
    /// [`tailwind.stylesheet`](https://biomejs.dev/reference/configuration/#tailwindstylesheet)
    /// option points at the CSS file that holds your Tailwind CSS configuration, Biome also uses
    /// the utilities defined there with `@utility`. For example, with `@utility slide-in-from-top-*`,
    /// the key `slide-in-from-top-*` matches `slide-in-from-top-4`.
    ///
    /// Biome scans your project when this rule is enabled, so that it can read the
    /// stylesheet and the files it imports.
    ///
    /// When several keys match the same class or variant, the rule uses the first one.
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
    ///
    /// ## Options
    ///
    /// ### `classes`
    ///
    /// Default: `{}`
    ///
    /// The classes to report. Each key is a class, written without variants or `!`.
    /// Each value is the message to show, or an object with these optional fields:
    ///
    /// - `message`: the message to show.
    /// - `use`: the class to use instead. When the key doesn't contain `*`,
    ///   the rule offers a fix that replaces the class with this value and keeps its variants, `!`, and modifier.
    ///   The rule doesn't offer the fix when this value is also restricted.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "classes": {
    ///             "bg-green-500": {
    ///                 "message": "Use the success color from the design system.",
    ///                 "use": "bg-success"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="hover:bg-green-500/50" />;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```jsx,use_options
    /// <div className="hover:bg-success/50 bg-green-600" />;
    /// ```
    ///
    /// ### `variants`
    ///
    /// Default: `{}`
    ///
    /// The variants to report, such as `dark` or `group-hover`.
    /// Each key is a variant, written without the `:` that follows it.
    /// A variant is matched without its modifier, so `group-hover` also matches `group-hover/menu`.
    /// The values work the same way as in `classes`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "variants": {
    ///             "dark": "This project doesn't support dark mode.",
    ///             "[*]": "Add a named variant to the Tailwind CSS configuration instead."
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="dark:bg-black" />;
    /// ```
    ///
    /// ```jsx,expect_diagnostic,use_options
    /// <div className="[&_p]:mt-2" />;
    /// ```
    ///
    /// #### Valid
    ///
    /// ```jsx,use_options
    /// <div className="hover:bg-black md:mt-2" />;
    /// ```
    ///
    pub NoTailwindRestrictedClasses {
        version: "next",
        name: "noTailwindRestrictedClasses",
        language: "jsx",
        sources: &[RuleSource::EslintBetterTailwindcss("no-restricted-classes").inspired()],
        domains: &[RuleDomain::Tailwind, RuleDomain::Project],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoTailwindRestrictedClasses {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = RestrictedClass;
    type Signals = Box<[Self::State]>;
    type Options = NoTailwindRestrictedClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        restricted_classes(&ctx.query().tailwind_root(), ctx.options())
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (_, restriction) = state.restriction(ctx.options())?;
        let kind = state.kind;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            host_range(ctx.query().node(), state.range)?,
            markup! {
                "The Tailwind CSS "{kind}" "<Emphasis>{ClassStringText(state.text(&ctx.query().tailwind_root()))}</Emphasis>" isn't allowed in this project."
            },
        );
        let diagnostic = match restriction.message() {
            Some(message) => diagnostic.note(markup! { {message} }),
            None => diagnostic.note(markup! {
                "The configuration of this rule restricts this "{kind}"."
            }),
        };
        Some(match restriction.use_instead() {
            Some(replacement) => diagnostic.note(markup! {
                "Use "<Emphasis>{replacement}</Emphasis>" instead."
            }),
            None if restriction.message().is_none() => diagnostic.note(markup! {
                "Remove it, or replace it with a "{kind}" that's allowed."
            }),
            None => diagnostic,
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let design = match ctx.get_service::<Arc<TailwindDesignSystem>>() {
            Some(design) => design.as_ref(),
            None => TailwindDesignSystem::default_ref(),
        };
        let fixed = state.fixed_class_string(
            &ctx.query().tailwind_root(),
            ctx.options(),
            design.parser_options(),
        )?;
        let (_, restriction) = state.restriction(ctx.options())?;
        let mut mutation = ctx.root().begin();
        apply_fixed_class_string(
            &mut mutation,
            ctx.query().node(),
            &fixed,
            ctx.preferred_quote(),
            ctx.preferred_jsx_quote(),
        );
        Some(RuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>{restriction.use_instead()?}</Emphasis>" instead." }
                .to_owned(),
            mutation,
        ))
    }
}
