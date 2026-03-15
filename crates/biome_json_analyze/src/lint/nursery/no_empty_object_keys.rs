use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize::json::unescape_json_string;
use biome_json_syntax::JsonMemberName;
use biome_rowan::AstNode;
use biome_rule_options::no_empty_object_keys::NoEmptyObjectKeysOptions;

declare_lint_rule! {
    /// Disallow empty keys in JSON objects.
    ///
    /// In JSON, using empty keys (keys that are empty strings or contain only whitespace) can lead to accessibility and maintenance issues.
    /// While technically valid in JSON, empty keys make objects harder to read, can cause confusion when debugging, and may create problems with some JSON parsers or processors.
    /// Additionally, empty keys often indicate mistakes or oversights in the processes.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "": "value"
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "validKey": "value",
    ///   "": "another value"
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   " ": "space as key"
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "\t": "tab as key"
    /// }
    /// ```
    ///
    /// ```json,expect_diagnostic
    /// {
    ///   "\n": "newline as key"
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```json
    /// {
    ///   "key": "value"
    /// }
    /// ```
    ///
    pub NoEmptyObjectKeys {
        version: "2.4.7",
        name: "noEmptyObjectKeys",
        language: "json",
        recommended: false,
        sources: &[RuleSource::EslintJson("no-empty-keys").same()],
    }
}

impl Rule for NoEmptyObjectKeys {
    type Query = Ast<JsonMemberName>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoEmptyObjectKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let value = node.inner_string_text().ok()?;
        let binding = unescape_json_string(value);
        if binding.trim().is_empty() {
            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let span = ctx.query().range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected empty object key."
                },
            )
            .note(markup! {
                "Empty keys often cause confusion and may cause issues with parsers or processors. Either remove this property or provide a meaningful key name."
            }),
        )
    }
}
