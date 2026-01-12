use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{JsNewExpression, JsSyntaxKind};
use biome_rowan::{AstNode, SyntaxNodeOptionExt};
use biome_rule_options::no_floating_classes::NoFloatingClassesOptions;

declare_lint_rule! {
    /// Disallow `new` operators outside of assignments or comparisons.
    ///
    /// The goal of using `new` with a constructor is typically to create an object of a particular type and store that object in a variable, such as:
    ///
    /// ```js
    /// const person = new Person();
    /// ```
    ///
    /// It's less common to use `new` and not store the result, such as:
    ///
    /// ```js,ignore
    /// new Person();
    /// ```
    ///
    /// In this case, the created object is thrown away because its reference isn't stored anywhere, and in many cases, this means that the constructor should be replaced with a function that doesn't require `new` to be used.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Thing();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const thing = new Thing();
    /// ```
    ///
    pub NoFloatingClasses {
        version: "next",
        name: "noFloatingClasses",
        language: "js",
        recommended: false,
    }
}

impl Rule for NoFloatingClasses {
    type Query = Ast<JsNewExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoFloatingClassesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let Some(kind) = node.syntax().parent().kind()
            && kind == JsSyntaxKind::JS_EXPRESSION_STATEMENT
        {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Using the `new` operator outside of assignments or comparisons is not allowed."
                },
            )
            .note(markup! {
                "The created object is thrown away because its reference isn't stored anywhere. Assign the object to a variable or replace with a function that doesn't require `new` to be used."
            }),
        )
    }
}
