use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsIfStatement, JsLogicalOperator};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_duplicate_else_if::NoDuplicateElseIfOptions;
use rustc_hash::{FxHashMap, FxHasher};
use std::hash::{Hash, Hasher};

use crate::utils::{is_node_equal, node_hash};

declare_lint_rule! {
    /// Disallow duplicate conditions in if-else-if chains
    ///
    /// if-else-if chains are commonly used when there is a need to execute only one branch
    /// (or at most one branch) out of several possible branches, based on certain conditions.
    ///
    /// Two identical test conditions in the same chain are almost always a mistake in the code.
    /// Unless there are side effects in the expressions,
    /// a duplicate will evaluate to the same true or false value as the identical expression earlier in the chain,
    /// meaning that its branch can never execute.
    ///
    /// Please note that this rule does not compare conditions from the chain with conditions inside statements
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (a) {
    ///     foo();
    /// } else if (b) {
    ///     bar();
    /// } else if (b) {
    ///     baz();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (a) {
    ///     foo();
    /// } else if (b) {
    ///     bar();
    /// } else if (c) {
    ///     baz();
    /// }
    /// ```
    ///
    pub NoDuplicateElseIf {
        version: "1.6.2",
        name: "noDuplicateElseIf",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Eslint("no-dupe-else-if").same()],
    }
}

impl Rule for NoDuplicateElseIf {
    type Query = Ast<JsIfStatement>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateElseIfOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // The earlier conditions of an `if` are those of the `if` statements reached by
        // repeatedly taking the grandparent, as for `else if`. Each tree of such statements
        // is analysed once, from the `if` statement that has no such ancestor.
        if node
            .syntax()
            .grand_parent()
            .is_some_and(|grand_parent| JsIfStatement::can_cast(grand_parent.kind()))
        {
            return Box::default();
        }

        if grand_children(node).next().is_none() {
            return Box::default();
        }

        let mut signals = Vec::new();
        let mut earlier_conditions = EarlierConditions::default();
        let mut stack = vec![Visit::Enter(node.clone())];
        while let Some(visit) = stack.pop() {
            let if_stmt = match visit {
                Visit::Enter(if_stmt) => if_stmt,
                Visit::Leave(len) => {
                    earlier_conditions.truncate(len);
                    continue;
                }
            };
            stack.push(Visit::Leave(earlier_conditions.len()));
            let first_grand_child = stack.len();
            stack.extend(grand_children(&if_stmt).map(Visit::Enter));
            stack[first_grand_child..].reverse();
            let has_grand_children = stack.len() > first_grand_child;
            if !has_grand_children && earlier_conditions.is_empty() {
                continue;
            }
            let Ok(test) = if_stmt.test() else {
                continue;
            };
            let mut conjunctions = split_into_conjunctions(test.clone());
            if earlier_conditions.covers(&test, &mut conjunctions) {
                signals.push(test.range());
            }
            if has_grand_children {
                for conjunction in conjunctions {
                    earlier_conditions.insert(conjunction);
                }
            }
        }

        signals.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, text_range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                text_range,
                markup! {
                    "This condition is a duplicate of, or is covered by, previous conditions in the if-else-if chain."
                },
            )
            .note(markup! {
                "An earlier branch already handles every case this condition matches, so this branch can never execute."
            })
            .note(markup! {
                "Remove this branch, or change its condition to the one you meant."
            }),
        )
    }
}

/// Returns the `if` statements whose grandparent is `node`.
fn grand_children(node: &JsIfStatement) -> impl Iterator<Item = JsIfStatement> {
    node.syntax()
        .children()
        .flat_map(|child| child.children())
        .filter_map(JsIfStatement::cast)
}

enum Visit {
    Enter(JsIfStatement),
    /// Restores the earlier conditions to their first `n` conjunctions.
    Leave(usize),
}

/// The `&&` operands of one `||` operand of a condition.
struct Conjunction {
    operands: Vec<AnyJsExpression>,
    /// [condition_hash] of each operand, empty until [Self::compute_hashes] is called.
    hashes: Vec<Option<u64>>,
}

impl Conjunction {
    fn new(operand: AnyJsExpression) -> Self {
        Self {
            operands: split_by_logical_operator_wrapper(JsLogicalOperator::LogicalAnd, operand),
            hashes: Vec::new(),
        }
    }

    fn compute_hashes(&mut self) {
        if self.hashes.is_empty() {
            self.hashes = self.operands.iter().map(condition_hash).collect();
        }
    }
}

/// Splits `condition` into its `||` operands.
fn split_into_conjunctions(condition: AnyJsExpression) -> Vec<Conjunction> {
    split_by_logical_operator_wrapper(JsLogicalOperator::LogicalOr, condition)
        .into_iter()
        .map(Conjunction::new)
        .collect()
}

/// Up to this many earlier conjunctions, a lookup scans all of them. Above it, they are
/// indexed by hash, which costs more for the short chains that are the common case.
const INDEX_THRESHOLD: usize = 16;

/// The conjunctions of the conditions of the enclosing `if` statements, as a stack.
#[derive(Default)]
struct EarlierConditions {
    conjunctions: Vec<Conjunction>,
    /// Present exactly when there are more than [INDEX_THRESHOLD] conjunctions.
    index: Option<ConjunctionIndex>,
}

#[derive(Default)]
struct ConjunctionIndex {
    /// The hash under which each conjunction of [EarlierConditions::conjunctions] is indexed,
    /// if it is.
    keys: Vec<Option<u64>>,
    /// Indices into [EarlierConditions::conjunctions], by the [condition_hash] of one of
    /// their operands.
    by_hash: FxHashMap<u64, Vec<usize>>,
}

impl ConjunctionIndex {
    fn push(&mut self, position: usize, hashes: &[Option<u64>]) {
        // A conjunction with an operand that is never `equal` to anything is never a subset,
        // so it isn't indexed. Otherwise, a conjunction can only be a subset of a conjunction
        // that contains its key operand, so any operand works as the key. Picking the least
        // used one keeps the lookups short for chains that repeat an operand, such as
        // `a && b === 1`, `a && b === 2`.
        let key = hashes
            .iter()
            .copied()
            .collect::<Option<Vec<_>>>()
            .and_then(|hashes| {
                hashes
                    .into_iter()
                    .min_by_key(|hash| self.by_hash.get(hash).map_or(0, Vec::len))
            });
        if let Some(key) = key {
            self.by_hash.entry(key).or_default().push(position);
        }
        self.keys.push(key);
    }

    fn pop(&mut self) {
        if let Some(Some(key)) = self.keys.pop()
            && let Some(positions) = self.by_hash.get_mut(&key)
        {
            positions.pop();
        }
    }
}

impl EarlierConditions {
    fn len(&self) -> usize {
        self.conjunctions.len()
    }

    fn is_empty(&self) -> bool {
        self.conjunctions.is_empty()
    }

    fn insert(&mut self, conjunction: Conjunction) {
        self.conjunctions.push(conjunction);
        if let Some(index) = &mut self.index {
            let position = self.conjunctions.len() - 1;
            let conjunction = &mut self.conjunctions[position];
            conjunction.compute_hashes();
            index.push(position, &conjunction.hashes);
        } else if self.conjunctions.len() > INDEX_THRESHOLD {
            let mut index = ConjunctionIndex::default();
            for (position, conjunction) in self.conjunctions.iter_mut().enumerate() {
                conjunction.compute_hashes();
                index.push(position, &conjunction.hashes);
            }
            self.index = Some(index);
        }
    }

    fn truncate(&mut self, len: usize) {
        while self.conjunctions.len() > len {
            self.conjunctions.pop();
            if let Some(index) = &mut self.index {
                index.pop();
            }
        }
        if self.conjunctions.len() <= INDEX_THRESHOLD {
            self.index = None;
        }
    }

    /// Returns whether `condition`, or one of its `&&` operands, is true only when an
    /// earlier condition is true: each of its `||` operands contains all the `&&`
    /// operands of a `||` operand of an earlier condition.
    ///
    /// `conjunctions` are the `||` operands of `condition`.
    fn covers(&self, condition: &AnyJsExpression, conjunctions: &mut [Conjunction]) -> bool {
        if self.conjunctions.is_empty() {
            return false;
        }
        if conjunctions
            .iter_mut()
            .all(|conjunction| self.covers_conjunction(conjunction))
        {
            return true;
        }
        if let Some(logical_expression) = condition.as_js_logical_expression()
            && let Ok(operator_token) = logical_expression.operator()
            && operator_token == JsLogicalOperator::LogicalAnd
        {
            return split_by_logical_operator_wrapper(operator_token, condition.clone())
                .into_iter()
                .any(|operand| {
                    split_into_conjunctions(operand)
                        .iter_mut()
                        .all(|conjunction| self.covers_conjunction(conjunction))
                });
        }
        false
    }

    /// Returns whether an earlier conjunction is a subset of `conjunction`.
    fn covers_conjunction(&self, conjunction: &mut Conjunction) -> bool {
        let Some(index) = &self.index else {
            return self
                .conjunctions
                .iter()
                .any(|earlier| is_subset(&earlier.operands, &conjunction.operands));
        };
        conjunction.compute_hashes();
        conjunction.hashes.iter().flatten().any(|hash| {
            index.by_hash.get(hash).is_some_and(|positions| {
                positions.iter().any(|&position| {
                    let earlier = &self.conjunctions[position];
                    earlier
                        .hashes
                        .iter()
                        .all(|hash| conjunction.hashes.contains(hash))
                        && is_subset(&earlier.operands, &conjunction.operands)
                })
            })
        })
    }
}

fn split_by_logical_operator_wrapper(
    operator: JsLogicalOperator,
    node: AnyJsExpression,
) -> Vec<AnyJsExpression> {
    let mut result: Vec<AnyJsExpression> = vec![];
    split_by_logical_operator(operator, node, &mut result);
    result
}

fn split_by_logical_operator(
    operator: JsLogicalOperator,
    node: AnyJsExpression,
    result: &mut Vec<AnyJsExpression>,
) {
    let node = node.omit_parentheses();
    match &node {
        AnyJsExpression::JsLogicalExpression(logic_expression) => {
            if let Ok(operator_token) = logic_expression.operator()
                && operator_token != operator
            {
                result.push(node);
                return;
            }
            if let (Ok(left_node), Ok(right_node)) =
                (logic_expression.left(), logic_expression.right())
            {
                split_by_logical_operator(operator, left_node, result);
                split_by_logical_operator(operator, right_node, result);
            } else {
                result.push(node);
            }
        }
        _ => {
            result.push(node);
        }
    }
}

fn equal(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    if a.syntax().kind() != b.syntax().kind() {
        return false;
    }
    if let (Some(a_exp), Some(b_exp)) = (a.as_js_logical_expression(), b.as_js_logical_expression())
    {
        if a_exp.operator() == b_exp.operator()
            && matches!(
                a_exp.operator(),
                Ok(JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr)
            )
        {
            match (a_exp.left(), a_exp.right(), b_exp.left(), b_exp.right()) {
                (Ok(left_a), Ok(right_a), Ok(left_b), Ok(right_b)) => {
                    return (equal(&left_a, &left_b) && equal(&right_a, &right_b))
                        || (equal(&left_a, &right_b) && equal(&right_a, &left_b));
                }
                _ => return false,
            }
        }
        return false;
    }
    is_node_equal(a.syntax(), b.syntax())
}

fn is_subset(arr_a: &[AnyJsExpression], arr_b: &[AnyJsExpression]) -> bool {
    arr_a.iter().all(|a| arr_b.iter().any(|b| equal(a, b)))
}

/// Hashes `expr` so that expressions that are [equal] have the same hash, or returns `None`
/// when `expr` isn't [equal] to any expression.
fn condition_hash(expr: &AnyJsExpression) -> Option<u64> {
    let Some(logical_expression) = expr.as_js_logical_expression() else {
        return Some(node_hash(expr.syntax()));
    };
    let operator @ (JsLogicalOperator::LogicalAnd | JsLogicalOperator::LogicalOr) =
        logical_expression.operator().ok()?
    else {
        return None;
    };
    let left = condition_hash(&logical_expression.left().ok()?)?;
    let right = condition_hash(&logical_expression.right().ok()?)?;
    // `equal` accepts the operands in either order.
    let mut hasher = FxHasher::default();
    operator.hash(&mut hasher);
    left.min(right).hash(&mut hasher);
    left.max(right).hash(&mut hasher);
    Some(hasher.finish())
}
