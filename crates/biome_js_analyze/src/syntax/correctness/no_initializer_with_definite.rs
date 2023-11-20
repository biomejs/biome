use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};

use biome_diagnostics::category;
use biome_js_syntax::{JsSyntaxKind, TextRange, TsDefiniteVariableAnnotation};
use biome_rowan::AstNode;

declare_rule! {
    /// Disallow initializing a variable with a definite assertion to prevent `SyntaxError`.
    ///
    /// ## Examples
    ///
    /// ```js
    /// let foo!: string = "bar";
    /// ```
    pub(crate) NoInitializerWithDefinite {
        version: "1.0.0",
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
        if let Some(parent) = node.clone().into_syntax().parent().clone() {
            if parent.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR {
                if let Some(initializer) = parent
                    .children()
                    .find(|sibling| sibling.kind() == JsSyntaxKind::JS_INITIALIZER_CLAUSE)
                {
                    return Some(initializer.text_range());
                }
            }
        }
        None
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
