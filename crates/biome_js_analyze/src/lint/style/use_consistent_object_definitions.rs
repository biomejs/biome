use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, JsLanguage, T, inner_string_text,
};
use biome_rowan::{AstNode, BatchMutationExt, TriviaPieceKind};
use biome_rule_options::use_consistent_object_definitions::{
    ObjectPropertySyntax, UseConsistentObjectDefinitionsOptions,
};

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
    /// - `shorthand`: enforces the use of shorthand object property syntax when possible.
    /// - `explicit`: enforces the use of explicit object property syntax in every case.
    ///
    /// **Default:** `shorthand`
    ///
    pub UseConsistentObjectDefinitions {
        version: "2.0.0",
        name: "useConsistentObjectDefinitions",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Safe,
        severity: Severity::Warning,
        sources: &[RuleSource::Eslint("object-shorthand").inspired()],
    }
}

impl Rule for UseConsistentObjectDefinitions {
    type Query = Ast<AnyJsObjectMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseConsistentObjectDefinitionsOptions;

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

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleAction<JsLanguage>> {
        let node = ctx.query();
        let options = ctx.options();
        let mut mutation = ctx.root().begin();

        let new_node = match node {
            AnyJsObjectMember::JsShorthandPropertyObjectMember(node) => {
                let leading_trivia = node.syntax().first_leading_trivia()?;
                let node = node.clone().with_leading_trivia_pieces([])?;

                make::js_property_object_member(
                    make::js_literal_member_name(node.name().ok()?.value_token().ok()?).into(),
                    make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    make::js_identifier_expression(node.name().ok()?).into(),
                )
                .with_leading_trivia_pieces(leading_trivia.pieces())?
                .into()
            }
            AnyJsObjectMember::JsMethodObjectMember(node) => {
                let leading_trivia = node.syntax().first_leading_trivia()?;
                let node = node.clone().with_leading_trivia_pieces([])?;

                let mut func_token = make::token(T![function]);
                if node.star_token().is_none() {
                    func_token =
                        func_token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                }

                let mut func = make::js_function_expression(
                    func_token,
                    node.parameters().ok()?,
                    node.body().ok()?,
                );

                if let Some(token) = node.async_token() {
                    func = func.with_async_token(token);
                }

                if let Some(token) = node.star_token() {
                    // `*foo() {}` -> `foo: function* () {}`
                    func = func.with_star_token(
                        token.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    );
                }

                if let Some(type_params) = node.type_parameters() {
                    func = func.with_type_parameters(type_params);
                }

                if let Some(return_type) = node.return_type_annotation() {
                    func = func.with_return_type_annotation(return_type);
                }

                make::js_property_object_member(
                    node.name().ok()?,
                    make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    func.build().into(),
                )
                .with_leading_trivia_pieces(leading_trivia.pieces())?
                .into()
            }
            AnyJsObjectMember::JsPropertyObjectMember(node) => {
                let leading_trivia = node.syntax().first_leading_trivia()?;
                let node = node.clone().with_leading_trivia_pieces([])?;

                match node.value().ok()? {
                    AnyJsExpression::JsIdentifierExpression(expr) => {
                        make::js_shorthand_property_object_member(expr.name().ok()?)
                            .with_leading_trivia_pieces(leading_trivia.pieces())?
                            .into()
                    }
                    AnyJsExpression::JsFunctionExpression(expr) => {
                        let mut func = make::js_method_object_member(
                            node.name().ok()?,
                            expr.parameters().ok()?,
                            expr.body().ok()?,
                        );

                        if let Some(token) = expr.async_token() {
                            func = func.with_async_token(token);
                        }

                        if let Some(token) = expr.star_token() {
                            // `foo: function* () {}` -> `*foo() {}`
                            func = func.with_star_token(token.trim_trailing_trivia());
                        }

                        if let Some(type_params) = expr.type_parameters() {
                            func = func.with_type_parameters(type_params);
                        }

                        if let Some(return_type) = expr.return_type_annotation() {
                            func = func.with_return_type_annotation(return_type);
                        }

                        func.build()
                            .with_leading_trivia_pieces(leading_trivia.pieces())?
                            .into()
                    }
                    _ => return None,
                }
            }
            _ => return None,
        };

        mutation.replace_node(node.clone(), new_node);

        let message = match options.syntax {
            ObjectPropertySyntax::Explicit => {
                markup! { "Use explicit object property syntax." }.to_owned()
            }
            ObjectPropertySyntax::Shorthand => {
                markup! { "Use shorthand object property syntax." }.to_owned()
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message,
            mutation,
        ))
    }
}
