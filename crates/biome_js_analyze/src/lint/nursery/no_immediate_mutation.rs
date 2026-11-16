use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsArrayElement,
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsExpression, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsStatement,
    AnyTsVariableAnnotation, JsArrayAssignmentPattern, JsAssignmentExpression,
    JsAssignmentOperator, JsAwaitExpression, JsCallExpression, JsExpressionStatement,
    JsIdentifierAssignment, JsImportCallExpression, JsLanguage, JsModuleItemList, JsNewExpression,
    JsObjectExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsReferenceIdentifier,
    JsSpread, JsStatementList, JsSyntaxToken, JsTemplateExpression, JsUnaryExpression,
    JsUnaryOperator, JsVariableDeclaration, JsVariableDeclarator, JsVariableStatement,
    JsYieldExpression, T, global_identifier,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutation, BatchMutationExt, SyntaxKindSet, TextRange,
    TriviaPieceKind, WalkEvent, declare_node_union,
};
use biome_rule_options::no_immediate_mutation::NoImmediateMutationOptions;

use crate::JsRuleAction;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow changing a value right after it is created.
    ///
    /// Adding to an array, object, `Set`, or `Map` in the statement after it is created splits its
    /// initial contents in two. The same goes for declaring a variable and assigning it in the next
    /// statement. Put the contents in the initial value instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const array = [1, 2];
    /// array.push(3, 4);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const array = [3, 4];
    /// array.unshift(1, 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = { foo: 1 };
    /// object.bar = 2;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const object = { foo: 1 };
    /// Object.assign(object, { bar: 2 });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const set = new Set([1, 2]);
    /// set.add(3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const map = new Map([["foo", 1]]);
    /// map.set("bar", 2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let foo;
    /// foo = 1;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const array = [1, 2, 3, 4];
    ///
    /// const object = { foo: 1, bar: 2, ...other };
    ///
    /// const set = new Set([1, 2, 3]);
    ///
    /// const map = new Map([
    ///     ["foo", 1],
    ///     ["bar", 2],
    /// ]);
    ///
    /// const items = [];
    /// items.push(items.length);
    ///
    /// let foo = 1;
    /// ```
    ///
    pub NoImmediateMutation {
        version: "next",
        name: "noImmediateMutation",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-immediate-mutation").inspired()],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub AnyJsVariableCreationStatement = JsVariableStatement | JsExpressionStatement
}

pub struct NoImmediateMutationState {
    kind: ImmediateMutationKind,
    /// The statement right after the queried statement, which changes or assigns the variable.
    next_statement: JsExpressionStatement,
    /// The expression of `next_statement`, without parentheses.
    expression: AnyJsExpression,
}

impl Rule for NoImmediateMutation {
    type Query = Semantic<AnyJsVariableCreationStatement>;
    type State = NoImmediateMutationState;
    type Signals = Option<Self::State>;
    type Options = NoImmediateMutationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let statement = ctx.query();
        if !STATEMENT_LIST_KINDS.matches(statement.syntax().parent()?.kind()) {
            return None;
        }
        let next_statement = JsExpressionStatement::cast(statement.syntax().next_sibling()?)?;
        let expression = next_statement.expression().ok()?.omit_parentheses();
        let targets = match statement {
            AnyJsVariableCreationStatement::JsVariableStatement(statement) => {
                initialized_declarators(&statement.declaration().ok()?, &expression)
            }
            AnyJsVariableCreationStatement::JsExpressionStatement(_) => None,
        };
        let kind = match targets {
            Some(targets) => ImmediateMutationKind::Initialization(targets),
            None => {
                let (name, value) = created_value(statement)?;
                let collection =
                    find_mutation(ctx.model(), name.text_trimmed(), &value, &expression)?;
                ImmediateMutationKind::Mutation { collection, value }
            }
        };
        Some(NoImmediateMutationState {
            kind,
            next_statement,
            expression,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let statement = ctx.query();
        let expression = &state.expression;
        match &state.kind {
            ImmediateMutationKind::Mutation { collection, value } => {
                let range = match expression {
                    // `object.foo =`
                    AnyJsExpression::JsAssignmentExpression(assignment) => TextRange::new(
                        assignment.range().start(),
                        assignment.operator_token().ok()?.text_trimmed_range().end(),
                    ),
                    // `array.push`
                    AnyJsExpression::JsCallExpression(call) => call.callee().ok()?.range(),
                    _ => return None,
                };
                Some(
                    RuleDiagnostic::new(
                        rule_category!(),
                        range,
                        markup! {
                            "This "{collection}" is changed right after it is created."
                        },
                    )
                    .detail(
                        value.range(),
                        markup! { "The "{collection}" is created here." },
                    )
                    .note(markup! {
                        "Creating a value and changing it in the next statement splits its initial contents across two statements, which makes them harder to read."
                    })
                    .note(markup! {
                        "Include the change in the initial value instead."
                    }),
                )
            }
            ImmediateMutationKind::Initialization(targets) => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    expression.range(),
                    if targets.count_ones() == 1 {
                        markup! { "This variable is assigned right after it is declared." }
                    } else {
                        markup! { "These variables are assigned right after they are declared." }
                    },
                )
                .detail(statement.range(), markup! { "The declaration is here." })
                .note(markup! {
                    "Declaring a variable and assigning its first value in two statements makes the code longer and harder to follow."
                })
                .note(markup! {
                    "Assign the value in the declaration instead."
                }),
            ),
        }
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let statement = ctx.query();
        let next_statement = &state.next_statement;
        if next_statement.syntax().has_comments_descendants() {
            return None;
        }
        let mut mutation = ctx.root().begin();
        let message = match &state.kind {
            ImmediateMutationKind::Mutation { collection, value } => {
                if value.syntax().has_inner_comments()
                    || !can_remove_after(statement, next_statement)
                {
                    return None;
                }
                merge_mutation(&mut mutation, *collection, value, &state.expression)?;
                mutation.remove_node(next_statement.clone());
                markup! { "Move the change into the initial value." }
            }
            ImmediateMutationKind::Initialization(targets) => {
                let AnyJsVariableCreationStatement::JsVariableStatement(statement) = statement
                else {
                    return None;
                };
                merge_initialization(
                    &mut mutation,
                    statement,
                    next_statement,
                    &state.expression,
                    *targets,
                )?;
                markup! { "Assign the value in the declaration." }
            }
        };
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            message.to_owned(),
            mutation,
        ))
    }
}

enum ImmediateMutationKind {
    /// The next statement changes `value`, the value that the queried statement created.
    Mutation {
        collection: Collection,
        value: AnyJsExpression,
    },
    /// The next statement assigns variables that the queried declaration declared without a value.
    ///
    /// Bit `i` is set when the `i`-th declarator of the declaration is assigned.
    Initialization(u64),
}

/// The kind of value that the queried statement creates.
#[derive(Clone, Copy)]
enum Collection {
    /// `prepend` is set when the elements are added with `unshift()`.
    Array {
        prepend: bool,
    },
    Object,
    Set,
    WeakSet,
    Map,
    WeakMap,
}

impl Collection {
    /// Returns the collection created by `new <name>()`.
    fn from_constructor_name(name: &str) -> Option<Self> {
        Some(match name {
            "Set" => Self::Set,
            "WeakSet" => Self::WeakSet,
            "Map" => Self::Map,
            "WeakMap" => Self::WeakMap,
            _ => return None,
        })
    }

    fn is_set(self) -> bool {
        matches!(self, Self::Set | Self::WeakSet)
    }
}

impl biome_console::fmt::Display for Collection {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Array { .. } => "array",
            Self::Object => "object",
            Self::Set => "Set",
            Self::WeakSet => "WeakSet",
            Self::Map => "Map",
            Self::WeakMap => "WeakMap",
        })
    }
}

const STATEMENT_LIST_KINDS: SyntaxKindSet<JsLanguage> =
    JsStatementList::KIND_SET.union(JsModuleItemList::KIND_SET);

/// The nodes that can refer to a variable by its name.
const VARIABLE_USE_KINDS: SyntaxKindSet<JsLanguage> =
    JsReferenceIdentifier::KIND_SET.union(JsIdentifierAssignment::KIND_SET);

/// Expressions that can do more than produce a value, such as calling a function or iterating
/// over a spread value. Unary expressions are included so that `delete` can be found.
const SIDE_EFFECT_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsSpread::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET);

/// Returns the name of the variable that `statement` creates or assigns, and the value it gets,
/// without parentheses.
///
/// Only the last declarator of a `const`, `let`, or `var` declaration is considered, because the
/// value of an earlier declarator isn't the last thing created before the next statement.
/// Only plain `variable = value` assignments are considered.
fn created_value(
    statement: &AnyJsVariableCreationStatement,
) -> Option<(JsSyntaxToken, AnyJsExpression)> {
    match statement {
        AnyJsVariableCreationStatement::JsVariableStatement(statement) => {
            let declaration = statement.declaration().ok()?;
            if declaration.await_token().is_some() || declaration.kind().ok()?.kind() == T![using] {
                return None;
            }
            let declarator = declaration.declarators().last()?.ok()?;
            let name = identifier_binding_name(&declarator)?;
            let value = declarator.initializer()?.expression().ok()?;
            Some((name, value.omit_parentheses()))
        }
        AnyJsVariableCreationStatement::JsExpressionStatement(statement) => {
            let expression = statement.expression().ok()?.omit_parentheses();
            let assignment = expression.as_js_assignment_expression()?;
            if assignment.operator().ok()? != JsAssignmentOperator::Assign {
                return None;
            }
            let AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                target,
            )) = assignment.left().ok()?
            else {
                return None;
            };
            let value = assignment.right().ok()?;
            Some((target.name_token().ok()?, value.omit_parentheses()))
        }
    }
}

/// Returns the kind of `value` if `expression` changes it in a way that could be written in
/// `value` directly. `name` is the variable that holds `value`.
fn find_mutation(
    model: &SemanticModel,
    name: &str,
    value: &AnyJsExpression,
    expression: &AnyJsExpression,
) -> Option<Collection> {
    match value {
        AnyJsExpression::JsArrayExpression(_) => {
            let (call, method) = method_call(expression, |object| is_identifier(object, name))?;
            let prepend = match method.text_trimmed() {
                "push" => false,
                "unshift" => true,
                _ => return None,
            };
            let arguments = call.arguments().ok()?.args();
            (!arguments.is_empty() && !uses_variable(&arguments, name))
                .then_some(Collection::Array { prepend })
        }
        AnyJsExpression::JsObjectExpression(_) => match expression {
            AnyJsExpression::JsAssignmentExpression(assignment) => {
                is_property_assignment(assignment, name).then_some(Collection::Object)
            }
            AnyJsExpression::JsCallExpression(_) => {
                is_object_assign(model, expression, name).then_some(Collection::Object)
            }
            _ => None,
        },
        AnyJsExpression::JsNewExpression(new_expression) => {
            let collection = constructed_collection(model, new_expression)?;
            let (call, method) = method_call(expression, |object| is_identifier(object, name))?;
            let (expected_method, expected_arguments) = if collection.is_set() {
                ("add", 1)
            } else {
                ("set", 2)
            };
            let arguments = call.arguments().ok()?.args();
            (method.text_trimmed() == expected_method
                && arguments.len() == expected_arguments
                && arguments
                    .iter()
                    .all(|argument| matches!(argument, Ok(AnyJsCallArgument::AnyJsExpression(_))))
                && !uses_variable(&arguments, name))
            .then_some(collection)
        }
        _ => None,
    }
}

/// Returns the call and the method name if `expression` is a call like `object.method(…)`,
/// without optional chaining, where `is_object` returns `true` for `object`.
fn method_call(
    expression: &AnyJsExpression,
    is_object: impl FnOnce(&AnyJsExpression) -> bool,
) -> Option<(JsCallExpression, JsSyntaxToken)> {
    let call = expression.as_js_call_expression()?;
    if call.is_optional() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    if member.is_optional() || !is_object(&member.object().ok()?) {
        return None;
    }
    let method = member.member().ok()?.as_js_name()?.value_token().ok()?;
    Some((call.clone(), method))
}

/// Returns `true` if `assignment` is like `name.key = value` or `name[key] = value`, and
/// neither the key nor the value uses `name`.
fn is_property_assignment(assignment: &JsAssignmentExpression, name: &str) -> bool {
    if assignment.operator() != Ok(JsAssignmentOperator::Assign) {
        return false;
    }
    let (Ok(AnyJsAssignmentPattern::AnyJsAssignment(target)), Ok(value)) =
        (assignment.left(), assignment.right())
    else {
        return false;
    };
    match target {
        AnyJsAssignment::JsStaticMemberAssignment(target) => {
            target
                .member()
                .is_ok_and(|member| member.as_js_name().is_some())
                && target
                    .object()
                    .is_ok_and(|object| is_identifier(&object, name))
                && !uses_variable(&value, name)
        }
        AnyJsAssignment::JsComputedMemberAssignment(target) => {
            target
                .object()
                .is_ok_and(|object| is_identifier(&object, name))
                && target.member().is_ok_and(|key| !uses_variable(&key, name))
                && !uses_variable(&value, name)
        }
        _ => false,
    }
}

/// Returns `true` if `expression` is like `Object.assign(name, source, …)`, where `source` isn't
/// a spread and doesn't use `name`.
fn is_object_assign(model: &SemanticModel, expression: &AnyJsExpression, name: &str) -> bool {
    let Some((call, method)) = method_call(expression, |object| is_global(model, object, "Object"))
    else {
        return false;
    };
    let Ok(arguments) = call.arguments() else {
        return false;
    };
    let mut arguments = arguments.args().iter();
    let (Some(Ok(AnyJsCallArgument::AnyJsExpression(target))), Some(Ok(source))) =
        (arguments.next(), arguments.next())
    else {
        return false;
    };
    method.text_trimmed() == "assign"
        && is_identifier(&target, name)
        && matches!(source, AnyJsCallArgument::AnyJsExpression(_))
        && !uses_variable(&source, name)
}

/// Returns the collection that `new_expression` creates, if it is a global `Set`, `WeakSet`,
/// `Map`, or `WeakMap` created without a value or from an array literal.
fn constructed_collection(
    model: &SemanticModel,
    new_expression: &JsNewExpression,
) -> Option<Collection> {
    let callee = new_expression.callee().ok()?;
    let (reference, name) = global_identifier(&callee.as_any_global_identifier_expression()?)?;
    if model.binding(&reference).is_some() {
        return None;
    }
    let collection = Collection::from_constructor_name(name.text())?;
    if let Some(arguments) = new_expression.arguments() {
        let mut arguments = arguments.args().iter();
        match (arguments.next(), arguments.next()) {
            (None, _) => {}
            (Some(Ok(AnyJsCallArgument::AnyJsExpression(iterable))), None)
                if iterable
                    .clone()
                    .omit_parentheses()
                    .as_js_array_expression()
                    .is_some() => {}
            _ => return None,
        }
    }
    Some(collection)
}

/// Returns the declarators of `declaration` that `next_statement` assigns, as a bit set,
/// if `declaration` is a `let` or `var` declaration whose variables have no value, and
/// `next_statement` only assigns some of them, as in `foo = value` or `[foo, bar] = value`.
fn initialized_declarators(
    declaration: &JsVariableDeclaration,
    expression: &AnyJsExpression,
) -> Option<u64> {
    if !(declaration.is_let() || declaration.is_var()) {
        return None;
    }
    let declarators = declaration.declarators();
    if declarators.len() > u64::BITS as usize {
        return None;
    }
    let names = declarators
        .iter()
        .map(|declarator| {
            let declarator = declarator.ok()?;
            if declarator.initializer().is_some() {
                return None;
            }
            identifier_binding_name(&declarator)
        })
        .collect::<Option<Box<[_]>>>()?;
    let assignment = expression.as_js_assignment_expression()?;
    if assignment.operator().ok()? != JsAssignmentOperator::Assign {
        return None;
    }

    let mut targets = 0u64;
    let mut add_target = |target: &JsIdentifierAssignment| -> Option<()> {
        let target = target.name_token().ok()?;
        let index = names
            .iter()
            .position(|name| name.text_trimmed() == target.text_trimmed())?;
        let bit = 1u64 << index;
        if targets & bit != 0 {
            return None;
        }
        targets |= bit;
        Some(())
    };
    match assignment.left().ok()? {
        AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
            target,
        )) => add_target(&target)?,
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(pattern) => {
            for target in array_pattern_targets(&pattern)?.iter().flatten() {
                add_target(target)?;
            }
        }
        _ => return None,
    }
    if targets == 0 {
        return None;
    }

    // A `let` variable can't be read until its declaration has finished. Creating a function
    // doesn't run its body, so a function that uses the variable is fine.
    let value = assignment.right().ok()?;
    if declaration.is_let()
        && !value.clone().omit_type_wrappers().is_some_and(|value| {
            matches!(
                value,
                AnyJsExpression::JsArrowFunctionExpression(_)
                    | AnyJsExpression::JsFunctionExpression(_)
            )
        })
        && names.iter().enumerate().any(|(index, name)| {
            targets & (1u64 << index) != 0 && uses_variable(&value, name.text_trimmed())
        })
    {
        return None;
    }
    Some(targets)
}

/// Returns the name of the variable that `declarator` declares, if it declares a single variable.
fn identifier_binding_name(declarator: &JsVariableDeclarator) -> Option<JsSyntaxToken> {
    let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) =
        declarator.id().ok()?
    else {
        return None;
    };
    binding.name_token().ok()
}

/// Returns the variables that `pattern` assigns, with `None` for each hole, if every element is a
/// variable without a default value.
fn array_pattern_targets(
    pattern: &JsArrayAssignmentPattern,
) -> Option<Vec<Option<JsIdentifierAssignment>>> {
    pattern
        .elements()
        .iter()
        .map(|element| match element.ok()? {
            AnyJsArrayAssignmentPatternElement::JsArrayHole(_) => Some(None),
            AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(element)
                if element.init().is_none() =>
            {
                match element.pattern().ok()? {
                    AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::JsIdentifierAssignment(target),
                    ) => Some(Some(target)),
                    _ => None,
                }
            }
            _ => None,
        })
        .collect()
}

/// Returns `true` if `object` has a getter, a setter, or a `__proto__` key.
fn has_accessor_or_prototype(object: &JsObjectExpression) -> bool {
    object.members().iter().flatten().any(|member| {
        matches!(
            member,
            AnyJsObjectMember::JsGetterObjectMember(_) | AnyJsObjectMember::JsSetterObjectMember(_)
        ) || member.name().is_some_and(|name| name.text() == "__proto__")
    })
}

/// Returns `true` if `expression` is the variable `name`, possibly in parentheses.
fn is_identifier(expression: &AnyJsExpression, name: &str) -> bool {
    expression
        .clone()
        .omit_parentheses()
        .as_js_identifier_expression()
        .and_then(|identifier| identifier.name().ok()?.value_token().ok())
        .is_some_and(|identifier| identifier.text_trimmed() == name)
}

/// Returns whether `expression` refers to the global variable `name` and isn't shadowed
/// by a local declaration.
fn is_global(model: &SemanticModel, expression: &AnyJsExpression, name: &str) -> bool {
    expression
        .as_any_global_identifier_expression()
        .and_then(|expression| global_identifier(&expression))
        .is_some_and(|(reference, global_name)| {
            global_name.text() == name && model.binding(&reference).is_none()
        })
}

/// Returns `true` if `node` reads or assigns a variable called `name`.
///
/// Variables of the same name declared inside `node` count too, which can only make the rule
/// report less.
fn uses_variable(node: &impl AstNode<Language = JsLanguage>, name: &str) -> bool {
    node.syntax()
        .preorder_of_kinds(VARIABLE_USE_KINDS)
        .filter_map(|event| match event {
            WalkEvent::Enter(node) => Some(node),
            WalkEvent::Leave(_) => None,
        })
        .any(|node| {
            let token = JsReferenceIdentifier::cast_ref(&node)
                .and_then(|reference| reference.value_token().ok())
                .or_else(|| {
                    JsIdentifierAssignment::cast_ref(&node)
                        .and_then(|assignment| assignment.name_token().ok())
                });
            token.is_some_and(|token| token.text_trimmed() == name)
        })
}

/// Returns `true` if evaluating `node` may do more than produce a value, such as calling a
/// function or assigning to a variable.
fn may_have_side_effects(node: &impl AstNode<Language = JsLanguage>) -> bool {
    node.syntax()
        .preorder_of_kinds(SIDE_EFFECT_KINDS)
        .any(|event| match event {
            WalkEvent::Enter(node) => JsUnaryExpression::cast(node)
                .is_none_or(|unary| unary.operator() == Ok(JsUnaryOperator::Delete)),
            WalkEvent::Leave(_) => false,
        })
}

/// Returns `true` if `next_statement` can be removed without joining `statement` with the
/// statement after `next_statement`.
///
/// A `;` that ends `next_statement` also ends `statement` when `statement` has no `;` of its own,
/// as in `const array = []\narray.push(1);[2].forEach(log)`.
fn can_remove_after(
    statement: &AnyJsVariableCreationStatement,
    next_statement: &JsExpressionStatement,
) -> bool {
    // Both kinds of statement end with their optional `;`.
    let has_semicolon = statement
        .syntax()
        .last_token()
        .is_some_and(|token| token.kind() == T![;]);
    has_semicolon || next_statement.semicolon_token().is_none()
}

/// Adds the change made by `expression` to `value`, the initial value of a `collection`.
fn merge_mutation(
    mutation: &mut BatchMutation<JsLanguage>,
    collection: Collection,
    value: &AnyJsExpression,
    expression: &AnyJsExpression,
) -> Option<()> {
    match collection {
        Collection::Array { prepend } => {
            let array = value.as_js_array_expression()?;
            let call = expression.as_js_call_expression()?;
            let new_elements = call
                .arguments()
                .ok()?
                .args()
                .elements()
                .map(|element| {
                    let separator = element.trailing_separator().ok()?.cloned();
                    let element = match element.into_node().ok()? {
                        AnyJsCallArgument::AnyJsExpression(expression) => {
                            AnyJsArrayElement::AnyJsExpression(expression)
                        }
                        AnyJsCallArgument::JsSpread(spread) => AnyJsArrayElement::JsSpread(spread),
                    };
                    Some((element, separator))
                })
                .collect::<Option<Vec<_>>>()?;
            let existing_elements = separated_elements(&array.elements())?;
            let elements = if prepend {
                // The new elements are evaluated before the existing ones.
                if array
                    .elements()
                    .iter()
                    .flatten()
                    .any(|element| may_have_side_effects(&element))
                    || new_elements
                        .iter()
                        .any(|(element, _)| may_have_side_effects(element))
                {
                    return None;
                }
                join_lists(new_elements, existing_elements)?
            } else {
                join_lists(existing_elements, new_elements)?
            };
            mutation.replace_node(
                array.elements(),
                make::js_array_element_list(elements.0, elements.1),
            );
        }
        Collection::Object => {
            let object = value.as_js_object_expression()?;
            // An assignment and `Object.assign()` call a setter of the object or of its prototype
            // set with `__proto__`, while a property in the object literal replaces it.
            if has_accessor_or_prototype(object) {
                return None;
            }
            let new_members = match expression {
                AnyJsExpression::JsAssignmentExpression(assignment) => {
                    vec![(property_from_assignment(assignment)?, None)]
                }
                AnyJsExpression::JsCallExpression(call) => {
                    let arguments = call.arguments().ok()?.args();
                    if arguments.len() != 2 {
                        return None;
                    }
                    let Some(Ok(AnyJsCallArgument::AnyJsExpression(source))) = arguments.last()
                    else {
                        return None;
                    };
                    match source.omit_parentheses() {
                        AnyJsExpression::JsObjectExpression(source) => members_to_copy(&source)?,
                        source => vec![(
                            AnyJsObjectMember::JsSpread(make::js_spread(
                                make::token(T![...]),
                                source,
                            )),
                            None,
                        )],
                    }
                }
                _ => return None,
            };
            let existing_members = separated_elements(&object.members())?;
            let members = join_lists(existing_members, new_members)?;
            mutation.replace_node(
                object.members(),
                make::js_object_member_list(members.0, members.1),
            );
        }
        Collection::Set | Collection::WeakSet | Collection::Map | Collection::WeakMap => {
            let new_expression = value.as_js_new_expression()?;
            let call = expression.as_js_call_expression()?;
            let arguments = call.arguments().ok()?.args();
            let element = if collection.is_set() {
                let Some(Ok(AnyJsCallArgument::AnyJsExpression(element))) = arguments.first()
                else {
                    return None;
                };
                AnyJsArrayElement::AnyJsExpression(element)
            } else {
                // `map.set(key, value)` becomes the entry `[key, value]`.
                let mut entry = arguments.elements();
                let key = entry.next()?;
                let separator = key.trailing_separator().ok()??.clone();
                let Ok(AnyJsCallArgument::AnyJsExpression(key)) = key.into_node() else {
                    return None;
                };
                let Ok(AnyJsCallArgument::AnyJsExpression(value)) = entry.next()?.into_node()
                else {
                    return None;
                };
                AnyJsArrayElement::AnyJsExpression(AnyJsExpression::JsArrayExpression(
                    make::js_array_expression(
                        make::token(T!['[']),
                        make::js_array_element_list(
                            [
                                AnyJsArrayElement::AnyJsExpression(key),
                                AnyJsArrayElement::AnyJsExpression(value),
                            ],
                            [separator],
                        ),
                        make::token(T![']']),
                    ),
                ))
            };
            let iterable = new_expression
                .arguments()
                .and_then(|arguments| arguments.args().first()?.ok());
            if let Some(AnyJsCallArgument::AnyJsExpression(iterable)) = iterable {
                let array = iterable.omit_parentheses();
                let array = array.as_js_array_expression()?;
                let elements = join_lists(
                    separated_elements(&array.elements())?,
                    vec![(element, None)],
                )?;
                mutation.replace_node(
                    array.elements(),
                    make::js_array_element_list(elements.0, elements.1),
                );
            } else {
                let iterable = AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::JsArrayExpression(make::js_array_expression(
                        make::token(T!['[']),
                        make::js_array_element_list([element], []),
                        make::token(T![']']),
                    )),
                );
                let arguments = make::js_call_arguments(
                    make::token(T!['(']),
                    make::js_call_argument_list([iterable], []),
                    make::token(T![')']),
                );
                // The trivia after `new Set` moves after the new arguments, so that a comment
                // there can't swallow them.
                let new_expression = new_expression.clone();
                let with_arguments = match new_expression.type_arguments() {
                    Some(type_arguments) => new_expression
                        .clone()
                        .with_type_arguments(Some(type_arguments.with_trailing_trivia_pieces([])?)),
                    None => new_expression.clone().with_callee(
                        new_expression
                            .callee()
                            .ok()?
                            .with_trailing_trivia_pieces([])?,
                    ),
                }
                .with_arguments(Some(arguments));
                mutation.replace_node(new_expression, with_arguments);
            }
        }
    }
    Some(())
}

/// Returns the object property written by `assignment`: `key: value` for `object.key = value`,
/// and `[key]: value` for `object[key] = value`.
fn property_from_assignment(assignment: &JsAssignmentExpression) -> Option<AnyJsObjectMember> {
    let AnyJsAssignmentPattern::AnyJsAssignment(target) = assignment.left().ok()? else {
        return None;
    };
    let name = match target {
        AnyJsAssignment::JsStaticMemberAssignment(target) => {
            let member = target.member().ok()?;
            let name = member.as_js_name()?.value_token().ok()?;
            // `object.__proto__ = value` changes the prototype, while `__proto__: value` in the
            // object literal does too but only once.
            if name.text_trimmed() == "__proto__" {
                return None;
            }
            AnyJsObjectMemberName::JsLiteralMemberName(make::js_literal_member_name(make::ident(
                name.text_trimmed(),
            )))
        }
        AnyJsAssignment::JsComputedMemberAssignment(target) => {
            let key = target.member().ok()?;
            // `object["__proto__"] = value` changes the prototype, while `["__proto__"]: value`
            // in an object literal creates a property.
            if key
                .as_static_value()
                .is_some_and(|key| key.text() == "__proto__")
            {
                return None;
            }
            AnyJsObjectMemberName::JsComputedMemberName(make::js_computed_member_name(
                make::token(T!['[']),
                key,
                make::token(T![']']),
            ))
        }
        _ => return None,
    };
    Some(AnyJsObjectMember::JsPropertyObjectMember(
        make::js_property_object_member(
            name,
            make::token(T![:]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            assignment.right().ok()?,
        ),
    ))
}

/// Returns the members of `source`, the object literal passed to `Object.assign()`, if
/// writing them in the target object literal has the same effect.
fn members_to_copy(
    source: &JsObjectExpression,
) -> Option<Vec<(AnyJsObjectMember, Option<JsSyntaxToken>)>> {
    // `Object.assign()` reads getters and doesn't copy a `__proto__` key that sets the prototype,
    // while the target object literal would keep the getter and change its prototype.
    if has_accessor_or_prototype(source) {
        return None;
    }
    separated_elements(&source.members())
}

/// Returns the nodes of `list` with the separator that follows each of them.
fn separated_elements<L>(list: &L) -> Option<Vec<(L::Node, Option<JsSyntaxToken>)>>
where
    L: AstSeparatedList<Language = JsLanguage>,
{
    list.elements()
        .map(|element| {
            let separator = element.trailing_separator().ok()?.cloned();
            Some((element.into_node().ok()?, separator))
        })
        .collect()
}

/// Returns the nodes and separators of a list that holds the nodes of `first` followed by the
/// nodes of `second`, adding a separator between the two parts when `first` doesn't end with one.
fn join_lists<N: AstNode<Language = JsLanguage>>(
    first: Vec<(N, Option<JsSyntaxToken>)>,
    second: Vec<(N, Option<JsSyntaxToken>)>,
) -> Option<(Vec<N>, Vec<JsSyntaxToken>)> {
    let length = first.len() + second.len();
    let mut nodes = Vec::with_capacity(length);
    let mut separators = Vec::with_capacity(length);
    let first_length = first.len();
    for (index, (node, separator)) in first.into_iter().enumerate() {
        if index + 1 < first_length || second.is_empty() {
            nodes.push(node);
            separators.extend(separator);
            continue;
        }
        let (node, separator) = match separator {
            // `[1, 2,]` followed by `3` becomes `[1, 2, 3]`.
            Some(separator) if separator.trailing_trivia().pieces().next().is_none() => (
                node,
                separator.with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            ),
            Some(separator) => (node, separator),
            // The space before the closing bracket in `{ foo: 1 }` belongs to the last node.
            // The replaced list gives it to the new last node.
            None => (
                node.trim_trailing_trivia()?,
                make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
            ),
        };
        nodes.push(node);
        separators.push(separator);
    }
    for (node, separator) in second {
        nodes.push(node);
        separators.extend(separator);
    }
    Some((nodes, separators))
}

/// Moves the assignment in `next_statement`, whose expression is `expression`, into `statement`,
/// the declaration of the variables whose declarators are set in `targets`.
fn merge_initialization(
    mutation: &mut BatchMutation<JsLanguage>,
    statement: &JsVariableStatement,
    next_statement: &JsExpressionStatement,
    expression: &AnyJsExpression,
    targets: u64,
) -> Option<()> {
    let declaration = statement.declaration().ok()?;
    let declarators = declaration.declarators();
    if declarators.syntax().has_comments_descendants() {
        return None;
    }
    let assignment = expression.as_js_assignment_expression()?;
    let value = assignment.right().ok()?;
    let initializer = make::js_initializer_clause(make::token_decorated_with_space(T![=]), value);
    let is_target = |index: usize| targets & (1u64 << index) != 0;

    let declarator = match assignment.left().ok()? {
        AnyJsAssignmentPattern::JsArrayAssignmentPattern(pattern) => {
            // A type annotation of a destructured variable can't be kept.
            if declarators.iter().enumerate().any(|(index, declarator)| {
                is_target(index)
                    && declarator.is_ok_and(|declarator| declarator.variable_annotation().is_some())
            }) {
                return None;
            }
            let elements = array_pattern_targets(&pattern)?
                .into_iter()
                .map(|target| {
                    let Some(target) = target else {
                        return Some(AnyJsArrayBindingPatternElement::JsArrayHole(
                            make::js_array_hole(),
                        ));
                    };
                    let binding = make::js_identifier_binding(target.name_token().ok()?);
                    Some(
                        AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                            make::js_array_binding_pattern_element(
                                AnyJsBindingPattern::AnyJsBinding(
                                    AnyJsBinding::JsIdentifierBinding(binding),
                                ),
                            )
                            .build(),
                        ),
                    )
                })
                .collect::<Option<Vec<_>>>()?;
            let separators = pattern
                .elements()
                .separators()
                .collect::<Result<Vec<_>, _>>()
                .ok()?;
            let pattern = make::js_array_binding_pattern(
                pattern.l_brack_token().ok()?,
                make::js_array_binding_pattern_element_list(elements, separators),
                pattern.r_brack_token().ok()?,
            )
            .trim_trivia()?;
            make::js_variable_declarator(AnyJsBindingPattern::JsArrayBindingPattern(pattern))
                .with_initializer(initializer)
                .build()
        }
        _ => {
            let declarator = declarators
                .iter()
                .nth(targets.trailing_zeros() as usize)?
                .ok()?;
            // `let foo!: number` says that the variable is assigned before it is read, and can't
            // have a value.
            if matches!(
                declarator.variable_annotation(),
                Some(AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(_))
            ) {
                return None;
            }
            declarator
                .trim_trivia()?
                .with_initializer(Some(initializer))
        }
    };

    let remaining = declarators
        .iter()
        .enumerate()
        .filter(|(index, _)| !is_target(*index))
        .map(|(_, declarator)| declarator.ok()?.trim_trivia())
        .collect::<Option<Vec<_>>>()?;
    if remaining.is_empty() {
        if !can_remove_after(
            &AnyJsVariableCreationStatement::JsVariableStatement(statement.clone()),
            next_statement,
        ) {
            return None;
        }
        mutation.replace_node(
            declarators,
            make::js_variable_declarator_list([declarator], []),
        );
        mutation.remove_node(next_statement.clone());
    } else {
        let separators = (1..remaining.len())
            .map(|_| make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]))
            .collect::<Vec<_>>();
        mutation.replace_node(
            declarators,
            make::js_variable_declarator_list(remaining, separators),
        );
        let kind = declaration.kind().ok()?;
        let kind =
            make::token(kind.kind()).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]);
        let new_declaration = make::js_variable_declaration(
            kind,
            make::js_variable_declarator_list([declarator], []),
        )
        .build();
        let mut new_statement = make::js_variable_statement(new_declaration);
        if let Some(semicolon) = next_statement.semicolon_token() {
            new_statement = new_statement.with_semicolon_token(semicolon);
        }
        mutation.replace_node(
            AnyJsStatement::JsExpressionStatement(next_statement.clone()),
            AnyJsStatement::JsVariableStatement(new_statement.build()),
        );
    }
    Some(())
}
