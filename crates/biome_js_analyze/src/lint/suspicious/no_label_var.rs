use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsLabeledStatement, JsSyntaxNode, JsSyntaxToken};
use biome_rowan::AstNode;

declare_lint_rule! {
    ///  Disallow labels that share a name with a variable
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const x1 = "test";
    /// x1: expr;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const x = "test";
    /// z: expr;
    /// ```
    pub NoLabelVar {
        version: "1.0.0",
        name: "noLabelVar",
        language: "js",
        sources: &[RuleSource::Eslint("no-label-var")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoLabelVar {
    type Query = Semantic<JsLabeledStatement>;
    /// The first element of the tuple is the name of the binding, the second element of the tuple is the label name
    type State = (JsSyntaxNode, JsSyntaxToken);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let label_statement = ctx.query();

        let label_token = label_statement.label_token().ok()?;
        let name = label_token.text_trimmed();
        let model = ctx.model();
        // We search each scope from current scope until the global scope
        // if we find a binding that has its name equal to label name, then we found a  `LabelVar` issue.
        for scope in model.scope(label_statement.syntax()).ancestors() {
            if let Some(binding) = scope.get_binding(name) {
                return Some((binding.syntax().clone(), label_token));
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (binding_syntax_node, label_token) = state;
        let name = label_token.text_trimmed();

        Some(RuleDiagnostic::new(rule_category!(),
            label_token.text_trimmed_range(),
            markup! {
                "Do not use the "<Emphasis>{name}</Emphasis>" variable name as a label"
            },
        )
        .detail(binding_syntax_node.text_trimmed_range(), markup! {
            "The variable is declared here"
        },)
        .note(markup! {"Creating a label with the same name as an in-scope variable leads to confusion."}))
    }
}
