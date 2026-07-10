use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsClass, AnyJsClassMember, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsStatement,
};
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_rule_options::use_consistent_error_subclassing::UseConsistentErrorSubclassingOptions;

declare_lint_rule! {
    /// Enforce consistent conventions when subclassing the built-in `Error`.
    ///
    /// A custom error class should be a well-behaved `Error` subclass so that
    /// instances are readable, serializable, and distinguishable at runtime. This
    /// rule mirrors [`unicorn/custom-error-definition`](https://github.com/sindresorhus/eslint-plugin-unicorn/blob/main/docs/rules/custom-error-definition.md)
    /// and reports a class that `extends` a built-in error (`Error`, `TypeError`,
    /// `RangeError`, `SyntaxError`, `EvalError`, `ReferenceError`, `URIError`,
    /// `AggregateError`) when:
    ///
    /// - the class name does not end in `Error`;
    /// - the constructor does not assign `this.name`;
    /// - `this.name` is assigned dynamically (e.g. `this.constructor.name`) instead
    ///   of a string literal; or
    /// - `this.name` is assigned a string literal that does not match the class name.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo extends Error {
    ///     constructor(message) {
    ///         super(message);
    ///         this.name = "Foo";
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message) {
    ///         super(message);
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message) {
    ///         super(message);
    ///         this.name = this.constructor.name;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class FooError extends Error {
    ///     constructor(message) {
    ///         super(message);
    ///         this.name = "WrongName";
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class FooError extends Error {
    ///     constructor(message) {
    ///         super(message);
    ///         this.name = "FooError";
    ///     }
    /// }
    /// ```
    ///
    pub UseConsistentErrorSubclassing {
        version: "next",
        name: "useConsistentErrorSubclassing",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintUnicorn("custom-error-definition").same()],
    }
}

/// Built-in error constructors a custom error class typically extends.
const BUILTIN_ERRORS: &[&str] = &[
    "Error",
    "TypeError",
    "RangeError",
    "SyntaxError",
    "EvalError",
    "ReferenceError",
    "URIError",
    "AggregateError",
];

#[derive(Debug, Clone)]
pub enum RuleState {
    /// The class extends a built-in error but its name doesn't end in `Error`.
    NameNotEndingInError(TextRange),
    /// The constructor never assigns `this.name`.
    MissingName(TextRange),
    /// `this.name` is assigned from a non-string-literal (e.g. `this.constructor.name`).
    DynamicName(TextRange),
    /// `this.name` is a string literal that doesn't match the class name.
    MismatchedName { range: TextRange, expected: String },
}

impl Rule for UseConsistentErrorSubclassing {
    type Query = Ast<AnyJsClass>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseConsistentErrorSubclassingOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let class = ctx.query();

        // Only classes that extend a built-in error are in scope.
        let extends_clause = class.extends_clause()?;
        let super_class = extends_clause.super_class().ok()?;
        let super_name = super_class
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .value_token()
            .ok()?;
        if !BUILTIN_ERRORS.contains(&super_name.text_trimmed()) {
            return None;
        }

        // We need the class name to compare against `this.name`; skip anonymous
        // class expressions.
        let id = class.id()?;
        let name_token = id.as_js_identifier_binding()?.name_token().ok()?;
        let class_name = name_token.text_trimmed().to_string();

        // Report a name that doesn't end in `Error` first — it's the cheapest,
        // most obvious signal.
        if !class_name.ends_with("Error") {
            return Some(RuleState::NameNotEndingInError(
                name_token.text_trimmed_range(),
            ));
        }

        // Inspect the constructor's `this.name` assignment, if a constructor exists.
        let constructor = class.members().iter().find_map(|member| match member {
            AnyJsClassMember::JsConstructorClassMember(ctor) => Some(ctor),
            _ => None,
        })?;

        let body = constructor.body().ok()?;
        for statement in body.statements() {
            let AnyJsStatement::JsExpressionStatement(expr_statement) = statement else {
                continue;
            };
            let Ok(AnyJsExpression::JsAssignmentExpression(assignment)) =
                expr_statement.expression()
            else {
                continue;
            };
            let Ok(AnyJsAssignmentPattern::AnyJsAssignment(
                AnyJsAssignment::JsStaticMemberAssignment(target),
            )) = assignment.left()
            else {
                continue;
            };
            // Target must be `this.name`.
            if target
                .object()
                .ok()
                .as_ref()
                .and_then(AnyJsExpression::as_js_this_expression)
                .is_none()
            {
                continue;
            }
            if target.member().ok()?.value_token().ok()?.text_trimmed() != "name" {
                continue;
            }

            // Found a `this.name = …` assignment — validate its right-hand side.
            let right = assignment.right().ok()?;
            let range = assignment.range();
            return match right {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(literal),
                ) => {
                    let value = literal.inner_string_text().ok()?;
                    if value.text() == class_name {
                        None
                    } else {
                        Some(RuleState::MismatchedName {
                            range,
                            expected: class_name,
                        })
                    }
                }
                _ => Some(RuleState::DynamicName(range)),
            };
        }

        // No `this.name` assignment in the constructor.
        Some(RuleState::MissingName(constructor.range()))
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match state {
            RuleState::NameNotEndingInError(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The name of a custom error class should end in "<Emphasis>"Error"</Emphasis>"."
                },
            )
            .note(markup! {
                "A name ending in "<Emphasis>"Error"</Emphasis>" makes the class recognizable as an error type."
            }),
            RuleState::MissingName(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This custom error class doesn't set "<Emphasis>"this.name"</Emphasis>"."
                },
            )
            .note(markup! {
                "Set "<Emphasis>"this.name"</Emphasis>" to a string literal matching the class name so the error is identifiable at runtime."
            }),
            RuleState::DynamicName(range) => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    ""<Emphasis>"this.name"</Emphasis>" should be assigned a string literal, not a computed value."
                },
            )
            .note(markup! {
                "Computed names like "<Emphasis>"this.constructor.name"</Emphasis>" break after minification. Use a string literal matching the class name."
            }),
            RuleState::MismatchedName { range, expected } => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    ""<Emphasis>"this.name"</Emphasis>" should match the class name "<Emphasis>{expected}</Emphasis>"."
                },
            ),
        };
        Some(diagnostic)
    }
}
