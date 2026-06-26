#[cfg(feature = "html_embeds")]
pub(crate) mod html;
#[cfg(feature = "js_embeds")]
pub(crate) mod js;

use biome_rowan::{TextRange, TextSize, TokenText};

/// The text content and position information for an embed site.
#[derive(Clone)]
#[cfg_attr(
    not(any(feature = "html_embeds", feature = "js_embeds")),
    expect(
        dead_code,
        reason = "Embed content fields are read only when embed features are enabled."
    )
)]
pub(crate) struct EmbedContent {
    /// The text range of the entire host element (including tags/delimiters).
    pub element_range: TextRange,

    /// The text range of just the embedded content.
    pub content_range: TextRange,

    /// Offset where embedded content starts in the parent document.
    pub content_offset: TextSize,

    /// The raw text of the embedded content.
    pub text: TokenText,
}
