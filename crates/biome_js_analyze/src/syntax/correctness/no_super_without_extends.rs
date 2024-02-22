use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::category;
use biome_js_syntax::{JsClassDeclaration, JsSuperExpression};
use biome_rowan::AstNode;

declare_rule! {
    /// Catch a `SyntaxError` when writing calling `super()` on a class that doesn't extends any class
    ///
    /// ## Examples
    ///
    /// ```js
    /// class A {
    //     constructor() {
    //         super()
    //     }
    // }
    /// ```
    pub NoSuperWithoutExtends {
        version: "1.0.0",
        name: "noSuperWithoutExtends",
    }
}

impl Rule for NoSuperWithoutExtends {
    type Query = Ast<JsSuperExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let Some(class_declaration) =
            node.syntax().ancestors().find_map(JsClassDeclaration::cast)
        {
            if class_declaration.extends_clause().is_none() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            category!("parse/noSuperWithoutExtends"),
            node.syntax().text_trimmed_range(),
            markup! {
                "super() is only valid in derived class constructors"
            },
        ))
    }
}
