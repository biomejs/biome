use biome_languages::CssFileSource;
use biome_languages::css::{CssEmbeddingKind, EmbeddingHtmlKind};

/// Framework-specific behavior used while building CSS semantics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SemanticFlavor {
    /// Standard CSS semantics.
    #[default]
    Vanilla,
    /// CSS embedded in an Astro document.
    Astro,
}

impl From<&CssFileSource> for SemanticFlavor {
    fn from(source: &CssFileSource) -> Self {
        if matches!(
            source.as_embedding_kind(),
            CssEmbeddingKind::Html(EmbeddingHtmlKind::Astro { .. })
        ) {
            Self::Astro
        } else {
            Self::Vanilla
        }
    }
}
