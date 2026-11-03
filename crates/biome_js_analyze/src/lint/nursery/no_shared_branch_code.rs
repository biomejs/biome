use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsStatement, JsBlockStatement, JsElseClause,
    JsIdentifierAssignment, JsIfStatement, JsLanguage, JsReferenceIdentifier, JsSyntaxToken,
    assign_ext::AnyJsMemberAssignment,
};
use biome_rowan::{AstNode, AstNodeList, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::no_shared_branch_code::NoSharedBranchCodeOptions;
use std::iter;

use crate::services::semantic::Semantic;
use crate::utils::is_node_equal;

declare_lint_rule! {
    /// Disallow the same code at the start or end of every branch of an `if` statement.
    ///
    /// When every branch of an `if` statement starts or ends with the same code,
    /// that code runs no matter which branch is taken.
    /// Repeating it in each branch makes the code longer, and a later change to it has to be made in every copy.
    /// Move the shared code before or after the `if` statement instead.
    ///
    /// The rule only checks `if` statements that end with an `else` branch,
    /// including chains of `else if` branches, where every branch is a block wrapped in curly braces.
    /// The rule doesn't report shared code that can't be moved out of the branches without changing what it does:
    ///
    /// - shared code that uses a variable declared elsewhere in the same branch;
    /// - shared code at the start of the branches that assigns a variable, or a property of a variable, used in a condition of the `if` statement,
    ///   because moving it before the `if` statement changes the result of the condition.
    ///
    /// Moving shared code before the `if` statement makes it run before the conditions are checked, instead of after.
    /// When a condition calls a function, check that the function doesn't depend on the shared code before moving it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     console.log("Hello");
    ///     doSomething();
    /// } else {
    ///     console.log("Hello");
    ///     doSomethingElse();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (condition) {
    ///     doSomething();
    ///     cleanup();
    /// } else {
    ///     doSomethingElse();
    ///     cleanup();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// console.log("Hello");
    /// if (condition) {
    ///     doSomething();
    /// } else {
    ///     doSomethingElse();
    /// }
    ///
    /// if (condition) {
    ///     doSomething();
    /// } else {
    ///     doSomethingElse();
    /// }
    /// cleanup();
    ///
    /// if (condition) {
    ///     const message = "yes";
    ///     console.log(message);
    /// } else {
    ///     const message = "no";
    ///     console.log(message);
    /// }
    /// ```
    ///
    pub NoSharedBranchCode {
        version: "next",
        name: "noSharedBranchCode",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Oxc("branches-sharing-code").inspired()],
    }
}

impl Rule for NoSharedBranchCode {
    type Query = Semantic<JsIfStatement>;
    type State = SharedCode;
    type Signals = Box<[Self::State]>;
    type Options = NoSharedBranchCodeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_statement = ctx.query();
        // Each chain of `else if` branches is checked once, from its first `if` statement.
        if if_statement.parent::<JsElseClause>().is_some() {
            return Box::default();
        }
        let Some(branches) = collect_branches(if_statement) else {
            return Box::default();
        };
        let model = ctx.model();
        let mut signals = Vec::new();

        let shortest_len = branches
            .iter()
            .map(|branch| branch.statements().len())
            .min()
            .unwrap_or_default();
        let Some((first, others)) = branches.split_first() else {
            return Box::default();
        };

        let mut other_statements: Vec<_> = others
            .iter()
            .map(|branch| branch.statements().iter())
            .collect();
        let start_count = first
            .statements()
            .iter()
            .take(shortest_len)
            .take_while(|statement| all_next_equal(statement, &mut other_statements))
            .count();
        // When a branch only contains shared code, nothing remains in it after moving the
        // code out, so the branches are identical rather than sharing code.
        if start_count == shortest_len {
            return Box::default();
        }

        let mut other_statements: Vec<_> = others
            .iter()
            .map(|branch| branch.statements().iter().rev())
            .collect();
        let end_count = first
            .statements()
            .iter()
            .rev()
            .take(shortest_len - start_count)
            .take_while(|statement| all_next_equal(statement, &mut other_statements))
            .count();

        for (position, count) in [
            (SharedCodePosition::Start, start_count),
            (SharedCodePosition::End, end_count),
        ] {
            if count == 0 {
                continue;
            }
            let only_empty_statements = shared_statements(first, position, count)
                .all(|statement| matches!(statement, AnyJsStatement::JsEmptyStatement(_)));
            if only_empty_statements {
                continue;
            }
            if branches
                .iter()
                .any(|branch| uses_branch_variable(model, branch, position, count))
            {
                continue;
            }
            if matches!(position, SharedCodePosition::Start)
                && assigns_condition_variable(if_statement, first, count)
            {
                continue;
            }
            let Some(range) = shared_statements_range(first, position, count) else {
                continue;
            };
            signals.push(SharedCode { position, range });
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                match state.position {
                    SharedCodePosition::Start => markup! {
                        "Every branch of this "<Emphasis>"if"</Emphasis>" statement starts with the same code."
                    },
                    SharedCodePosition::End => markup! {
                        "Every branch of this "<Emphasis>"if"</Emphasis>" statement ends with the same code."
                    },
                },
            )
            .note(markup! {
                "This code runs no matter which branch is taken. Repeating it in each branch makes every change to it happen in several places."
            })
            .note(match state.position {
                SharedCodePosition::Start => markup! {
                    "Move the shared code before the "<Emphasis>"if"</Emphasis>" statement."
                },
                SharedCodePosition::End => markup! {
                    "Move the shared code after the "<Emphasis>"if"</Emphasis>" statement."
                },
            }),
        )
    }
}

/// Shared code found in every branch of an `if` statement.
pub struct SharedCode {
    position: SharedCodePosition,
    /// The range of the shared statements in the first branch.
    range: TextRange,
}

#[derive(Clone, Copy)]
pub enum SharedCodePosition {
    Start,
    End,
}

impl SharedCodePosition {
    /// Returns the index of the first of `count` shared statements in a branch of `len` statements.
    fn skip(self, len: usize, count: usize) -> usize {
        match self {
            Self::Start => 0,
            Self::End => len - count,
        }
    }
}

/// Returns the blocks of `if_statement` and of its `else if` and `else` branches, in source order.
///
/// Returns `None` when the chain doesn't end with an `else` branch, or when a branch isn't a block.
fn collect_branches(if_statement: &JsIfStatement) -> Option<Vec<JsBlockStatement>> {
    let mut branches = Vec::new();
    let mut current = if_statement.clone();
    loop {
        let AnyJsStatement::JsBlockStatement(consequent) = current.consequent().ok()? else {
            return None;
        };
        branches.push(consequent);
        match current.else_clause()?.alternate().ok()? {
            AnyJsStatement::JsIfStatement(next) => current = next,
            AnyJsStatement::JsBlockStatement(alternate) => {
                branches.push(alternate);
                return Some(branches);
            }
            _ => return None,
        }
    }
}

/// Advances every iterator of `others` and returns whether each yields a statement equal to `statement`.
fn all_next_equal(
    statement: &AnyJsStatement,
    others: &mut [impl Iterator<Item = AnyJsStatement>],
) -> bool {
    others.iter_mut().all(|statements| {
        statements
            .next()
            .is_some_and(|other| is_node_equal(statement.syntax(), other.syntax()))
    })
}

/// Returns the `count` shared statements at `position` in `branch`.
fn shared_statements(
    branch: &JsBlockStatement,
    position: SharedCodePosition,
    count: usize,
) -> impl Iterator<Item = AnyJsStatement> {
    let statements = branch.statements();
    let skip = position.skip(statements.len(), count);
    statements.iter().skip(skip).take(count)
}

/// Returns the range covering the `count` shared statements at `position` in `branch`.
fn shared_statements_range(
    branch: &JsBlockStatement,
    position: SharedCodePosition,
    count: usize,
) -> Option<TextRange> {
    let mut shared = shared_statements(branch, position, count);
    let first = shared.next()?.range();
    Some(shared.last().map_or(first, |last| first.cover(last.range())))
}

/// Returns whether the `count` shared statements at `position` in `branch` refer to a variable
/// declared in `branch`, outside of the shared statements. Such code stops working when it's
/// moved out of the branch.
///
/// `var` declarations are ignored: they belong to the enclosing function, so the shared code
/// still finds them after it's moved.
fn uses_branch_variable(
    model: &SemanticModel,
    branch: &JsBlockStatement,
    position: SharedCodePosition,
    count: usize,
) -> bool {
    let Some(shared_range) = shared_statements_range(branch, position, count) else {
        return false;
    };
    let branch_range = branch.range();
    shared_statements(branch, position, count).any(|statement| {
        statement
            .syntax()
            .preorder_of_kinds(IDENTIFIER_REFERENCE_KINDS)
            .filter_map(|event| match event {
                WalkEvent::Enter(node) => AnyJsIdentifierReference::cast(node),
                WalkEvent::Leave(_) => None,
            })
            .filter_map(|reference| model.binding(&reference))
            .any(|binding| {
                binding.declaration_kind() != JsDeclarationKind::HoistedValue
                    && branch_range.contains_range(binding.range())
                    && !shared_range.contains_range(binding.range())
            })
    })
}

/// Returns whether the first `count` statements of `first_branch` assign a variable, or a
/// property of a variable, that a condition of `if_statement` or of its `else if` branches uses.
///
/// Variables are compared by name, so that variables declared outside of the file are covered too.
fn assigns_condition_variable(
    if_statement: &JsIfStatement,
    first_branch: &JsBlockStatement,
    count: usize,
) -> bool {
    let mut assigned_names = Vec::new();
    for statement in shared_statements(first_branch, SharedCodePosition::Start, count) {
        for event in statement.syntax().preorder_of_kinds(ASSIGNMENT_KINDS) {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let name = if let Some(assignment) = JsIdentifierAssignment::cast_ref(&node) {
                assignment.name_token().ok()
            } else {
                AnyJsMemberAssignment::cast(node)
                    .and_then(|assignment| assignment.object().ok())
                    .and_then(object_root_name)
            };
            assigned_names.extend(name.map(|token| token.token_text_trimmed()));
        }
    }
    if assigned_names.is_empty() {
        return false;
    }
    iter::successors(Some(if_statement.clone()), |current| {
        current
            .else_clause()?
            .alternate()
            .ok()?
            .as_js_if_statement()
            .cloned()
    })
    .filter_map(|current| current.test().ok())
    .any(|test| {
        test.syntax()
            .preorder_of_kinds(JsReferenceIdentifier::KIND_SET)
            .filter_map(|event| match event {
                WalkEvent::Enter(node) => JsReferenceIdentifier::cast(node),
                WalkEvent::Leave(_) => None,
            })
            .filter_map(|reference| reference.value_token().ok())
            .any(|token| assigned_names.contains(&token.token_text_trimmed()))
    })
}

/// Returns the name of the variable at the start of a chain of property accesses, such as `a`
/// in `a.b[c].d`.
fn object_root_name(mut object: AnyJsExpression) -> Option<JsSyntaxToken> {
    loop {
        object = match object.omit_parentheses() {
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                return identifier.name().ok()?.value_token().ok();
            }
            AnyJsExpression::JsStaticMemberExpression(member) => member.object().ok()?,
            AnyJsExpression::JsComputedMemberExpression(member) => member.object().ok()?,
            _ => return None,
        };
    }
}

const IDENTIFIER_REFERENCE_KINDS: SyntaxKindSet<JsLanguage> = AnyJsIdentifierReference::KIND_SET;

const ASSIGNMENT_KINDS: SyntaxKindSet<JsLanguage> =
    JsIdentifierAssignment::KIND_SET.union(AnyJsMemberAssignment::KIND_SET);
