use crate::services::embedded::EmbeddedService;
use biome_analyze::context::RuleContext;
use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_embeds::{VueDirectiveResolution, vue_directive_binding_name};
use biome_html_syntax::{HtmlSyntaxToken, VueDirective};
use biome_languages::HtmlFileSource;
use biome_rule_options::no_undeclared_variables::NoUndeclaredVariablesOptions;

declare_lint_rule! {
    /// Prevents the usage of custom Vue directives that have not been declared.
    ///
    /// A custom directive is considered declared when any of the following registers it:
    ///
    /// - a top-level `<script setup>` binding named after the directive, using the
    ///   camelCase form prefixed with `v`, such as `vHighlight` for `v-highlight`;
    /// - the component's `directives` option, written either in `export default`,
    ///   in `defineComponent(...)`, or in `defineOptions(...)`;
    /// - the [`javascript.globals`](https://biomejs.dev/reference/configuration/#javascriptglobals)
    ///   configuration, which is how a directive registered globally with
    ///   `app.directive(...)` is declared to Biome.
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
    /// A directive registered globally is declared through `javascript.globals`, under the
    /// name its `<script setup>` binding would have:
    ///
    /// ```json,full_options
    /// {
    ///     "javascript": {
    ///         "globals": ["vHighlight"]
    ///     }
    /// }
    /// ```
    ///
    /// ```vue,use_options
    /// <template>
    ///     <div v-highlight></div>
    /// </template>
    /// ```
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
    pub NoUndeclaredVariables {
        version: "next",
        name: "noUndeclaredVariables",
        language: "html",
        sources: &[RuleSource::Eslint("no-undef").inspired()],
        recommended: false,
        severity: Severity::Error,
    }
}

impl Rule for NoUndeclaredVariables {
    type Query = Ast<VueDirective>;
    type State = HtmlSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoUndeclaredVariablesOptions;

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
        let binding_name = vue_directive_binding_name(name)?;
        if ctx.is_global(&binding_name) {
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
                "Declare "<Emphasis>{binding_name}</Emphasis>" in <script setup>, register the directive in the component's directives option, or add it to javascript.globals."
            }),
        )
    }
}
