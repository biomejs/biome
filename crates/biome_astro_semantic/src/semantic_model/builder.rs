use super::model::*;
use biome_rowan::{RawSyntaxKind, TextRange, TextSize, TokenText};

#[derive(Default)]
pub(crate) struct ModelBuilder {
    pub(crate) data: SemanticModelData,
}

pub(crate) struct NewSymbol {
    pub(crate) name: TokenText,
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
        let id = ScopeId::new(self.data.scopes.len());
        self.data.scopes.push(ScopeData {
            kind,
            parent,
            symbols: Vec::new(),
            open: [OpenReasons::default(); Namespace::COUNT],
        });
        id
    }

    pub(crate) fn open(&mut self, scope: ScopeId, namespace: Namespace, reason: OpenReason) {
        self.data.scopes[scope.index()].open[namespace.index()].insert(reason);
    }

    pub(crate) fn add_block(
        &mut self,
        kind: BlockKind,
        range: TextRange,
        content: Option<TextRange>,
        scope: Option<ScopeId>,
    ) -> BlockId {
        let id = BlockId::new(self.data.blocks.len());
        self.data.blocks.push(BlockData {
            kind,
            range,
            content,
            scope,
            snippet: None,
        });
        id
    }

    pub(crate) fn add_snippet(
        &mut self,
        range: TextRange,
        offset: TextSize,
        source: biome_languages::JsFileSource,
        host: SnippetHost,
        scope: Option<ScopeId>,
    ) -> SnippetId {
        let id = SnippetId::new(self.data.snippets.len());
        self.data.snippets.push(SnippetData {
            range,
            offset,
            source,
            host,
            scope,
        });
        match host {
            SnippetHost::Frontmatter(block) | SnippetHost::Script(block) => {
                self.data.blocks[block.index()].snippet = Some(id);
            }
            SnippetHost::Expression | SnippetHost::Unknown => {}
        }
        id
    }

    pub(crate) fn add_symbol(&mut self, symbol: NewSymbol) -> SymbolId {
        let id = SymbolId::new(self.data.symbols.len());
        self.data.scopes[symbol.scope.index()].symbols.push(id);
        self.data.symbols.push(SymbolData {
            name: symbol.name,
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
        self.data.uses.push(Vec::new());
        id
    }

    pub(crate) fn add_reference(&mut self, reference: ReferenceData) {
        self.data.references.push(reference);
    }

    pub(crate) fn finish(mut self) -> AstroSemanticModel {
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
        for uses in &mut self.data.uses {
            uses.clear();
        }
        for (index, reference) in self.data.references.iter().enumerate() {
            if let Resolution::Symbol(symbol) = reference.resolution {
                self.data.uses[symbol.index()].push(ReferenceId::new(index));
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
