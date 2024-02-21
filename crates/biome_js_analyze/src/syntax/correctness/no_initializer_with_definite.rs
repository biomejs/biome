use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};

use biome_diagnostics::category;
use biome_js_syntax::{JsVariableDeclarator, TextRange, TsDefiniteVariableAnnotation};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow initializing a variable with a definite assertion to prevent `SyntaxError`.
    ///
    /// ## Examples
    ///
    /// ```js
    /// let foo!: string = "bar";
    /// ```
    pub NoInitializerWithDefinite {
        version: "1.4.0",
        name: "noInitializerWithDefinite",
    }
}

impl Rule for NoInitializerWithDefinite {
    type Query = Ast<TsDefiniteVariableAnnotation>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        node.parent::<JsVariableDeclarator>()
            .and_then(|var_declarator| var_declarator.initializer())
            .map(|init| init.into_syntax().text_range())
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            category!("parse/noInitializerWithDefinite"),
            state,
            "Declarations with initializers cannot also have definite assignment assertions.",
        );
        Some(diagnostic)
    }
}
