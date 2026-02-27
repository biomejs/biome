use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::AnyVueDirective;
use biome_rowan::AstNode;
use biome_rule_options::use_vue_consistent_v_bind_style::{
    UseVueConsistentVBindStyleOptions, VueDirectiveStyle,
};

declare_lint_rule! {
    /// Enforce a consistent style for `v-bind` in Vue templates.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-bind:foo="bar" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div :foo="bar" />
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Configures the preferred directive style. Default: `"shorthand"`.
    ///
    /// ```json,options
    /// {
    ///   "options": {
    ///     "style": "longhand"
    ///   }
    /// }
    /// ```
    ///
    /// #### Invalid
    ///
    /// ```vue,expect_diagnostic,use_options
    /// <div :foo="bar" />
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <div v-bind:foo="bar" />
    /// ```
    ///
    pub UseVueConsistentVBindStyle {
        version: "2.3.11",
        name: "useVueConsistentVBindStyle",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-bind-style").same()],
        fix_kind: biome_analyze::FixKind::Unsafe,
    }
}

impl Rule for UseVueConsistentVBindStyle {
    type Query = Ast<AnyVueDirective>;
    type State = AnyVueDirective;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentVBindStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let style = ctx.options().style();
        match node {
            AnyVueDirective::VueDirective(dir) => {
                // Only v-bind normal form
                if dir.name_token().ok()?.text_trimmed() != "v-bind" {
                    return None;
                }
                // If prefer shorthand, normal form is invalid
                if style == VueDirectiveStyle::Shorthand {
                    return Some(node.clone());
                }
                None
            }
            AnyVueDirective::VueVBindShorthandDirective(_) => {
                // If prefer longhand, shorthand is invalid
                if style == VueDirectiveStyle::Longhand {
                    return Some(node.clone());
                }
                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let prefer = ctx.options().style();
        let message = match (state, prefer) {
            (AnyVueDirective::VueDirective(_), VueDirectiveStyle::Shorthand) => {
                markup! { "Use shorthand ':' syntax instead of v-bind." }
            }
            (AnyVueDirective::VueVBindShorthandDirective(_), VueDirectiveStyle::Longhand) => {
                markup! { "Use longhand 'v-bind' syntax instead of ':'." }
            }
            _ => {
                // should be unreachable, but just in case
                debug_assert!(
                    false,
                    "Diagnostic should only be created for invalid states."
                );
                return None;
            }
        };
        let note = match (state, prefer) {
            (AnyVueDirective::VueDirective(_), VueDirectiveStyle::Shorthand) => {
                markup! { "This project prefers to use shorthand syntax for v-bind." }
            }
            (AnyVueDirective::VueVBindShorthandDirective(_), VueDirectiveStyle::Longhand) => {
                markup! { "This project prefers to use longhand syntax for v-bind." }
            }
            _ => {
                // should be unreachable, but just in case
                debug_assert!(
                    false,
                    "Diagnostic should only be created for invalid states."
                );
                return None;
            }
        };
        Some(RuleDiagnostic::new(rule_category!(), state.range(), message).note(note))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<crate::HtmlRuleAction> {
        let prefer = ctx.options().style();
        let mut mutation = biome_rowan::BatchMutationExt::begin(ctx.root());
        match (state, prefer) {
            // Convert longhand v-bind:prop to :prop
            (AnyVueDirective::VueDirective(dir), VueDirectiveStyle::Shorthand) => {
                let arg = dir.arg()?;
                let mut builder = make::vue_v_bind_shorthand_directive(arg, dir.modifiers());
                if let Some(init) = dir.initializer() {
                    builder = builder.with_initializer(init);
                }
                let new_node = builder.build();
                mutation.replace_node(
                    AnyVueDirective::VueDirective(dir.clone()),
                    AnyVueDirective::VueVBindShorthandDirective(new_node),
                );
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use the shorthand ':' syntax instead." }.to_owned(),
                    mutation,
                ))
            }
            // Convert shorthand :prop to v-bind:prop
            (AnyVueDirective::VueVBindShorthandDirective(sh), VueDirectiveStyle::Longhand) => {
                let arg = sh.arg().ok()?;
                let mut builder =
                    make::vue_directive(make::ident("v-bind"), sh.modifiers()).with_arg(arg);
                if let Some(init) = sh.initializer() {
                    builder = builder.with_initializer(init);
                }
                let new_node = builder.build();
                mutation.replace_node(
                    AnyVueDirective::VueVBindShorthandDirective(sh.clone()),
                    AnyVueDirective::VueDirective(new_node),
                );
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Use longhand 'v-bind' syntax instead." }.to_owned(),
                    mutation,
                ))
            }
            _ => None,
        }
    }
}
