use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsStatement, JsElseClause, JsForStatement, JsIfStatement, JsSyntaxElement, T, inner_string_text,
};
use biome_rowan::{AstNode, AstNodeList, Direction, TextRange, WalkEvent};
use biome_rule_options::no_duplicate_if_branches::NoDuplicateIfBranchesOptions;

declare_lint_rule! {
    /// Disallow two branches next to each other in an `if` statement from running the same code.
    ///
    /// When two branches that follow each other in an `if`, `else if`, `else` chain contain the same code,
    /// the condition that chooses between them makes no difference.
    /// This is often a copy-paste mistake, or a leftover from an unfinished change.
    ///
    /// Only branches next to each other are compared.
    /// Two branches with the same code that are separated by a different branch are allowed,
    /// because the branch in between can change which one runs.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (isAdmin) {
    ///     showDashboard();
    ///     loadStats();
    /// } else {
    ///     showDashboard();
    ///     loadStats();
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (value === "a") {
    ///     doThing();
    /// } else if (value === "b") {
    ///     doThing();
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (isAdmin) {
    ///     showDashboard();
    ///     loadStats();
    /// } else {
    ///     showLogin();
    /// }
    ///
    /// if (value === "a" || value === "b") {
    ///     doThing();
    /// }
    ///
    /// if (format === "json") {
    ///     parseJson();
    /// } else if (format === "yaml") {
    ///     parseYaml();
    /// } else {
    ///     parseJson();
    /// }
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow the same condition appearing twice in an `if` statement, see [`noDuplicateElseIf`](https://biomejs.dev/linter/rules/no-duplicate-else-if/).
    /// - If you want to disallow the same code at the start or end of every branch of an `if` statement, see [`noSharedBranchCode`](https://biomejs.dev/linter/rules/no-shared-branch-code/).
    ///
    pub NoDuplicateIfBranches {
        version: "next",
        name: "noDuplicateIfBranches",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::EslintUnicorn("no-duplicate-if-branches").inspired()],
    }
}

impl Rule for NoDuplicateIfBranches {
    type Query = Ast<JsIfStatement>;
    type State = DuplicateBranch;
    type Signals = Box<[Self::State]>;
    type Options = NoDuplicateIfBranchesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let if_statement = ctx.query();
        // Each chain of `else if` branches is checked once, from its first `if` statement.
        if if_statement.parent::<JsElseClause>().is_some() {
            return Box::default();
        }
        let Some(branches) = collect_branches(if_statement) else {
            return Box::default();
        };
        branches
            .windows(2)
            .filter_map(|pair| {
                let [previous, branch] = pair else {
                    return None;
                };
                has_same_code(previous, branch).then(|| DuplicateBranch {
                    range: branch.range(),
                    previous_range: previous.range(),
                })
            })
            .collect()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This branch runs the same code as the branch before it."
                },
            )
            .detail(
                state.previous_range,
                markup! { "The branch before it is here:" },
            )
            .note(markup! {
                "The condition that chooses between these branches makes no difference, which is often a copy-paste mistake."
            })
            .note(markup! {
                "Change the code of one of the branches, or merge the branches into one."
            }),
        )
    }
}

/// A branch that runs the same code as the branch before it.
pub struct DuplicateBranch {
    range: TextRange,
    previous_range: TextRange,
}

/// Returns the bodies of `if_statement` and of its `else if` and `else` branches, in source order.
///
/// Returns `None` when a body is missing, because the branches around it would otherwise be
/// compared as if they were next to each other.
fn collect_branches(if_statement: &JsIfStatement) -> Option<Vec<AnyJsStatement>> {
    let mut branches = Vec::new();
    let mut current = if_statement.clone();
    loop {
        branches.push(current.consequent().ok()?);
        let Some(else_clause) = current.else_clause() else {
            return Some(branches);
        };
        match else_clause.alternate().ok()? {
            AnyJsStatement::JsIfStatement(next) => current = next,
            alternate => {
                branches.push(alternate);
                return Some(branches);
            }
        }
    }
}

/// Returns whether the branch bodies `a` and `b` contain the same statements, ignoring empty
/// statements. Bodies without any other statement never have the same code.
fn has_same_code(a: &AnyJsStatement, b: &AnyJsStatement) -> bool {
    let mut a_statements = branch_statements(a).peekable();
    if a_statements.peek().is_none() {
        return false;
    }
    let mut b_statements = branch_statements(b);
    loop {
        match (a_statements.next(), b_statements.next()) {
            (None, None) => return true,
            (Some(a), Some(b)) if is_same_statement(&a, &b) => {}
            _ => return false,
        }
    }
}

/// Returns the statements of a branch body: the statements of a block, or the body itself
/// when it isn't a block. Empty statements are skipped because they do nothing.
fn branch_statements(body: &AnyJsStatement) -> impl Iterator<Item = AnyJsStatement> {
    let (block_statements, single_statement) = match body {
        AnyJsStatement::JsBlockStatement(block) => (Some(block.statements().iter()), None),
        statement => (None, Some(statement.clone())),
    };
    block_statements
        .into_iter()
        .flatten()
        .chain(single_statement)
        .filter(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)))
}

/// Returns whether `a` and `b` have the same syntax tree and the same tokens, ignoring comments,
/// formatting, optional semicolons, and the quotes around strings.
///
/// Optional semicolons can be ignored because the syntax trees are compared as well: when a
/// missing semicolon changes how the code is parsed, the trees are different.
fn is_same_statement(a: &AnyJsStatement, b: &AnyJsStatement) -> bool {
    let mut a_elements = elements_without_optional_semicolons(a);
    let mut b_elements = elements_without_optional_semicolons(b);
    loop {
        let (a_event, b_event) = match (a_elements.next(), b_elements.next()) {
            (None, None) => return true,
            (Some(a_event), Some(b_event)) => (a_event, b_event),
            _ => return false,
        };
        let (a_element, b_element) = match (a_event, b_event) {
            (WalkEvent::Enter(a), WalkEvent::Enter(b)) => (a, b),
            (WalkEvent::Leave(_), WalkEvent::Leave(_)) => continue,
            _ => return false,
        };
        if a_element.kind() != b_element.kind() {
            return false;
        }
        if let (Some(a_token), Some(b_token)) = (a_element.as_token(), b_element.as_token())
            && inner_string_text(a_token) != inner_string_text(b_token)
        {
            return false;
        }
    }
}

/// Returns the preorder traversal of the nodes and tokens of `statement`, without its optional
/// semicolons.
///
/// The semicolons in the header of a `for` statement are kept: they separate the initializer,
/// the condition, and the update, which can each be omitted, so `for (; a;)` and `for (a;;)`
/// only differ by their semicolons.
fn elements_without_optional_semicolons(
    statement: &AnyJsStatement,
) -> impl Iterator<Item = WalkEvent<JsSyntaxElement>> {
    statement
        .syntax()
        .preorder_with_tokens(Direction::Next)
        .filter(|event| {
            let (WalkEvent::Enter(element) | WalkEvent::Leave(element)) = event;
            element.kind() != T![;]
                || element
                    .parent()
                    .is_some_and(|parent| JsForStatement::can_cast(parent.kind()))
        })
}

