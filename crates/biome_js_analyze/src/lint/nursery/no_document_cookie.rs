use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{
    global_identifier, AnyJsAssignment, AnyJsAssignmentPattern, JsAssignmentExpression,
};
use biome_rowan::AstNode;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow direct assignments to `document.cookie`.
    ///
    /// It's not recommended to use document.cookie directly as it's easy to get the string wrong.
    /// Instead, you should use the [Cookie Store API](https://developer.mozilla.org/en-US/docs/Web/API/CookieStore).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// document.cookie = "foo=bar";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// document.cookie += "; foo=bar";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// await cookieStore
    ///   .set({
    ///     name: "foo",
    ///     value: "bar",
    ///     expires: Date.now() + 24 * 60 * 60,
    ///     domain: "example.com",
    /// })
    /// ```
    ///
    /// ```js
    /// const array = document.cookie.split("; ");
    /// ```
    ///
    /// ```js
    /// import Cookies from 'js-cookie';
    ///
    /// Cookies.set('foo', 'bar');
    /// ```
    ///
    pub NoDocumentCookie {
        version: "next",
        name: "noDocumentCookie",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("no-document-cookie")],
    }
}

impl Rule for NoDocumentCookie {
    type Query = Semantic<JsAssignmentExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let left = node.left().ok()?;

        let static_assignment = match &left {
            AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsStaticMemberAssignment(
                static_assignment,
            )) => static_assignment,
            _ => {
                return None;
            }
        };

        // Check `document` is global
        let expr = static_assignment.object().ok()?;
        let (reference, name) = global_identifier(&expr)?;

        if name.text() != "document" {
            return None;
        }

        let property = static_assignment.member().ok()?;

        if property.text() != "cookie" {
            return None;
        }

        let model = ctx.model();
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Direct assigning to" <Emphasis>"document.cookie"</Emphasis>" is not recommended"
                },
            )
            .note(markup! {
                "Consider using the "<Hyperlink href = "https://developer.mozilla.org/en-US/docs/Web/API/CookieStore">"Cookie Store API"</Hyperlink>"."
            }),
        )
    }
}
