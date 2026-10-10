//! Shared state for the passes that build a [`SemanticModel`].

use super::model::*;
use biome_rowan::{RawSyntaxKind, TextRange, TextSize, TokenText};
use rustc_hash::FxHashMap;

/// What a script-level JavaScript binding stands for in a component.
///
/// The reference pass uses it to follow `props.title`, `emit('change')` and
/// similar expressions back to the component's declarations.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Role {
    pub(crate) origin: Origin,
    pub(crate) component: ComponentId,
    /// For a variable destructured from props, the prop it holds.
    pub(crate) prop: Option<SymbolId>,
}

/// Accumulates the tables of a model.
#[derive(Default)]
pub(crate) struct ModelBuilder {
    pub(crate) data: SemanticModelData,
    /// Roles of JavaScript bindings, keyed by the host range of the binding's
    /// name.
    pub(crate) roles: FxHashMap<TextRange, Role>,
    /// Whether a `<script src>` block moves declarations of the file to
    /// another file while no component was found to record that on.
    pub(crate) has_unread_script: bool,
}

/// Everything needed to describe a new symbol.
pub(crate) struct NewSymbol {
    pub(crate) name: TokenText,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) scope: ScopeId,
    pub(crate) namespace: Namespace,
    pub(crate) kind: SymbolKind,
    pub(crate) component: Option<ComponentId>,
}

impl ModelBuilder {
    pub(crate) fn add_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId::new(self.data.scopes.len());
        self.data.scopes.push(ScopeData {
            kind,
            parent,
            symbols: Vec::new(),
        });
        id
    }

    pub(crate) fn add_symbol(&mut self, symbol: NewSymbol) -> SymbolId {
        let id = SymbolId::new(self.data.symbols.len());
        self.data.scopes[symbol.scope.index()].symbols.push(id);
        // A template ref is declared in the template, not in the tree that
        // defines the component, so it is not one of its declarations.
        if let Some(component) = symbol.component
            && symbol.kind != SymbolKind::TemplateRef
        {
            self.data.components[component.index()].symbols.push(id);
        }
        self.data.symbols.push(SymbolData {
            name: symbol.name,
            range: symbol.range,
            declaration: symbol.declaration,
            snippet: symbol.snippet,
            scope: symbol.scope,
            namespace: symbol.namespace,
            kind: symbol.kind,
            origin: Origin::None,
            parent: None,
            component: symbol.component,
            detail: Detail::None,
            type_only: false,
        });
        self.data.uses.push(Vec::new());
        id
    }

    pub(crate) fn symbol_mut(&mut self, id: SymbolId) -> &mut SymbolData {
        &mut self.data.symbols[id.index()]
    }

    pub(crate) fn add_component(
        &mut self,
        kind: ComponentKind,
        definition: TextRange,
        snippet: Option<SnippetId>,
    ) -> ComponentId {
        let id = ComponentId::new(self.data.components.len());
        let instance = self.add_scope(ScopeKind::Instance(id), None);
        self.data.components.push(ComponentData {
            kind,
            definition,
            snippet,
            has_template: false,
            instance,
            name: None,
            inherit_attrs: Tri::Unknown,
            decl_open: [OpenReasons::default(); Namespace::COUNT],
            uses_open: [false; Namespace::COUNT],
            macro_conflict: false,
            merged: None,
            symbols: Vec::new(),
        });
        id
    }

    /// Returns `component` followed by the component Vue merges into it, if
    /// any.
    fn with_merged(&self, component: ComponentId) -> impl Iterator<Item = ComponentId> {
        std::iter::once(component).chain(self.data.components[component.index()].merged)
    }

    /// Returns whether the declarations `component` makes in `namespace` may
    /// be incomplete.
    pub(crate) fn is_open(&self, component: Option<ComponentId>, namespace: Namespace) -> bool {
        match component {
            Some(component) => self.with_merged(component).any(|id| {
                !self.data.components[id.index()].decl_open[namespace.index()].is_empty()
            }),
            None => self.has_unread_script,
        }
    }

    /// Finds the declaration of `component` in `namespace` that `matches`.
    ///
    /// When there is none, the result says whether one could exist where the
    /// model cannot see it.
    pub(crate) fn resolve_declaration(
        &self,
        component: Option<ComponentId>,
        namespace: Namespace,
        matches: impl Fn(&SymbolData) -> bool,
    ) -> Resolution {
        let is_match = |id: &SymbolId| {
            let symbol = &self.data.symbols[id.index()];
            symbol.namespace == namespace && symbol.parent.is_none() && matches(symbol)
        };
        let found = if namespace == Namespace::TemplateRef {
            // Template refs belong to the template, which is always readable.
            return (0..self.data.symbols.len())
                .map(SymbolId::new)
                .find(is_match)
                .map_or(Resolution::Unresolved, Resolution::Symbol);
        } else {
            component.and_then(|component| {
                self.with_merged(component).find_map(|id| {
                    self.data.components[id.index()]
                        .symbols
                        .iter()
                        .copied()
                        .find(is_match)
                })
            })
        };
        match found {
            Some(id) => Resolution::Symbol(id),
            None if self.is_open(component, namespace) => Resolution::Unknowable,
            None => Resolution::Unresolved,
        }
    }

    pub(crate) fn component_mut(&mut self, id: ComponentId) -> &mut ComponentData {
        &mut self.data.components[id.index()]
    }

    /// Marks the declarations of `namespace` as possibly incomplete.
    pub(crate) fn open(&mut self, component: ComponentId, namespace: Namespace, why: OpenReasons) {
        self.component_mut(component).decl_open[namespace.index()].insert(why);
    }

    /// Marks every namespace of `component` as possibly incomplete.
    pub(crate) fn open_all(&mut self, component: ComponentId, why: OpenReasons) {
        for reasons in &mut self.component_mut(component).decl_open {
            reasons.insert(why);
        }
    }

    pub(crate) fn add_reference(&mut self, reference: ReferenceData) -> ReferenceId {
        let id = ReferenceId::new(self.data.references.len());
        if let Resolution::Symbol(symbol) = reference.resolution {
            self.data.uses[symbol.index()].push(id);
        }
        self.data.references.push(reference);
        id
    }

    pub(crate) fn finish(self) -> SemanticModel {
        SemanticModel::new(self.data)
    }
}

/// Creates token text for a name that is not written as a single token.
pub(crate) fn synthetic_name(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}

/// Converts a range of a snippet's tree to the host document.
pub(crate) fn to_host(range: TextRange, offset: TextSize) -> TextRange {
    range + offset
}
