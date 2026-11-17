use super::model::*;
use biome_rowan::{RawSyntaxKind, Text, TextRange, TextSize, TokenText};

#[derive(Default)]
pub(crate) struct ModelBuilder {
    pub(crate) data: SemanticModelData,
}

pub(crate) struct NewBlock {
    pub(crate) kind: BlockKind,
    pub(crate) range: TextRange,
    pub(crate) content: Option<TextRange>,
    pub(crate) supported: bool,
    pub(crate) globally_scoped: bool,
    pub(crate) external: bool,
    pub(crate) scope: Option<ScopeId>,
}

pub(crate) struct NewSymbol {
    pub(crate) name: TokenText,
    pub(crate) semantic_name: Text,
    pub(crate) range: TextRange,
    pub(crate) declaration: TextRange,
    pub(crate) snippet: Option<SnippetId>,
    pub(crate) scope: ScopeId,
    pub(crate) namespace: Namespace,
    pub(crate) kind: SymbolKind,
    pub(crate) origin: Origin,
    pub(crate) declares_value: bool,
    pub(crate) declares_type: bool,
    pub(crate) declares_namespace: bool,
}

impl ModelBuilder {
    pub(crate) fn add_scope(&mut self, kind: ScopeKind, parent: Option<ScopeId>) -> ScopeId {
        self.data.scopes.push(ScopeData {
            kind,
            parent,
            symbols: Vec::new(),
            open: [OpenReasons::default(); Namespace::COUNT],
        })
    }

    pub(crate) fn open(&mut self, scope: ScopeId, namespace: Namespace, reason: OpenReason) {
        self.data.scopes[scope].open[namespace.index()].insert(reason);
    }

    pub(crate) fn add_block(&mut self, block: NewBlock) -> BlockId {
        self.data.blocks.push(BlockData {
            kind: block.kind,
            range: block.range,
            content: block.content,
            supported: block.supported,
            globally_scoped: block.globally_scoped,
            external: block.external,
            scope: block.scope,
            snippet: None,
            define_vars: Vec::new(),
            define_vars_open: OpenReasons::default(),
        })
    }

    pub(crate) fn add_snippet(
        &mut self,
        range: TextRange,
        offset: TextSize,
        source: biome_languages::JsFileSource,
        host: SnippetHost,
        scope: Option<ScopeId>,
    ) -> SnippetId {
        let id = self.data.snippets.push(SnippetData {
            range,
            offset,
            source,
            host,
            scope,
        });
        match host {
            SnippetHost::Frontmatter(block) | SnippetHost::Script(block) => {
                self.data.blocks[block].snippet = Some(id);
            }
            SnippetHost::ScriptDefineVars(_)
            | SnippetHost::StyleDefineVars(_)
            | SnippetHost::Expression
            | SnippetHost::Unknown => {}
        }
        id
    }

    pub(crate) fn add_symbol(&mut self, symbol: NewSymbol) -> SymbolId {
        let id = self.data.symbols.push(SymbolData {
            name: symbol.name,
            semantic_name: symbol.semantic_name,
            range: symbol.range,
            declaration: symbol.declaration,
            snippet: symbol.snippet,
            scope: symbol.scope,
            namespace: symbol.namespace,
            kind: symbol.kind,
            origin: symbol.origin,
            declares_value: symbol.declares_value,
            declares_type: symbol.declares_type,
            declares_namespace: symbol.declares_namespace,
        });
        self.data.scopes[symbol.scope].symbols.push(id);
        let uses = self.data.uses.push(Vec::new());
        debug_assert_eq!(uses, id, "`uses` must have one row per symbol");
        id
    }

    pub(crate) fn add_reference(&mut self, reference: ReferenceData) {
        self.data.references.push(reference);
    }

    pub(crate) fn finish(mut self) -> AstroSemanticModel {
        let symbols = &self.data.symbols;
        for scope in self.data.scopes.iter_mut() {
            scope.symbols.sort_by(|left, right| {
                let left = &symbols[*left];
                let right = &symbols[*right];
                left.range
                    .start()
                    .cmp(&right.range.start())
                    .then_with(|| left.range.end().cmp(&right.range.end()))
                    .then_with(|| left.semantic_name.text().cmp(right.semantic_name.text()))
                    .then_with(|| (left.kind as u8).cmp(&(right.kind as u8)))
            });
        }
        self.data.references.sort_by(|left, right| {
            left.range
                .start()
                .cmp(&right.range.start())
                .then_with(|| left.range.end().cmp(&right.range.end()))
                .then_with(|| left.name.text().cmp(right.name.text()))
                .then_with(|| (left.site as u8).cmp(&(right.site as u8)))
                .then_with(|| (left.namespace as u8).cmp(&(right.namespace as u8)))
                .then_with(|| (left.access as u8).cmp(&(right.access as u8)))
        });
        for uses in self.data.uses.iter_mut() {
            uses.clear();
        }
        for (index, reference) in self.data.references.iter().enumerate() {
            if let Resolution::Symbol(symbol) = reference.resolution {
                self.data.uses[symbol].push(ReferenceId::new(index));
            }
        }
        AstroSemanticModel::new(self.data)
    }
}

pub(crate) fn synthetic_name(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}

pub(crate) fn to_host(range: TextRange, offset: TextSize) -> TextRange {
    range + offset
}
