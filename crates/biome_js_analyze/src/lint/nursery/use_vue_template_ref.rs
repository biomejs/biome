use crate::services::vue::VueComponent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_rowan::TextRange;
use biome_rule_options::use_vue_template_ref::UseVueTemplateRefOptions;
use biome_vue_semantic::{Namespace, Origin, Symbol, SymbolKind};

declare_lint_rule! {
    /// Require `useTemplateRef()` instead of `ref()` or `shallowRef()` to access elements of the template.
    ///
    /// A template ref gives your script access to an element or a child component of the
    /// template. You name the element with the `ref` attribute, as in `<input ref="input" />`.
    ///
    /// In `<script setup>`, Vue fills in a variable created with `ref()` or `shallowRef()` when the
    /// variable has the same name as a `ref` attribute. The link between the two is only the
    /// name, so renaming the variable silently breaks it. The variable also looks like any other
    /// piece of state, which makes it hard to tell that the template fills it in.
    ///
    /// Vue 3.5 added `useTemplateRef()`, which takes the name of the `ref` attribute as an
    /// argument. The variable can then have any name, and the call shows that the value comes from
    /// the template.
    ///
    /// This rule reports a variable declared at the top level of `<script setup>` with `ref()` or
    /// `shallowRef()` when an element of the template has a `ref` attribute with the same name.
    ///
    /// A `ref` attribute with a dynamic value, such as `:ref="(el) => (input = el)"`, isn't
    /// reported, because the function decides where the element is stored.
    ///
    /// :::note
    /// This rule needs to read the template, which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <input ref="input" />
    /// </template>
    ///
    /// <script setup>
    /// import { ref } from "vue";
    ///
    /// const input = ref();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <template>
    ///   <div ref="root" />
    /// </template>
    ///
    /// <script setup>
    /// import { shallowRef } from "vue";
    ///
    /// const root = shallowRef(null);
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <template>
    ///   <input ref="input" />
    ///   <button :ref="(el) => (button = el)">Send</button>
    /// </template>
    ///
    /// <script setup>
    /// import { ref, useTemplateRef } from "vue";
    ///
    /// const inputElement = useTemplateRef("input");
    /// const button = ref();
    /// </script>
    /// ```
    ///
    pub UseVueTemplateRef {
        version: "next",
        name: "useVueTemplateRef",
        language: "js",
        recommended: false,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-use-template-ref").inspired()],
    }
}

impl Rule for UseVueTemplateRef {
    type Query = VueComponent;
    /// A variable created with `ref()` or `shallowRef()` that has the name of
    /// a template ref.
    type State = Symbol;
    type Signals = Box<[Self::State]>;
    type Options = UseVueTemplateRefOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        if !component.has_template() {
            return Box::default();
        }
        let scope = component.instance_scope();
        component
            .declarations()
            .filter(|symbol| {
                symbol.kind() == SymbolKind::Local
                    && RefFunction::from_origin(symbol.origin()).is_some()
                    && scope
                        .lookup(Namespace::TemplateRef, symbol.name())
                        .is_some()
            })
            .collect()
    }

    fn text_range(_ctx: &RuleContext<Self>, symbol: &Self::State) -> Option<TextRange> {
        Some(symbol.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, symbol: &Self::State) -> Option<RuleDiagnostic> {
        let function = RefFunction::from_origin(symbol.origin())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                symbol.range(),
                markup! {
                    "The template ref "<Emphasis>{symbol.name()}</Emphasis>" is created with "<Emphasis>{function}</Emphasis>"."
                },
            )
            .note(markup! {
                "Vue links this variable to the element only by its name, so renaming the variable breaks the link without warning."
            })
            .note(markup! {
                "Use "<Emphasis>"useTemplateRef(\""{symbol.name()}"\")"</Emphasis>" instead, which names the element explicitly."
            }),
        )
    }
}

/// The function that creates a variable the template can fill in.
#[derive(Clone, Copy)]
enum RefFunction {
    Ref,
    ShallowRef,
}

impl RefFunction {
    fn from_origin(origin: Origin) -> Option<Self> {
        match origin {
            Origin::Ref => Some(Self::Ref),
            Origin::ShallowRef => Some(Self::ShallowRef),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for RefFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Ref => "ref()",
            Self::ShallowRef => "shallowRef()",
        })
    }
}
