use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, global_identifier};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::use_symbol_description::UseSymbolDescriptionOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require a description parameter for the `Symbol()`.
    ///
    /// `Symbol` can have an optional description parameter which can be useful for
    /// debugging and making the purpose of the symbol clearer.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Symbol();
    /// ```
    /// ```js,expect_diagnostic
    /// Symbol('');
    /// ```
    ///```js,expect_diagnostic
    /// Symbol(``);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// Symbol('description');
    /// ```
    ///
    pub UseSymbolDescription {
        version: "2.0.0",
        name: "useSymbolDescription",
        language: "js",
        sources: &[RuleSource::Eslint("symbol-description").same()],
        recommended: false,
    }
}

impl Rule for UseSymbolDescription {
    type Query = Semantic<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseSymbolDescriptionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call_expression = ctx.query();
        let callee = call_expression.callee().ok()?;
        let model = ctx.model();
        let (reference, name) = global_identifier(&callee)?;

        if name.text() != "Symbol" || model.binding(&reference).is_some() {
            return None;
        }

        let arguments = call_expression.arguments().ok()?;
        let args = arguments.args();

        let is_missing_description = match args.len() {
            0 => true,
            1 => {
                let first_arg = args.into_iter().next()?.ok()?;
                let first_arg = first_arg.as_any_js_expression()?;

                is_expr_empty_string(first_arg)
            }
            // native Symbol() can accept only one argument.
            _ => false,
        };

        if is_missing_description {
            return Some(arguments.range());
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    <Emphasis>"Symbol()"</Emphasis>" is missing a description parameter."
                },
            )
            .note(markup! {
                "Add explicit description which can be useful in debugging and making the purpose of the symbol clearer."
            }),
        )
    }
}

fn is_expr_empty_string(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(literal) => literal
            .as_js_string_literal_expression()
            .and_then(|str_literal| {
                let is_empty = str_literal.inner_string_text().ok()?.text() == "";

                if is_empty { Some(true) } else { None }
            })
            .is_some(),
        AnyJsExpression::JsTemplateExpression(template) => {
            template.tag().is_none() && template.elements().into_iter().next().is_none()
        }
        _ => false,
    }
}
