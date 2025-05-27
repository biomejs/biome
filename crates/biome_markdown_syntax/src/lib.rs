#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::{RawSyntaxKind, SyntaxKind, TriviaPieceKind};
pub use syntax_node::*;

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
        matches!(self, Self::MD_BOGUS)
    }

    fn to_bogus(&self) -> Self {
        Self::MD_BOGUS
    }

    fn to_raw(&self) -> biome_rowan::RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: biome_rowan::RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        todo!()
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, Self::NEWLINE | Self::WHITESPACE | Self::TAB)
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<MarkdownSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: MarkdownSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                MarkdownSyntaxKind::NEWLINE => Ok(Self::Newline),
                MarkdownSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
                MarkdownSyntaxKind::TAB => Ok(Self::Skipped),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
