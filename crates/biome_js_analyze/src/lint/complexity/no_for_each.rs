use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Prefer `for...of` statement instead of `Array.forEach`.
    ///
    /// Here's a summary of why `forEach` may be disallowed, and why `for...of` is preferred for almost any use-case of `forEach`:
    /// - Performance: Using `forEach` can lead to performance issues, especially when working with large arrays.
    /// When more requirements are added on, `forEach` typically gets chained with other methods like `filter` or `map`, causing multiple iterations over the same Array.
    /// Encouraging for loops discourages chaining and encourages single-iteration logic (e.g. using a continue instead of `filter`).
    ///
    /// - Readability: While `forEach` is a simple and concise way to iterate over an array, it can make the code less readable, especially when the callback function is complex.
    /// In contrast, using a for loop or a `for...of` loop can make the code more explicit and easier to read.
    ///
    /// - Debugging: `forEach` can make debugging more difficult, because it hides the iteration process.
    ///
    /// ## Caveat
    ///
    /// We consider all objects with a method named `forEach` to be iterable.
    /// This way, this rule applies to all objects with a method called `forEach`, not just `Array` instances.
    ///
    /// ## Exception for Index Usage
    ///
    /// When the index is explicitly used in the `forEach` callback, it is acceptable to use `forEach`. This is because:
    /// - The index is directly available as the second argument in `forEach`, making it convenient for scenarios where the index is necessary.
    /// - In sparse arrays, `forEach` will skip undefined entries, which differs from the behavior of `for...of` with `Object.entries` that includes these entries.
    ///   This can be important for certain array operations, particularly in TypeScript environments with strict type checking.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// els.forEach((el) => {
    ///   f(el);
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// els["forEach"](el => {
    ///   f(el);
    /// })
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// els.forEach((el, i) => {
    ///   f(el, i)
    /// })
    /// ```
    ///
    /// ```js
    /// for (const el of els) {
    ///   f(el);
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// **Since v2.0.0**
    ///
    /// The rule provides a `validIdentifiers` option that allows specific variable names to call `forEach`.
    /// In the following configuration, it's allowed to call `forEach` with expressions that match `Effect` or `_`:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allowedIdentifiers": ["Effect", "_"]
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// Effect.forEach((el) => {
    ///   f(el);
    /// })
    /// _.forEach((el) => {
    ///   f(el);
    /// })
    /// ```
    ///
    /// Values with dots (e.g., "lib._") will not be accepted.
    pub NoForEach {
        version: "1.0.0",
        name: "noForEach",
        language: "js",
        sources: &[
            RuleSource::EslintUnicorn("no-array-for-each"),
            RuleSource::Clippy("needless_for_each"),
        ],
        severity: Severity::Warning,
    }
}

impl Rule for NoForEach {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoForEachOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let member_expression =
            AnyJsMemberExpression::cast(node.callee().ok()?.omit_parentheses().into_syntax())?;
        if member_expression.member_name()?.text() != "forEach" {
            return None;
        }

        let options = ctx.options();
        // Check if `forEach` is called by a valid identifier.
        if !options.allowed_identifiers.is_empty() {
            let object = member_expression.object().ok()?;
            if let Some(reference) = object.as_js_reference_identifier() {
                let value_token = reference.value_token().ok()?;
                let name = value_token.text_trimmed();
                if options
                    .allowed_identifiers
                    .iter()
                    .any(|identifier| identifier.as_ref() == name)
                {
                    return None;
                }
            }
        }

        // Extract first parameter and ensure we have no more than 2 parameters.
        let [Some(first), _, None] = node.arguments().ok()?.get_arguments_by_index([0, 1, 2])
        else {
            return None;
        };
        // Report calls that use a callbacks with 0 or 1 parameter.
        let parameter_count = match first.as_any_js_expression()? {
            AnyJsExpression::JsArrowFunctionExpression(function) => {
                function.parameters().ok()?.len()
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                function.parameters().ok()?.items().len()
            }
            _ => return None,
        };
        (parameter_count <= 1).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Prefer "<Emphasis>"for...of"</Emphasis>" instead of "<Emphasis>"forEach"</Emphasis>"."
            },
        ).note(markup!{
            <Emphasis>"forEach"</Emphasis>" may lead to performance issues when working with large arrays. When combined with functions like "<Emphasis>"filter"</Emphasis>" or "<Emphasis>"map"</Emphasis>", this causes multiple iterations over the same type."
        }))
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoForEachOptions {
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    /// A list of variable names allowed for `forEach` calls.
    pub allowed_identifiers: Box<[Box<str>]>,
}

impl DeserializableValidator for NoForEachOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        if self
            .allowed_identifiers
            .iter()
            .any(|identifier| identifier.is_empty() || identifier.contains('.'))
        {
            ctx
                .report(
                    DeserializationDiagnostic::new(markup!(
                        <Emphasis>"'allowedIdentifiers'"</Emphasis>" does not accept empty values or values with dots."
                    ))
                    .with_range(range)
                );
            return false;
        }

        true
    }
}
