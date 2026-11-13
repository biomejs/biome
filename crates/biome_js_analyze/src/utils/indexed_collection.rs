//! Recognizes values that are known not to be arrays or typed arrays, so that rules about array
//! methods such as `sort()` or `forEach()` can skip calls on other objects with methods of the
//! same name.
//!
//! The checks look at the syntax of a value, the type annotation and `const` initial value of a
//! variable, and type aliases declared in the file. They don't use type inference. A `false`
//! result therefore means "unknown", not "array".

use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsLanguage, TsBigintLiteralType, TsBigintType,
    TsBooleanLiteralType, TsBooleanType, TsConstructorType, TsFunctionType, TsNeverType,
    TsNullLiteralType, TsNumberLiteralType, TsNumberType, TsStringLiteralType, TsStringType,
    TsSymbolType, TsTemplateLiteralType, TsTypeAnnotation, TsUndefinedType, TsVoidType,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet};

/// Returns `true` if `expression` is known not to be an array or a typed array.
///
/// This recognizes literals, object literals, functions, classes, untagged template strings,
/// `new` expressions that don't create an array or a typed array, and variables whose type
/// annotation or `const` initial value is one of those. Type annotations are recognized when they
/// name a built-in type that isn't an array, such as `Map<K, V>` or `string`, or a union of such
/// types.
pub(crate) fn is_known_non_indexed_collection(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    Resolver {
        model,
        numbers_only: false,
    }
    .expression(expression, MAX_RESOLUTION_DEPTH)
}

/// Like [`is_known_non_indexed_collection`], but also returns `true` for `BigInt64Array` and
/// `BigUint64Array` values, whose items are `bigint` values instead of numbers, and for variables
/// whose type annotation is a union that includes one of them.
pub(crate) fn is_known_non_numeric_indexed_collection(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    Resolver {
        model,
        numbers_only: true,
    }
    .expression(expression, MAX_RESOLUTION_DEPTH)
}

/// Returns `true` if the variable or parameter declared by `binding` is known not to hold an
/// array or a typed array, following the rules of [`is_known_non_indexed_collection`].
pub(crate) fn is_known_non_indexed_collection_binding(
    binding: &Binding,
    model: &SemanticModel,
) -> bool {
    Resolver {
        model,
        numbers_only: false,
    }
    .binding(binding, MAX_RESOLUTION_DEPTH)
}

/// Constructors of typed arrays, sorted.
pub(crate) const TYPED_ARRAY_CONSTRUCTORS: &[&str] = &[
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

/// How many variables and type aliases are followed before giving up.
const MAX_RESOLUTION_DEPTH: u8 = 8;

/// Type names of built-in objects that aren't arrays or typed arrays, sorted.
const NON_INDEXED_COLLECTION_TYPES: &[&str] = &[
    "CanvasRenderingContext2D",
    "FormData",
    "Headers",
    "Map",
    "OffscreenCanvasRenderingContext2D",
    "ReadonlyMap",
    "ReadonlySet",
    "Set",
    "URLSearchParams",
    "WeakMap",
    "WeakSet",
];

/// Types whose values are neither arrays nor typed arrays.
///
/// Object types such as `{ length: number }` are absent because an array can match them.
const NON_INDEXED_TYPE_KINDS: SyntaxKindSet<JsLanguage> = TsStringType::KIND_SET
    .union(TsNumberType::KIND_SET)
    .union(TsBooleanType::KIND_SET)
    .union(TsBigintType::KIND_SET)
    .union(TsSymbolType::KIND_SET)
    .union(TsNeverType::KIND_SET)
    .union(TsVoidType::KIND_SET)
    .union(TsUndefinedType::KIND_SET)
    .union(TsNullLiteralType::KIND_SET)
    .union(TsStringLiteralType::KIND_SET)
    .union(TsNumberLiteralType::KIND_SET)
    .union(TsBooleanLiteralType::KIND_SET)
    .union(TsBigintLiteralType::KIND_SET)
    .union(TsTemplateLiteralType::KIND_SET)
    .union(TsFunctionType::KIND_SET)
    .union(TsConstructorType::KIND_SET);

#[derive(Clone, Copy)]
struct Resolver<'a> {
    model: &'a SemanticModel,
    /// Whether typed arrays of `bigint` values count as non-indexed collections.
    numbers_only: bool,
}

impl Resolver<'_> {
    fn expression(&self, expression: &AnyJsExpression, depth: u8) -> bool {
        let Some(expression) = expression.clone().omit_type_wrappers() else {
            return false;
        };
        match expression {
            AnyJsExpression::AnyJsLiteralExpression(_)
            | AnyJsExpression::JsObjectExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)
            | AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsClassExpression(_) => true,
            // A tagged template can return anything, but a plain template is always a string.
            AnyJsExpression::JsTemplateExpression(template) => template.tag().is_none(),
            AnyJsExpression::JsNewExpression(new) => new
                .callee()
                .ok()
                .and_then(|callee| callee.omit_parentheses().as_js_reference_identifier())
                .and_then(|callee| callee.value_token().ok())
                .is_none_or(|name| !self.is_indexed_constructor(name.text_trimmed())),
            // `BigInt64Array.from(values)` and `BigInt64Array.of(...values)`
            AnyJsExpression::JsCallExpression(call) => call
                .callee()
                .ok()
                .and_then(|callee| {
                    let callee = callee.omit_parentheses();
                    let member = callee.as_js_static_member_expression()?;
                    let method = member.member().ok()?.as_js_name()?.value_token().ok()?;
                    if !matches!(method.text_trimmed(), "from" | "of") {
                        return None;
                    }
                    member
                        .object()
                        .ok()?
                        .omit_parentheses()
                        .as_js_reference_identifier()?
                        .value_token()
                        .ok()
                })
                .is_some_and(|name| {
                    let name = name.text_trimmed();
                    is_typed_array_constructor(name) && !self.is_indexed_constructor(name)
                }),
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let Some(depth) = depth.checked_sub(1) else {
                    return false;
                };
                identifier
                    .name()
                    .ok()
                    .and_then(|reference| self.model.binding(&reference))
                    .is_some_and(|binding| self.binding(&binding, depth))
            }
            _ => false,
        }
    }

    /// An explicit type annotation decides the result on its own, even when a `const` initial
    /// value is also known, because the annotation states what the variable is meant to hold.
    fn binding(&self, binding: &Binding, depth: u8) -> bool {
        if let Some(annotation) = binding_type_annotation(binding) {
            return annotation.ty().is_ok_and(|ty| self.ty(&ty, depth));
        }
        let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
            binding.tree().declaration()
        else {
            return false;
        };
        // Only a `const` is known to still hold its initial value.
        declarator
            .declaration()
            .is_some_and(|declaration| declaration.is_const())
            && declarator
                .initializer()
                .and_then(|initializer| initializer.expression().ok())
                .is_some_and(|value| self.expression(&value, depth))
    }

    /// Returns `true` if `ty` names a type that is neither an array nor a typed array, or a union
    /// whose members all do. Type aliases declared in the file are followed up to `depth` times;
    /// any other type declared in the file is unknown.
    fn ty(&self, ty: &AnyTsType, depth: u8) -> bool {
        if NON_INDEXED_TYPE_KINDS.matches(ty.syntax().kind()) {
            return true;
        }
        match ty {
            AnyTsType::TsReferenceType(reference) => {
                let Some(name) = reference
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_reference_identifier().cloned())
                else {
                    return false;
                };
                let Some(binding) = self.model.binding(&name) else {
                    return name.value_token().is_ok_and(|name| {
                        let name = name.text_trimmed();
                        NON_INDEXED_COLLECTION_TYPES.binary_search(&name).is_ok()
                            || (is_typed_array_constructor(name)
                                && !self.is_indexed_constructor(name))
                    });
                };
                let Some(depth) = depth.checked_sub(1) else {
                    return false;
                };
                let Some(AnyJsBindingDeclaration::TsTypeAliasDeclaration(alias)) =
                    binding.tree().declaration()
                else {
                    return false;
                };
                alias.ty().is_ok_and(|ty| self.ty(&ty, depth))
            }
            AnyTsType::TsUnionType(union) => {
                union
                    .types()
                    .iter()
                    .all(|member| member.is_ok_and(|member| self.ty(&member, depth)))
                    // A union that includes a typed array of `bigint` values may hold `bigint`
                    // items even if its other members are arrays of numbers.
                    || (self.numbers_only
                        && union.types().iter().any(|member| {
                            member.is_ok_and(|member| {
                                self.ty(&member, depth)
                                    && !Self {
                                        numbers_only: false,
                                        ..*self
                                    }
                                    .ty(&member, depth)
                            })
                        }))
            }
            AnyTsType::TsParenthesizedType(parenthesized) => {
                parenthesized.ty().is_ok_and(|inner| self.ty(&inner, depth))
            }
            _ => false,
        }
    }

    /// Returns `true` if `name` is the constructor of an array or a typed array that this
    /// resolver accepts.
    fn is_indexed_constructor(&self, name: &str) -> bool {
        name == "Array"
            || (is_typed_array_constructor(name)
                && !(self.numbers_only && matches!(name, "BigInt64Array" | "BigUint64Array")))
    }
}

/// Returns the type annotation of the variable or parameter declared by `binding`, including a
/// constructor parameter property such as `constructor(private names: string[])`.
pub(crate) fn binding_type_annotation(binding: &Binding) -> Option<TsTypeAnnotation> {
    match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => declarator
            .variable_annotation()?
            .type_annotation()
            .ok()
            .flatten(),
        AnyJsBindingDeclaration::JsFormalParameter(parameter) => parameter.type_annotation(),
        AnyJsBindingDeclaration::TsPropertyParameter(parameter) => parameter
            .formal_parameter()
            .ok()?
            .as_js_formal_parameter()?
            .type_annotation(),
        _ => None,
    }
}

fn is_typed_array_constructor(name: &str) -> bool {
    TYPED_ARRAY_CONSTRUCTORS.binary_search(&name).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_array_constructors_sorted() {
        for pair in TYPED_ARRAY_CONSTRUCTORS.windows(2) {
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
