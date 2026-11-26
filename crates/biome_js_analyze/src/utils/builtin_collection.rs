//! Recognizes values that are built-in `Set`, `Map`, `WeakSet`, or `WeakMap`
//! collections, from how they are created, declared, or asserted, without type inference.

use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsNewExpression, JsReferenceIdentifier, T, TsGlobalDeclaration,
    TsReferenceType, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList};

/// The kind of a built-in collection.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CollectionKind {
    Set,
    Map,
    WeakSet,
    WeakMap,
}

impl biome_console::fmt::Display for CollectionKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Set => "Set",
            Self::Map => "Map",
            Self::WeakSet => "WeakSet",
            Self::WeakMap => "WeakMap",
        })
    }
}

/// The maximum number of variables that are looked up to find the value of an expression.
const MAX_VARIABLE_LOOKUPS: u8 = 16;

/// Returns the kind of built-in collection that `expression` evaluates to, or `None` if
/// the kind can't be determined.
///
/// A value is recognized if it is created with `new Set()`, `new Map()`, `new WeakSet()`,
/// or `new WeakMap()`, or if it is
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
pub fn collection_kind(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<CollectionKind> {
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
/// `Set`, `Map`, `WeakSet`, or `WeakMap`.
pub fn constructed_collection_kind(
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
        "WeakSet" => Some(CollectionKind::WeakSet),
        "WeakMap" => Some(CollectionKind::WeakMap),
        _ => None,
    }
}

/// What a type tells about the kind of collection that a value is.
enum TypeCollectionKind {
    /// The type is a built-in collection type, or a union of types of the same kind,
    /// such as `Set<string> | ReadonlySet<number>`.
    Collection(CollectionKind),
    /// The type is a union of built-in `Set` and `Map` types.
    SetOrMap,
    /// The type, or a member of the union type, isn't a built-in collection type, or
    /// the type is a union of a `WeakSet` or `WeakMap` type with a type of another kind.
    Other,
}

/// Returns what `ty` tells about the kind of collection that a value is.
///
/// The built-in types are `Set`, `ReadonlySet`, `Map`, `ReadonlyMap`, `WeakSet`, and
/// `WeakMap`.
fn type_collection_kind(ty: AnyTsType, model: &SemanticModel) -> TypeCollectionKind {
    let mut kind = None;
    let mut has_mixed_kinds = false;
    let mut has_weak_kind = false;
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
        has_mixed_kinds |= kind.is_some_and(|kind| kind != found);
        has_weak_kind |= matches!(found, CollectionKind::WeakSet | CollectionKind::WeakMap);
        kind = Some(found);
    }
    match kind {
        Some(_) if has_mixed_kinds && has_weak_kind => TypeCollectionKind::Other,
        Some(_) if has_mixed_kinds => TypeCollectionKind::SetOrMap,
        Some(kind) => TypeCollectionKind::Collection(kind),
        None => TypeCollectionKind::Other,
    }
}

/// Returns the kind of collection that `reference` names, if it names one of the
/// built-in collection types.
fn builtin_collection_type(
    reference: &TsReferenceType,
    model: &SemanticModel,
) -> Option<CollectionKind> {
    let name = reference.name().ok()?;
    let name = name.as_js_reference_identifier()?;
    let kind = match name.value_token().ok()?.text_trimmed() {
        "Set" | "ReadonlySet" => CollectionKind::Set,
        "Map" | "ReadonlyMap" => CollectionKind::Map,
        "WeakSet" => CollectionKind::WeakSet,
        "WeakMap" => CollectionKind::WeakMap,
        _ => return None,
    };
    is_global_reference(name, model).then_some(kind)
}

/// Returns `true` if `reference` names a global value or type, and not one that the file
/// declares or imports.
///
/// A declaration in a `declare global` block describes the global itself.
pub fn is_global_reference(reference: &JsReferenceIdentifier, model: &SemanticModel) -> bool {
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
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            declarator.variable_annotation()?.type_annotation().ok()?
        }
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
