use crate::tailwind::host_range;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_embeds::components::component_name_segments;
use biome_html_syntax::{HtmlAttribute, element_ext::AnyHtmlTagElement};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_tailwind_restyled_components::NoTailwindRestyledComponentsOptions;
use biome_tailwind_logic::no_tailwind_restyled_components::{
    matches_component_name, restyled_component_ranges,
};
use biome_tailwind_logic::syntax_service::TailwindSyntax;

declare_lint_rule! {
    /// Disallow Tailwind utilities that override the appearance of components.
    ///
    /// A design system should own its components' appearance. Use component props
    /// for supported visual variants instead of overriding them with utility classes.
    ///
    /// The rule reports utilities in these categories:
    ///
    /// - `color`, such as `bg-red-500` and `text-white`
    /// - `typography`, such as `text-sm` and `font-bold`
    /// - `spacing`, such as `p-4` and `gap-2`
    /// - `shape`, such as `rounded-none` and `border-2`
    /// - `effects`, such as `shadow` and `opacity-50`
    /// - `motion`, such as `transition` and `animate-spin`
    ///
    /// Variants, important modifiers, and arbitrary values don't change the category, so
    /// `hover:bg-red-500`, `rounded-none!`, and `p-[3px]` are reported too. Arbitrary
    /// properties that set these styles, such as `[font-size:14px]`, are also reported.
    /// Other utilities, such as sizing, positioning, and margins, are ignored.
    ///
    /// Components are elements with capitalized names such as `MyButton` and custom
    /// elements with hyphenated names such as `my-button`. Native elements are not
    /// checked. The rule checks static `class` attributes. In Astro, Svelte, and Vue
    /// files, it also checks class expressions such as `class={...}`, Astro's
    /// `class:list={...}`, and Vue's `:class="..."`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```html,expect_diagnostic
    /// <my-button class="rounded-none"></my-button>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```html
    /// <my-button variant="danger" class="mt-4 w-full"></my-button>
    /// <button class="rounded-none"></button>
    /// ```
    ///
    /// ## Recognized class strings
    ///
    /// This rule checks the attributes and functions recognized by the top-level
    /// [`tailwind` configuration](https://biomejs.dev/reference/configuration/#tailwind).
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
    /// A name matches any segment of a member name, so both `Card` and `Root` match
    /// `Card.Root`. Dotted names such as `Card.Root` match consecutive segments.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "allow": [
    ///       { "components": ["my-button", "MyButton"], "categories": ["shape"], "classes": ["hover:shadow-lg"] }
    ///     ]
    ///   }
    /// }
    /// ```
    ///
    /// ```html,use_options
    /// <my-button class="rounded-none hover:shadow-lg"></my-button>
    /// <MyButton class="rounded-none hover:shadow-lg"></MyButton>
    /// ```
    ///
    pub NoTailwindRestyledComponents {
        version: "next",
        name: "noTailwindRestyledComponents",
        language: "html",
        domains: &[RuleDomain::Tailwind],
        recommended: false,
        issue_number: Some("11342"),
        sources: &[RuleSource::EslintShadcn("no-restyle").inspired()],
    }
}

impl Rule for NoTailwindRestyledComponents {
    type Query = TailwindSyntax<HtmlAttribute>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = NoTailwindRestyledComponentsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx.query().tailwind_has_errors() {
            return vec![];
        }
        let attribute = ctx.query().node();
        let Some(element) = attribute
            .syntax()
            .parent()
            .and_then(|list| list.parent())
            .and_then(AnyHtmlTagElement::cast)
        else {
            return vec![];
        };
        let Some(segments) = component_name_segments(&element) else {
            return vec![];
        };
        let allowances: Vec<_> = ctx
            .options()
            .allow()
            .iter()
            .filter(|allow| {
                allow
                    .components
                    .matches(|name| matches_component_name(&segments, name))
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
