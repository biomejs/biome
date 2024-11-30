use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsxAttribute, AnyJsxAttributeName, JsFunctionExpression, JsxAttribute, JsxAttributeInitializerClause, JsxExpressionAttributeValue
};
use biome_rowan::AstNode;
declare_lint_rule! {
    /// Disallow .bind() or function declaration in JSX props
    ///
    /// Using `.bind()` on a function or declaring a function directly in props
    /// creates a new function on every render, which is treated as a completely different function.
    ///
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
    ///
    pub NoJsxPropsBind {
        version: "next",
        name: "noJsxPropsBind",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-no-bind")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for NoJsxPropsBind {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    // TODO: Find how to use option
    type Options = ();

fn run(ctx: &RuleContext<Self>) -> Self::Signals {
    let attribute = ctx.query();
    let attribute_name = attribute.name().ok()?;
    
    /*
    TODO: Too many if, find simple way
    rust analyzer doc: https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md
    biome_js_syntax doc: https://docs.rs/biome_js_syntax/latest/biome_js_syntax/index.html
    */
    if is_event_handler(&attribute_name) {
        if let Some(initializer) = attribute.initializer() {
            if let Some(attribute_value) = initializer.value().ok() {
                if let Some(expression_value) = attribute_value.as_jsx_expression_attribute_value() {
                    if let Some(expression) = expression_value.expression().ok() {
                            match expression {
                                AnyJsExpression::JsArrowFunctionExpression(_) => return Some(()),
                                AnyJsExpression::JsFunctionExpression(_) => return Some(()),
                                AnyJsExpression::JsCallExpression(_) => return Some(()),
                                _ => {}
                            }
                        
                    }
                }
            }
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
                    "Disallow .bind() or function declaration in JSX props"
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

fn is_event_handler(name: &AnyJsxAttributeName) -> bool {
    match name {
        AnyJsxAttributeName::JsxName(identifier) => {
            if let Some(value_token) = identifier.value_token().ok() {
                let name_text = value_token.text_trimmed();
                name_text.starts_with("on")
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_arrow_or_anonymous(expression: AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    )
}

fn has_bind_call() -> bool {
    false
}