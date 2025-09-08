#![deny(clippy::use_self)]

#[macro_use]
mod generated;
pub mod metadata;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

use crate::TailwindSyntaxKind::{
    TW_BOGUS, TW_BOGUS_CANDIDATE, TW_BOGUS_MODIFIER, TW_BOGUS_VALUE, TW_BOGUS_VARIANT,
};
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind};

impl From<u16> for TailwindSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<TailwindSyntaxKind> for u16 {
    fn from(k: TailwindSyntaxKind) -> Self {
        k as Self
    }
}

impl biome_rowan::SyntaxKind for TailwindSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            TW_BOGUS | TW_BOGUS_CANDIDATE | TW_BOGUS_MODIFIER | TW_BOGUS_VARIANT
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyTwCandidate::can_cast(*kind) => TW_BOGUS_CANDIDATE,
            kind if AnyTwFullCandidate::can_cast(*kind) => TW_BOGUS_CANDIDATE,
            kind if AnyTwVariant::can_cast(*kind) => TW_BOGUS_VARIANT,
            kind if AnyTwModifier::can_cast(*kind) => TW_BOGUS_MODIFIER,
            kind if AnyTwValue::can_cast(*kind) => TW_BOGUS_VALUE,
            _ => TW_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::TW_ROOT)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, Self::NEWLINE)
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<TailwindSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: TailwindSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            // We intentionally don't consider whitespace to be trivia because it's a required part of the syntax.
            // There must be spaces between Candidates in order for tailwind to parse them.
            match value {
                TailwindSyntaxKind::NEWLINE => Ok(Self::Newline),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
