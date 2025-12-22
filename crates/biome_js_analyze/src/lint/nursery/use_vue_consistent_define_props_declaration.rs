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
        if let Some(callee_name) = get_callee_name(node)
            && callee_name != "defineProps"
        {
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

fn get_callee_name(expr: &JsCallExpression) -> Option<TokenText> {
    let callee = expr.callee().ok()?;
    let name = callee.get_callee_object_name()?.token_text_trimmed();
    Some(name)
}

fn is_type_declaration(node: &JsCallExpression) -> bool {
    node.type_arguments().is_some()
}

fn is_runtime_declaration(node: &JsCallExpression) -> bool {
    node.arguments()
        .ok()
        .is_some_and(|args| args.args().into_iter().next().is_some())
}
