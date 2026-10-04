use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsBinding, JsFunctionExportDefaultDeclaration, JsFunctionExpression, JsSyntaxToken,
};
use biome_rowan::{AstNode, SyntaxResult, TextRange, declare_node_union};
use biome_rule_options::use_named_function::UseNamedFunctionOptions;

declare_lint_rule! {
    /// Require `function` expressions to have a name.
    ///
    /// A function expression is a function written with the `function` keyword inside other code,
    /// for example when it is passed to another function or stored in a property.
    /// Unlike a function declaration, it does not need a name.
    ///
    /// When a function has no name, error messages and debugging tools can show it as `<anonymous>`,
    /// which makes it harder to find where an error came from.
    ///
    /// JavaScript automatically names some functions, such as a function stored in a variable.
    /// This rule still asks for a name in those cases, so that every function is named the same way
    /// no matter where it is written.
    ///
    /// The rule also checks `async` functions, generator functions (`function*`), and
    /// `export default function() {}`. Arrow functions and methods are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Foo.prototype.bar = function() {};
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const cat = {
    ///     meow: function() {}
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (function() {
    ///     // ...
    /// }());
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// export default function() {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Foo.prototype.bar = function bar() {};
    ///
    /// const cat = {
    ///     meow() {}
    /// };
    ///
    /// (function setup() {
    ///     // ...
    /// }());
    ///
    /// const handler = () => {};
    ///
    /// export default function main() {}
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to replace `function` expressions with arrow functions, see [`useArrowFunction`](https://biomejs.dev/linter/rules/use-arrow-function/).
    ///
    pub UseNamedFunction {
        version: "next",
        name: "useNamedFunction",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("func-names").inspired()],
    }
}

impl Rule for UseNamedFunction {
    type Query = Ast<AnyUnnamedFunctionCandidate>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseNamedFunctionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        ctx.query().id().is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let head_end = match node.star_token() {
            Some(star) => star,
            None => node.function_token().ok()?,
        };
        let range = TextRange::new(node.range().start(), head_end.text_trimmed_range().end());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This function has no name."
                },
            )
            .note(markup! {
                "Error messages and debugging tools can show a function without a name as "<Emphasis>"<anonymous>"</Emphasis>", which makes it harder to find where an error came from."
            })
            .note(markup! {
                "Give this function a name."
            }),
        )
    }
}

declare_node_union! {
    pub AnyUnnamedFunctionCandidate = JsFunctionExpression | JsFunctionExportDefaultDeclaration
}

impl AnyUnnamedFunctionCandidate {
    fn id(&self) -> Option<AnyJsBinding> {
        match self {
            Self::JsFunctionExpression(function) => function.id(),
            Self::JsFunctionExportDefaultDeclaration(function) => function.id(),
        }
    }

    fn function_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsFunctionExpression(function) => function.function_token(),
            Self::JsFunctionExportDefaultDeclaration(function) => function.function_token(),
        }
    }

    fn star_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsFunctionExpression(function) => function.star_token(),
            Self::JsFunctionExportDefaultDeclaration(function) => function.star_token(),
        }
    }
}
