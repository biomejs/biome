use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsArrowFunctionExpression, JsAssignmentExpression,
    JsAwaitExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsComputedMemberExpression, JsConstructorParameters, JsFormalParameter, JsFunctionBody,
    JsFunctionExpression, JsImportCallExpression, JsLanguage, JsNewExpression,
    JsParameters, JsParenthesizedExpression, JsPostUpdateExpression, JsPreUpdateExpression,
    JsReferenceIdentifier, JsStaticMemberExpression, JsSyntaxNode, JsSyntaxToken, JsTemplateExpression,
    JsUnaryExpression, JsUnaryOperator, JsYieldExpression, T, TsGlobalDeclaration,
    TsNonNullAssertionExpression, TsReferenceType, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_useless_set_construction::NoUselessSetConstructionOptions;

use crate::{JsRuleAction, ast_utils::starts_unterminated_statement, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow creating a `Set` that a `Set` method such as `union()` doesn't need.
    ///
    /// The `Set` methods `union()`, `intersection()`, `difference()`, `symmetricDifference()`, `isSubsetOf()`, `isSupersetOf()`, and `isDisjointFrom()` combine or compare two collections without changing either of them.
    /// Copying a value into a new `Set` right before or right after calling one of these methods does extra work, and creates a `Set` that is thrown away right after.
    ///
    /// The rule reports three kinds of unnecessary copies:
    ///
    /// - A copy of the argument, such as `a.union(new Set(b))`.
    ///   These methods read a `Set` directly, so `a.union(b)` gives the same result.
    ///   They also read a `Map` as the collection of its keys, so `a.union(new Set(map.keys()))` can be written as `a.union(map)`.
    /// - A copy of the `Set` that the method is called on, such as `new Set(a).union(b)`.
    ///   The method doesn't change `a`, so `a.union(b)` gives the same result.
    /// - A copy of the result, such as `new Set(a.union(b))`.
    ///   `union()`, `intersection()`, `difference()`, and `symmetricDifference()` already return a new `Set`.
    ///
    /// The rule only reports code when it can tell that the values are a `Set` or a `Map`. This is the case when a value is:
    ///
    /// - created with `new Set()` or `new Map()`;
    /// - a variable or a function parameter with the type `Set`, `ReadonlySet`, `Map`, or `ReadonlyMap`;
    /// - a `const` variable that is initialized with a `Set` or a `Map`, and that doesn't have one of these types;
    /// - marked as one of these types with `as` or `satisfies`;
    /// - a conditional expression (`condition ? a : b`) whose two results are both a `Set`, or both a `Map`.
    ///
    /// When a `const` variable, or a value marked with `as` or `satisfies`, has another type, such as `Iterable<string>`, the rule looks at the value itself.
    /// For example, `const values: Iterable<string> = new Set()` is a `Set`.
    ///
    /// A copy of the `Set` that the method is called on is kept when the argument contains a function call, an assignment, or another operation that can change the original `Set`.
    /// For example, `new Set(a).union(update(a))` reads the values that `a` had before `update(a)` ran.
    /// Reading a property and using a tagged template, such as ``tag`text` ``, don't count as such operations, although both can run code.
    ///
    /// The rule ignores a `new Set()` with explicit type arguments, such as `new Set<string>(values)`, because removing it can change the types of the code.
    /// For the same reason, it ignores a copy of the argument, and a copy of the `Set` that the method is called on, when `as` or `satisfies` is applied to the `new Set()`, as in `a.union(new Set(b) as Set<string>)`.
    ///
    /// :::caution
    /// The fix is unsafe because the rule trusts type annotations and `as` expressions without checking them.
    /// If a value isn't a `Set` or a `Map` when the code runs, the fixed code throws an error.
    /// This happens for an array that was cast with `as Set<string>`, and for an optional parameter, such as `values?: Set<string>`, that is `undefined`.
    /// The fixed code can also fail type checking when a value has a type that the method doesn't accept, such as `Iterable<string>`.
    /// The fixed code can also behave differently if a class that extends `Set` or `Map` replaces the methods that the copy reads, or if a property or a tagged template in the argument of the method runs code that changes the original `Set`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// selected.union(new Set(other));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const records = new Map([[2, "value"]]);
    /// selected.intersection(new Set(records.keys()));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// new Set(selected).isSubsetOf(other);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// const combined = new Set(selected.union(other));
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function overlap(selected: Set<number>, other: ReadonlySet<number>) {
    ///     return selected.intersection(new Set(other));
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const selected = new Set([1, 2]);
    /// const other = new Set([2, 3]);
    /// const records = new Map([[2, "value"]]);
    ///
    /// selected.union(other);
    /// selected.intersection(records);
    /// selected.isSubsetOf(other);
    /// const combined = selected.union(other);
    ///
    /// // A copy that isn't used with one of these methods is allowed.
    /// const copy = new Set(selected);
    ///
    /// // The values of a `Map` are different from its keys.
    /// selected.intersection(new Set(records.values()));
    ///
    /// // The rule can't tell whether `items` is a `Set`.
    /// selected.union(new Set(items));
    ///
    /// // `isSubsetOf()` returns a boolean, not a `Set`.
    /// const flags = new Set(selected.isSubsetOf(other));
    /// ```
    ///
    pub NoUselessSetConstruction {
        version: "next",
        name: "noUselessSetConstruction",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-set-construction").same()],
        recommended: true,
        severity: Severity::Warning,
        // Unsafe because the rule can't prove that a value with a `Set` or `Map` type
        // annotation is one of the built-in collections when the code runs, and because
        // a value with another type, such as `Iterable`, may not be accepted by the method.
        fix_kind: FixKind::Unsafe,
    }
}

/// A `new Set()` that only copies a value used with one of the [`SET_METHODS`].
pub struct UselessSetConstruction {
    position: SetPosition,
    /// The name of the `Set` method, such as `union`.
    method: JsSyntaxToken,
    /// The expression that can replace the `new Set()`.
    replacement: AnyJsExpression,
}

impl Rule for NoUselessSetConstruction {
    type Query = Semantic<JsNewExpression>;
    type State = UselessSetConstruction;
    type Signals = Option<Self::State>;
    type Options = NoUselessSetConstructionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if node.type_arguments().is_some()
            || constructed_collection_kind(node, model)? != CollectionKind::Set
        {
            return None;
        }
        let argument = single_argument(&node.arguments()?)?;

        if let AnyJsExpression::JsCallExpression(call) = argument.clone().omit_parentheses()
            && let Some(call) = SetMethodCall::from_call(&call)
            && call.returns_set()
            && collection_kind(&call.receiver, model) == Some(CollectionKind::Set)
        {
            return Some(UselessSetConstruction {
                position: SetPosition::Result,
                method: call.method,
                replacement: argument,
            });
        }

        let parent = parent_outside_parentheses(&AnyJsExpression::from(node.clone()))?;
        if let Some(arguments) = JsCallArgumentList::cast_ref(&parent) {
            let call = arguments
                .parent::<JsCallArguments>()?
                .parent::<JsCallExpression>()?;
            let call = SetMethodCall::from_call(&call)?;
            if collection_kind(&call.receiver, model)? != CollectionKind::Set {
                return None;
            }
            let (replacement, _) = copied_collection(&argument, model)?;
            return Some(UselessSetConstruction {
                position: SetPosition::Argument,
                method: call.method,
                replacement,
            });
        }

        // The object of a member expression is its only expression child, and so is the
        // callee of a call.
        let member = JsStaticMemberExpression::cast(parent)?;
        let call = JsCallExpression::cast(parent_outside_parentheses(&member.into())?)?;
        let call = SetMethodCall::from_call(&call)?;
        let (replacement, CollectionKind::Set) = copied_collection(&argument, model)? else {
            return None;
        };
        // The copy is made before the argument is evaluated, so it is only useless if the
        // argument can't change the original `Set`, neither while it is evaluated nor
        // while the method reads it.
        if collection_kind(&call.argument, model).is_none() || may_have_side_effects(&call.argument)
        {
            return None;
        }
        Some(UselessSetConstruction {
            position: SetPosition::Receiver,
            method: call.method,
            replacement,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"Set"</Emphasis>" copy isn't needed to use "<Emphasis>{state.method.text_trimmed()}"()"</Emphasis>"."
                },
            )
            .note(match state.position {
                SetPosition::Argument => markup! {
                    "The method reads a "<Emphasis>"Set"</Emphasis>", or the keys of a "<Emphasis>"Map"</Emphasis>", directly and doesn't change them. Creating another "<Emphasis>"Set"</Emphasis>" only takes extra time and memory."
                },
                SetPosition::Receiver => markup! {
                    "The method doesn't change the "<Emphasis>"Set"</Emphasis>" it is called on. Copying the "<Emphasis>"Set"</Emphasis>" first only takes extra time and memory."
                },
                SetPosition::Result => markup! {
                    "The method already returns a new "<Emphasis>"Set"</Emphasis>". Copying it only takes extra time and memory."
                },
            })
            .note(match state.position {
                SetPosition::Argument => markup! {
                    "Pass the original "<Emphasis>"Set"</Emphasis>" or "<Emphasis>"Map"</Emphasis>" to the method instead."
                },
                SetPosition::Receiver => markup! {
                    "Call the method on the original "<Emphasis>"Set"</Emphasis>" instead."
                },
                SetPosition::Result => markup! {
                    "Use the "<Emphasis>"Set"</Emphasis>" that the method returns instead."
                },
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        if has_comments_outside(node, &state.replacement) {
            return None;
        }

        let replacement = state.replacement.clone().trim_trivia()?;
        let needs_parentheses = match state.position {
            SetPosition::Argument => false,
            SetPosition::Receiver => needs_parentheses_as_object(&replacement),
            SetPosition::Result => {
                !matches!(replacement, AnyJsExpression::JsParenthesizedExpression(_))
                    && is_in_new_callee(node)
            }
        };
        let replacement = if needs_parentheses {
            make::parenthesized(replacement).into()
        } else {
            replacement
        };

        let node = AnyJsExpression::from(node.clone());
        let starts_with_parenthesis = replacement
            .syntax()
            .first_token()
            .is_some_and(|token| token.kind() == T!['(']);
        if starts_with_parenthesis && starts_unterminated_statement(&node) {
            return None;
        }

        let mut mutation = ctx.root().begin();
        mutation.replace_node(node, replacement);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove the unnecessary copy." },
            mutation,
        ))
    }
}

/// Where a useless `new Set()` is, relative to the call of the `Set` method.
#[derive(Clone, Copy)]
enum SetPosition {
    /// The argument of the method, as in `a.union(new Set(b))`.
    Argument,
    /// The object the method is called on, as in `new Set(a).union(b)`.
    Receiver,
    /// A copy of the value the method returns, as in `new Set(a.union(b))`.
    Result,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CollectionKind {
    Set,
    Map,
}

/// The `Set` methods that take another collection and don't change the `Set` they are
/// called on, sorted.
const SET_METHODS: &[&str] = &[
    "difference",
    "intersection",
    "isDisjointFrom",
    "isSubsetOf",
    "isSupersetOf",
    "symmetricDifference",
    "union",
];

/// The maximum number of variables that are looked up to find the value of an expression.
const MAX_VARIABLE_LOOKUPS: u8 = 16;

/// The nodes between a `new` keyword and an expression that the callee of `new` reads a
/// property from, when the expression is their first child.
const NEW_CALLEE_OBJECT_PARENT_KINDS: SyntaxKindSet<JsLanguage> =
    JsStaticMemberExpression::KIND_SET
        .union(JsTemplateExpression::KIND_SET)
        .union(TsNonNullAssertionExpression::KIND_SET);

/// The nodes that hold the parameters and the body of a function. This code runs when
/// the function is called, not when the expression that creates the function is evaluated.
const FUNCTION_KINDS: SyntaxKindSet<JsLanguage> = JsArrowFunctionExpression::KIND_SET
    .union(JsFunctionExpression::KIND_SET)
    .union(JsParameters::KIND_SET)
    .union(JsConstructorParameters::KIND_SET)
    .union(JsFormalParameter::KIND_SET)
    .union(JsFunctionBody::KIND_SET);

/// The nodes that [`may_have_side_effects`] looks for, and the [`FUNCTION_KINDS`] that it
/// skips. A unary expression only has a side effect if its operator is `delete`.
const SIDE_EFFECT_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsCallExpression::KIND_SET
    .union(JsNewExpression::KIND_SET)
    .union(JsImportCallExpression::KIND_SET)
    .union(JsAssignmentExpression::KIND_SET)
    .union(JsPreUpdateExpression::KIND_SET)
    .union(JsPostUpdateExpression::KIND_SET)
    .union(JsAwaitExpression::KIND_SET)
    .union(JsYieldExpression::KIND_SET)
    .union(JsUnaryExpression::KIND_SET)
    .union(FUNCTION_KINDS);

/// A call to one of the [`SET_METHODS`] with a single argument, such as `a.union(b)`.
struct SetMethodCall {
    /// The name of the method, such as `union`.
    method: JsSyntaxToken,
    /// The object the method is called on, such as `a` in `a.union(b)`.
    receiver: AnyJsExpression,
    argument: AnyJsExpression,
}

impl SetMethodCall {
    /// Returns `None` for optional calls such as `a?.union(b)`, computed methods such as
    /// `a["union"](b)`, and calls with a spread argument or with more or less than one
    /// argument.
    fn from_call(call: &JsCallExpression) -> Option<Self> {
        if call.is_optional() {
            return None;
        }
        let callee = call.callee().ok()?.omit_parentheses();
        let callee = callee.as_js_static_member_expression()?;
        if callee.is_optional() {
            return None;
        }
        let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
        SET_METHODS.binary_search(&method.text_trimmed()).ok()?;
        Some(Self {
            method,
            receiver: callee.object().ok()?,
            argument: single_argument(&call.arguments().ok()?)?,
        })
    }

    /// Returns `true` if the method returns a new `Set`, and `false` if it returns a boolean.
    fn returns_set(&self) -> bool {
        matches!(
            self.method.text_trimmed(),
            "difference" | "intersection" | "symmetricDifference" | "union"
        )
    }
}

/// Returns the only argument of a call, or `None` if the call has a spread argument or
/// more or less than one argument.
fn single_argument(arguments: &JsCallArguments) -> Option<AnyJsExpression> {
    let mut arguments = arguments.args().iter();
    let argument = arguments.next()?.ok()?;
    if arguments.next().is_some() {
        return None;
    }
    argument.as_any_js_expression().cloned()
}

/// Returns the parent of `expression`, ignoring the parentheses around `expression`.
fn parent_outside_parentheses(expression: &AnyJsExpression) -> Option<JsSyntaxNode> {
    expression
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !JsParenthesizedExpression::can_cast(ancestor.kind()))
}

/// Returns the `Set` or `Map` that `new Set(argument)` copies, and its kind.
///
/// The argument is either a `Set`, or a call that iterates the values of a `Set` or the
/// keys of a `Map`, such as `set.values()` or `map.keys()`. The entries of a `Set` and
/// the values and entries of a `Map` are different from what the methods read from the
/// collection itself.
fn copied_collection(
    argument: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<(AnyJsExpression, CollectionKind)> {
    if collection_kind(argument, model) == Some(CollectionKind::Set) {
        return Some((argument.clone(), CollectionKind::Set));
    }

    let call = argument.clone().omit_parentheses();
    let call = call.as_js_call_expression()?;
    if call.is_optional() || !call.arguments().ok()?.args().is_empty() {
        return None;
    }
    let callee = call.callee().ok()?.omit_parentheses();
    let callee = callee.as_js_static_member_expression()?;
    if callee.is_optional() {
        return None;
    }
    let method = callee.member().ok()?.as_js_name()?.value_token().ok()?;
    let object = callee.object().ok()?;
    let kind = collection_kind(&object, model)?;
    match (kind, method.text_trimmed()) {
        (CollectionKind::Set, "keys" | "values") | (CollectionKind::Map, "keys") => {
            Some((object, kind))
        }
        _ => None,
    }
}

/// Returns whether `expression` evaluates to a built-in `Set` or `Map`, or `None` if the
/// rule can't tell.
///
/// A value is recognized if it is created with `new Set()` or `new Map()`, or if it is
/// declared or asserted with a type that [`type_collection_kind`] recognizes. When a
/// `const` variable or an asserted expression has no such type, the value that the
/// variable is initialized with, or the expression itself, is checked instead:
///
/// ```ts
/// const values: Iterable<string> = new Set();
/// const sameValues = values as Iterable<string>;
/// ```
///
/// Both variables are a `Set`. A conditional expression is recognized if both of its
/// results are collections of the same kind.
fn collection_kind(expression: &AnyJsExpression, model: &SemanticModel) -> Option<CollectionKind> {
    let mut kind = None;
    let mut remaining_lookups = MAX_VARIABLE_LOOKUPS;
    let mut next = Some(expression.clone());
    // The second results of the conditional expressions whose first result is being checked.
    let mut pending = Vec::new();
    while let Some(expression) = next.take().or_else(|| pending.pop()) {
        // `ty` is the type that the expression is declared or asserted with. `value` is
        // the expression that produces the value when `found` is `None` and `ty` isn't
        // a collection type.
        let (found, ty, value) = match expression.omit_parentheses() {
            AnyJsExpression::JsNewExpression(new) => {
                (constructed_collection_kind(&new, model), None, None)
            }
            AnyJsExpression::TsAsExpression(assertion) => {
                (None, assertion.ty().ok(), assertion.expression().ok())
            }
            AnyJsExpression::TsTypeAssertionExpression(assertion) => {
                (None, assertion.ty().ok(), assertion.expression().ok())
            }
            AnyJsExpression::TsSatisfiesExpression(assertion) => {
                (None, assertion.ty().ok(), assertion.expression().ok())
            }
            AnyJsExpression::TsNonNullAssertionExpression(assertion) => {
                (None, None, assertion.expression().ok())
            }
            AnyJsExpression::JsConditionalExpression(conditional) => {
                pending.push(conditional.alternate().ok()?);
                (None, None, conditional.consequent().ok())
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                remaining_lookups = remaining_lookups.checked_sub(1)?;
                let declaration = model
                    .binding(&identifier.name().ok()?)?
                    .tree()
                    .declaration()?;
                (
                    None,
                    annotated_type(&declaration),
                    const_initializer(&declaration),
                )
            }
            _ => return None,
        };
        let found = match ty.map(|ty| type_collection_kind(ty, model)) {
            Some(TypeCollectionKind::Collection(found)) => Some(found),
            Some(TypeCollectionKind::SetOrMap) => return None,
            Some(TypeCollectionKind::Other) | None => found,
        };
        let Some(found) = found else {
            next = Some(value?);
            continue;
        };
        if kind.is_some_and(|kind| kind != found) {
            return None;
        }
        kind = Some(found);
    }
    kind
}

/// Returns the kind of collection that `new` creates, if its callee is the built-in
/// `Set` or `Map`.
fn constructed_collection_kind(
    new: &JsNewExpression,
    model: &SemanticModel,
) -> Option<CollectionKind> {
    let callee = new.callee().ok()?.omit_parentheses();
    let name = callee.as_js_identifier_expression()?.name().ok()?;
    if !is_global_reference(&name, model) {
        return None;
    }
    match name.value_token().ok()?.text_trimmed() {
        "Set" => Some(CollectionKind::Set),
        "Map" => Some(CollectionKind::Map),
        _ => None,
    }
}

/// What a type tells about the kind of collection that a value is.
enum TypeCollectionKind {
    /// The type is a built-in `Set` type, a built-in `Map` type, or a union of types of
    /// the same kind, such as `Set<string> | ReadonlySet<number>`.
    Collection(CollectionKind),
    /// The type is a union of built-in `Set` and `Map` types.
    SetOrMap,
    /// The type, or a member of the union type, isn't a built-in `Set` or `Map` type.
    Other,
}

/// Returns what `ty` tells about the kind of collection that a value is.
///
/// The built-in types are `Set`, `ReadonlySet`, `Map`, and `ReadonlyMap`.
fn type_collection_kind(ty: AnyTsType, model: &SemanticModel) -> TypeCollectionKind {
    let mut kind = None;
    let mut has_both_kinds = false;
    let mut next = Some(ty);
    // The members of the union types that are still to check.
    let mut pending = Vec::new();
    while let Some(ty) = next.take().or_else(|| pending.pop()) {
        let found = match ty.omit_parentheses() {
            AnyTsType::TsReferenceType(reference) => builtin_collection_type(&reference, model),
            AnyTsType::TsUnionType(union) => {
                for member in union.types().iter() {
                    let Ok(member) = member else {
                        return TypeCollectionKind::Other;
                    };
                    pending.push(member);
                }
                continue;
            }
            AnyTsType::TsTypeOperatorType(operator)
                if operator
                    .operator_token()
                    .is_ok_and(|token| token.kind() == T![readonly]) =>
            {
                let Ok(ty) = operator.ty() else {
                    return TypeCollectionKind::Other;
                };
                next = Some(ty);
                continue;
            }
            _ => None,
        };
        let Some(found) = found else {
            return TypeCollectionKind::Other;
        };
        has_both_kinds |= kind.is_some_and(|kind| kind != found);
        kind = Some(found);
    }
    match kind {
        Some(_) if has_both_kinds => TypeCollectionKind::SetOrMap,
        Some(kind) => TypeCollectionKind::Collection(kind),
        None => TypeCollectionKind::Other,
    }
}

/// Returns the kind of collection that `reference` names, if it names one of the
/// built-in `Set` and `Map` types.
fn builtin_collection_type(
    reference: &TsReferenceType,
    model: &SemanticModel,
) -> Option<CollectionKind> {
    let name = reference.name().ok()?;
    let name = name.as_js_reference_identifier()?;
    let kind = match name.value_token().ok()?.text_trimmed() {
        "Set" | "ReadonlySet" => CollectionKind::Set,
        "Map" | "ReadonlyMap" => CollectionKind::Map,
        _ => return None,
    };
    is_global_reference(name, model).then_some(kind)
}

/// Returns `true` if `reference` names a global value or type, and not one that the file
/// declares or imports.
///
/// A declaration in a `declare global` block describes the global itself.
fn is_global_reference(reference: &JsReferenceIdentifier, model: &SemanticModel) -> bool {
    model.binding(reference).is_none_or(|binding| {
        binding
            .syntax()
            .ancestors()
            .any(|ancestor| TsGlobalDeclaration::can_cast(ancestor.kind()))
    })
}

/// Returns the type that a variable or a function parameter is declared with.
///
/// The type of an optional parameter, such as `set?: Set<string>`, is returned as it is
/// written, although the parameter can also be `undefined`.
fn annotated_type(declaration: &AnyJsBindingDeclaration) -> Option<AnyTsType> {
    let annotation = match declaration {
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter.type_annotation(),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => parameter
            .formal_parameter()
            .ok()?
            .as_js_formal_parameter()?
            .type_annotation(),
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => declarator
            .variable_annotation()?
            .type_annotation()
            .ok()?,
        _ => None,
    }?;
    annotation.ty().ok()
}

/// Returns the value that a `const` variable is initialized with.
///
/// The value of a `let` or `var` variable isn't returned, because the variable can be
/// given another value later.
fn const_initializer(declaration: &AnyJsBindingDeclaration) -> Option<AnyJsExpression> {
    let AnyJsBindingDeclaration::JsVariableDeclarator(declarator) = declaration else {
        return None;
    };
    if !declarator.declaration()?.is_const() {
        return None;
    }
    declarator.initializer()?.expression().ok()
}

/// Returns `true` if evaluating `expression` may do more than produce a value, such as
/// calling a function or assigning to a variable.
///
/// Reading a property and evaluating a template literal are assumed to have no side
/// effects. The parameters and the body of a function that `expression` creates aren't
/// searched, because creating a function doesn't run it.
fn may_have_side_effects(expression: &AnyJsExpression) -> bool {
    let mut preorder = expression
        .syntax()
        .preorder_of_kinds(SIDE_EFFECT_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if FUNCTION_KINDS.matches(node.kind()) {
            preorder.skip_subtree();
        } else if JsUnaryExpression::cast(node)
            .is_none_or(|unary| unary.operator() == Ok(JsUnaryOperator::Delete))
        {
            return true;
        }
    }
    false
}

/// Returns `true` if a comment inside `node` is outside of `replacement`, so the fix
/// would remove it.
fn has_comments_outside(node: &JsNewExpression, replacement: &AnyJsExpression) -> bool {
    let node_range = node.syntax().text_trimmed_range();
    let replacement_range = replacement.syntax().text_trimmed_range();
    node.syntax().borrowed_tokens().any(|token| {
        token
            .leading_trivia_pieces()
            .chain(token.trailing_trivia_pieces())
            .any(|piece| {
                piece.kind().is_comment()
                    && node_range.contains_range(piece.text_range())
                    && !replacement_range.contains_range(piece.text_range())
            })
    })
}

/// Returns `true` if `object` must be wrapped in parentheses before a property can be
/// read from it.
///
/// `new Set` without arguments would read the property from `Set` itself, and
/// `a ? b : c` would read the property from `c` only.
fn needs_parentheses_as_object(object: &AnyJsExpression) -> bool {
    match object {
        AnyJsExpression::JsIdentifierExpression(_)
        | AnyJsExpression::JsParenthesizedExpression(_)
        | AnyJsExpression::TsNonNullAssertionExpression(_) => false,
        AnyJsExpression::JsNewExpression(new) => new.arguments().is_none(),
        _ => true,
    }
}

/// Returns `true` if `node` is the object that a `new` callee reads a property from,
/// such as in `new new Set(a.union(b)).constructor()`, or if it is the callee itself.
///
/// A call in that place needs parentheses, because `new a.union(b).constructor()` calls
/// `a.union` with `new`.
fn is_in_new_callee(node: &JsNewExpression) -> bool {
    let mut child = node.syntax().clone();
    for ancestor in node.syntax().ancestors().skip(1) {
        // The arguments of `new` are wrapped in another node, so `child` is the callee.
        if JsNewExpression::can_cast(ancestor.kind()) {
            return true;
        }
        let is_object = match JsComputedMemberExpression::cast_ref(&ancestor) {
            Some(member) => member
                .object()
                .is_ok_and(|object| object.syntax() == &child),
            None => NEW_CALLEE_OBJECT_PARENT_KINDS.matches(ancestor.kind()),
        };
        if !is_object {
            return false;
        }
        child = ancestor;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_methods_sorted() {
        for pair in SET_METHODS.windows(2) {
            assert!(pair[0] < pair[1], "{} must come after {}", pair[0], pair[1]);
        }
    }
}
