use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_json_syntax::{AnyJsonValue, JsonRoot};
use biome_rowan::AstNode;
use biome_rule_options::no_top_level_literals::NoTopLevelLiteralsOptions;

declare_lint_rule! {
    /// Require the JSON top-level value to be an array or object.
    ///
    /// The JSON specification technically allows any JSON value (object, array, string, number, boolean, or null) to be used as the top-level element of a JSON document.
    /// However, some older JSON parsers, especially those created before [RFC 7158](https://datatracker.ietf.org/doc/html/rfc7158)/[4627](https://datatracker.ietf.org/doc/html/rfc4627) was fully adopted, only support objects or arrays as the root element.
    ///
    /// Additionally, some security practices (such as those preventing JSON hijacking attacks) rely on the assumption that the top-level value is an object or array.
    /// Using an object or array at the top level also provides better extensibility for your data structures over time.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// "just a string"
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// 42
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// true
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// null
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "property": "value",
    ///   "otherProperty": 123
    /// }
    /// ```
    ///
    /// ```json
    /// ["element", "anotherElement"]
    /// ```
    ///
    /// ```json
    /// {}
    /// ```
    ///
    /// ```json
    /// []
    /// ```
    ///
    pub NoTopLevelLiterals {
        version: "2.4.7",
        name: "noTopLevelLiterals",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintJson("top-level-interop").same()],
    }
}

impl Rule for NoTopLevelLiterals {
    type Query = Ast<JsonRoot>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoTopLevelLiteralsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value = node.value().ok()?;

        match value {
            AnyJsonValue::JsonObjectValue(_) => None,
            AnyJsonValue::JsonArrayValue(_) => None,
            _ => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Expected the top-level value to be an array or object."
                },
            )
            .note(markup! {
                "Some JSON parsers only support objects or arrays as the root element. Wrap your value in an array or object to ensure compatibility."
            }),
        )
    }
}
