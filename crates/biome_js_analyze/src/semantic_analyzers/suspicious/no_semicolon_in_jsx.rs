use biome_analyze::{
    context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{jsx_ext::AnyJsxElement, JsLanguage, JsxChildList, JsxElement};
use biome_rowan::{AstNode, AstNodeList, SyntaxToken, TextRange};

declare_rule! {
    /// Remove semicolons from JSX elements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const Component = () => {
    ///   return (
    ///     <div>
    ///       <div />;
    ///     </div>
    ///  );
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const Component = () => {
    ///   return (
    ///     <div>
    ///       <div />
    ///       ;
    ///     </div>
    ///   );
    /// }
    /// const Component2 = () => {
    ///   return (
    ///     <div>
    ///       <span>;</span>
    ///     </div>
    ///   );
    /// }
    /// ```
    ///
    pub(crate) NoSemicolonInJsx {
        version: "1.0.0",
        name: "noSemicolonInJsx",
        source: RuleSource::Eslint("no-semicolons-in-jsx"),
        source_kind: RuleSourceKind::Inspired,
        recommended: true,
    }
}

impl Rule for NoSemicolonInJsx {
    type Query = Ast<AnyJsxElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let jsx_element = node.parent::<JsxElement>()?;
        if let AnyJsxElement::JsxOpeningElement(_) = node {
            let has_semicolon = has_suspicious_semicolon(&jsx_element.children());
            if let Some(incorrect_semicolon) = has_semicolon {
                return Some(incorrect_semicolon);
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "There is suspicious "<Emphasis>"Semicolon"</Emphasis>" in the JSX element."
            },
        )
        .note(markup! {
            "Remove the "<Emphasis>"Semicolon"</Emphasis>" from the JSX element."
        });
        Some(diagnostic)
    }
}

fn has_suspicious_semicolon(node: &JsxChildList) -> Option<TextRange> {
    node.iter().find_map(|c| {
        let jsx_text = c.as_jsx_text()?;
        let jsx_text_value = jsx_text.value_token().ok()?;
        // We should also check for \r and \r\n
        if jsx_text_value.text().starts_with(";\n")
            || jsx_text_value.text().starts_with(";\r")
            || jsx_text_value.text().starts_with(";\r\n")
        {
            return Some(jsx_text_value.text_range());
        }

        c.as_jsx_element()
            .and_then(|e| has_suspicious_semicolon(&e.children()));

        None
    })
}
