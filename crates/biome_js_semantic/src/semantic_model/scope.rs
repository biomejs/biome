use super::*;
use biome_js_syntax::TextRange;
use biome_rowan::TokenText;
use rustc_hash::FxHashMap;
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct SemanticModelScopeData {
    // The scope range
    pub(crate) range: TextRange,
    // The parent scope of this scope
    pub(crate) parent: Option<ScopeId>,
    // All children scope of this scope
    pub(crate) children: Vec<ScopeId>,
    // All bindings of this scope (points to SemanticModelData::bindings)
    pub(crate) bindings: Vec<BindingId>,
    // Map pointing to the [bindings] vec of each bindings by its name
    pub(crate) bindings_by_name: FxHashMap<TokenText, BindingId>,
    // All read references of a scope
    pub(crate) read_references: Vec<ReferenceId>,
    // All write references of a scope
    pub(crate) write_references: Vec<ReferenceId>,
    // Identify if this scope is from a closure or not
    pub(crate) is_closure: bool,
}

/// Provides all information regarding a specific scope.
/// Allows navigation to parent and children scope and binding information.
#[derive(Clone, Debug)]
pub struct Scope {
    pub(crate) data: Rc<SemanticModelData>,
    pub(crate) id: ScopeId,
}

impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.data == other.data
    }
}

impl Eq for Scope {}

impl Scope {
    pub fn is_global_scope(&self) -> bool {
        self.id.index() == 0
    }

    /// Returns all parents of this scope. Starting with the current
    /// [Scope].
    pub fn ancestors(&self) -> impl Iterator<Item = Scope> {
        std::iter::successors(Some(self.clone()), |scope| scope.parent())
    }

    /// Returns all descendents of this scope in breadth-first order. Starting with the current
    /// [Scope].
    pub fn descendents(&self) -> impl Iterator<Item = Scope> {
        let mut q = VecDeque::new();
        q.push_back(self.id);

        ScopeDescendentsIter {
            data: self.data.clone(),
            q,
        }
    }

    /// Returns this scope parent.
    pub fn parent(&self) -> Option<Scope> {
        // id will always be a valid scope because
        // it was created by [SemanticModel::scope] method.
        debug_assert!((self.id.index()) < self.data.scopes.len());

        let parent = self.data.scopes[self.id.index()].parent?;
        Some(Scope {
            data: self.data.clone(),
            id: parent,
        })
    }

    /// Returns all bindings that were bound in this scope. It **does
    /// not** returns bindings of parent scopes.
    pub fn bindings(&self) -> ScopeBindingsIter {
        ScopeBindingsIter {
            data: self.data.clone(),
            scope_id: self.id,
            binding_index: 0,
        }
    }

    /// Returns a binding by its name, like it appears on code.  It **does
    /// not** returns bindings of parent scopes.
    pub fn get_binding(&self, name: impl AsRef<str>) -> Option<Binding> {
        let data = &self.data.scopes[self.id.index()];

        let name = name.as_ref();
        let id = *data.bindings_by_name.get(name)?;

        Some(Binding {
            data: self.data.clone(),
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
    pub fn is_ancestor_of(&self, other: &Scope) -> bool {
        other.ancestors().any(|s| s == *self)
    }

    pub fn range(&self) -> TextRange {
        self.data.scopes[self.id.index()].range
    }

    pub fn syntax(&self) -> &JsSyntaxNode {
        &self.data.scope_node_by_range[&self.range()]
    }

    /// Return the [Closure] associated with this scope if
    /// it has one, otherwise returns None.
    /// See [HasClosureAstNode] for nodes that have closure.
    pub fn closure(&self) -> Option<Closure> {
        Closure::from_scope(self.data.clone(), self.id)
    }
}

/// Iterate all descendents scopes of the specified scope in breadth-first order.
pub struct ScopeDescendentsIter {
    data: Rc<SemanticModelData>,
    q: VecDeque<ScopeId>,
}

impl Iterator for ScopeDescendentsIter {
    type Item = Scope;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(id) = self.q.pop_front() {
            let scope = &self.data.scopes[id.index()];
            self.q.extend(scope.children.iter());
            Some(Scope {
                data: self.data.clone(),
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
    data: Rc<SemanticModelData>,
    scope_id: ScopeId,
    binding_index: u32,
}

impl Iterator for ScopeBindingsIter {
    type Item = Binding;

    fn next(&mut self) -> Option<Self::Item> {
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
        debug_assert!(self.scope_id.index() < self.data.scopes.len());

        let id = *self.data.scopes[self.scope_id.index()]
            .bindings
            .get(self.binding_index as usize)?;

        self.binding_index += 1;

        Some(Binding {
            data: self.data.clone(),
            id,
        })
    }
}

impl ExactSizeIterator for ScopeBindingsIter {
    fn len(&self) -> usize {
        // scope_id will always be a valid scope because
        // it was created by [Scope::bindings] method.
        debug_assert!(self.scope_id.index() < self.data.scopes.len());

        self.data.scopes[self.scope_id.index()].bindings.len()
    }
}

impl FusedIterator for ScopeBindingsIter {}

// Extensions

/// Extension method to allow [AstNode] to easily
/// get its [Scope].
pub trait SemanticScopeExtensions {
    /// Returns the [Scope] which this object is part of.
    /// See [scope](crate::SemanticModel::scope)
    fn scope(&self, model: &SemanticModel) -> Scope;

    /// Returns the [Scope] which this object was hosted to, if any.
    /// See [scope](crate::SemanticModel::scope_hoisted_to)
    fn scope_hoisted_to(&self, model: &SemanticModel) -> Option<Scope>;
}

impl<T: AstNode<Language = JsLanguage>> SemanticScopeExtensions for T {
    fn scope(&self, model: &SemanticModel) -> Scope {
        model.scope(self.syntax())
    }

    fn scope_hoisted_to(&self, model: &SemanticModel) -> Option<Scope> {
        model.scope_hoisted_to(self.syntax())
    }
}
