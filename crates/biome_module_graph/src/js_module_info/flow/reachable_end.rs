//! Shows from syntax alone that a path reaches the end of a function's body,
//! so that return type inference can skip building the function's graph.
//!
//! The walk follows the control flow that the graph builder gives each
//! statement and looks for one path from the start of the body to its end.
//! The graph ends a path at `return`, `throw`, `break`, and `continue`, in a
//! branch that a condition written as the literal `true` or `false` never
//! takes, at a statement call that may never return, and on the path that
//! matches no case of a `switch` statement that may be exhaustive. The walk
//! crosses none of these, and a statement that it doesn't model counts as
//! never completing. So a path that the walk finds is a path of the graph,
//! and a function without one is left to the graph.

use super::builder::{MAX_DEPTH, never_has_outcome};
use super::guards::GuardFunctions;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsStatement, AnyJsTryStatement, JsDoWhileStatement, JsForStatement,
    JsFunctionBody, JsIfStatement, JsStatementList, JsSwitchStatement, JsWhileStatement,
};
use biome_rowan::{AstNode, AstNodeList};

/// Bounds the statements and operands that one walk visits.
const MAX_END_STEPS: usize = 4096;

/// Returns whether the syntax of `body`, the body of a function, shows a
/// path from its start to its end in the function's graph. `functions`
/// decides which statement calls may never return, as it does for the graph
/// builder.
///
/// Returns false when no such path exists, and also when the walk can't
/// show one: the path crosses a statement that the walk doesn't model, or
/// the walk exceeds its work limit.
pub(super) fn reaches_end(
    body: &JsFunctionBody,
    model: &SemanticModel,
    functions: &mut GuardFunctions,
) -> bool {
    let mut walk = EndWalk {
        model,
        functions,
        remaining: MAX_END_STEPS,
    };
    walk.statements(body.statements()).completes
}

/// The paths through a statement that the walk has shown, from a start of
/// the statement that a path reaches.
#[derive(Clone, Copy)]
struct Paths {
    /// A path reaches the end of the statement.
    completes: bool,
    /// A path reaches a `break` without a label that targets the innermost
    /// loop or `switch` statement around the statement.
    breaks: bool,
}

impl Paths {
    const NONE: Self = Self {
        completes: false,
        breaks: false,
    };

    const COMPLETES: Self = Self {
        completes: true,
        breaks: false,
    };

    /// Returns the paths of a statement that runs `self` or `other`.
    fn or(self, other: Self) -> Self {
        Self {
            completes: self.completes || other.completes,
            breaks: self.breaks || other.breaks,
        }
    }
}

struct EndWalk<'a, 'b> {
    model: &'a SemanticModel,
    functions: &'a mut GuardFunctions<'b>,
    remaining: usize,
}

impl EndWalk<'_, '_> {
    /// Spends one step of the work limit, or returns false when none is left.
    fn visit(&mut self) -> bool {
        match self.remaining.checked_sub(1) {
            Some(remaining) => {
                self.remaining = remaining;
                true
            }
            None => false,
        }
    }

    /// Returns the paths through `statements`, which run in order.
    fn statements(&mut self, statements: JsStatementList) -> Paths {
        let mut breaks = false;
        for statement in statements.iter() {
            let paths = self.statement(statement);
            breaks |= paths.breaks;
            if !paths.completes {
                return Paths {
                    completes: false,
                    breaks,
                };
            }
        }
        Paths {
            completes: true,
            breaks,
        }
    }

    #[expect(
        clippy::wildcard_enum_match_arm,
        reason = "Every other statement counts as never completing."
    )]
    fn statement(&mut self, statement: AnyJsStatement) -> Paths {
        if !self.visit() {
            return Paths::NONE;
        }
        match statement {
            AnyJsStatement::JsBlockStatement(block) => self.statements(block.statements()),
            AnyJsStatement::JsExpressionStatement(statement) => match statement.expression() {
                Ok(expression) if !self.may_end_path(&expression, 1) => Paths::COMPLETES,
                Ok(_) | Err(_) => Paths::NONE,
            },
            // Declarations run no code that can end a path. The graph treats
            // a class or an enum as an opaque region, which completes.
            AnyJsStatement::JsVariableStatement(_)
            | AnyJsStatement::JsEmptyStatement(_)
            | AnyJsStatement::JsDebuggerStatement(_)
            | AnyJsStatement::JsFunctionDeclaration(_)
            | AnyJsStatement::JsClassDeclaration(_)
            | AnyJsStatement::TsEnumDeclaration(_)
            | AnyJsStatement::TsDeclareFunctionDeclaration(_)
            | AnyJsStatement::TsTypeAliasDeclaration(_)
            | AnyJsStatement::TsInterfaceDeclaration(_)
            | AnyJsStatement::TsDeclareStatement(_) => Paths::COMPLETES,
            AnyJsStatement::JsIfStatement(statement) => self.if_statement(&statement),
            AnyJsStatement::JsWhileStatement(statement) => self.while_statement(&statement),
            AnyJsStatement::JsDoWhileStatement(statement) => self.do_while_statement(&statement),
            AnyJsStatement::JsForStatement(statement) => self.for_statement(&statement),
            // The head of the loop is also its exit.
            AnyJsStatement::JsForInStatement(_) | AnyJsStatement::JsForOfStatement(_) => {
                Paths::COMPLETES
            }
            AnyJsStatement::JsSwitchStatement(statement) => self.switch_statement(&statement),
            AnyJsStatement::JsTryStatement(_) | AnyJsStatement::JsTryFinallyStatement(_) => {
                match AnyJsTryStatement::cast_ref(statement.syntax()) {
                    Some(statement) => self.try_statement(&statement),
                    None => Paths::NONE,
                }
            }
            AnyJsStatement::JsBreakStatement(statement) => Paths {
                completes: false,
                breaks: statement.label().is_none(),
            },
            _ => Paths::NONE,
        }
    }

    /// Returns whether a statement call in `expression`, the expression of
    /// an expression statement, may never return. The statement calls are
    /// those that the graph builder adds: the expression itself when it is a
    /// call, and each operand of the comma operator that is a call, through
    /// any parentheses around the comma expression.
    ///
    /// A call may never return when it calls a local function that never
    /// returns, or when the graph keeps it as a call point that inference
    /// decides. Missing syntax, a nesting deeper than the graph allows, and
    /// the work limit count as a call that may never return.
    fn may_end_path(&mut self, expression: &AnyJsExpression, depth: usize) -> bool {
        if depth > MAX_DEPTH || !self.visit() {
            return true;
        }
        match expression {
            AnyJsExpression::JsSequenceExpression(sequence) => {
                match (sequence.left(), sequence.right()) {
                    (Ok(left), Ok(right)) => {
                        self.may_end_path(&left, depth + 1) || self.may_end_path(&right, depth + 1)
                    }
                    _ => true,
                }
            }
            AnyJsExpression::JsParenthesizedExpression(parenthesized)
                if matches!(
                    parenthesized
                        .expression()
                        .map(AnyJsExpression::omit_parentheses),
                    Ok(AnyJsExpression::JsSequenceExpression(_))
                ) =>
            {
                parenthesized
                    .expression()
                    .is_ok_and(|inner| self.may_end_path(&inner, depth + 1))
            }
            AnyJsExpression::JsCallExpression(call) => {
                self.functions.local_call_ends(call, self.model)
                    || self.functions.keeps_statement_call(call, self.model)
            }
            _ => false,
        }
    }

    fn if_statement(&mut self, statement: &JsIfStatement) -> Paths {
        let Ok(test) = statement.test() else {
            return Paths::NONE;
        };
        let mut paths = Paths::NONE;
        if !never_has_outcome(&test, true)
            && let Ok(consequent) = statement.consequent()
        {
            paths = self.statement(consequent);
        }
        if !never_has_outcome(&test, false) {
            paths = paths.or(
                match statement.else_clause().map(|clause| clause.alternate()) {
                    None => Paths::COMPLETES,
                    Some(Ok(alternate)) => self.statement(alternate),
                    Some(Err(_)) => Paths::NONE,
                },
            );
        }
        paths
    }

    /// Returns the paths through a loop. The loop completes when its test
    /// can be falsy, or when a path through its body reaches a `break` that
    /// targets it.
    fn while_statement(&mut self, statement: &JsWhileStatement) -> Paths {
        let Ok(test) = statement.test() else {
            return Paths::NONE;
        };
        let completes = !never_has_outcome(&test, false)
            || (!never_has_outcome(&test, true) && self.breaks_out_of_body(statement.body().ok()));
        Paths {
            completes,
            breaks: false,
        }
    }

    /// Returns the paths through a `do...while` loop. Its test runs after a
    /// path through its body completes.
    fn do_while_statement(&mut self, statement: &JsDoWhileStatement) -> Paths {
        let body = statement
            .body()
            .map_or(Paths::NONE, |body| self.statement(body));
        let exits = body.completes
            && statement
                .test()
                .is_ok_and(|test| !never_has_outcome(&test, false));
        Paths {
            completes: exits || body.breaks,
            breaks: false,
        }
    }

    /// Returns the paths through a `for` loop. Without a test, only a
    /// `break` leaves it.
    fn for_statement(&mut self, statement: &JsForStatement) -> Paths {
        let (exits, enters) = match statement.test() {
            Some(test) => (
                !never_has_outcome(&test, false),
                !never_has_outcome(&test, true),
            ),
            None => (false, true),
        };
        let completes = exits || (enters && self.breaks_out_of_body(statement.body().ok()));
        Paths {
            completes,
            breaks: false,
        }
    }

    /// Returns whether a path through `body`, the body of a loop, reaches a
    /// `break` that targets the loop.
    fn breaks_out_of_body(&mut self, body: Option<AnyJsStatement>) -> bool {
        body.is_some_and(|body| self.statement(body).breaks)
    }

    /// Returns the paths through a `switch` statement, ignoring the path
    /// that matches no case. The statements of each clause start where one
    /// of its cases matches, so a path reaches each of them. The statement
    /// completes when a path reaches the end of the last clause, or a
    /// `break` that targets the statement.
    fn switch_statement(&mut self, statement: &JsSwitchStatement) -> Paths {
        let mut breaks = false;
        let mut last = Paths::NONE;
        for clause in statement.cases().iter() {
            last = self.statements(clause.consequent());
            breaks |= last.breaks;
        }
        Paths {
            completes: last.completes || breaks,
            breaks: false,
        }
    }

    /// Returns the paths through a `try` statement. An exception can leave
    /// the `try` block at its start, so a path reaches the `catch` clause
    /// and the `finally` block. A `break` leaves the statement without
    /// running the `finally` block.
    fn try_statement(&mut self, statement: &AnyJsTryStatement) -> Paths {
        let block = statement
            .body()
            .map_or(Paths::NONE, |block| self.statements(block.statements()));
        let handler = statement
            .catch_clause()
            .and_then(|clause| clause.body().ok())
            .map_or(Paths::NONE, |block| self.statements(block.statements()));
        let paths = block.or(handler);
        let Some(finally) = statement.finally_clause() else {
            return paths;
        };
        let finally = finally
            .body()
            .map_or(Paths::NONE, |block| self.statements(block.statements()));
        Paths {
            completes: paths.completes && finally.completes,
            breaks: paths.breaks || finally.breaks,
        }
    }
}
