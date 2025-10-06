use std::{collections::VecDeque, iter::FusedIterator, sync::Arc};

use biome_js_syntax::TextRange;
use biome_js_type_info::{BindingId, ScopeId, TypeReferenceQualifier};
use biome_rowan::TokenText;
use rustc_hash::FxHashMap;

use super::{
    JsModuleInfoInner,
    binding::{JsBinding, JsDeclarationKind},
};

#[derive(Debug)]
pub struct JsScopeData {
    // The scope range
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

/// Reference to one or two bindings.
///
/// Tracks whether the bindings refer to a type or the type of a value.
/// See [`biome_js_type_info::DualReference`] for why this is necessary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TsBindingReference {
    Type(BindingId),
    ValueType(BindingId),
    TypeAndValueType(BindingId),
    NamespaceAndValueType(BindingId),
    Merged {
        ty: Option<BindingId>,
        value_ty: Option<BindingId>,
        namespace_ty: Option<BindingId>,
    },
}

impl TsBindingReference {
    pub fn from_binding_and_declaration_kind(
        binding_id: BindingId,
        declaration_id: JsDeclarationKind,
    ) -> Self {
        match (
            declaration_id.declares_namespace(),
            declaration_id.declares_type(),
            declaration_id.declares_value(),
        ) {
            (true, _, _) => Self::NamespaceAndValueType(binding_id),
            (_, true, true) => Self::TypeAndValueType(binding_id),
            (_, true, false) => Self::Type(binding_id),
            (_, false, _) => Self::ValueType(binding_id),
        }
    }

    pub fn get_binding_id_for_qualifier(
        self,
        qualifier: &TypeReferenceQualifier,
    ) -> Option<BindingId> {
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

    /// Returns the namespace binding or type binding, in that order.
    ///
    /// Returns `None` if the reference only references a value type.
    pub fn namespace_ty_or_ty(self) -> Option<BindingId> {
        match self {
            Self::Type(binding_id)
            | Self::TypeAndValueType(binding_id)
            | Self::NamespaceAndValueType(binding_id) => Some(binding_id),
            Self::Merged {
                ty, namespace_ty, ..
            } => namespace_ty.or(ty),
            _ => None,
        }
    }

    /// Creates a union from this binding reference with another.
    ///
    /// If both bindings refer to the same kind of type, the binding ID(s) from
    /// `other` takes precedence.
    pub fn union_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Type(own_binding_id), Self::ValueType(other_binding_id)) => {
                if own_binding_id == other_binding_id {
                    Self::TypeAndValueType(other_binding_id)
                } else {
                    Self::Merged {
                        ty: Some(own_binding_id),
                        value_ty: Some(other_binding_id),
                        namespace_ty: None,
                    }
                }
            }
            (Self::Type(own_binding_id), Self::NamespaceAndValueType(other_binding_id)) => {
                Self::Merged {
                    ty: Some(own_binding_id),
                    value_ty: Some(other_binding_id),
                    namespace_ty: Some(other_binding_id),
                }
            }
            (
                Self::Type(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) if ty.is_none() => Self::Merged {
                ty: Some(own_binding_ty),
                value_ty,
                namespace_ty,
            },

            (Self::ValueType(own_binding_id), Self::Type(other_binding_id)) => {
                if own_binding_id == other_binding_id {
                    Self::TypeAndValueType(other_binding_id)
                } else {
                    Self::Merged {
                        ty: Some(other_binding_id),
                        value_ty: Some(own_binding_id),
                        namespace_ty: None,
                    }
                }
            }
            (
                Self::ValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) if value_ty.is_none() => Self::Merged {
                ty,
                value_ty: Some(own_binding_ty),
                namespace_ty,
            },

            (Self::TypeAndValueType(own_binding_id), Self::Type(other_binding_id)) => {
                Self::Merged {
                    ty: Some(other_binding_id),
                    value_ty: Some(own_binding_id),
                    namespace_ty: None,
                }
            }
            (Self::TypeAndValueType(own_binding_id), Self::ValueType(other_binding_id)) => {
                Self::Merged {
                    ty: Some(own_binding_id),
                    value_ty: Some(other_binding_id),
                    namespace_ty: None,
                }
            }
            (
                Self::TypeAndValueType(own_binding_id),
                Self::NamespaceAndValueType(other_binding_id),
            ) => Self::Merged {
                ty: Some(own_binding_id),
                value_ty: Some(other_binding_id),
                namespace_ty: Some(other_binding_id),
            },
            (
                Self::TypeAndValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) => Self::Merged {
                ty: ty.or(Some(own_binding_ty)),
                value_ty: value_ty.or(Some(own_binding_ty)),
                namespace_ty,
            },

            (Self::NamespaceAndValueType(own_binding_id), Self::Type(other_binding_id)) => {
                Self::Merged {
                    ty: Some(other_binding_id),
                    value_ty: Some(own_binding_id),
                    namespace_ty: Some(own_binding_id),
                }
            }
            (Self::NamespaceAndValueType(own_binding_id), Self::ValueType(other_binding_id)) => {
                Self::Merged {
                    ty: None,
                    value_ty: Some(other_binding_id),
                    namespace_ty: Some(own_binding_id),
                }
            }
            (
                Self::NamespaceAndValueType(own_binding_id),
                Self::TypeAndValueType(other_binding_id),
            ) => Self::Merged {
                ty: Some(other_binding_id),
                value_ty: Some(other_binding_id),
                namespace_ty: Some(own_binding_id),
            },
            (
                Self::NamespaceAndValueType(own_binding_ty),
                Self::Merged {
                    ty,
                    value_ty,
                    namespace_ty,
                },
            ) => Self::Merged {
                ty,
                value_ty: value_ty.or(Some(own_binding_ty)),
                namespace_ty: namespace_ty.or(Some(own_binding_ty)),
            },

            (
                Self::Merged {
                    value_ty,
                    namespace_ty,
                    ..
                },
                Self::Type(other_binding_ty),
            ) => Self::Merged {
                ty: Some(other_binding_ty),
                value_ty,
                namespace_ty,
            },
            (
                Self::Merged {
                    ty, namespace_ty, ..
                },
                Self::ValueType(other_binding_ty),
            ) => Self::Merged {
                ty,
                value_ty: Some(other_binding_ty),
                namespace_ty,
            },
            (Self::Merged { namespace_ty, .. }, Self::TypeAndValueType(other_binding_ty))
                if namespace_ty.is_some() =>
            {
                Self::Merged {
                    ty: Some(other_binding_ty),
                    value_ty: Some(other_binding_ty),
                    namespace_ty,
                }
            }
            (Self::Merged { ty, .. }, Self::NamespaceAndValueType(other_binding_ty))
                if ty.is_some() =>
            {
                Self::Merged {
                    ty,
                    value_ty: Some(other_binding_ty),
                    namespace_ty: Some(other_binding_ty),
                }
            }
            (
                Self::Merged {
                    ty: own_ty,
                    value_ty: own_value_ty,
                    namespace_ty: own_namespace_ty,
                },
                Self::Merged {
                    ty: other_ty,
                    value_ty: other_value_ty,
                    namespace_ty: other_namespace_ty,
                },
            ) => Self::Merged {
                ty: other_ty.or(own_ty),
                value_ty: other_value_ty.or(own_value_ty),
                namespace_ty: other_namespace_ty.or(own_namespace_ty),
            },

            (_, other) => other,
        }
    }

    /// Returns the value type binding.
    pub fn value_ty(self) -> Option<BindingId> {
        match self {
            Self::ValueType(binding_id)
            | Self::TypeAndValueType(binding_id)
            | Self::NamespaceAndValueType(binding_id) => Some(binding_id),
            Self::Merged { value_ty, .. } => value_ty,
            Self::Type(_) => None,
        }
    }

    /// Returns the value type binding, or the type binding if the value type
    /// binding is unknown.
    pub fn value_ty_or_ty(self) -> BindingId {
        match self {
            Self::ValueType(binding_id)
            | Self::TypeAndValueType(binding_id)
            | Self::NamespaceAndValueType(binding_id) => binding_id,
            Self::Merged {
                ty,
                value_ty,
                namespace_ty,
            } => value_ty
                .or(namespace_ty)
                .or(ty)
                .expect("a merged reference must have at least two fields set to `Some`"),
            Self::Type(binding_id) => binding_id,
        }
    }
}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
#[derive(Clone, Debug)]
pub struct JsScope {
    pub(crate) info: Arc<JsModuleInfoInner>,
    pub(crate) id: ScopeId,
}

impl PartialEq for JsScope {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && Arc::ptr_eq(&self.info, &other.info)
    }
}

impl Eq for JsScope {}

impl JsScope {
    pub fn is_global_scope(&self) -> bool {
        self.id.index() == 0
    }

    /// Returns all parents of this scope. Starting with the current
    /// [JsScope].
    pub fn ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        std::iter::successors(Some(self.clone()), |scope| scope.parent())
    }

    /// Returns all descendents of this scope in breadth-first order. Starting
    /// with the current [JsScope].
    pub fn descendents(&self) -> impl Iterator<Item = Self> + use<> {
        let mut q = VecDeque::new();
        q.push_back(self.id);

        ScopeDescendentsIter {
            info: self.info.clone(),
            q,
        }
    }

    /// Returns this scope parent.
    pub fn parent(&self) -> Option<Self> {
        debug_assert!((self.id.index()) < self.info.scopes.len());

        let parent = self.info.scopes[self.id.index()].parent?;
        Some(Self {
            info: self.info.clone(),
            id: parent,
        })
    }

    /// Returns all bindings that were bound in this scope.
    ///
    /// It **does not** return bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            info: self.info.clone(),
            scope_id: self.id,
            binding_index: 0,
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
        other.ancestors().any(|s| s == *self)
    }

    pub fn range(&self) -> TextRange {
        self.info.scopes[self.id.index()].range
    }
}

/// Iterates all descendent scopes of the specified scope in breadth-first
/// order.
pub struct ScopeDescendentsIter {
    info: Arc<JsModuleInfoInner>,
    q: VecDeque<ScopeId>,
}

impl Iterator for ScopeDescendentsIter {
    type Item = JsScope;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.q.pop_front() {
            let scope = &self.info.scopes[id.index()];
            self.q.extend(scope.children.iter());
            Some(JsScope {
                info: self.info.clone(),
                id,
            })
        } else {
            None
        }
    }
}

impl FusedIterator for ScopeDescendentsIter {}

/// Iterates all bindings that were bound in a given scope.
///
/// It **does not** return bindings of parent scopes.
#[derive(Debug)]
pub struct ScopeBindingsIter {
    info: Arc<JsModuleInfoInner>,
    scope_id: ScopeId,
    binding_index: u32,
}

impl Iterator for ScopeBindingsIter {
    type Item = JsBinding;

    fn next(&mut self) -> Option<Self::Item> {
        debug_assert!(self.scope_id.index() < self.info.scopes.len());

        let id = *self.info.scopes[self.scope_id.index()]
            .bindings
            .get(self.binding_index as usize)?;

        self.binding_index += 1;

        Some(JsBinding {
            data: self.info.clone(),
            id,
        })
    }
}

impl ExactSizeIterator for ScopeBindingsIter {
    fn len(&self) -> usize {
        debug_assert!(self.scope_id.index() < self.info.scopes.len());

        self.info.scopes[self.scope_id.index()].bindings.len()
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
