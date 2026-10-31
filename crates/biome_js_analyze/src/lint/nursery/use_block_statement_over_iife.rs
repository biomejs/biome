use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, AnyJsVariableDeclaration, JsArrowFunctionExpression,
    JsCallExpression, JsConstructorParameters, JsExpressionStatement, JsFunctionBody,
    JsIdentifierAssignment, JsLabeledStatement, JsLanguage, JsParameters, JsReferenceIdentifier,
    JsSyntaxKind, JsxReferenceIdentifier,
};
use biome_languages::JsFileSource;
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, BatchMutationExt, Direction, SyntaxKindSet, TextRange,
    WalkEvent,
};
use biome_rule_options::use_block_statement_over_iife::UseBlockStatementOverIifeOptions;

declare_lint_rule! {
    /// Prefer block statements over functions that are called immediately only to create a scope.
    ///
    /// A function that is called as soon as it is created is known as an immediately invoked
    /// function expression (IIFE). Before `let` and `const` existed, wrapping code in such a
    /// function was the only way to stop its variables from being visible to the code around it.
    ///
    /// Variables declared with `let`, `const`, or `class` are only visible inside the block
    /// statement (code wrapped in `{` and `}`) that contains them. A block statement therefore
    /// hides variables in the same way, with less code and without a function call.
    ///
    /// This rule only reports functions that a block statement can replace without changing how
    /// the code behaves. It ignores a function that is called immediately when:
    ///
    /// - the result of the call is used, or the call passes arguments;
    /// - the function has a name, has parameters, is `async`, or is a generator;
    /// - the function contains a `return` statement, a `var` declaration, a call to `eval`, or a
    ///   directive such as `"use strict"`;
    /// - the function is declared with the `function` keyword and uses `this`, `arguments`, or
    ///   `new.target`;
    /// - the function contains a function declaration, and the file is a script instead of a
    ///   module, such as a `.cjs` file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// (() => {
    ///     const value = getValue();
    ///     run(value);
    /// })();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// (function () {
    ///     const value = getValue();
    ///     run(value);
    /// })();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// {
    ///     const value = getValue();
    ///     run(value);
    /// }
    ///
    /// const result = (() => {
    ///     return getValue();
    /// })();
    ///
    /// (async () => {
    ///     await run();
    /// })();
    ///
    /// (function () {
    ///     this.run();
    /// })();
    ///
    /// (() => {
    ///     var count = 1;
    ///     run(count);
    /// })();
    /// ```
    ///
    pub UseBlockStatementOverIife {
        version: "2.6.3",
        name: "useBlockStatementOverIife",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-block-statement-over-iife").same()],
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseBlockStatementOverIife {
    type Query = Ast<JsExpressionStatement>;
    type State = ScopeOnlyFunction;
    type Signals = Option<Self::State>;
    type Options = UseBlockStatementOverIifeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();
        let call = statement.expression().ok()?.omit_parentheses();
        let call = call.as_js_call_expression()?;
        if call.is_optional()
            || call.type_arguments().is_some()
            || !call.arguments().ok()?.args().is_empty()
        {
            return None;
        }

        let (body, has_own_this) = match call.callee().ok()?.omit_parentheses() {
            AnyJsExpression::JsArrowFunctionExpression(arrow) => {
                if arrow.async_token().is_some()
                    || arrow.type_parameters().is_some()
                    || !arrow
                        .parameters()
                        .ok()?
                        .as_js_parameters()?
                        .items()
                        .is_empty()
                {
                    return None;
                }
                (arrow.body().ok()?.as_js_function_body()?.clone(), false)
            }
            AnyJsExpression::JsFunctionExpression(function) => {
                if function.async_token().is_some()
                    || function.star_token().is_some()
                    || function.id().is_some()
                    || function.type_parameters().is_some()
                    || !function.parameters().ok()?.items().is_empty()
                {
                    return None;
                }
                (function.body().ok()?, true)
            }
            _ => return None,
        };

        if !body.directives().is_empty() {
            return None;
        }

        let is_module = ctx.source_type::<JsFileSource>().is_module();
        if needs_function_scope(statement, &body, has_own_this, is_module) {
            return None;
        }

        Some(ScopeOnlyFunction {
            has_comments_outside_body: has_comments_outside_body(statement, &body),
            body,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let start = ctx.query().syntax().text_trimmed_range().start();
        let end = state.body.l_curly_token().ok()?.text_trimmed_range().end();
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            TextRange::new(start, end),
            markup! {
                "This function is called immediately, and a block statement can replace it."
            },
        )
        .note(markup! {
            "A block statement hides "<Emphasis>"let"</Emphasis>", "<Emphasis>"const"</Emphasis>", and "<Emphasis>"class"</Emphasis>" declarations from the surrounding code in the same way, with less code and without a function call."
        });

        Some(if state.has_comments_outside_body {
            diagnostic.note(markup! {
                "Replace the function and its call with a block statement, and move the comments that are outside of the function body."
            })
        } else {
            diagnostic
        })
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        if state.has_comments_outside_body {
            return None;
        }

        let statement = ctx.query();
        let body = &state.body;
        let block = make::js_block_statement(
            body.l_curly_token().ok()?,
            body.statements(),
            body.r_curly_token().ok()?,
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node(AnyJsStatement::from(statement.clone()), block.into());

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a block statement instead." }.to_owned(),
            mutation,
        ))
    }
}

/// A function that is called immediately, and that a block statement can replace.
pub struct ScopeOnlyFunction {
    body: JsFunctionBody,
    /// Whether the statement that calls the function has a comment that is neither inside `body`,
    /// nor before or after the whole statement. Replacing the statement with `body` would drop
    /// such a comment.
    has_comments_outside_body: bool,
}

/// Parts of a function that have their own `this`, `arguments`, `return` target, and `var` scope,
/// unless the function is an arrow function.
///
/// The names of object and class members are not included, because a computed name such as
/// `[this.key]() {}` is evaluated by the code around the member.
const FUNCTION_SCOPES: SyntaxKindSet<JsLanguage> = JsFunctionBody::KIND_SET
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET);

/// Returns whether the code in `body` behaves differently when `body` becomes a block statement
/// that replaces `statement`.
///
/// `body` belongs to the function that `statement` calls immediately. `has_own_this` tells whether
/// that function is declared with the `function` keyword, which gives it its own `this`,
/// `arguments`, and `new.target`.
///
/// Class field initializers and static blocks are searched as if they were part of `body`, so a
/// `this` or a `var` inside them also counts.
fn needs_function_scope(
    statement: &JsExpressionStatement,
    body: &JsFunctionBody,
    has_own_this: bool,
    is_module: bool,
) -> bool {
    // Nested arrow functions share `this`, `arguments`, and `new.target` with `body`, but have
    // their own `return` target and `var` scope.
    let mut arrow_depth = 0u32;
    let mut preorder = body.statements().syntax().preorder();
    while let Some(event) = preorder.next() {
        let node = match event {
            WalkEvent::Enter(node) => node,
            WalkEvent::Leave(node) => {
                if JsArrowFunctionExpression::can_cast(node.kind()) {
                    arrow_depth -= 1;
                }
                continue;
            }
        };

        if JsArrowFunctionExpression::can_cast(node.kind()) {
            arrow_depth += 1;
            if !has_own_this {
                preorder.skip_subtree();
            }
            continue;
        }
        if FUNCTION_SCOPES.matches(node.kind())
            && !node
                .parent()
                .is_some_and(|parent| JsArrowFunctionExpression::can_cast(parent.kind()))
        {
            preorder.skip_subtree();
            continue;
        }

        let needs_function_scope = match node.kind() {
            JsSyntaxKind::JS_THIS_EXPRESSION | JsSyntaxKind::JS_NEW_TARGET_EXPRESSION => {
                has_own_this
            }
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                has_own_this && JsReferenceIdentifier::unwrap_cast(node).has_name("arguments")
            }
            // e.g. `arguments = []`
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                has_own_this
                    && JsIdentifierAssignment::unwrap_cast(node)
                        .name_token()
                        .is_ok_and(|name| name.text_trimmed() == "arguments")
            }
            // e.g. `<this.Component />`
            JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                has_own_this
                    && JsxReferenceIdentifier::unwrap_cast(node)
                        .value_token()
                        .is_ok_and(|name| name.text_trimmed() == "this")
            }
            // The code that `eval` runs can use `this` and `arguments`, even when the call is in
            // a nested arrow function.
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                is_direct_eval_call(&JsCallExpression::unwrap_cast(node))
            }
            _ if arrow_depth > 0 => false,
            JsSyntaxKind::JS_RETURN_STATEMENT => true,
            JsSyntaxKind::JS_VARIABLE_DECLARATION | JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                AnyJsVariableDeclaration::unwrap_cast(node).is_var()
            }
            // In a script, a function declared in a block statement is also visible outside of
            // the block.
            JsSyntaxKind::JS_FUNCTION_DECLARATION => !is_module,
            JsSyntaxKind::JS_LABELED_STATEMENT => {
                reuses_enclosing_label(statement, &JsLabeledStatement::unwrap_cast(node))
            }
            _ => false,
        };
        if needs_function_scope {
            return true;
        }
    }

    false
}

/// Returns whether `call` is a call such as `eval(code)`, which runs `code` as if it were written
/// in place of the call.
///
/// The code can then declare variables in the surrounding function, or read its `this` and
/// `arguments`.
fn is_direct_eval_call(call: &JsCallExpression) -> bool {
    !call.is_optional()
        && call
            .callee()
            .ok()
            .and_then(|callee| callee.inner_expression())
            .and_then(|callee| callee.as_js_reference_identifier())
            .is_some_and(|callee| callee.has_name("eval"))
}

/// Returns whether `label` has the same name as a label around `statement`.
///
/// Code can't declare a label inside a statement that already has that label, unless a function
/// separates the two.
fn reuses_enclosing_label(statement: &JsExpressionStatement, label: &JsLabeledStatement) -> bool {
    let Ok(label) = label.label_token() else {
        return false;
    };
    statement
        .syntax()
        .ancestors()
        .skip(1)
        .take_while(|ancestor| !JsFunctionBody::can_cast(ancestor.kind()))
        .filter_map(JsLabeledStatement::cast)
        .any(|enclosing| {
            enclosing
                .label_token()
                .is_ok_and(|enclosing| enclosing.text_trimmed() == label.text_trimmed())
        })
}

/// Returns whether `statement` has a comment that is neither inside `body`, nor before or after
/// the whole statement.
fn has_comments_outside_body(statement: &JsExpressionStatement, body: &JsFunctionBody) -> bool {
    let statement_range = statement.syntax().text_trimmed_range();
    let body_range = body.syntax().text_trimmed_range();
    statement
        .syntax()
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
        })
        .any(|piece| {
            piece.is_comments()
                && statement_range.contains_range(piece.text_range())
                && !body_range.contains_range(piece.text_range())
        })
}
