use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, TextRange, TextSize};

/// One JavaScript or TypeScript tree embedded in an Astro host document.
pub struct AstroSnippet<'a> {
    /// The parsed guest tree.
    pub root: &'a AnyJsRoot,
    /// JavaScript semantics local to the guest tree.
    pub js: &'a JsSemanticModel,
    /// The language and embedding mode used to parse the guest tree.
    pub source: JsFileSource,
    /// The snippet content in host-document coordinates.
    pub content_range: TextRange,
    /// The host-document offset of guest coordinate zero.
    pub content_offset: TextSize,
    /// Whether parsing the guest tree produced an error diagnostic.
    pub has_errors: bool,
}

impl AstroSnippet<'_> {
    /// Returns the offset that converts ranges from this parsed tree to host-document ranges.
    /// Workspace parses already start at the snippet's host offset, while direct callers may
    /// provide trees whose ranges start at zero.
    pub(crate) fn range_offset(&self) -> TextSize {
        let root_offset = self.root.syntax().text_range_with_trivia().start();
        debug_assert!(root_offset <= self.content_offset);
        self.content_offset
            .checked_sub(root_offset)
            .unwrap_or_default()
    }
}
