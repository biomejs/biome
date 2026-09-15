use crate::tailwind::{AnyTailwindClassString, host_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsxAttribute;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_tailwind_restyled_components::NoTailwindRestyledComponentsOptions;
use biome_tailwind_logic::no_tailwind_restyled_components::restyled_component_ranges;
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use smallvec::SmallVec;

declare_lint_rule! {
    /// Disallow Tailwind utilities that restyle components at their call sites.
    ///
    /// A design system should own its components' appearance. Use component props
    /// for supported visual variants instead of overriding them with utility classes.
    ///
    /// This rule reports color, typography, spacing, shape, effects, and motion
    /// utilities on components. Variants,
    /// important modifiers, and arbitrary values in these families are also checked.
    /// Explicit arbitrary CSS properties for these styles are checked as well.
    /// No categories are allowed by default. Layout utilities without a category,
    /// such as sizing, positioning, and margins, are ignored. Native elements are not checked.
    ///
    /// Components include capitalized names, member names such as `UI.Button`, and
    /// custom elements with hyphenated names. The rule checks `class` and `className`,
    /// including literal branches of conditionals and common class helper calls.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <Button className="rounded-none" />;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Button variant="danger" className="mt-4 w-full" />;
    /// <button className="rounded-none" />;
    /// ```
    ///
    /// ## Options
    ///
    /// ### allow
    ///
    /// Default: `[]`.
    ///
    /// Allows categories or classes on selected components. Each entry contains:
    ///
    /// - `components`: a component name, an array of names, or `"*"` for all components.
    /// - `categories`: any of `color`, `typography`, `spacing`, `shape`, `effects`, or `motion`. Default: `[]`.
    /// - `classes`: exact classes, including variants and modifiers. Default: `[]`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allow": [
    ///       { "components": "Button", "categories": ["shape"], "classes": ["hover:shadow-lg"] }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ```jsx,use_options
    /// <Button className="rounded-none hover:shadow-lg" />;
    /// ```
    ///
    pub NoTailwindRestyledComponents {
        version: "next",
        name: "noTailwindRestyledComponents",
        language: "jsx",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        issue_number: Some("11342"),
        sources: &[RuleSource::EslintShadcn("no-restyle").inspired()],
    }
}

impl Rule for NoTailwindRestyledComponents {
    type Query = TailwindSyntax<AnyTailwindClassString>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoTailwindRestyledComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let attribute = ctx
            .query()
            .node()
            .syntax()
            .ancestors()
            .find_map(JsxAttribute::cast);
        let element = attribute.and_then(|attribute| {
            if !matches!(
                attribute.name_value_token().ok()?.text_trimmed(),
                "class" | "className"
            ) {
                return None;
            }
            attribute
                .syntax()
                .parent()?
                .parent()
                .and_then(AnyJsxElement::cast)
        });
        let Some(element) = element else {
            return vec![];
        };
        if !element.is_custom_component() && !element.is_custom_element() {
            return vec![];
        }
        if ctx.query().tailwind_has_errors() {
            return vec![];
        }
        let allowances: SmallVec<[_; 2]> = ctx
            .options()
            .allow()
            .iter()
            .filter(|allow| {
                allow
                    .components
                    .matches(|name| element.matches_name(name).unwrap_or(false))
            })
            .collect();
        restyled_component_ranges(&ctx.query().tailwind_root().candidates(), &allowances)
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                host_range(ctx.query().node(), *range)?,
                markup! { "This Tailwind utility restyles a component." },
            )
            .note(markup! { "The design system should manage the component's appearance." })
            .note(markup! { "Use a supported component variant or move this style into the component's definition." }),
        )
    }
}
