use std::ops::Not;

use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_vue_consistent_define_props_declaration::{
    DeclarationStyle, UseVueConsistentDefinePropsDeclarationOptions,
};

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Enforce a consistent declaration style for Vue's `defineProps` macro.
    ///
    /// Vue accepts two ways to declare component properties: a TypeScript type passed between
    /// angle brackets, or a runtime object passed as an argument. This rule enforces one style
    /// throughout the project. The default style is `type`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// const props = defineProps({
    ///   kind: { type: String },
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const props = defineProps<{
    ///   kind: string;
    /// }>();
    /// </script>
    /// ```
    ///
    /// ## Options
    ///
    /// ### `style`
    ///
    /// Selects `type` or `runtime` declarations. Defaults to `type`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "style": "runtime"
    ///     }
    /// }
    /// ```
    ///
    /// With `runtime`, a type-based declaration is invalid:
    ///
    /// ```vue,use_options,expect_diagnostic
    /// <script setup lang="ts">
    /// const props = defineProps<{ kind: string }>();
    /// </script>
    /// ```
    ///
    /// A runtime declaration is valid:
    ///
    /// ```vue,use_options
    /// <script setup lang="ts">
    /// const props = defineProps({ kind: { type: String } });
    /// </script>
    /// ```
    ///
    pub UseVueConsistentDefinePropsDeclaration {
        version: "2.3.11",
        name: "useVueConsistentDefinePropsDeclaration",
        language: "js",
        sources: &[RuleSource::EslintVueJs("define-props-declaration").same()],
        recommended: false,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for UseVueConsistentDefinePropsDeclaration {
    type Query = Semantic<JsCallExpression>;
    type State = DeclarationError;
    type Signals = Option<Self::State>;
    type Options = UseVueConsistentDefinePropsDeclarationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        if ctx
            .source_type::<JsFileSource>()
            .as_embedding_kind()
            .is_vue_setup()
            .not()
        {
            return None;
        }

        let node = ctx.query();
        if !is_vue_compiler_macro_call(node, ctx.model(), "defineProps") {
            return None;
        }

        let is_type_declaration = is_type_declaration(node);
        let is_runtime_declaration = is_runtime_declaration(node);
        let style = ctx.options().style.clone().unwrap_or_default();

        match (style, is_type_declaration, is_runtime_declaration) {
            (_, true, true) => Some(DeclarationError::InvalidDeclaration),
            (DeclarationStyle::Type, _, true) => Some(DeclarationError::WrongStyle),
            (DeclarationStyle::Runtime, true, _) => Some(DeclarationError::WrongStyle),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let style = ctx.options().style.clone().unwrap_or_default();
        let (target_type, current_type, correct_example) = match style {
            DeclarationStyle::Type => ("type", "runtime", "defineProps<...>()"),
            DeclarationStyle::Runtime => ("runtime", "type", "defineProps(...)"),
        };

        let node = ctx.query();

        let diagnostic = match state {
            DeclarationError::WrongStyle => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"defineProps"</Emphasis>" declaration uses "<Emphasis>{current_type}</Emphasis>" declaration."
                },
            )
            .note(markup! {
                 "It should be defined using "<Emphasis>{target_type}</Emphasis>" declaration like "<Emphasis>{correct_example}</Emphasis>". "
            }),
            DeclarationError::InvalidDeclaration => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>"defineProps"</Emphasis>" declaration is invalid."
                },
            )
            .note(markup! {
                 "It should be defined using "<Emphasis>{target_type}</Emphasis>" declaration like "<Emphasis>{correct_example}</Emphasis>". "
            }),
        };

        Some(diagnostic)
    }
}

pub enum DeclarationError {
    WrongStyle,
    InvalidDeclaration,
}

fn is_type_declaration(node: &JsCallExpression) -> bool {
    node.type_arguments().is_some()
}

fn is_runtime_declaration(node: &JsCallExpression) -> bool {
    node.arguments().is_ok_and(|args| args.args().into_iter().next().is_some())
}
