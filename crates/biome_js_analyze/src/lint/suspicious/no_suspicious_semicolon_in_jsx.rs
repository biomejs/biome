use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsxTag, JsxChildList};
use biome_rowan::{AstNodeList, TextRange};

declare_lint_rule! {
    /// It detects possible "wrong" semicolons inside JSX elements.
    ///
    /// Semicolons that appear after a self-closing element or a closing element are usually the result of a typo of a refactor gone wrong.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
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
    /// ```jsx
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
    pub NoSuspiciousSemicolonInJsx {
        version: "1.6.0",
        name: "noSuspiciousSemicolonInJsx",
        language: "jsx",
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoSuspiciousSemicolonInJsx {
    type Query = Ast<AnyJsxTag>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if let Some(children) = match node {
            AnyJsxTag::JsxElement(element) => Some(element.children()),
            AnyJsxTag::JsxFragment(fragment) => Some(fragment.children()),
            _ => None,
        } {
            let has_semicolon = has_suspicious_semicolon(&children);
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
                "There is a suspicious "<Emphasis>"semicolon"</Emphasis>" in the JSX element."
            },
        )
        .note(markup! {
            "This is usually the result of a typo or some refactor gone wrong."
        })
        .note(markup! {
            "Remove the "<Emphasis>"semicolon"</Emphasis>", or move it inside a JSX element."
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
