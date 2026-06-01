use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsImportClause, AnyJsImportLike, JsFileSource};
use biome_rowan::{AstNode, TextRange, TokenText};
use biome_rule_options::no_vue_import_compiler_macros::NoVueImportCompilerMacrosOptions;

const COMPILER_MACROS: &[&str] = &[
    "defineProps",
    "defineEmits",
    "defineExpose",
    "withDefaults",
    "defineModel",
    "defineOptions",
    "defineSlots",
];

const VUE_MODULES: &[&str] = &["vue", "@vue/runtime-core", "@vue/runtime-dom"];

declare_lint_rule! {
    /// Disallow importing Vue compiler macros.
    ///
    /// Vue compiler macros are globally available inside `<script setup>` blocks and must not be imported.
    /// Outside of `<script setup>`, compiler macros are not valid Vue runtime imports.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { defineProps } from "vue";
    /// defineProps({});
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// defineProps({});
    /// </script>
    /// ```
    ///
    pub NoVueImportCompilerMacros {
        version: "2.4.15",
        name: "noVueImportCompilerMacros",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-import-compiler-macros").same()],
        recommended: true,
        domains: &[RuleDomain::Vue],
    }
}

pub struct RuleState {
    name: TokenText,
    range: TextRange,
}

impl Rule for NoVueImportCompilerMacros {
    type Query = Ast<AnyJsImportLike>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoVueImportCompilerMacrosOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if !ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
        {
            return Vec::new();
        }

        let AnyJsImportLike::JsModuleSource(source) = ctx.query() else {
            return Vec::new();
        };

        let Ok(source_text) = source.inner_string_text() else {
            return Vec::new();
        };
        if !VUE_MODULES.contains(&source_text.text()) {
            return Vec::new();
        }

        let Some(clause) = source.parent::<AnyJsImportClause>() else {
            return Vec::new();
        };
        let Some(named_specifiers) = clause.named_specifiers() else {
            return Vec::new();
        };

        named_specifiers
            .specifiers()
            .into_iter()
            .flatten()
            .filter_map(|specifier| {
                let imported_name = specifier.imported_name()?;
                let name = imported_name.token_text_trimmed();

                COMPILER_MACROS.contains(&name.text()).then(|| RuleState {
                    name,
                    range: specifier.range(),
                })
            })
            .collect()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let RuleState { name, range } = state;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The Vue compiler macro "<Emphasis>{name.text()}</Emphasis>" should not be imported."
                },
            )
                .note(markup! {
                    "Compiler macros are automatically available inside "<Emphasis>"<script setup>"</Emphasis>" blocks."
                })
                .note(markup! {
                    "Remove this import and use the macro directly."
                }),
        )
    }
}
