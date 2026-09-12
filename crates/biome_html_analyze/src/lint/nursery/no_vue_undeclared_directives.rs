use crate::services::embedded::EmbeddedService;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_embeds::{
    VueDirectiveResolution, vue_directive_binding_name, vue_directive_name_matches_option_name,
};
use biome_html_syntax::{HtmlSyntaxToken, VueDirective};
use biome_languages::HtmlFileSource;
use biome_rule_options::no_vue_undeclared_directives::NoVueUndeclaredDirectivesOptions;

declare_lint_rule! {
    /// Disallow custom Vue directives that are not declared.
    ///
    /// Vue resolves a custom directive such as `v-highlight` at runtime. When nothing
    /// registers it, Vue logs a warning and the element silently loses the behavior the
    /// directive was supposed to add.
    ///
    /// A custom directive is considered declared when any of the following registers it:
    ///
    /// - a top-level `<script setup>` binding named after the directive, using the
    ///   camelCase form prefixed with `v`, such as `vHighlight` for `v-highlight`;
    /// - the component's `directives` option, written either in `export default`,
    ///   in `defineComponent(...)`, or in `defineOptions(...)`;
    /// - the rule's [`globals`](#globals) option, which is how a directive registered
    ///   globally with `app.directive(...)` is declared to Biome.
    ///
    /// Built-in directives such as `v-if` are never reported. Nothing is reported either
    /// when the component's options cannot be resolved statically, which happens when they
    /// use `extends`, `mixins`, a spread, or a default export that is not an object literal.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <template>
    ///     <div v-highlight></div>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// A `<script setup>` binding declares the directive:
    ///
    /// ```vue,ignore
    /// <script setup>
    /// const vHighlight = {};
    /// </script>
    ///
    /// <template><div v-highlight></div></template>
    /// ```
    ///
    /// So does the component's `directives` option:
    ///
    /// ```vue,ignore
    /// <script>
    /// export default {
    ///     directives: { highlight: {} },
    /// };
    /// </script>
    ///
    /// <template><div v-highlight></div></template>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `globals`
    ///
    /// A list of directive names that are registered globally with `app.directive(...)`.
    /// Use the name passed to `app.directive`, such as `highlight` for `v-highlight`.
    /// Names may be written in camelCase, PascalCase, or kebab-case.
    ///
    /// Default: `[]`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "globals": ["highlight"]
    ///     }
    /// }
    /// ```
    ///
    /// #### Valid (using `globals`)
    ///
    /// ```vue,use_options
    /// <template>
    ///     <div v-highlight></div>
    /// </template>
    /// ```
    pub NoVueUndeclaredDirectives {
        version: "next",
        name: "noVueUndeclaredDirectives",
        language: "html",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for NoVueUndeclaredDirectives {
    type Query = Ast<VueDirective>;
    type State = HtmlSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoVueUndeclaredDirectivesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx.source_type::<HtmlFileSource>().is_vue() {
            return None;
        }

        let directive = ctx.query();
        if directive.is_builtin() {
            return None;
        }

        let token = directive.name_token().ok()?;
        let name = token.text_trimmed();
        if ctx
            .options()
            .globals()
            .iter()
            .any(|global| vue_directive_name_matches_option_name(name, global))
        {
            return None;
        }

        match ctx
            .get_service::<EmbeddedService>()?
            .resolve_vue_directive(name)
        {
            VueDirectiveResolution::Undeclared => Some(token),
            VueDirectiveResolution::Declared | VueDirectiveResolution::Unknown => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, token: &Self::State) -> Option<RuleDiagnostic> {
        let binding_name = vue_directive_binding_name(token.text_trimmed())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                token.text_trimmed_range(),
                markup! {
                    "The custom directive "<Emphasis>{token.text_trimmed()}</Emphasis>" is undeclared."
                },
            )
            .note(markup! {
                "Declare "<Emphasis>{binding_name}</Emphasis>" in <script setup>, register the directive in the component's directives option, or list it in the rule's globals option."
            }),
        )
    }
}
