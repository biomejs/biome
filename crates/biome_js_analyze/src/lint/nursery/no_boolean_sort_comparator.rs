use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsFormalParameter,
    AnyJsFunction, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsMemberExpression,
    AnyJsParameter, AnyJsStatement, AnyTsCastExpression, AnyTsReturnType, AnyTsType,
    AnyTsTypeMember, JsBinaryOperator, JsCallExpression, JsReferenceIdentifier, JsSyntaxToken,
    JsUnaryOperator, OperatorPrecedence, T, TsTypeMemberList,
};
use biome_rowan::{AstNode, AstNodeList, AstSeparatedList, BatchMutationExt};
use biome_rule_options::no_boolean_sort_comparator::NoBooleanSortComparatorOptions;

use crate::{JsRuleAction, services::semantic::Semantic, utils::is_node_equal};

declare_lint_rule! {
    /// Disallow sort comparators that return a boolean.
    ///
    /// `Array#sort()` and `Array#toSorted()` accept a comparator: a function that receives two items and returns a number.
    /// A negative number puts the first item before the second, a positive number puts it after, and zero keeps their current order.
    ///
    /// When a comparator returns `true` or `false`, the sort method treats `true` as `1` and `false` as `0`.
    /// The comparator can then never say that the first item comes first, so the items aren't sorted correctly,
    /// and the result can differ between browsers and JavaScript runtimes.
    ///
    /// Return a number instead: subtract numbers from each other, or use `String#localeCompare()` for strings.
    ///
    /// This rule checks comparators whose body is a single expression or a single `return` statement,
    /// comparators with a `boolean` return type, and references to functions that return a boolean,
    /// such as `Boolean` or `Number.isNaN`. It doesn't check comparators that contain several `return` statements.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.sort((a, b) => a > b);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.toSorted((a, b) => a.score >= b.score);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// strings.sort((a, b) => a === b);
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// array.sort((a: number, b: number): boolean => compare(a, b));
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array.sort((a, b) => a - b);
    /// array.toSorted((a, b) => a.score - b.score);
    /// strings.sort((a, b) => a.localeCompare(b));
    /// array.sort((a, b) => (a > b ? 1 : a < b ? -1 : 0));
    /// ```
    ///
    pub NoBooleanSortComparator {
        version: "2.6.3",
        name: "noBooleanSortComparator",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::EslintUnicorn("no-boolean-sort-comparator").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoBooleanSortComparator {
    type Query = Semantic<JsCallExpression>;
    /// The expression returned by the comparator when the comparator is a function written
    /// inline without a type assertion, such as `(a, b) => a > b`. `None` for every other
    /// comparator.
    type State = Option<AnyJsExpression>;
    type Signals = Option<Self::State>;
    type Options = NoBooleanSortComparatorOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee =
            AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
        if !is_sort_method(&callee, model) {
            return None;
        }

        let state = boolean_comparator(&comparator(call)?, model)?;

        // Resolving the receiver is the least likely check to reject a call, so it runs last.
        if is_known_non_indexed_collection(&callee.object().ok()?, model, MAX_RESOLUTION_DEPTH) {
            return None;
        }

        Some(state)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let comparator = comparator(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                comparator.range(),
                markup! {
                    "This sort comparator returns a boolean instead of a number."
                },
            )
            .note(markup! {
                "The sort method treats "<Emphasis>"true"</Emphasis>" as "<Emphasis>"1"</Emphasis>" and "<Emphasis>"false"</Emphasis>" as "<Emphasis>"0"</Emphasis>", so the comparator can never put the first item before the second. The items aren't sorted correctly, and the result can differ between JavaScript engines."
            })
            .note(markup! {
                "Return a negative number, zero, or a positive number instead, such as "<Emphasis>"a - b"</Emphasis>" for numbers or "<Emphasis>"a.localeCompare(b)"</Emphasis>" for strings."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, return_expression: &Self::State) -> Option<JsRuleAction> {
        let comparison = return_expression
            .clone()?
            .omit_parentheses()
            .as_js_binary_expression()?
            .clone();
        let operator = comparison.operator().ok()?;
        // The replacement is built from the operands alone, so comments between them would be lost.
        if operator.precedence() != OperatorPrecedence::Relational
            || comparison.syntax().has_inner_comments()
        {
            return None;
        }

        let function =
            AnyJsFunction::cast(comparator(ctx.query())?.omit_parentheses().into_syntax())?;
        if function.type_parameters().is_some() || function.return_type_annotation().is_some() {
            return None;
        }
        let (first, second) = plain_parameter_names(&function)?;

        let left = comparison.left().ok()?.omit_parentheses();
        let right = comparison.right().ok()?.omit_parentheses();
        if !is_parameter_mirror(&left, &right, &first, &second) {
            return None;
        }

        let (minuend, subtrahend) = match operator {
            JsBinaryOperator::GreaterThan | JsBinaryOperator::GreaterThanOrEqual => (left, right),
            _ => (right, left),
        };
        let subtraction = make::js_binary_expression(
            minuend.trim_trivia()?,
            make::token_decorated_with_space(T![-]),
            subtrahend.trim_trivia()?,
        );

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::from(comparison),
            AnyJsExpression::from(subtraction),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Subtract the compared values instead." }.to_owned(),
            mutation,
        ))
    }
}

/// Bounds how many variables, functions, and type aliases a single check follows, so that
/// recursive definitions such as `function compare(a, b) { return compare(a, b); }` terminate.
const MAX_RESOLUTION_DEPTH: u8 = 8;

/// Global functions that always return a boolean.
const BOOLEAN_GLOBAL_FUNCTIONS: &[&str] = &["Boolean", "isFinite", "isNaN"];

/// Static methods of global objects that always return a boolean, sorted by object name,
/// then by method name.
const BOOLEAN_STATIC_METHODS: &[(&str, &str)] = &[
    ("Array", "isArray"),
    ("ArrayBuffer", "isView"),
    ("Atomics", "isLockFree"),
    ("Error", "isError"),
    ("Number", "isFinite"),
    ("Number", "isInteger"),
    ("Number", "isNaN"),
    ("Number", "isSafeInteger"),
    ("Object", "hasOwn"),
    ("Object", "is"),
    ("Object", "isExtensible"),
    ("Object", "isFrozen"),
    ("Object", "isSealed"),
    ("Reflect", "deleteProperty"),
    ("Reflect", "has"),
    ("URL", "canParse"),
];

/// Constructors whose instances have a `sort()` method that accepts a comparator, sorted.
const INDEXED_COLLECTION_CONSTRUCTORS: &[&str] = &[
    "Array",
    "BigInt64Array",
    "BigUint64Array",
    "Float16Array",
    "Float32Array",
    "Float64Array",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
];

/// Type names of built-in collections that have no `sort()` method, sorted.
const NON_INDEXED_COLLECTION_TYPES: &[&str] = &[
    "CanvasRenderingContext2D",
    "Map",
    "OffscreenCanvasRenderingContext2D",
    "ReadonlyMap",
    "ReadonlySet",
    "Set",
    "WeakMap",
    "WeakSet",
];

/// Returns `true` if `callee` reads the `sort` or `toSorted` property, either by name or
/// through a `const` variable that holds the name, as in `const method = "sort"; array[method]`.
fn is_sort_method(callee: &AnyJsMemberExpression, model: &SemanticModel) -> bool {
    let is_sort_name = |name: &str| matches!(name, "sort" | "toSorted");
    if let Some(name) = callee.member_name() {
        return is_sort_name(name.text());
    }
    let AnyJsMemberExpression::JsComputedMemberExpression(callee) = callee else {
        return false;
    };
    let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) = callee
        .member()
        .ok()
        .and_then(|member| member.omit_parentheses().as_js_reference_identifier())
        .and_then(|reference| model.binding(&reference))
        .and_then(|binding| binding.tree().declaration())
    else {
        return false;
    };
    declarator
        .declaration()
        .is_some_and(|declaration| declaration.is_const())
        && declarator
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .and_then(|value| value.omit_parentheses().as_static_value())
            .is_some_and(|value| is_sort_name(value.text()))
}

/// Returns the first argument of `call`, unless it's missing or a spread argument.
fn comparator(call: &JsCallExpression) -> Option<AnyJsExpression> {
    match call.arguments().ok()?.args().first()?.ok()? {
        AnyJsCallArgument::AnyJsExpression(expression) => Some(expression),
        AnyJsCallArgument::JsSpread(_) => None,
    }
}

/// Returns the rule state if `comparator` returns a boolean, or `None` if it doesn't or if
/// that can't be determined.
fn boolean_comparator(
    comparator: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<Option<AnyJsExpression>> {
    let unwrapped = comparator.clone().omit_type_wrappers()?;

    if let Some(function) = AnyJsFunction::cast_ref(unwrapped.syntax()) {
        if !is_boolean_function(&function, model, MAX_RESOLUTION_DEPTH)
            && !has_boolean_function_type_assertion(comparator, model)
        {
            return None;
        }
        let is_unwrapped = comparator.clone().omit_parentheses().syntax() == unwrapped.syntax();
        return Some(if is_unwrapped {
            return_expression(&function)
        } else {
            None
        });
    }

    (is_boolean_function_reference(&unwrapped, model, MAX_RESOLUTION_DEPTH)
        || is_known_boolean_function(&unwrapped, model)
        || has_boolean_function_type_assertion(comparator, model))
    .then_some(None)
}

/// Returns `true` if `expression` is wrapped in a type assertion to a function type that
/// returns a boolean, such as `compare as (a: number, b: number) => boolean`.
fn has_boolean_function_type_assertion(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    let mut expression = expression.clone();
    loop {
        let inner = match &expression {
            AnyJsExpression::JsParenthesizedExpression(inner) => inner.expression().ok(),
            AnyJsExpression::TsNonNullAssertionExpression(inner) => inner.expression().ok(),
            _ => {
                let Some(cast) = AnyTsCastExpression::cast_ref(expression.syntax()) else {
                    return false;
                };
                if cast
                    .cast_type()
                    .is_some_and(|ty| is_boolean_function_type(&ty, model, MAX_RESOLUTION_DEPTH))
                {
                    return true;
                }
                cast.inner_expression()
            }
        };
        let Some(inner) = inner else {
            return false;
        };
        expression = inner;
    }
}

/// Returns the expression that `function` returns when its body is a single expression or a
/// single `return` statement.
fn return_expression(function: &AnyJsFunction) -> Option<AnyJsExpression> {
    match function.body().ok()? {
        AnyJsFunctionBody::AnyJsExpression(expression) => Some(expression),
        AnyJsFunctionBody::JsFunctionBody(body) => {
            let statements = body.statements();
            if statements.len() != 1 {
                return None;
            }
            match statements.first()? {
                AnyJsStatement::JsReturnStatement(statement) => statement.argument(),
                _ => None,
            }
        }
    }
}

/// Returns `true` if `function` always returns a boolean, based on its return type or on the
/// expression it returns.
fn is_boolean_function(function: &AnyJsFunction, model: &SemanticModel, depth: u8) -> bool {
    if function.is_async() || function.is_generator() {
        return false;
    }
    if function
        .return_type_annotation()
        .and_then(|annotation| annotation.ty().ok())
        .is_some_and(|ty| is_boolean_return_type(&ty, model, depth))
    {
        return true;
    }
    return_expression(function)
        .is_some_and(|expression| is_boolean_expression(&expression, model, depth))
}

/// Returns `true` if `expression` always evaluates to a boolean.
fn is_boolean_expression(expression: &AnyJsExpression, model: &SemanticModel, depth: u8) -> bool {
    match expression {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsBooleanLiteralExpression(_),
        )
        | AnyJsExpression::JsInExpression(_)
        | AnyJsExpression::JsInstanceofExpression(_) => true,
        AnyJsExpression::JsUnaryExpression(unary) => matches!(
            unary.operator(),
            Ok(JsUnaryOperator::LogicalNot | JsUnaryOperator::Delete)
        ),
        AnyJsExpression::JsBinaryExpression(binary) => binary.operator().is_ok_and(|operator| {
            matches!(
                operator.precedence(),
                OperatorPrecedence::Relational | OperatorPrecedence::Equality
            )
        }),
        AnyJsExpression::JsLogicalExpression(logical) => {
            logical
                .left()
                .is_ok_and(|left| is_boolean_expression(&left, model, depth))
                && logical
                    .right()
                    .is_ok_and(|right| is_boolean_expression(&right, model, depth))
        }
        AnyJsExpression::JsConditionalExpression(conditional) => {
            conditional
                .consequent()
                .is_ok_and(|consequent| is_boolean_expression(&consequent, model, depth))
                && conditional
                    .alternate()
                    .is_ok_and(|alternate| is_boolean_expression(&alternate, model, depth))
        }
        AnyJsExpression::JsParenthesizedExpression(inner) => inner
            .expression()
            .is_ok_and(|inner| is_boolean_expression(&inner, model, depth)),
        AnyJsExpression::TsNonNullAssertionExpression(inner) => inner
            .expression()
            .is_ok_and(|inner| is_boolean_expression(&inner, model, depth)),
        AnyJsExpression::JsSequenceExpression(sequence) => sequence
            .right()
            .is_ok_and(|right| is_boolean_expression(&right, model, depth)),
        AnyJsExpression::JsAssignmentExpression(assignment) => {
            assignment
                .operator_token()
                .is_ok_and(|token| token.kind() == T![=])
                && assignment
                    .right()
                    .is_ok_and(|right| is_boolean_expression(&right, model, depth))
        }
        AnyJsExpression::TsAsExpression(_)
        | AnyJsExpression::TsSatisfiesExpression(_)
        | AnyJsExpression::TsTypeAssertionExpression(_) => {
            AnyTsCastExpression::cast_ref(expression.syntax()).is_some_and(|cast| {
                cast.cast_type()
                    .is_some_and(|ty| is_boolean_type(&ty, model, depth))
                    || cast
                        .inner_expression()
                        .is_some_and(|inner| is_boolean_expression(&inner, model, depth))
            })
        }
        AnyJsExpression::JsCallExpression(call) => call.callee().is_ok_and(|callee| {
            let callee = callee.omit_parentheses();
            is_known_boolean_function(&callee, model)
                || is_boolean_function_reference(&callee, model, depth)
        }),
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .is_ok_and(|reference| is_boolean_variable(&reference, model, depth)),
        _ => false,
    }
}

/// Returns `true` if `reference` refers to a variable or parameter that always holds a
/// boolean, based on its type annotation or on the value of a `const` declaration.
fn is_boolean_variable(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
    depth: u8,
) -> bool {
    let Some(depth) = depth.checked_sub(1) else {
        return false;
    };
    let Some(declaration) = model
        .binding(reference)
        .and_then(|binding| binding.tree().declaration())
    else {
        return false;
    };
    match declaration {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            if let Some(annotation) = declarator
                .variable_annotation()
                .and_then(|annotation| annotation.type_annotation().ok().flatten())
            {
                return annotation
                    .ty()
                    .is_ok_and(|ty| is_boolean_type(&ty, model, depth));
            }
            declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_const())
                && declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .is_some_and(|value| is_boolean_expression(&value, model, depth))
        }
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
            parameter.question_mark_token().is_none()
                && parameter
                    .type_annotation()
                    .and_then(|annotation| annotation.ty().ok())
                    .is_some_and(|ty| is_boolean_type(&ty, model, depth))
        }
        _ => false,
    }
}

/// Returns `true` if `expression` refers to a function declared in the file that always
/// returns a boolean, or to a variable or parameter whose type is such a function.
fn is_boolean_function_reference(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    depth: u8,
) -> bool {
    let Some(depth) = depth.checked_sub(1) else {
        return false;
    };
    let Some(binding) = expression
        .as_js_reference_identifier()
        .and_then(|reference| model.binding(&reference))
    else {
        return false;
    };
    let Some(declaration) = binding.tree().declaration() else {
        return false;
    };
    match declaration {
        AnyJsBindingDeclaration::JsFunctionDeclaration(_)
        | AnyJsBindingDeclaration::JsFunctionExpression(_) => {
            binding.all_writes().next().is_none()
                && AnyJsFunction::cast(declaration.into_syntax())
                    .is_some_and(|function| is_boolean_function(&function, model, depth))
        }
        AnyJsBindingDeclaration::TsDeclareFunctionDeclaration(function) => function
            .return_type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| is_boolean_return_type(&ty, model, depth)),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            if let Some(annotation) = declarator
                .variable_annotation()
                .and_then(|annotation| annotation.type_annotation().ok().flatten())
            {
                return annotation
                    .ty()
                    .is_ok_and(|ty| is_boolean_function_type(&ty, model, depth));
            }
            declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_const())
                && declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .and_then(|value| AnyJsFunction::cast(value.omit_parentheses().into_syntax()))
                    .is_some_and(|function| is_boolean_function(&function, model, depth))
        }
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter
            .type_annotation()
            .and_then(|annotation| annotation.ty().ok())
            .is_some_and(|ty| is_boolean_function_type(&ty, model, depth)),
        _ => false,
    }
}

/// Returns `true` if `expression` is a built-in function that always returns a boolean, such
/// as `Boolean` or `Number.isNaN`, and isn't shadowed by a local variable.
fn is_known_boolean_function(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let is_global = |reference: &JsReferenceIdentifier| model.binding(reference).is_none();

    if let Some(reference) = expression.as_js_reference_identifier() {
        return reference
            .value_token()
            .is_ok_and(|token| BOOLEAN_GLOBAL_FUNCTIONS.contains(&token.text_trimmed()))
            && is_global(&reference);
    }

    let Some(member) = AnyJsMemberExpression::cast_ref(expression.syntax()) else {
        return false;
    };
    if member.is_optional_chain() {
        return false;
    }
    let Some(object) = member
        .object()
        .ok()
        .and_then(|object| object.omit_parentheses().as_js_reference_identifier())
    else {
        return false;
    };
    let (Some(method), Ok(object_name)) = (member.member_name(), object.value_token()) else {
        return false;
    };
    BOOLEAN_STATIC_METHODS
        .binary_search(&(object_name.text_trimmed(), method.text()))
        .is_ok()
        && is_global(&object)
}

/// Returns `true` if a function with the return type `ty` always returns a boolean.
fn is_boolean_return_type(ty: &AnyTsReturnType, model: &SemanticModel, depth: u8) -> bool {
    match ty {
        AnyTsReturnType::AnyTsType(ty) => is_boolean_type(ty, model, depth),
        AnyTsReturnType::TsPredicateReturnType(_) => true,
        AnyTsReturnType::TsAssertsReturnType(_) => false,
    }
}

/// Returns `true` if `ty` only accepts booleans, such as `boolean`, `true | false`, or an
/// alias of one of them.
fn is_boolean_type(ty: &AnyTsType, model: &SemanticModel, depth: u8) -> bool {
    match ty {
        AnyTsType::TsBooleanType(_) | AnyTsType::TsBooleanLiteralType(_) => true,
        AnyTsType::TsParenthesizedType(inner) => inner
            .ty()
            .is_ok_and(|inner| is_boolean_type(&inner, model, depth)),
        AnyTsType::TsUnionType(union) => union
            .types()
            .iter()
            .all(|variant| variant.is_ok_and(|variant| is_boolean_type(&variant, model, depth))),
        AnyTsType::TsReferenceType(_) => {
            type_declaration(ty, model, depth).is_some_and(|(declaration, depth)| match declaration
            {
                AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias) => alias
                    .ty()
                    .is_ok_and(|aliased| is_boolean_type(&aliased, model, depth)),
                _ => false,
            })
        }
        _ => false,
    }
}

/// Returns `true` if `ty` is a function type that always returns a boolean, such as
/// `(a: number, b: number) => boolean`, or an alias or interface for one.
fn is_boolean_function_type(ty: &AnyTsType, model: &SemanticModel, depth: u8) -> bool {
    match ty {
        AnyTsType::TsFunctionType(function) => function
            .return_type()
            .is_ok_and(|return_type| is_boolean_return_type(&return_type, model, depth)),
        AnyTsType::TsObjectType(object) => {
            has_only_boolean_call_signatures(&object.members(), model, depth)
        }
        AnyTsType::TsParenthesizedType(inner) => inner
            .ty()
            .is_ok_and(|inner| is_boolean_function_type(&inner, model, depth)),
        AnyTsType::TsUnionType(union) => union.types().iter().all(|variant| {
            variant.is_ok_and(|variant| is_boolean_function_type(&variant, model, depth))
        }),
        AnyTsType::TsReferenceType(_) => {
            type_declaration(ty, model, depth).is_some_and(|(declaration, depth)| match declaration
            {
                AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias) => alias
                    .ty()
                    .is_ok_and(|aliased| is_boolean_function_type(&aliased, model, depth)),
                AnyJsBindingDeclaration::TsInterfaceDeclaration(interface) => {
                    has_only_boolean_call_signatures(&interface.members(), model, depth)
                }
                _ => false,
            })
        }
        _ => false,
    }
}

/// Returns `true` if `members` contain at least one call signature, and every call
/// signature returns a boolean.
fn has_only_boolean_call_signatures(
    members: &TsTypeMemberList,
    model: &SemanticModel,
    depth: u8,
) -> bool {
    let mut call_signatures = members
        .iter()
        .filter_map(|member| match member {
            AnyTsTypeMember::TsCallSignatureTypeMember(signature) => Some(signature),
            _ => None,
        })
        .peekable();
    call_signatures.peek().is_some()
        && call_signatures.all(|signature| {
            signature
                .return_type_annotation()
                .and_then(|annotation| annotation.ty().ok())
                .is_some_and(|ty| is_boolean_return_type(&ty, model, depth))
        })
}

/// Returns the declaration that the type reference `ty` refers to, together with the
/// remaining resolution depth.
fn type_declaration(
    ty: &AnyTsType,
    model: &SemanticModel,
    depth: u8,
) -> Option<(AnyJsBindingDeclaration, u8)> {
    let depth = depth.checked_sub(1)?;
    let reference = ty.as_ts_reference_type()?.name().ok()?;
    let declaration = model
        .binding(reference.as_js_reference_identifier()?)?
        .tree()
        .declaration()?;
    Some((declaration, depth))
}

/// Returns `true` if `object` is known not to be an array or a typed array, such as an
/// object literal, a `new Set()`, or a variable with the type `Map`.
fn is_known_non_indexed_collection(
    object: &AnyJsExpression,
    model: &SemanticModel,
    depth: u8,
) -> bool {
    let Some(object) = object.clone().omit_type_wrappers() else {
        return false;
    };
    match object {
        AnyJsExpression::JsObjectExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_) => true,
        AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
        AnyJsExpression::JsNewExpression(new) => new
            .callee()
            .ok()
            .and_then(|callee| callee.as_js_reference_identifier())
            .and_then(|callee| callee.value_token().ok())
            .is_none_or(|name| {
                INDEXED_COLLECTION_CONSTRUCTORS
                    .binary_search(&name.text_trimmed())
                    .is_err()
            }),
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let Some(depth) = depth.checked_sub(1) else {
                return false;
            };
            let Some(declaration) = identifier
                .name()
                .ok()
                .and_then(|reference| model.binding(&reference))
                .and_then(|binding| binding.tree().declaration())
            else {
                return false;
            };
            let annotation = match &declaration {
                AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => declarator
                    .variable_annotation()
                    .and_then(|annotation| annotation.type_annotation().ok().flatten()),
                AnyJsBindingDeclaration::JsFormalParameter(parameter) => {
                    parameter.type_annotation()
                }
                _ => None,
            };
            if let Some(annotation) = annotation {
                return annotation
                    .ty()
                    .ok()
                    .as_ref()
                    .and_then(AnyTsType::as_ts_reference_type)
                    .and_then(|reference| reference.name().ok())
                    .and_then(|name| name.as_js_reference_identifier()?.value_token().ok())
                    .is_some_and(|name| {
                        NON_INDEXED_COLLECTION_TYPES
                            .binary_search(&name.text_trimmed())
                            .is_ok()
                    });
            }
            let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
                return false;
            };
            declarator
                .declaration()
                .is_some_and(|declaration| declaration.is_const())
                && declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                    .is_some_and(|value| is_known_non_indexed_collection(&value, model, depth))
        }
        _ => false,
    }
}

/// Returns the names of the two parameters of `function` when both are plain identifiers
/// with distinct names, no type annotation, no default value, and no `?`.
fn plain_parameter_names(function: &AnyJsFunction) -> Option<(JsSyntaxToken, JsSyntaxToken)> {
    let parameters = function.parenthesized_parameters()?.items();
    if parameters.len() != 2 {
        return None;
    }
    let mut names = parameters.iter().map(|parameter| {
        let AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
            parameter,
        )) = parameter.ok()?
        else {
            return None;
        };
        if parameter.question_mark_token().is_some()
            || parameter.type_annotation().is_some()
            || parameter.initializer().is_some()
        {
            return None;
        }
        let AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) =
            parameter.binding().ok()?
        else {
            return None;
        };
        binding.name_token().ok()
    });
    let first = names.next()??;
    let second = names.next()??;
    (first.text_trimmed() != second.text_trimmed()).then_some((first, second))
}

/// Returns `true` if `left` and `right` read the same value from the two different
/// parameters, such as `a` and `b`, or `a.score` and `b.score`.
fn is_parameter_mirror(
    left: &AnyJsExpression,
    right: &AnyJsExpression,
    first: &JsSyntaxToken,
    second: &JsSyntaxToken,
) -> bool {
    if let (Some(left), Some(right)) = (
        left.as_js_reference_identifier(),
        right.as_js_reference_identifier(),
    ) {
        let (Ok(left), Ok(right)) = (left.value_token(), right.value_token()) else {
            return false;
        };
        let (left, right) = (left.text_trimmed(), right.text_trimmed());
        let (first, second) = (first.text_trimmed(), second.text_trimmed());
        return (left == first && right == second) || (left == second && right == first);
    }

    let (Some(left), Some(right)) = (
        AnyJsMemberExpression::cast_ref(left.syntax()),
        AnyJsMemberExpression::cast_ref(right.syntax()),
    ) else {
        return false;
    };
    let same_member = match (&left, &right) {
        (
            AnyJsMemberExpression::JsStaticMemberExpression(left),
            AnyJsMemberExpression::JsStaticMemberExpression(right),
        ) => left
            .member()
            .ok()
            .zip(right.member().ok())
            .is_some_and(|(left, right)| is_node_equal(left.syntax(), right.syntax())),
        (
            AnyJsMemberExpression::JsComputedMemberExpression(left),
            AnyJsMemberExpression::JsComputedMemberExpression(right),
        ) => left
            .member()
            .ok()
            .zip(right.member().ok())
            .is_some_and(|(left, right)| is_node_equal(left.syntax(), right.syntax())),
        _ => false,
    };
    same_member
        && left
            .object()
            .ok()
            .zip(right.object().ok())
            .is_some_and(|(left, right)| {
                is_parameter_mirror(
                    &left.omit_parentheses(),
                    &right.omit_parentheses(),
                    first,
                    second,
                )
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_static_methods_sorted() {
        for pair in BOOLEAN_STATIC_METHODS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{:?} must come after {:?}",
                pair[0],
                pair[1]
            );
        }
    }

    #[test]
    fn indexed_collection_constructors_sorted() {
        for pair in INDEXED_COLLECTION_CONSTRUCTORS.windows(2) {
            assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
        }
    }

    #[test]
    fn non_indexed_collection_types_sorted() {
        for pair in NON_INDEXED_COLLECTION_TYPES.windows(2) {
            assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
        }
    }
}
