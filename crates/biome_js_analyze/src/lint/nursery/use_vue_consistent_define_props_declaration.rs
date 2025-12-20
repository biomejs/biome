use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::{AstNode, TokenText};
use biome_rule_options::use_vue_consistent_define_props_declaration::{
    DeclarationStyle, UseVueConsistentDefinePropsDeclarationOptions,
};

declare_lint_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // var a = 1;
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
        // TODO: Need to run this only on <script setup> blocks.
        let path = ctx.file_path();
        if path.extension() != Some("vue") {
            return None;
        }

        let node = ctx.query();
        if let Some(callee_name) = get_callee_name(&node)
            && callee_name != "defineProps"
        {
            return None;
        }

        let style = ctx.options().style.clone().unwrap_or_default();
        println!("style: {:?}", style);
        if let Some(kind) = get_declaration_kind(&node) {
            if kind != style {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

fn get_callee_name(expr: &JsCallExpression) -> Option<TokenText> {
    let callee = expr.callee().ok()?;
    let name = callee.get_callee_object_name()?.token_text_trimmed();
    Some(name)
}

fn get_declaration_kind(expr: &JsCallExpression) -> Option<DeclarationStyle> {
    if let Ok(arguments) = expr.arguments() {
        if arguments.args().into_iter().count() != 0 {
            return Some(DeclarationStyle::Runtime);
        }
    }

    if let Some(_) = expr.type_arguments() {
        return Some(DeclarationStyle::Type);
    }

    None
}
