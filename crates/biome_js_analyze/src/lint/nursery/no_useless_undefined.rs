use biome_analyze::{
    context::RuleContext, declare_lint_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
    RuleSource,
};
use biome_console::markup;
use biome_js_factory::make::{self, js_function_body, js_variable_declarator_list};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, JsArrayBindingPatternElement, JsArrowFunctionExpression,
    JsCallExpression, JsFormalParameter, JsLanguage, JsObjectBindingPatternShorthandProperty,
    JsReturnStatement, JsSyntaxToken, JsVariableDeclarator, JsVariableStatement, JsYieldArgument,
    T,
};
use biome_rowan::{
    chain_trivia_pieces, declare_node_union, AstNode, AstSeparatedList, BatchMutationExt,
    SyntaxTriviaPiece, TextRange, TriviaPieceKind,
};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow useless `undefined`.
    ///
    /// `undefined` is the default value for new variables, parameters, return statements, etc… so specifying it doesn't make any difference.
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
    /// ```js,expect_diagnostic
    /// foo(undefined);
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
        fix_kind: FixKind::Unsafe,
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
        | JsCallExpression
        | JsArrowFunctionExpression
        | JsFormalParameter
}

static FUNCTION_NAMES: [&str; 28] = [
    // `set.add(undefined)`
    "add",
    // Function#bind()
    "bind",
    // `React.createContext(undefined)`
    "createContext",
    // Compare function names
    "equal",
    // `set.has(undefined)`
    "has",
    // `array.includes(undefined)`
    "include",
    // `array.includes(undefined)`
    "includes",
    // Compare function names
    "is",
    "not",
    "notEqual",
    "notPropertyVal",
    "notSame",
    "notStrictEqual",
    "property",
    "propertyVal",
    // `array.push(undefined)`
    "push",
    // https://vuejs.org/api/reactivity-core.html#ref
    "ref",
    "same",
    // `map.set(foo, undefined)`
    "set",
    "strictEqual",
    "strictNotSame",
    "strictSame",
    "toBe",
    "toContain",
    "toContainEqual",
    "toEqual",
    "toHaveBeenCalledWith",
    // `array.unshift(undefined)`
    "unshift",
];

fn should_ignore_function(expr: &AnyJsExpression) -> bool {
    let name = match expr {
        AnyJsExpression::JsIdentifierExpression(ident) => ident.text(),
        AnyJsExpression::JsStaticMemberExpression(member_expr) => {
            if let Ok(member) = member_expr.member() {
                member.text()
            } else {
                return false;
            }
        }
        _ => return false,
    };

    FUNCTION_NAMES.binary_search(&name.as_str()).is_ok() ||
    // setState(undefined), setXXX(undefined)
    name.starts_with("set")
}

fn is_undefined(expr: Option<&AnyJsExpression>) -> Option<(String, TextRange)> {
    let ident = expr?.as_js_reference_identifier()?;
    if ident.is_undefined() {
        Some((ident.text().to_string(), ident.range()))
    } else {
        None
    }
}

impl Rule for NoUselessUndefined {
    type Query = Ast<AnyUndefinedNode>;
    type State = (String, TextRange);
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut signals = vec![];

        match node {
            // let foo = undefined;
            AnyUndefinedNode::JsVariableStatement(statement) => {
                let Ok(node) = statement.declaration() else {
                    return signals;
                };
                let let_or_var = node.is_let() || node.is_var();
                if !let_or_var {
                    return signals;
                }

                for declarator in node.declarators() {
                    let Ok(decl) = declarator else { continue };
                    if let Some(initializer) = decl.initializer() {
                        let expr = initializer.expression().ok();
                        if let Some(state) = is_undefined(expr.as_ref()) {
                            signals.push(state);
                        }
                    }
                }
            }
            // foo(bar, undefined, undefined);
            AnyUndefinedNode::JsCallExpression(js_call_expr) => {
                if let Ok(callee) = js_call_expr.callee() {
                    if should_ignore_function(&callee) {
                        return signals;
                    }
                };

                let Some(js_call_argument_list) = js_call_expr.arguments().ok() else {
                    return signals;
                };
                let call_argument_list = js_call_argument_list.args();
                let mut non_undefined_found = false;
                for argument in call_argument_list.iter().rev().flatten() {
                    if non_undefined_found {
                        return signals;
                    }
                    let expr = argument.as_any_js_expression();
                    if let Some((name, range)) = is_undefined(expr) {
                        signals.push((name, range));
                    } else {
                        non_undefined_found = true;
                    }
                }
            }
            // { a: undefined }
            AnyUndefinedNode::JsObjectBindingPatternShorthandProperty(
                js_object_binding_pattern_shorthand_property,
            ) => {
                if let Some(init) = js_object_binding_pattern_shorthand_property.init() {
                    let expr = init.expression().ok();
                    if let Some((name, range)) = is_undefined(expr.as_ref()) {
                        signals.push((name, range));
                    }
                }
            }
            // function foo([bar = undefined]) {}
            AnyUndefinedNode::JsArrayBindingPatternElement(js_array_binding_pattern_element) => {
                if let Some(init) = js_array_binding_pattern_element.init() {
                    let expr = init.expression().ok();
                    if let Some((name, range)) = is_undefined(expr.as_ref()) {
                        signals.push((name, range));
                    }
                }
            }
            // yield undefined
            AnyUndefinedNode::JsYieldArgument(yield_argument) => {
                if yield_argument.star_token().is_some() {
                    return signals;
                }
                let expr = yield_argument.expression().ok();
                if let Some((name, range)) = is_undefined(expr.as_ref()) {
                    signals.push((name, range));
                }
            }
            // return undefined
            AnyUndefinedNode::JsReturnStatement(js_return_statement) => {
                let expr = js_return_statement.argument();
                if let Some((name, range)) = is_undefined(expr.as_ref()) {
                    signals.push((name, range));
                }
            }
            // const noop = () => undefined
            AnyUndefinedNode::JsArrowFunctionExpression(js_arrow_function_expression) => {
                if let Ok(body) = js_arrow_function_expression.body() {
                    let expr = body.as_any_js_expression();
                    if let Some((name, range)) = is_undefined(expr) {
                        signals.push((name, range));
                    }
                }
            }
            // function foo(bar = undefined) {}
            AnyUndefinedNode::JsFormalParameter(js_formal_parameter) => {
                if let Some(init) = js_formal_parameter.initializer() {
                    let expr = init.expression().ok();
                    if let Some((name, range)) = is_undefined(expr.as_ref()) {
                        signals.push((name, range));
                    }
                }
            }
        };

        signals
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.1,
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
                let assignment_statement = js_variable_statement.clone();
                let current_declaration_statement = js_variable_statement.declaration().ok()?;
                let declarators = current_declaration_statement.declarators();

                let current_declaration = declarators
                    .clone()
                    .into_iter()
                    .filter_map(|declarator| declarator.ok())
                    .find(|decl| decl.id().is_ok_and(|id| id.text() == state.0))?;

                let current_initializer = current_declaration.initializer()?;

                let eq_token_trivia = current_initializer
                    .eq_token()
                    .map(|token| token.trailing_trivia())
                    .ok()?
                    .pieces();

                let expression_trivia = current_initializer
                    .expression()
                    .ok()?
                    .as_js_reference_identifier()
                    .map(|reference| reference.value_token())?
                    .ok()?
                    .trailing_trivia()
                    .pieces();

                // Save the separators too
                let separators_syntax = declarators.clone().into_syntax();
                let separators: Vec<JsSyntaxToken> = separators_syntax.tokens().collect();

                let new_declaration = current_declaration.clone().with_initializer(None);
                let new_declarators: Vec<JsVariableDeclarator> = declarators
                    .clone()
                    .into_iter()
                    .filter_map(|decl| decl.ok())
                    .map(|decl| {
                        if decl == current_declaration {
                            new_declaration.clone()
                        } else {
                            decl
                        }
                    })
                    .collect();

                // Recreate the declaration statement with updated declarators
                let new_declaration_statement = current_declaration_statement
                    .with_declarators(js_variable_declarator_list(new_declarators, separators));

                let chained_comments: Vec<SyntaxTriviaPiece<JsLanguage>> =
                    chain_trivia_pieces(eq_token_trivia, expression_trivia)
                        .filter(|trivia| trivia.is_comments())
                        .collect();

                // Create the whole statement using updated subtree and append comments to the statement
                let new_node = assignment_statement
                    .clone()
                    .with_declaration(new_declaration_statement)
                    .append_trivia_pieces(chained_comments)?;

                mutation.replace_node_discard_trivia(assignment_statement, new_node);

                return Some(JsRuleAction::new(
                    ActionCategory::QuickFix,
                    ctx.metadata().applicability(),
                    markup! { "Remove the undefined."}.to_owned(),
                    mutation,
                ));
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
            AnyUndefinedNode::JsCallExpression(js_call_expression) => {
                let arguments = js_call_expression.arguments().ok()?;
                let argument_list = arguments.args();

                let mut non_undefined_index = None;
                for (idx, arg) in argument_list.iter().rev().enumerate() {
                    let expr = arg.ok()?;
                    let expr = expr.as_any_js_expression();
                    if is_undefined(expr).is_none() {
                        non_undefined_index = Some(idx);
                        break;
                    }
                }

                match non_undefined_index {
                    Some(idx) => {
                        let new_arguments = argument_list
                            .iter()
                            .take(argument_list.len() - idx)
                            .filter_map(Result::ok)
                            .collect::<Vec<_>>();

                        let separators = make::token(T![,])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
                        let last_token = argument_list.syntax().last_token()?;

                        let new_argument_list =
                            make::js_call_argument_list(new_arguments, Some(separators))
                                .with_leading_trivia_pieces(last_token.leading_trivia().pieces())?;
                        mutation.replace_node(argument_list, new_argument_list);
                    }
                    None => mutation.remove_node(argument_list),
                };
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
            ActionCategory::QuickFix,
            ctx.metadata().applicability(),
            markup! { "Remove the undefined."}.to_owned(),
            mutation,
        ))
    }
}

#[test]
fn test_order() {
    for items in FUNCTION_NAMES.windows(2) {
        assert!(items[0] < items[1]);
    }
}
