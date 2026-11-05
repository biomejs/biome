use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_rowan::{TextRange, TextSize};

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
