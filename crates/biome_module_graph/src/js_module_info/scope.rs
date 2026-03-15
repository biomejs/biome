use std::{iter::FusedIterator, sync::Arc};

use biome_js_semantic::{BindingId, ScopeId, TsBindingReference};
use biome_js_syntax::TextRange;
use biome_js_type_info::TypeReferenceQualifier;
use biome_rowan::TokenText;
use rustc_hash::FxHashMap;

use super::{JsModuleInfoInner, binding::JsBinding};

#[derive(Debug)]
pub struct JsScopeData {
    // The scope range
    #[expect(dead_code, reason = "May be used in future for scope analysis")]
    pub range: TextRange,
    // The parent scope of this scope
    pub parent: Option<ScopeId>,
    // All children scope of this scope
    pub children: Vec<ScopeId>,
    // All bindings of this scope (points to SemanticModelData::bindings)
    pub bindings: Vec<BindingId>,
    // Map pointing to the [bindings] vec of each bindings by its name
    pub bindings_by_name: FxHashMap<TokenText, TsBindingReference>,
}

/// Extension trait for `TsBindingReference` that adds methods depending on
/// `biome_js_type_info` types, which are not available in `biome_js_semantic`.
pub(crate) trait TsBindingReferenceExt {
    /// Returns the binding ID that matches the given type reference qualifier,
    /// taking into account type-only lookups and excluded binding IDs.
    fn get_binding_id_for_qualifier(self, qualifier: &TypeReferenceQualifier) -> Option<BindingId>;
}

impl TsBindingReferenceExt for TsBindingReference {
    fn get_binding_id_for_qualifier(self, qualifier: &TypeReferenceQualifier) -> Option<BindingId> {
        if let Some(excluded_binding_id) = qualifier.excluded_binding_id {
            match self {
                Self::Type(binding_id)
                | Self::ValueType(binding_id)
                | Self::TypeAndValueType(binding_id)
                | Self::NamespaceAndValueType(binding_id) => {
                    (binding_id != excluded_binding_id).then_some(binding_id)
                }
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                } => match (ty, value_ty, namespace_ty) {
                    (_, _, Some(namespace_ty)) if namespace_ty != excluded_binding_id => {
                        Some(namespace_ty)
                    }
                    (_, Some(value_ty), _)
                        if value_ty != excluded_binding_id && !qualifier.type_only =>
                    {
                        Some(value_ty)
                    }
                    (Some(ty), _, _) if ty != excluded_binding_id => Some(ty),
                    _ => None,
                },
            }
        } else if qualifier.type_only {
            self.namespace_ty_or_ty()
        } else {
            Some(self.value_ty_or_ty())
        }
    }
}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
#[derive(Clone, Debug)]
pub struct JsScope {
    pub(crate) info: Arc<JsModuleInfoInner>,
    pub(crate) scope: biome_js_semantic::Scope,
}

impl PartialEq for JsScope {
    fn eq(&self, other: &Self) -> bool {
        self.scope == other.scope && Arc::ptr_eq(&self.info, &other.info)
    }
}

impl Eq for JsScope {}

impl JsScope {
    pub fn is_global_scope(&self) -> bool {
        self.scope.is_global_scope()
    }

    /// Returns all parents of this scope. Starting with the current
    /// [JsScope].
    pub fn ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        let info = self.info.clone();
        self.scope.ancestors().map(move |scope| Self {
            info: info.clone(),
            scope,
        })
    }

    /// Returns all descendents of this scope in breadth-first order. Starting
    /// with the current [JsScope].
    pub fn descendents(&self) -> impl Iterator<Item = Self> + use<> {
        let info = self.info.clone();
        self.scope.descendents().map(move |scope| Self {
            info: info.clone(),
            scope,
        })
    }

    /// Returns this scope parent.
    pub fn parent(&self) -> Option<Self> {
        self.scope.parent().map(|scope| Self {
            info: self.info.clone(),
            scope,
        })
    }

    /// Returns all bindings that were bound in this scope.
    ///
    /// It **does not** return bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            info: self.info.clone(),
            semantic_bindings: self.scope.bindings(),
        }
    }

    /// Checks if the current scope is an ancestor of `other`.
    ///
    /// Given that [Self::ancestors()] returns `self` as the first scope,
    /// the following snippet always returns `true`:
    ///
    /// ```rust,ignore
    /// assert!(scope.is_ancestor_of(scope));
    /// ```
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        self.scope.is_ancestor_of(&other.scope)
    }

    pub fn range(&self) -> TextRange {
        self.scope.range()
    }
}

/// Iterates all bindings that were bound in a given scope.
///
/// It **does not** return bindings of parent scopes.
pub struct ScopeBindingsIter {
    info: Arc<JsModuleInfoInner>,
    semantic_bindings: biome_js_semantic::ScopeBindingsIter,
}

impl Iterator for ScopeBindingsIter {
    type Item = JsBinding;

    fn next(&mut self) -> Option<Self::Item> {
        let semantic_binding = self.semantic_bindings.next()?;
        Some(JsBinding::from_semantic_binding(
            self.info.clone(),
            semantic_binding,
        ))
    }
}

impl ExactSizeIterator for ScopeBindingsIter {
    fn len(&self) -> usize {
        self.semantic_bindings.len()
    }
}

impl FusedIterator for ScopeBindingsIter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binding_reference_merging() {
        assert_eq!(
            TsBindingReference::Type(BindingId::new(0))
                .union_with(TsBindingReference::Type(BindingId::new(1))),
            TsBindingReference::Type(BindingId::new(1))
        );

        assert_eq!(
            TsBindingReference::Type(BindingId::new(0))
                .union_with(TsBindingReference::ValueType(BindingId::new(0))),
            TsBindingReference::TypeAndValueType(BindingId::new(0))
        );
        assert_eq!(
            TsBindingReference::Type(BindingId::new(0))
                .union_with(TsBindingReference::ValueType(BindingId::new(1))),
            TsBindingReference::Merged {
                ty: Some(BindingId::new(0)),
                value_ty: Some(BindingId::new(1)),
                namespace_ty: None
            }
        );
        assert_eq!(
            TsBindingReference::Type(BindingId::new(0))
                .union_with(TsBindingReference::NamespaceAndValueType(BindingId::new(0))),
            TsBindingReference::Merged {
                ty: Some(BindingId::new(0)),
                value_ty: Some(BindingId::new(0)),
                namespace_ty: Some(BindingId::new(0)),
            }
        );

        assert_eq!(
            TsBindingReference::ValueType(BindingId::new(0))
                .union_with(TsBindingReference::Type(BindingId::new(1))),
            TsBindingReference::Merged {
                ty: Some(BindingId::new(1)),
                value_ty: Some(BindingId::new(0)),
                namespace_ty: None
            }
        );

        assert_eq!(
            TsBindingReference::ValueType(BindingId::new(0))
                .union_with(TsBindingReference::ValueType(BindingId::new(0))),
            TsBindingReference::ValueType(BindingId::new(0))
        );
        assert_eq!(
            TsBindingReference::ValueType(BindingId::new(0))
                .union_with(TsBindingReference::ValueType(BindingId::new(1))),
            TsBindingReference::ValueType(BindingId::new(1))
        );
        assert_eq!(
            TsBindingReference::ValueType(BindingId::new(0))
                .union_with(TsBindingReference::NamespaceAndValueType(BindingId::new(0))),
            TsBindingReference::NamespaceAndValueType(BindingId::new(0))
        );
    }
}
