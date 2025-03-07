use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, inner_string_text,
};
use biome_rowan::AstNode;
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Require the consistent declaration of object literals. Defaults to explicit definitions.
    ///
    /// ECMAScript 6 provides two ways to define an object literal: `{foo: foo}` and `{foo}`.
    /// The two styles are functionally equivalent.
    /// Using the same style consistently across your codebase makes it easier to quickly read and understand object definitions.
    ///
    /// ## Example
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "syntax": "explicit"
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// let foo = 1;
    /// let invalid = {
    ///     foo
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// let invalid = {
    ///     bar() { return "bar"; },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// let foo = 1;
    /// let valid = {
    ///     foo: foo,
    ///     bar: function() { return "bar"; },
    /// };
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "syntax": "shorthand"
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// let foo = 1;
    /// let invalid = {
    ///     foo: foo
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// let invalid = {
    ///     bar: function() { return "bar"; },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// let foo = 1;
    /// let valid = {
    ///     foo,
    ///     bar() { return "bar"; },
    /// };
    /// ```
    ///
    /// ## Options
    ///
    /// Use the options to specify the syntax of object literals to enforce.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "syntax": "explicit"
    ///     }
    /// }
    /// ```
    ///
    /// ### syntax
    ///
    /// The syntax to use:
    /// - `explicit`: enforces the use of explicit object property syntax in every case.
    /// - `shorthand`: enforces the use of shorthand object property syntax when possible.
    ///
    /// **Default:** `explicit`
    ///
    pub UseConsistentObjectDefinition {
        version: "next",
        name: "useConsistentObjectDefinition",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        sources: &[RuleSource::Eslint("object-shorthand")],
        source_kind: RuleSourceKind::Inspired,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentObjectDefinitionOptions {
    /// The preferred syntax to enforce.
    syntax: ObjectPropertySyntax,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ObjectPropertySyntax {
    /// `{foo: foo}`
    #[default]
    Explicit,
    /// `{foo}`
    Shorthand,
}

impl Rule for UseConsistentObjectDefinition {
    type Query = Ast<AnyJsObjectMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentObjectDefinitionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let options = ctx.options();
        match binding {
            AnyJsObjectMember::JsShorthandPropertyObjectMember(_) => match options.syntax {
                // Shorthand properties should error when explicit is expected
                ObjectPropertySyntax::Shorthand => None,
                ObjectPropertySyntax::Explicit => Some(()),
            },
            AnyJsObjectMember::JsMethodObjectMember(_) => match options.syntax {
                // Shorthand methods should error when explicit is expected
                ObjectPropertySyntax::Shorthand => None,
                ObjectPropertySyntax::Explicit => Some(()),
            },
            AnyJsObjectMember::JsPropertyObjectMember(source) => {
                let value_token = source.value().ok()?;
                let value_id = match value_token {
                    AnyJsExpression::JsIdentifierExpression(identifier_token) => {
                        // If expression is an identifier, get ID to compare it against the property name later
                        let variable_token = identifier_token.name().ok()?.value_token().ok()?;
                        inner_string_text(&variable_token)
                    }
                    AnyJsExpression::JsFunctionExpression(_function_token) => {
                        // Functions are always shorthandable
                        match options.syntax {
                            ObjectPropertySyntax::Shorthand => return Some(()),
                            ObjectPropertySyntax::Explicit => return None,
                        }
                    }
                    _ => return None,
                };
                let name_token = source.name().ok()?;
                match name_token {
                    AnyJsObjectMemberName::JsLiteralMemberName(literal_token) => {
                        match options.syntax {
                            ObjectPropertySyntax::Shorthand => {
                                // Throw shorthand error if the value is the same as the property name
                                // We use `text_trimmed` to preserve quotes when comparing, we need this
                                // because {foo: foo} can be shorthanded, but {"foo": foo} cannot
                                if literal_token.value().ok()?.text_trimmed() == value_id.trim() {
                                    Some(())
                                } else {
                                    None
                                }
                            }
                            ObjectPropertySyntax::Explicit => None,
                        }
                    }
                    AnyJsObjectMemberName::JsComputedMemberName(_computed_token) => {
                        let reference_token = source.value().ok()?;
                        // Computed is always shorthandable if the value is a function, else never
                        match reference_token {
                            AnyJsExpression::JsFunctionExpression(_function_token) => {
                                match options.syntax {
                                    ObjectPropertySyntax::Shorthand => Some(()),
                                    ObjectPropertySyntax::Explicit => None,
                                }
                            }
                            _ => None,
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let options = ctx.options();

        let title = match options.syntax {
            ObjectPropertySyntax::Shorthand => {
                "Do not use explicit object property syntax when shorthand syntax is possible."
            }
            ObjectPropertySyntax::Explicit => "Do not use shorthand object property syntax.",
        };

        let note = match options.syntax {
            ObjectPropertySyntax::Shorthand => {
                "Using shorthand object property syntax makes object definitions more concise."
            }
            ObjectPropertySyntax::Explicit => {
                "Using explicit object property syntax makes object definitions more readable and consistent."
            }
        };

        Some(
            RuleDiagnostic::new(rule_category!(), node.range(), markup! {{title}})
                .note(markup! {{note}}),
        )
    }
}
