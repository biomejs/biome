use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_html_factory::make;
use biome_html_syntax::{AnyVueDirective, HtmlSyntaxKind, HtmlSyntaxToken, T};
use biome_rowan::AstNode;
use biome_rule_options::use_vue_consistent_v_on_style::{
    UseVueConsistentVOnStyleOptions, VueDirectiveStyle,
};

declare_lint_rule! {
    /// Enforce a consistent style for `v-on` in Vue templates.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <div v-on:click="onClick" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <div @click="onClick" />
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Configures the preferred directive style. Accepts `"shorthand"` or `"longhand"`. Default: `"shorthand"`.
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
    /// <div @click="onClick" />
    /// ```
    ///
    /// #### Valid
    ///
    /// ```vue,use_options
    /// <div v-on:click="onClick" />
    /// ```
    ///
    pub UseVueConsistentVOnStyle {
        version: "2.3.11",
        name: "useVueConsistentVOnStyle",
        language: "html",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("v-on-style").same()],
        fix_kind: biome_analyze::FixKind::Unsafe,
    }
}

impl Rule for UseVueConsistentVOnStyle {
    type Query = Ast<AnyVueDirective>;
    type State = AnyVueDirective;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentVOnStyleOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let style = ctx.options().style();
        match node {
            AnyVueDirective::VueDirective(dir) => {
                // Only v-on normal form
                if dir.name_token().ok()?.text_trimmed() != "v-on" {
                    return None;
                }
                // If prefer shorthand, normal form is invalid
                if style == VueDirectiveStyle::Shorthand {
                    return Some(node.clone());
                }
                None
            }
            AnyVueDirective::VueVOnShorthandDirective(_) => {
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
                markup! { "Use shorthand '@' syntax instead of 'v-on'." }
            }
            (AnyVueDirective::VueVOnShorthandDirective(_), VueDirectiveStyle::Longhand) => {
                markup! { "Use longhand 'v-on' syntax instead of '@'." }
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
                markup! { "This project prefers to use shorthand syntax for v-on." }
            }
            (AnyVueDirective::VueVOnShorthandDirective(_), VueDirectiveStyle::Longhand) => {
                markup! { "This project prefers to use longhand syntax for v-on." }
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
            // Convert longhand v-on:event to @event
            (AnyVueDirective::VueDirective(dir), VueDirectiveStyle::Shorthand) => {
                let modifiers = dir.modifiers();
                let initializer = dir.initializer();
                // Build VueVOnShorthandDirective: at_token '@' + arg + modifiers (+initializer)
                let at_token = HtmlSyntaxToken::new_detached(T![@], "@", [], []);
                let inner = dir.arg()?.arg().ok()?;
                let mut builder = make::vue_v_on_shorthand_directive(at_token, inner, modifiers);
                if let Some(init) = initializer {
                    builder = builder.with_initializer(init);
                }
                let new_node = builder.build();
                mutation.replace_node(
                    AnyVueDirective::VueDirective(dir.clone()),
                    AnyVueDirective::VueVOnShorthandDirective(new_node),
                );
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Rewrite to shorthand '@' syntax." }.to_owned(),
                    mutation,
                ))
            }
            // Convert shorthand @event to v-on:event
            (AnyVueDirective::VueVOnShorthandDirective(sh), VueDirectiveStyle::Longhand) => {
                let name_token = make::ident("v-on");
                let modifiers = sh.modifiers();
                let initializer = sh.initializer();
                let any_arg = sh.arg().ok()?;

                // Build VueDirectiveArgument with ':' token and same inner arg
                let colon = HtmlSyntaxToken::new_detached(HtmlSyntaxKind::COLON, ":", [], []);
                let arg = make::vue_directive_argument(colon, any_arg);
                let mut builder = make::vue_directive(name_token, modifiers).with_arg(arg);
                if let Some(init) = initializer {
                    builder = builder.with_initializer(init);
                }
                let new_node = builder.build();
                mutation.replace_node(
                    AnyVueDirective::VueVOnShorthandDirective(sh.clone()),
                    AnyVueDirective::VueDirective(new_node),
                );
                Some(biome_analyze::RuleAction::new(
                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                    ctx.metadata().applicability(),
                    markup! { "Rewrite to longhand 'v-on' syntax." }.to_owned(),
                    mutation,
                ))
            }
            _ => None,
        }
    }
}
