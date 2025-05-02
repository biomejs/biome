use std::{collections::VecDeque, iter::FusedIterator, sync::Arc};

use biome_js_semantic::ScopeId;
use biome_js_syntax::TextRange;
use biome_rowan::TokenText;
use rustc_hash::FxHashMap;

use super::{
    JsModuleInfoInner,
    binding::{BindingId, JsBinding},
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
    pub bindings_by_name: FxHashMap<TokenText, BindingId>,
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
    /// [Scope].
    pub fn ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        std::iter::successors(Some(self.clone()), |scope| scope.parent())
    }

    /// Returns all descendents of this scope in breadth-first order. Starting with the current
    /// [Scope].
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
        // id will always be a valid scope because
        // it was created by [SemanticModel::scope] method.
        debug_assert!((self.id.index()) < self.info.scopes.len());

        let parent = self.info.scopes[self.id.index()].parent?;
        Some(Self {
            info: self.info.clone(),
            id: parent,
        })
    }

    /// Returns all bindings that were bound in this scope. It **does
    /// not** returns bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            info: self.info.clone(),
            scope_id: self.id,
            binding_index: 0,
        }
    }

    /// Returns a binding by its name, like it appears on code.  It **does
    /// not** returns bindings of parent scopes.
    pub fn get_binding(&self, name: impl AsRef<str>) -> Option<JsBinding> {
        let data = &self.info.scopes[self.id.index()];

        let name = name.as_ref();
        let id = *data.bindings_by_name.get(name)?;

        Some(JsBinding {
            data: self.info.clone(),
            id,
        })
    }

    /// Checks if the current scope is one of the ancestor of "other". Given
    /// that [Scope::ancestors] return "self" as the first scope,
    /// this function returns true for:
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

/// Iterate all descendents scopes of the specified scope in breadth-first order.
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

/// Iterate all bindings that were bound in a given scope. It **does
/// not** Returns bindings of parent scopes.
#[derive(Debug)]
pub struct ScopeBindingsIter {
    info: Arc<JsModuleInfoInner>,
    scope_id: ScopeId,
    binding_index: u32,
}

impl Iterator for ScopeBindingsIter {
    type Item = JsBinding;

    fn next(&mut self) -> Option<Self::Item> {
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
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
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
        debug_assert!(self.scope_id.index() < self.info.scopes.len());

        self.info.scopes[self.scope_id.index()].bindings.len()
    }
}

impl FusedIterator for ScopeBindingsIter {}
