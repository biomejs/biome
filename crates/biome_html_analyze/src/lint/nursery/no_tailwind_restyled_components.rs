use crate::tailwind::host_range;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_syntax::{AnyHtmlTagName, HtmlAttribute, element_ext::AnyHtmlTagElement};
use biome_languages::HtmlFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_tailwind_restyled_components::NoTailwindRestyledComponentsOptions;
use biome_tailwind_logic::no_tailwind_restyled_components::restyled_component_ranges;
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
    /// checked. The rule checks static `class` attributes.
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
    /// In Svelte, `Card` also matches members such as `Card.Root`.
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
        if !element.is_custom_component()
            && !element
                .tag_name()
                .is_some_and(|name| name.text().contains('-'))
        {
            return vec![];
        }
        let Ok(tag_name) = element.name() else {
            return vec![];
        };
        let is_svelte = ctx.source_type::<HtmlFileSource>().is_svelte();
        let allowances: Vec<_> = ctx
            .options()
            .allow()
            .iter()
            .filter(|allow| {
                allow.components.matches(|name| match &tag_name {
                    AnyHtmlTagName::HtmlMemberName(member) => {
                        member.syntax().text_trimmed() == name
                            || is_svelte
                                && member
                                    .syntax()
                                    .first_token()
                                    .is_some_and(|token| token.text_trimmed() == name)
                    }
                    _ => tag_name
                        .token_text_trimmed()
                        .is_some_and(|tag| tag.text() == name),
                })
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
