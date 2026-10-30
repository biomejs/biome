use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsForInOrOfInitializer, AnyJsForInitializer, AnyJsFunction, AnyJsStatement,
    JsReferenceIdentifier, JsReturnStatement, JsStatementList, JsVariableKind, JsVariableStatement,
};
use biome_rowan::{AstNode, Direction};
use biome_rule_options::no_useless_recursion::NoUselessRecursionOptions;

use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow functions that return a call to themselves when a loop can do the same work.
    ///
    /// A recursive function is a function that calls itself.
    /// Each of these calls uses a bit more memory until it finishes,
    /// so a function that calls itself too many times in a row crashes the program with a "Maximum call stack size exceeded" error.
    /// When the function only returns the result of calling itself again,
    /// a loop such as `while` can do the same work without that limit, and is often easier to follow.
    ///
    /// This rule only reports `return name(...)` inside a function declared with the `function` keyword
    /// and a name, where `name` is that function's own name.
    /// It doesn't report other kinds of recursion, such as calls whose result is used in a larger expression,
    /// calls inside methods or arrow functions, or calls inside `function*` functions.
    ///
    /// It also doesn't report calls in these places, because code there runs after each call finishes,
    /// so a loop can't replace the call directly:
    ///
    /// - inside a `try` statement;
    /// - after a `using` or `await using` declaration in the same block, or in a block that contains the call;
    /// - inside a `for` or `for...of` loop that declares its variable with `using` or `await using`, such as `for (using file of files)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function findRoot(node) {
    ///     if (node.parent) {
    ///         return findRoot(node.parent);
    ///     }
    ///
    ///     return node;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// function findRoot(node) {
    ///     while (node.parent) {
    ///         node = node.parent;
    ///     }
    ///
    ///     return node;
    /// }
    ///
    /// function flatten(value) {
    ///     if (Array.isArray(value)) {
    ///         return value.flatMap((item) => flatten(item));
    ///     }
    ///
    ///     return [value];
    /// }
    ///
    /// function sum(list) {
    ///     return list.length === 0 ? 0 : list[0] + sum(list.slice(1));
    /// }
    /// ```
    ///
    pub NoUselessRecursion {
        version: "2.6.4",
        name: "noUselessRecursion",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-recursion").same()],
        recommended: false,
        severity: Severity::Information,
    }
}

impl Rule for NoUselessRecursion {
    type Query = Semantic<JsReturnStatement>;
    /// The name of the called function in the returned call.
    type State = JsReferenceIdentifier;
    type Signals = Option<Self::State>;
    type Options = NoUselessRecursionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let return_statement = ctx.query();
        let call = return_statement
            .argument()?
            .omit_parentheses()
            .as_js_call_expression()?
            .clone();
        if call.is_optional() {
            return None;
        }
        let callee = call
            .callee()
            .ok()?
            .omit_parentheses()
            .as_js_identifier_expression()?
            .name()
            .ok()?;

        let function = enclosing_function(return_statement)?;
        if function.is_generator() {
            return None;
        }
        let function_id = function.id().ok()??;
        let function_id = function_id.as_js_identifier_binding()?;

        let binding = ctx.model().binding(&callee)?;
        (binding.range() == function_id.syntax().text_trimmed_range()).then_some(callee)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, callee: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                callee.range(),
                markup! {
                    "This function returns a call to itself, which can be replaced with a loop."
                },
            )
            .note(markup! {
                "Each nested call uses more memory, and too many nested calls crash the program with a stack overflow error."
            })
            .note(markup! {
                "Use a loop, such as "<Emphasis>"while"</Emphasis>", that updates the values on each step instead of calling the function again."
            }),
        )
    }
}

/// Returns the function that directly contains `return_statement`, or `None`
/// if it is a method or similar, or if cleanup code (`try`, `using`) runs
/// after the return.
fn enclosing_function(return_statement: &JsReturnStatement) -> Option<AnyJsFunction> {
    for node in return_statement.syntax().ancestors() {
        if AnyJsControlFlowRoot::can_cast(node.kind()) {
            return AnyJsFunction::cast(node);
        }
        let Some(statement) = AnyJsStatement::cast(node) else {
            continue;
        };
        if is_after_using_declaration(&statement) {
            return None;
        }
        match statement {
            AnyJsStatement::JsTryStatement(_) | AnyJsStatement::JsTryFinallyStatement(_) => {
                return None;
            }
            AnyJsStatement::JsForStatement(for_statement) => {
                if let Some(AnyJsForInitializer::JsVariableDeclaration(declaration)) =
                    for_statement.initializer()
                    && declaration.variable_kind() == Ok(JsVariableKind::Using)
                {
                    return None;
                }
            }
            AnyJsStatement::JsForOfStatement(for_of) => {
                if let Ok(AnyJsForInOrOfInitializer::JsForVariableDeclaration(declaration)) =
                    for_of.initializer()
                    && declaration.variable_kind() == Ok(JsVariableKind::Using)
                {
                    return None;
                }
            }
            _ => {}
        }
    }
    None
}

/// Returns `true` when a `using` or `await using` declaration appears before
/// `statement` in the same list of statements.
fn is_after_using_declaration(statement: &AnyJsStatement) -> bool {
    statement.parent::<JsStatementList>().is_some()
        && statement
            .syntax()
            .siblings(Direction::Prev)
            .skip(1)
            .filter_map(JsVariableStatement::cast)
            .any(|statement| {
                statement.declaration().is_ok_and(|declaration| {
                    declaration.variable_kind() == Ok(JsVariableKind::Using)
                })
            })
}
