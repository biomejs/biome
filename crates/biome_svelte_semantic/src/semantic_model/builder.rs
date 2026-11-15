//! Shared state for the passes that build a [`SvelteSemanticModel`].

use super::model::*;
use super::nodes::{HtmlNodeId, JsNodeId};
use biome_html_syntax::HtmlSyntaxNode;
use biome_js_syntax::{JsLanguage, JsSyntaxNode};
use biome_rowan::{RawSyntaxKind, SyntaxNodePtr, TextRange, TokenText};
use rustc_hash::FxHashMap;

/// Accumulates the tables of a model.
#[derive(Default)]
pub(crate) struct ModelBuilder {
    pub(crate) data: SemanticModelData,
    /// The id given to each node already in [`SemanticModelData::js_nodes`].
    js_node_ids: FxHashMap<(Option<SnippetId>, SyntaxNodePtr<JsLanguage>), JsNodeId>,
    /// The symbol declared at each host range of a name.
    symbols_by_range: FxHashMap<TextRange, SymbolId>,
}

/// Everything needed to describe a new symbol.
pub(crate) struct NewSymbol {
    pub(crate) name: TokenText,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) scope: ScopeId,
    pub(crate) kind: SymbolKind,
    pub(crate) declaration_kind: DeclarationKind,
    pub(crate) node: Option<JsNodeId>,
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
        self.symbols_by_range.insert(symbol.range, id);
        self.data.symbols.push(SymbolData {
            name: symbol.name,
            range: symbol.range,
            declaration: symbol.declaration,
            snippet: symbol.snippet,
            scope: symbol.scope,
            kind: symbol.kind,
            declaration_kind: symbol.declaration_kind,
            import: None,
            rune: None,
            store: None,
            each_source: None,
            exported: false,
            type_only: false,
            facts: ValueFacts::default(),
            node: symbol.node,
        });
        self.data.uses.push(Vec::new());
        id
    }

    pub(crate) fn symbol_mut(&mut self, id: SymbolId) -> &mut SymbolData {
        &mut self.data.symbols[id.index()]
    }

    /// Returns the symbol whose name is written at `range` in the host
    /// document.
    pub(crate) fn symbol_at(&self, range: TextRange) -> Option<SymbolId> {
        self.symbols_by_range.get(&range).copied()
    }

    /// Returns the id of `node`, a node of the tree of `snippet`.
    pub(crate) fn add_js_node(
        &mut self,
        snippet: Option<SnippetId>,
        node: &JsSyntaxNode,
    ) -> JsNodeId {
        let ptr = SyntaxNodePtr::new(node);
        *self.js_node_ids.entry((snippet, ptr)).or_insert_with(|| {
            let id = JsNodeId::new(self.data.js_nodes.len());
            self.data.js_nodes.push(JsNodeData { snippet, ptr });
            id
        })
    }

    /// Returns the id of `node`, a node of the host document.
    pub(crate) fn add_html_node(&mut self, node: &HtmlSyntaxNode) -> HtmlNodeId {
        let id = HtmlNodeId::new(self.data.html_nodes.len());
        self.data.html_nodes.push(SyntaxNodePtr::new(node));
        id
    }

    pub(crate) fn add_reference(&mut self, reference: ReferenceData) -> ReferenceId {
        let id = ReferenceId::new(self.data.references.len());
        if let Resolution::Symbol(symbol) = reference.resolution {
            self.data.uses[symbol.index()].push(id);
        }
        let slot = reference.snippet.map_or(0, |snippet| snippet.index() + 1);
        if self.data.references_by_snippet.len() <= slot {
            self.data
                .references_by_snippet
                .resize_with(slot + 1, Vec::new);
        }
        self.data.references_by_snippet[slot].push(id);
        self.data.references.push(reference);
        id
    }

    /// Looks `name` up in `scope` and its enclosing scopes.
    pub(crate) fn lookup(&self, scope: ScopeId, name: &str) -> Option<SymbolId> {
        let mut current = Some(scope);
        while let Some(id) = current {
            let scope = &self.data.scopes[id.index()];
            let found = scope
                .symbols
                .iter()
                .copied()
                .find(|symbol| self.data.symbols[symbol.index()].name.text() == name);
            if found.is_some() {
                return found;
            }
            current = scope.parent;
        }
        None
    }

    pub(crate) fn finish(self) -> SvelteSemanticModel {
        SvelteSemanticModel::new(self.data)
    }
}

/// Creates token text for a name that is not written as a single token.
pub(crate) fn synthetic_name(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}
