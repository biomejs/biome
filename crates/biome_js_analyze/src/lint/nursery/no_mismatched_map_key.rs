use std::iter;

use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsClass, AnyJsExpression, AnyJsFunction, AnyJsLiteralExpression,
    AnyJsMemberExpression, AnyJsStatement, JsCallExpression, JsComputedMemberAssignment,
    JsConditionalExpression, JsGetterObjectMember, JsIdentifierAssignment, JsIdentifierBinding,
    JsIfStatement, JsLanguage, JsLogicalExpression, JsLogicalOperator, JsMethodObjectMember,
    JsReferenceIdentifier, JsSetterObjectMember, JsStaticMemberAssignment, JsUnaryOperator,
    JsVariableDeclarator, JsWhileStatement,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, Direction, SyntaxKindSet, WalkEvent,
    declare_node_union,
};
use biome_rule_options::no_mismatched_map_key::NoMismatchedMapKeyOptions;

use crate::{services::semantic::Semantic, utils::same_reference::is_same_reference};

declare_lint_rule! {
    /// Disallow checking a map for one key and then using a different key.
    ///
    /// After `map.has(key)` checks whether a key exists, the code that runs because
    /// of that check usually reads or writes the same key with `map.get(key)` or
    /// `map.set(key, value)`. Using a different key there is almost always a copy-paste
    /// mistake.
    ///
    /// This rule checks the branches of `if` statements, `while` loops, ternaries
    /// (`? :`), and the right side of `&&` and `||` that run because of a `has()` check.
    /// It only reports simple cases where the keys are clearly different: two different
    /// literal values, or two different variables or properties.
    ///
    /// A branch that is a block with more than one statement isn't checked.
    /// A branch that assigns a new value to the map isn't checked either, because the
    /// `get()` or `set()` call may use a different map.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const value = map.has(key) ? map.get(otherKey) : fallback;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (!map.has(key)) {
    ///     map.set(otherKey, value);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function getUser(cache) {
    ///     if (cache.has("user")) {
    ///         return cache.get("users");
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const value = map.has(key) ? map.get(key) : fallback;
    ///
    /// if (!map.has(key)) {
    ///     map.set(key, value);
    /// }
    ///
    /// if (map.has(key)) {
    ///     otherMap.get(otherKey);
    /// }
    /// ```
    ///
    pub NoMismatchedMapKey {
        version: "next",
        name: "noMismatchedMapKey",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-mismatched-map-key").inspired()],
        recommended: false,
        severity: Severity::Warning,
    }
}

declare_node_union! {
    /// A node that runs some of its code only depending on the result of a `has()` check.
    pub AnyJsMapGuard = JsConditionalExpression | JsIfStatement | JsLogicalExpression | JsWhileStatement
}

/// A `get()` or `set()` call whose key differs from the key of the `has()` check that
/// decides whether the call runs.
pub struct MismatchedKey {
    method: MapMethod,
    /// The key passed to `has()`.
    checked_key: AnyJsExpression,
    /// The key passed to `get()` or `set()`.
    accessed_key: AnyJsExpression,
}

impl Rule for NoMismatchedMapKey {
    type Query = Semantic<AnyJsMapGuard>;
    type State = MismatchedKey;
    type Signals = Box<[Self::State]>;
    type Options = NoMismatchedMapKeyOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let guard = ctx.query();
        let Some(check) = guard.has_check() else {
            return Box::default();
        };
        let model = ctx.model();
        let mut mismatches = Vec::new();
        for branch in guard.guarded_branches().into_iter().flatten() {
            collect_mismatches(model, &check, &branch, &mut mismatches);
        }
        mismatches.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.accessed_key.range(),
                markup! {
                    "This "<Emphasis>{state.method}</Emphasis>" call uses a different key than the "<Emphasis>"has()"</Emphasis>" check before it."
                },
            )
            .detail(
                state.checked_key.range(),
                markup! { "The map is checked for this key." },
            )
            .note(markup! {
                "Checking one key and then using another is usually a copy-paste mistake, so this call may use the wrong entry."
            })
            .note(markup! {
                "Use the same key in both calls."
            }),
        )
    }
}

declare_node_union! {
    /// The code that a [`AnyJsMapGuard`] runs depending on its `has()` check.
    pub AnyJsGuardedBranch = AnyJsStatement | AnyJsExpression
}

impl AnyJsMapGuard {
    /// Returns the `has()` call whose result alone decides whether a branch runs.
    ///
    /// For `a && b.has(key)`, the check is the last operand, because the branch runs only
    /// when it is true. A negated check such as `!b.has(key)` counts too.
    fn has_check(&self) -> Option<MapMethodCall> {
        let test = match self {
            Self::JsLogicalExpression(logical) => {
                // `b.has(key) ?? x` never evaluates `x`, since `has()` returns a boolean.
                if logical.operator().ok()? == JsLogicalOperator::NullishCoalescing {
                    return None;
                }
                logical.left().ok()?.omit_type_wrappers()?
            }
            Self::JsConditionalExpression(conditional) => {
                last_and_operand(conditional.test().ok()?)?
            }
            Self::JsIfStatement(statement) => last_and_operand(statement.test().ok()?)?,
            Self::JsWhileStatement(statement) => last_and_operand(statement.test().ok()?)?,
        };
        let test = match test {
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                unary.argument().ok()?.omit_type_wrappers()?
            }
            test => test,
        };
        MapMethodCall::from_call(test.as_js_call_expression()?)
            .filter(|call| call.method == MapMethod::Has)
    }

    /// Returns the branches whose code runs depending on the `has()` check.
    ///
    /// A statement branch is checked only if it is a single statement, or a block with a
    /// single statement. An `else if` branch isn't checked, because its own test also
    /// decides whether it runs.
    fn guarded_branches(&self) -> [Option<AnyJsGuardedBranch>; 2] {
        match self {
            Self::JsConditionalExpression(conditional) => [
                conditional.consequent().ok().map(AnyJsGuardedBranch::from),
                conditional.alternate().ok().map(AnyJsGuardedBranch::from),
            ],
            Self::JsLogicalExpression(logical) => {
                [logical.right().ok().map(AnyJsGuardedBranch::from), None]
            }
            Self::JsWhileStatement(statement) => [
                statement
                    .body()
                    .ok()
                    .and_then(single_statement)
                    .map(AnyJsGuardedBranch::from),
                None,
            ],
            Self::JsIfStatement(statement) => [
                statement
                    .consequent()
                    .ok()
                    .and_then(single_statement)
                    .map(AnyJsGuardedBranch::from),
                statement
                    .else_clause()
                    .and_then(|clause| clause.alternate().ok())
                    .filter(|alternate| !matches!(alternate, AnyJsStatement::JsIfStatement(_)))
                    .and_then(single_statement)
                    .map(AnyJsGuardedBranch::from),
            ],
        }
    }

    /// Returns `true` if the condition of this node calls `has()` on `receiver`, outside
    /// of nested functions and classes.
    fn checks_receiver(&self, model: &SemanticModel, receiver: &AnyJsExpression) -> bool {
        let test = match self {
            Self::JsConditionalExpression(conditional) => conditional.test(),
            Self::JsIfStatement(statement) => statement.test(),
            Self::JsLogicalExpression(logical) => logical.left(),
            Self::JsWhileStatement(statement) => statement.test(),
        };
        let Ok(test) = test else {
            return false;
        };
        let mut preorder = test.syntax().preorder_of_kinds(HAS_CHECK_SEARCH_KINDS);
        while let Some(event) = preorder.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(call) = JsCallExpression::cast_ref(&node) {
                if MapMethodCall::from_call(&call).is_some_and(|call| {
                    call.method == MapMethod::Has
                        && is_same_value(model, &call.receiver, receiver) == Some(true)
                }) {
                    return true;
                }
            } else {
                preorder.skip_subtree();
            }
        }
        false
    }
}

/// A call to `has()`, `get()`, or `set()` with a plain key argument, such as `map.get(key)`.
struct MapMethodCall {
    method: MapMethod,
    /// The object the method is called on, such as `map` in `map.get(key)`.
    receiver: AnyJsExpression,
    /// The first argument.
    key: AnyJsExpression,
}

impl MapMethodCall {
    /// Returns `None` if `call` isn't a call to `has()` with exactly one argument, or to
    /// `get()` or `set()` with at least one argument.
    ///
    /// Optional calls such as `map?.get(key)`, computed methods such as `map["get"](key)`,
    /// and calls with spread arguments such as `map.get(...keys)` also return `None`.
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.is_optional() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_static_member_expression()?;
        if callee.is_optional() {
            return None;
        }
        let method = match callee.member().ok()?.as_js_name()?.value_token().ok()?.text_trimmed() {
            "has" => MapMethod::Has,
            "get" => MapMethod::Get,
            "set" => MapMethod::Set,
            _ => return None,
        };
        let arguments = call.arguments().ok()?.args();
        if method == MapMethod::Has && arguments.len() != 1 {
            return None;
        }
        if arguments
            .iter()
            .any(|argument| argument.ok().and_then(|a| a.as_any_js_expression().cloned()).is_none())
        {
            return None;
        }
        let key = arguments.first()?.ok()?.as_any_js_expression()?.clone();
        Some(Self {
            method,
            receiver: callee.object().ok()?,
            key,
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum MapMethod {
    Has,
    Get,
    Set,
}

impl biome_console::fmt::Display for MapMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Has => "has()",
            Self::Get => "get()",
            Self::Set => "set()",
        })
    }
}

/// How a key can be compared with another key without running the code.
#[derive(Clone, Copy, Eq, PartialEq)]
enum KeyKind {
    /// A literal value other than a regular expression, such as `"a"` or `1`.
    Literal,
    /// A variable, `this`, or a chain of property reads on them, such as `a.b["c"]`.
    Reference,
}

impl KeyKind {
    /// Returns `None` for keys that can't be compared, such as calls or arrays.
    fn from_expression(expression: &AnyJsExpression) -> Option<Self> {
        let expression = expression.clone().omit_type_wrappers()?;
        if is_comparable_literal(&expression) {
            Some(Self::Literal)
        } else {
            is_plain_reference(expression).then_some(Self::Reference)
        }
    }
}

/// Pushes to `mismatches` every `get()` or `set()` call in `branch` that uses the same map
/// as `check` with a different key.
///
/// Calls inside nested functions and classes are ignored, since they may run later.
/// Calls inside a nested node with its own `has()` check on the same map are ignored too,
/// since that check is the one that decides whether they run. If `branch` assigns a new
/// value to the map, nothing is pushed, because the calls may use a different map.
fn collect_mismatches(
    model: &SemanticModel,
    check: &MapMethodCall,
    branch: &AnyJsGuardedBranch,
    mismatches: &mut Vec<MismatchedKey>,
) {
    let start = mismatches.len();
    let mut preorder = branch.syntax().preorder_of_kinds(BRANCH_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if NESTED_SCOPE_KINDS.matches(node.kind()) {
            preorder.skip_subtree();
        } else if let Some(guard) = AnyJsMapGuard::cast_ref(&node) {
            if guard.checks_receiver(model, &check.receiver) {
                preorder.skip_subtree();
            }
        } else if let Some(call) = JsCallExpression::cast_ref(&node) {
            let Some(access) = MapMethodCall::from_call(&call) else {
                continue;
            };
            if access.method != MapMethod::Has
                && is_same_value(model, &check.receiver, &access.receiver) == Some(true)
                && are_different_keys(model, &check.key, &access.key)
            {
                mismatches.push(MismatchedKey {
                    method: access.method,
                    checked_key: check.key.clone(),
                    accessed_key: access.key,
                });
            }
        } else if let Some(target) = AnyJsAssignment::cast_ref(&node) {
            if assigns_receiver(&target, &check.receiver) {
                mismatches.truncate(start);
                return;
            }
        } else if let Some(declarator) = JsVariableDeclarator::cast(node)
            && declarator.initializer().is_some()
            && declares_receiver(&declarator, &check.receiver)
        {
            mismatches.truncate(start);
            return;
        }
    }
}

/// Returns the last operand of a chain of `&&` operators, or `test` itself if it isn't
/// such a chain.
///
/// Returns `None` if the syntax tree is incomplete.
fn last_and_operand(test: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut test = test.omit_type_wrappers()?;
    while let AnyJsExpression::JsLogicalExpression(logical) = &test
        && logical.operator().ok()? == JsLogicalOperator::LogicalAnd
    {
        test = logical.right().ok()?.omit_type_wrappers()?;
    }
    Some(test)
}

/// Returns `statement` itself, or its only statement if it is a block.
///
/// Returns `None` for a block that doesn't have exactly one statement.
fn single_statement(statement: AnyJsStatement) -> Option<AnyJsStatement> {
    let AnyJsStatement::JsBlockStatement(block) = statement else {
        return Some(statement);
    };
    let statements = block.statements();
    if statements.len() == 1 {
        statements.first()
    } else {
        None
    }
}

/// Returns `true` if `checked` and `accessed` are clearly different keys: two different
/// literal values, or two references that read different variables or properties.
fn are_different_keys(
    model: &SemanticModel,
    checked: &AnyJsExpression,
    accessed: &AnyJsExpression,
) -> bool {
    match (
        KeyKind::from_expression(checked),
        KeyKind::from_expression(accessed),
    ) {
        (Some(KeyKind::Literal), Some(KeyKind::Literal)) => {
            is_same_reference(checked.clone(), accessed.clone()) == Some(false)
        }
        (Some(KeyKind::Reference), Some(KeyKind::Reference)) => {
            is_same_value(model, checked, accessed) == Some(false)
        }
        _ => false,
    }
}

/// Returns whether `left` and `right` read the same value, as [`is_same_reference`] does,
/// and additionally whether the variables they start from are the same variable, so a
/// shadowing variable with the same name doesn't match.
///
/// Returns `None` if the syntax tree is incomplete.
fn is_same_value(
    model: &SemanticModel,
    left: &AnyJsExpression,
    right: &AnyJsExpression,
) -> Option<bool> {
    if !is_same_reference(left.clone(), right.clone())? {
        return Some(false);
    }
    Some(match (root_identifier(left), root_identifier(right)) {
        (Some(left), Some(right)) => model.binding(&left) == model.binding(&right),
        _ => true,
    })
}

/// Returns `expression` and the objects of its property reads, from the outermost to the
/// innermost: `a.b.c`, `a.b`, then `a`.
fn member_chain(expression: &AnyJsExpression) -> impl Iterator<Item = AnyJsExpression> {
    iter::successors(expression.clone().omit_type_wrappers(), |expression| {
        AnyJsMemberExpression::cast_ref(expression.syntax())?
            .object()
            .ok()?
            .omit_type_wrappers()
    })
}

/// Returns the variable that a chain of property reads starts from, such as `a` in
/// `a.b.c`, or `None` if it doesn't start from a variable.
fn root_identifier(expression: &AnyJsExpression) -> Option<JsReferenceIdentifier> {
    member_chain(expression).last()?.as_js_reference_identifier()
}

/// Returns `true` if assigning to `target` changes which map `receiver` reads, such as
/// assigning to `a` or `a.b` when `receiver` is `a.b`.
fn assigns_receiver(target: &AnyJsAssignment, receiver: &AnyJsExpression) -> bool {
    member_chain(receiver).any(|expression| {
        target
            .syntax()
            .descendants_tokens(Direction::Next)
            .map(|token| token.token_text_trimmed())
            .eq(expression
                .syntax()
                .descendants_tokens(Direction::Next)
                .map(|token| token.token_text_trimmed()))
    })
}

/// Returns `true` if `declarator` declares a variable with the same name as the variable
/// that `receiver` starts from.
fn declares_receiver(declarator: &JsVariableDeclarator, receiver: &AnyJsExpression) -> bool {
    let Some(name) = root_identifier(receiver).and_then(|root| root.value_token().ok()) else {
        return false;
    };
    let Ok(id) = declarator.id() else {
        return false;
    };
    id.syntax()
        .preorder_of_kinds(JsIdentifierBinding::KIND_SET)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => JsIdentifierBinding::cast(node),
            WalkEvent::Leave(_) => None,
        })
        .filter_map(|binding| binding.name_token().ok())
        .any(|binding| binding.text_trimmed() == name.text_trimmed())
}

/// Returns `true` if `expression` is a literal whose value can be compared without
/// running the code. Each regular expression literal creates a new object, so two of
/// them are never the same key.
fn is_comparable_literal(expression: &AnyJsExpression) -> bool {
    matches!(
        expression,
        AnyJsExpression::AnyJsLiteralExpression(literal)
            if !matches!(literal, AnyJsLiteralExpression::JsRegexLiteralExpression(_))
    )
}

/// Returns `true` if `expression` is a variable, `this`, `super`, or a chain of property
/// reads on them whose computed properties are themselves comparable literals or plain
/// references, such as `a.b[c][0]`.
///
/// Optional property reads such as `a?.b` return `false`: `a?.b` and `a.b` are compared as
/// different references, but they read the same value whenever `a.b` doesn't throw.
fn is_plain_reference(expression: AnyJsExpression) -> bool {
    let mut expression = expression;
    loop {
        let Some(current) = expression.omit_type_wrappers() else {
            return false;
        };
        expression = match current {
            AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsThisExpression(_)
            | AnyJsExpression::JsSuperExpression(_) => return true,
            AnyJsExpression::JsStaticMemberExpression(member) if !member.is_optional() => {
                match member.object() {
                    Ok(object) => object,
                    Err(_) => return false,
                }
            }
            AnyJsExpression::JsComputedMemberExpression(member) if !member.is_optional() => {
                let (Ok(object), Some(property)) = (
                    member.object(),
                    member.member().ok().and_then(AnyJsExpression::omit_type_wrappers),
                ) else {
                    return false;
                };
                if !is_comparable_literal(&property) && !is_plain_reference(property) {
                    return false;
                }
                object
            }
            _ => return false,
        };
    }
}

/// Nodes whose code may run later than the code around them.
const NESTED_SCOPE_KINDS: SyntaxKindSet<JsLanguage> = AnyJsFunction::KIND_SET
    .union(AnyJsClass::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET);

/// The nodes [`AnyJsMapGuard::checks_receiver`] stops at: calls, and the nodes it skips.
const HAS_CHECK_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsCallExpression::KIND_SET.union(NESTED_SCOPE_KINDS);

/// The nodes [`collect_mismatches`] stops at: calls, nested `has()` checks, assignments
/// that may replace the map, and the nodes it skips.
const BRANCH_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = HAS_CHECK_SEARCH_KINDS
    .union(AnyJsMapGuard::KIND_SET)
    .union(JsIdentifierAssignment::KIND_SET)
    .union(JsStaticMemberAssignment::KIND_SET)
    .union(JsComputedMemberAssignment::KIND_SET)
    .union(JsVariableDeclarator::KIND_SET);
