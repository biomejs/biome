//! Recognizes values that are known not to be arrays or typed arrays, so that rules about array
//! methods such as `sort()` or `some()` can skip calls on other objects with methods of the same name.

use biome_js_semantic::SemanticModel;
use biome_js_syntax::binding_ext::AnyJsBindingDeclaration;
use biome_js_syntax::{AnyJsExpression, AnyTsType};

/// Returns `true` if `object` is known not to be an array or a typed array, such as an
/// object literal, a `new Set()`, or a variable with the type `Map`.
pub(crate) fn is_known_non_indexed_collection(
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

/// Constructors of arrays and typed arrays, sorted.
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

/// Type names of built-in objects that aren't arrays or typed arrays, sorted.
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

#[cfg(test)]
mod tests {
    use super::*;

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
