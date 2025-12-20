use std::ops::Not;

use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsFileSource};
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::use_vue_consistent_define_props_declaration::{
    DeclarationStyle, UseVueConsistentDefinePropsDeclarationOptions,
};

declare_lint_rule! {
    /// Enforce consistent `defineProps` declaration style.
    ///
    /// This rule enforces `defineProps` typing style which you should use `type` or `runtime` declaration.
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
    pub UseVueConsistentDefinePropsDeclaration {
        version: "next",
        name: "useVueConsistentDefinePropsDeclaration",
        language: "js",
        sources: &[RuleSource::EslintVueJs("define-props-declaration").same()],
        recommended: false,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for UseVueConsistentDefinePropsDeclaration {
    type Query = Ast<JsCallExpression>;
    type State = ();
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
        if let Some(callee_name) = get_callee_name(node)
            && callee_name != "defineProps"
        {
            return None;
        }

        let style = ctx.options().style.clone().unwrap_or_default();
        if let Some(kind) = get_declaration_kind(node)
            && kind != style
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let style = ctx.options().style.clone().unwrap_or_default();
        let (current, opposite) = match style {
            DeclarationStyle::Type => ("type", "runtime"),
            DeclarationStyle::Runtime => ("runtime", "type"),
        };

        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "This "<Emphasis>"defineProps"</Emphasis>" declaration should be defined using "<Emphasis>{current}</Emphasis>" declaration instead of "<Emphasis>{opposite}</Emphasis>" declaration."
            },
        ))
    }
}

fn get_callee_name(expr: &JsCallExpression) -> Option<TokenText> {
    let callee = expr.callee().ok()?;
    let name = callee.get_callee_object_name()?.token_text_trimmed();
    Some(name)
}

fn get_declaration_kind(expr: &JsCallExpression) -> Option<DeclarationStyle> {
    // check if the expression has arguments
    if let Ok(arguments) = expr.arguments()
        && arguments.args().into_iter().next().is_some()
    {
        return Some(DeclarationStyle::Runtime);
    }

    // check if the expression has type arguments
    if expr.type_arguments().is_some() {
        return Some(DeclarationStyle::Type);
    }

    None
}
