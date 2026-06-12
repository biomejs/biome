#![deny(clippy::use_self)]

pub mod emphasis_ext;
#[macro_use]
mod generated;
pub mod block_ext;
pub mod inline_ext;
pub mod list_ext;
mod syntax_node;
pub mod text_ext;

pub use syntax_node::*;

pub use self::generated::*;
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind, TriviaPieceKind};

impl From<u16> for MarkdownSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl SyntaxKind for MarkdownSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;

    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            Self::MD_BOGUS | Self::MD_BOGUS_BLOCK | Self::MD_BOGUS_BULLET
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            // A degraded block stays a valid member of `MdBlockList`
            // (`AnyMdBlock` includes `MdBogusBlock`).
            kind if AnyMdBlock::can_cast(*kind) => Self::MD_BOGUS_BLOCK,
            Self::MD_BULLET | Self::MD_BOGUS_BULLET => Self::MD_BOGUS_BULLET,
            _ => Self::MD_BOGUS,
        }
    }

    fn to_raw(&self) -> biome_rowan::RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: biome_rowan::RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::MD_DOCUMENT)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        // Markdown is markup: whitespace is syntactic, and NEWLINE is explicit.
        // We intentionally avoid trivia for whitespace so it becomes part of text.
        false
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<MarkdownSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(_value: MarkdownSyntaxKind) -> Result<Self, Self::Error> {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every bogus kind must report `is_bogus()`, and `to_bogus()` must map
    /// each node kind to a bogus kind that is itself bogus. The parser's list
    /// recovery completes `MD_BOGUS_BULLET` nodes (see
    /// `biome_markdown_parser/src/syntax/list.rs`), so these kinds reach the
    /// tree sink and must round-trip through `SyntaxKind` soundly.
    #[test]
    fn bogus_kinds_are_bogus() {
        for kind in [
            MarkdownSyntaxKind::MD_BOGUS,
            MarkdownSyntaxKind::MD_BOGUS_BLOCK,
            MarkdownSyntaxKind::MD_BOGUS_BULLET,
        ] {
            assert!(
                kind.is_bogus(),
                "{kind:?} must report is_bogus(); update the match in \
                 `SyntaxKind::is_bogus` when adding a bogus kind"
            );
            assert!(
                kind.to_bogus().is_bogus(),
                "to_bogus({kind:?}) returned a non-bogus kind; update the \
                 match in `SyntaxKind::to_bogus`"
            );
        }
    }

    #[test]
    fn to_bogus_preserves_block_context() {
        // A block-level kind must degrade to MD_BOGUS_BLOCK so it stays a
        // valid member of MdBlockList (AnyMdBlock includes MdBogusBlock).
        assert_eq!(
            MarkdownSyntaxKind::MD_PARAGRAPH.to_bogus(),
            MarkdownSyntaxKind::MD_BOGUS_BLOCK
        );
        // A bullet must degrade to MD_BOGUS_BULLET.
        assert_eq!(
            MarkdownSyntaxKind::MD_BULLET.to_bogus(),
            MarkdownSyntaxKind::MD_BOGUS_BULLET
        );
        // Inline kinds fall back to MD_BOGUS.
        assert_eq!(
            MarkdownSyntaxKind::MD_TEXTUAL.to_bogus(),
            MarkdownSyntaxKind::MD_BOGUS
        );
    }
}
