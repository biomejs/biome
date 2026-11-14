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
    pub(crate) type_only: bool,
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

    pub(crate) fn add_block(
        &mut self,
        kind: BlockKind,
        range: TextRange,
        content: Option<TextRange>,
        scope: Option<ScopeId>,
    ) -> BlockId {
        self.data.blocks.push(BlockData {
            kind,
            range,
            content,
            scope,
            snippet: None,
        })
    }

    pub(crate) fn add_snippet(
        &mut self,
        range: TextRange,
        offset: TextSize,
        source: biome_languages::JsFileSource,
        host: SnippetHost,
    ) -> SnippetId {
        let id = self.data.snippets.push(SnippetData {
            range,
            offset,
            source,
            host,
        });
        match host {
            SnippetHost::Frontmatter(block) | SnippetHost::Script(block) => {
                self.data.blocks[block].snippet = Some(id);
            }
            SnippetHost::Expression | SnippetHost::Unknown => {}
        }
        id
    }

    pub(crate) fn add_symbol(&mut self, symbol: NewSymbol) -> SymbolId {
        let id = self.data.symbols.push(SymbolData {
            name: symbol.name,
            range: symbol.range,
            declaration: symbol.declaration,
            snippet: symbol.snippet,
            scope: symbol.scope,
            namespace: symbol.namespace,
            kind: symbol.kind,
            origin: symbol.origin,
            type_only: symbol.type_only,
        });
        self.data.scopes[symbol.scope].symbols.push(id);
        let uses = self.data.uses.push(Vec::new());
        debug_assert_eq!(uses, id, "`uses` must have one row per symbol");
        id
    }

    pub(crate) fn finish(self) -> AstroSemanticModel {
        AstroSemanticModel::new(self.data)
    }
}

pub(crate) fn synthetic_name(text: &str) -> TokenText {
    TokenText::new_raw(RawSyntaxKind(0), text)
}
