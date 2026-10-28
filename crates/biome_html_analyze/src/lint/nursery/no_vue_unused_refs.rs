use crate::services::vue::VueSemantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_html_syntax::HtmlRoot;
use biome_rule_options::no_vue_unused_refs::NoVueUnusedRefsOptions;
use biome_vue_semantic::{BlockKind, Namespace, ScopeKind, Symbol, SymbolKind, VueSemanticModel};

declare_lint_rule! {
    /// Disallow template refs that are never used.
    ///
    /// A `ref` attribute gives a name to an element of the template, so that the
    /// component's code can reach that element through `$refs` or `useTemplateRef()`.
    /// When nothing reads that name, the attribute has no effect. It is usually left
    /// behind after the code that used it was removed.
    ///
    /// A template ref counts as used when:
    ///
    /// - the template or the script reads it from `$refs`, as in `$refs.input` or
    ///   `this.$refs.input`;
    /// - the script passes its name to `useTemplateRef()`, as in `useTemplateRef("input")`;
    /// - the component exposes a variable with the same name from `<script setup>` or
    ///   from the object that `setup()` returns. Vue stores the element in that variable.
    ///
    /// Only refs written as plain text, such as `ref="input"`, are checked. A ref bound
    /// with `:ref` is never reported.
    ///
    /// :::note
    /// This rule needs to read both the template and the scripts of the component,
    /// which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`.
    /// :::
    ///
    /// The rule reports nothing when it can't know which refs are used. That is the case
    /// when:
    ///
    /// - a ref is read with a name that is not written as a string, as in `$refs[name]`
    ///   or `useTemplateRef(name)`;
    /// - `$refs` itself is stored or handed to other code, as in `const refs = this.$refs`;
    /// - a `<script src>` block loads part of the component from another file.
    ///
    /// Refs read from another component, for example through `$parent.$refs`, are not
    /// detected.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <input ref="input" />
    /// </template>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script>
    /// export default {
    ///   mounted() {
    ///     this.$refs.input.focus();
    ///   },
    /// };
    /// </script>
    ///
    /// <template>
    ///   <input ref="input" />
    ///   <button ref="submit">Submit</button>
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue,ignore
    /// <script setup>
    /// import { ref, useTemplateRef } from "vue";
    ///
    /// const input = useTemplateRef("input");
    /// const submit = ref(null);
    /// </script>
    ///
    /// <template>
    ///   <input ref="input" />
    ///   <button ref="submit">Submit</button>
    ///   <span ref="label" @click="$refs.label.focus()"></span>
    /// </template>
    /// ```
    ///
    pub NoVueUnusedRefs {
        version: "2.6.4",
        name: "noVueUnusedRefs",
        language: "html",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-unused-refs").same()],
    }
}

impl Rule for NoVueUnusedRefs {
    type Query = VueSemantic<HtmlRoot>;
    /// A template ref nothing reads.
    type State = Symbol;
    type Signals = Box<[Self::State]>;
    type Options = NoVueUnusedRefsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let refs: Vec<_> = model
            .symbols()
            .filter(|symbol| symbol.kind() == SymbolKind::TemplateRef)
            .collect();
        refs.iter()
            .filter(|template_ref| {
                let name = template_ref.name();
                // Vue stores every element with the same name under one key,
                // so a use of any of them uses all of them.
                template_ref.has_complete_uses()
                    && !refs
                        .iter()
                        .any(|other| other.name() == name && other.uses().next().is_some())
                    && !is_setup_binding(model, name)
            })
            .cloned()
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, template_ref: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                template_ref.host_range(),
                markup! {
                    "The template ref "<Emphasis>{template_ref.name()}</Emphasis>" is never used."
                },
            )
            .note(markup! {
                "Nothing reads this element through "<Emphasis>"$refs"</Emphasis>" or "<Emphasis>"useTemplateRef()"</Emphasis>", so the "<Emphasis>"ref"</Emphasis>" attribute has no effect."
            })
            .note(markup! {
                "Remove the "<Emphasis>"ref"</Emphasis>" attribute, or use the ref."
            }),
        )
    }
}

/// Returns `true` when the template can see a variable called `name` that a
/// setup function exposes: a top-level binding of `<script setup>`, or a
/// member of the object `setup()` returns.
///
/// Vue stores the element of a ref in such a variable when the names match.
fn is_setup_binding(model: &VueSemanticModel, name: &str) -> bool {
    let Some(template) = model
        .blocks()
        .find(|block| block.kind() == BlockKind::Template)
    else {
        return false;
    };
    template.element().scope().ancestors().any(|scope| {
        matches!(scope.kind(), ScopeKind::Module(_) | ScopeKind::Instance(_))
            && scope.symbols().any(|symbol| {
                symbol.namespace() == Namespace::Binding
                    && symbol.name() == name
                    && matches!(
                        symbol.kind(),
                        SymbolKind::Local | SymbolKind::Import | SymbolKind::SetupReturn
                    )
                    && !symbol.is_type_only()
            })
    })
}
