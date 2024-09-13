use biome_analyze::context::RuleContext;
use biome_analyze::{declare_syntax_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{JsClassDeclaration, JsClassExpression, JsSuperExpression};
use biome_rowan::AstNode;

declare_syntax_rule! {
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
        language: "js",
    }
}

impl Rule for NoSuperWithoutExtends {
    type Query = Ast<JsSuperExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        for syntax in node.syntax().ancestors() {
            match JsClassDeclaration::try_cast(syntax) {
                Ok(class_declaration) => {
                    // ancestor is class declaration
                    if class_declaration.extends_clause().is_none() {
                        return Some(());
                    }
                    return None;
                }
                Err(syntax) => {
                    // ancestor is class expression
                    if let Some(class_expression) = JsClassExpression::cast(syntax) {
                        if class_expression.extends_clause().is_none() {
                            return Some(());
                        }
                        return None;
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "super() is only valid in derived class constructors"
            },
        ))
    }
}
