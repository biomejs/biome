use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsBindingPattern, JsCatchClause, JsThrowStatement};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::no_rethrow_without_cause::NoRethrowWithoutCauseOptions;

declare_lint_rule! {
    /// Disallow rethrowing caught errors without wrapping them.
    ///
    /// When rethrowing a caught error, it's recommended to wrap it in a new `Error` object to preserve the original error's stack trace and context.
    /// The original error should be passed as the `cause` property of the new `Error` object.
    ///
    /// This rule enforces that practice, helping to maintain a clear and traceable error propagation chain, which is crucial for effective debugging.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///   // ...
    /// } catch (err) {
    ///   throw new Error(err.message);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    /// 	doSomething();
    /// } catch {
    /// 	throw new Error("Something went wrong");
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// try {
    ///   // ...
    /// } catch ({ message }) {
    ///   throw new Error(message);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// try {
    ///   // ...
    /// } catch (err) {
    ///   throw new Error("Something went wrong", { cause: err });
    /// }
    /// ```
    ///
    pub NoRethrowWithoutCause {
        version: "next",
        name: "noRethrowWithoutCause",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("preserve-caught-error").same()],
    }
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    WithoutCause(TextRange),
    NoErrorBinding(TextRange),
    DestructuringBinding(TextRange),
}

impl Rule for NoRethrowWithoutCause {
    type Query = Ast<JsThrowStatement>;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoRethrowWithoutCauseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let throw_statement = ctx.query();
        let options = ctx.options();

        let catch_clause = throw_statement
            .syntax()
            .ancestors()
            .find_map(JsCatchClause::cast)?;

        let throw_syntax = throw_statement.syntax();
        for ancestor in throw_syntax.ancestors() {
            // Stop traversing once we reach the catch clause itself
            if ancestor == *catch_clause.syntax() {
                break;
            }

            // Check for function-like nodes that introduce a new scope
            if biome_js_syntax::AnyJsFunction::can_cast(ancestor.kind())
                || biome_js_syntax::JsClassDeclaration::can_cast(ancestor.kind())
                || biome_js_syntax::JsClassExpression::can_cast(ancestor.kind())
                || biome_js_syntax::JsMethodObjectMember::can_cast(ancestor.kind())
            {
                // The throw is inside a function/method/class defined within the catch block.
                // This is likely an independent error and should be ignored by this rule.
                return None;
            }
        }

        if let Some(catch_declaration) = catch_clause.declaration() {
            let binding = catch_declaration.binding().ok()?;
            match binding {
                AnyJsBindingPattern::AnyJsBinding(catch_error_binding) => {
                    let identifier_binding = catch_error_binding.as_js_identifier_binding()?;

                    let catch_error_name = identifier_binding.name_token().ok()?;
                    let thrown_expression = throw_statement.argument().ok()?;

                    let Some(new_expression) = thrown_expression.as_js_new_expression() else {
                        // Not a `new` expression, so the rule does not apply. This handles `throw e;` cases.
                        return None;
                    };

                    let Some(arguments) = new_expression.arguments() else {
                        return Some(State::WithoutCause(throw_statement.range()));
                    };
                    let args = arguments.args();

                    if args.len() < 2 {
                        return Some(State::WithoutCause(throw_statement.range()));
                    }

                    let Some(Ok(second_arg_expr)) = args.iter().nth(1) else {
                        return Some(State::WithoutCause(throw_statement.range()));
                    };
                    let Some(expr) = second_arg_expr.as_any_js_expression() else {
                        return Some(State::WithoutCause(throw_statement.range()));
                    };
                    let Some(obj_expr) = expr.as_js_object_expression() else {
                        return Some(State::WithoutCause(throw_statement.range()));
                    };

                    for member in obj_expr.members().iter().flatten() {
                        if let Some(prop) = member.as_js_property_object_member() {
                            let is_cause_prop = prop
                                .name()
                                .ok()
                                .and_then(|name_node| name_node.name())
                                .is_some_and(|name| name == "cause");

                            if is_cause_prop {
                                let is_correct_error = prop.value().ok().is_some_and(|value| {
                                    value
                                        .as_js_identifier_expression()
                                        .and_then(|ident_expr| ident_expr.name().ok())
                                        .and_then(|name| name.value_token().ok())
                                        .is_some_and(|token| {
                                            token.token_text_trimmed()
                                                == catch_error_name.text_trimmed()
                                        })
                                });

                                if is_correct_error {
                                    return None;
                                }
                            }
                        }
                    }

                    // If no valid cause was found after checking all members, it's a violation.
                    Some(State::WithoutCause(throw_statement.range()))
                }
                AnyJsBindingPattern::JsArrayBindingPattern(_)
                | AnyJsBindingPattern::JsObjectBindingPattern(_) => {
                    Some(State::DestructuringBinding(binding.range()))
                }
            }
        } else {
            if !options.require_catch_parameter {
                return None;
            }
            // This is the case `catch {}`.
            // Any `throw` inside is a problem.
            Some(State::NoErrorBinding(throw_statement.range()))
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            State::WithoutCause(range) => Some(RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The original error is not being passed to the new `Error` object.\
                    Include the original error in the `cause` property to preserve it."
                },
            )),
            State::NoErrorBinding(range) => Some(RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The original error is being discarded because the `catch` clause doesn't have a parameter.\
                    Specify an error object in the `catch` clause to access the original error."
                },
            )),
            State::DestructuringBinding(range) => Some(RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Destructuring the error in a `catch` clause is not recommended, \
                    as it can lead to losing important information from the error object, \
                    such as the stack trace.\
                    Use a single variable to catch the error, and then access its properties."
                },
            )),
        }
    }
}
