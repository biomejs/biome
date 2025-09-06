use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_js_syntax::{AnyJsExpression, JsxAttribute};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_jsx_props_bind::NoJsxPropsBindOptions;

declare_lint_rule! {
    /// Disallow .bind(), arrow functions, or function expressions in JSX props
    ///
    /// Using `.bind()` or creating a function inline in props creates a new function
    /// on every render, changing identity and defeating memoisation,
    /// which may cause unnecessary rerenders.
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={this._handleClick.bind(this)}></Foo>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={() => console.log('Hello!')}></Foo>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={function () { console.log('Hello!'); }}></Foo>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Foo onClick={this._handleClick}></Foo>
    /// ```



    pub NoJsxPropsBind {
        version: "next",
        name: "noJsxPropsBind",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-no-bind").inspired()],
        domains: &[RuleDomain::React],
    }
}

enum InvalidKind {
    ArrowFunction,
    Function,
    Bind,
}

pub struct NoJsxPropsBindState {
    invalid_kind: InvalidKind,
    attribute_range: TextRange,
}

impl Rule for NoJsxPropsBind {
    type Query = Ast<JsxAttribute>;
    type State = NoJsxPropsBindState;
    type Signals = Option<Self::State>;
    type Options = NoJsxPropsBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx
            .query()
            .initializer()?
            .value()
            .ok()?
            .as_jsx_expression_attribute_value()?
            .expression()
            .ok()?;

        match &expression {
            AnyJsExpression::JsArrowFunctionExpression(_) => Some(NoJsxPropsBindState {
                invalid_kind: InvalidKind::ArrowFunction,
                attribute_range: expression.range(),
            }),

            AnyJsExpression::JsFunctionExpression(_) => Some(NoJsxPropsBindState {
                invalid_kind: InvalidKind::Function,
                attribute_range: expression.range(),
            }),
            AnyJsExpression::JsCallExpression(call) => {
                // This will still throw a false positive on e.g. window.bind()
                let is_bind = call
                    .callee()
                    .ok()
                    .and_then(|c| c.as_js_static_member_expression().cloned())
                    .and_then(|m| m.member().ok())
                    .and_then(|n| n.value_token().ok())
                    .map_or(false, |t| t.text() == "bind");
                if is_bind {
                    Some(NoJsxPropsBindState {
                        invalid_kind: InvalidKind::Bind,
                        attribute_range: expression.range(),
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let note = match state.invalid_kind {
            InvalidKind::ArrowFunction => "JSX props should not use arrow functions",
            InvalidKind::Bind => "JSX props should not use .bind()",
            InvalidKind::Function => "JSX props should not use function expressions",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_range,
                "Pass stable function references as props to avoid unnecessary rerenders.",
            )
            .note(note)
            .note("Consider extracting the function or wrapping it in useCallback"),
        )
    }
}
