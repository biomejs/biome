use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{self, js_function_body};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, JsArrayBindingPatternElement, JsArrowFunctionExpression,
    JsFormalParameter, JsObjectBindingPatternShorthandProperty, JsReturnStatement,
    JsVariableStatement, JsYieldArgument, T,
};
use biome_rowan::{declare_node_union, AstNode, BatchMutationExt, TextRange, TokenText};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow the use of useless `undefined`.
    ///
    /// `undefined` is the default value for new variables, parameters, return statements, etc., so specifying it doesn't make any difference.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = undefined;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const {foo = undefined} = bar;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const noop = () => undefined;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo() {
    ///    return undefined;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function* foo() {
    ///   yield undefined;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo(bar = undefined) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo({bar = undefined}) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo;
    /// const {foo} = bar;
    /// function foo() {
    ///   return;
    /// }
    /// function* foo() {
    ///   yield;
    /// }
    /// function foo(bar) {}
    /// function foo({bar}) {}
    /// foo();
    /// ```
    ///
    pub NoUselessUndefined {
        version: "next",
        name: "noUselessUndefined",
        language: "js",
        fix_kind: FixKind::Safe,
        sources: &[RuleSource::EslintUnicorn("no-useless-undefined")],
        recommended: false,
    }
}

declare_node_union! {
    pub AnyUndefinedNode = JsVariableStatement
        | JsObjectBindingPatternShorthandProperty
        | JsYieldArgument
        | JsReturnStatement
        | JsArrayBindingPatternElement
        | JsArrowFunctionExpression
        | JsFormalParameter
}

fn find_undefined_range(expr: Option<&AnyJsExpression>) -> Option<TextRange> {
    let ident = expr?.as_js_reference_identifier()?;
    if ident.is_undefined() {
        Some(ident.range())
    } else {
        None
    }
}

pub struct RuleState {
    binding_text: Option<TokenText>,
    diagnostic_range: TextRange,
}

impl Rule for NoUselessUndefined {
    type Query = Ast<AnyUndefinedNode>;
    type State = RuleState;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut signals = vec![];

        match node {
            // let foo = undefined;
            AnyUndefinedNode::JsVariableStatement(statement) => {
                let Ok(node) = statement.declaration() else {
                    return signals.into_boxed_slice();
                };
                let let_or_var = node.is_let() || node.is_var();
                if !let_or_var {
                    return signals.into_boxed_slice();
                }

                for declarator in node.declarators() {
                    let Ok(decl) = declarator else { continue };
                    let Some(initializer) = decl.initializer() else {
                        continue;
                    };
                    let expr = initializer.expression().ok();
                    if let Some(undefined_range) = find_undefined_range(expr.as_ref()) {
                        let Ok(binding_text) = decl.id() else {
                            continue;
                        };
                        if let Some(binding) = binding_text.as_any_js_binding() {
                            if let Some(ident_binding) = binding.as_js_identifier_binding() {
                                let binding_text = ident_binding
                                    .name_token()
                                    .map(|t| t.token_text_trimmed())
                                    .ok();
                                signals.push(RuleState {
                                    binding_text,
                                    diagnostic_range: undefined_range,
                                });
                            }
                        }
                    }
                }
            }
            // { a: undefined }
            AnyUndefinedNode::JsObjectBindingPatternShorthandProperty(
                js_object_binding_pattern_shorthand_property,
            ) => {
                if let Some(init) = js_object_binding_pattern_shorthand_property.init() {
                    let expr = init.expression().ok();
                    if let Some(range) = find_undefined_range(expr.as_ref()) {
                        signals.push(RuleState {
                            binding_text: None,
                            diagnostic_range: range,
                        });
                    }
                }
            }
            // function foo([bar = undefined]) {}
            AnyUndefinedNode::JsArrayBindingPatternElement(js_array_binding_pattern_element) => {
                if let Some(init) = js_array_binding_pattern_element.init() {
                    let expr = init.expression().ok();
                    if let Some(range) = find_undefined_range(expr.as_ref()) {
                        signals.push(RuleState {
                            binding_text: None,
                            diagnostic_range: range,
                        });
                    }
                }
            }
            // yield undefined
            AnyUndefinedNode::JsYieldArgument(yield_argument) => {
                if yield_argument.star_token().is_some() {
                    return signals.into_boxed_slice();
                }
                let expr = yield_argument.expression().ok();
                if let Some(range) = find_undefined_range(expr.as_ref()) {
                    signals.push(RuleState {
                        binding_text: None,
                        diagnostic_range: range,
                    });
                }
            }
            // return undefined
            AnyUndefinedNode::JsReturnStatement(js_return_statement) => {
                let expr = js_return_statement.argument();
                if let Some(range) = find_undefined_range(expr.as_ref()) {
                    signals.push(RuleState {
                        binding_text: None,
                        diagnostic_range: range,
                    });
                }
            }
            // const noop = () => undefined
            AnyUndefinedNode::JsArrowFunctionExpression(js_arrow_function_expression) => {
                if let Ok(body) = js_arrow_function_expression.body() {
                    let expr = body.as_any_js_expression();
                    if let Some(range) = find_undefined_range(expr) {
                        signals.push(RuleState {
                            binding_text: None,
                            diagnostic_range: range,
                        });
                    }
                }
            }
            // function foo(bar = undefined) {}
            AnyUndefinedNode::JsFormalParameter(js_formal_parameter) => {
                if let Some(init) = js_formal_parameter.initializer() {
                    let expr = init.expression().ok();
                    if let Some(range) = find_undefined_range(expr.as_ref()) {
                        signals.push(RuleState {
                            binding_text: None,
                            diagnostic_range: range,
                        });
                    }
                }
            }
        };

        signals.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.diagnostic_range,
                markup! {
                    "Don't use unnecessary "<Emphasis>"undefined"</Emphasis>"."
                },
            )
                .note(markup! {
                ""<Emphasis>"undefined"</Emphasis>" is the default value for new variables, parameters, return statements, etc… so specifying it doesn't make any difference."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            AnyUndefinedNode::JsVariableStatement(js_variable_statement) => {
                let current_declaration_statement = js_variable_statement.declaration().ok()?;
                let declarators = current_declaration_statement.declarators();

                let current_declaration =
                    declarators
                        .into_iter()
                        .filter_map(Result::ok)
                        .find(|decl| {
                            decl.id().is_ok_and(|id| {
                                id.syntax().text_trimmed()
                                    == state.binding_text.as_deref().unwrap_or("")
                            })
                        })?;

                let current_initializer = current_declaration.initializer()?;
                mutation.remove_node(current_initializer);
            }
            AnyUndefinedNode::JsObjectBindingPatternShorthandProperty(property) => {
                mutation.remove_node(property.init()?);
            }
            AnyUndefinedNode::JsYieldArgument(yield_argument) => {
                mutation.remove_node(yield_argument.expression().ok()?);
            }
            AnyUndefinedNode::JsReturnStatement(return_statement) => {
                mutation.remove_node(return_statement.argument()?);
            }
            AnyUndefinedNode::JsArrayBindingPatternElement(pattern_element) => {
                let init = pattern_element.init()?;
                mutation.remove_node(init)
            }
            AnyUndefinedNode::JsArrowFunctionExpression(js_arrow_function_expression) => {
                let undefined_body = js_arrow_function_expression.body().ok()?;
                let next_node = js_function_body(
                    make::token(T!['{']),
                    make::js_directive_list(None),
                    make::js_statement_list(None),
                    make::token(T!['}']),
                );
                mutation.replace_node_discard_trivia(
                    undefined_body,
                    AnyJsFunctionBody::JsFunctionBody(next_node),
                );
            }
            AnyUndefinedNode::JsFormalParameter(js_formal_parameter) => {
                let init = js_formal_parameter.initializer()?;
                mutation.remove_node(init);
            }
        };

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the undefined."}.to_owned(),
            mutation,
        ))
    }
}
