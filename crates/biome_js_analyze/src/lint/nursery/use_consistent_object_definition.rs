use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{inner_string_text, AnyJsExpression, AnyJsObjectMember};
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
    /// ```js,expect_diagnostic
    /// let foo = 1;
    /// let invalid = {
    ///     foo
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let invalid = {
    ///     bar() { return "bar"; },
    /// };
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// let foo = 1;
    /// let valid = {
    ///     foo: foo,
    ///     bar: function() { return "bar"; },
    ///     arrow: () => { "arrow" },
    ///     get getter() { return "getter"; },
    ///     set setter(value) { this._setter = value; }
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
    /// - `explicit`: enforces the use of explicit object property syntax in every case
    /// - `shorthand`: enforces the use of shorthand object property syntax when possible
    ///
    /// **Default:** `explicit`
    ///
    pub UseConsistentObjectDefinition {
        version: "next",
        name: "useConsistentObjectDefinition",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("object-shorthand")],
        source_kind: RuleSourceKind::Inspired,
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentObjectDefinitionOptions {
    syntax: ObjectPropertySyntax,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ObjectPropertySyntax {
    #[default]
    Explicit,
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
                ObjectPropertySyntax::Shorthand => None,
                ObjectPropertySyntax::Explicit => Some(()),
            },
            AnyJsObjectMember::JsPropertyObjectMember(source) => {
                let member_token = source
                    .name()
                    .ok()?
                    .as_js_literal_member_name()?
                    .value()
                    .ok()?;
                let member_id = inner_string_text(&member_token);
                let reference_token = source.value().ok()?;
                let reference_id = match reference_token {
                    AnyJsExpression::JsIdentifierExpression(identifier) => {
                        let variable_token = identifier.name().ok()?.value_token().ok()?;
                        inner_string_text(&variable_token)
                    }
                    AnyJsExpression::JsCallExpression(call) => {
                        let callee_token = call
                            .callee()
                            .ok()?
                            .as_js_identifier_expression()?
                            .name()
                            .ok()?
                            .value_token()
                            .ok()?;
                        inner_string_text(&callee_token)
                    }
                    AnyJsExpression::JsFunctionExpression(function) => {
                        let function_token = function.function_token().ok()?;
                        inner_string_text(&function_token)
                    }
                    _ => return None,
                };

                match options.syntax {
                    ObjectPropertySyntax::Shorthand => {
                        if member_id == reference_id || "function" == reference_id {
                            Some(())
                        } else {
                            None
                        }
                    }
                    ObjectPropertySyntax::Explicit => None,
                }
            }
            AnyJsObjectMember::JsMethodObjectMember(_) => match options.syntax {
                ObjectPropertySyntax::Shorthand => None,
                ObjectPropertySyntax::Explicit => Some(()),
            },
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
