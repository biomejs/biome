#![deny(clippy::use_self)]

mod generated;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{
    SyntaxNodeText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use syntax_node::*;

use crate::GlimmerSyntaxKind::*;
use biome_rowan::RawSyntaxKind;

impl From<u16> for GlimmerSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<GlimmerSyntaxKind> for u16 {
    fn from(k: GlimmerSyntaxKind) -> Self {
        k as Self
    }
}

impl GlimmerSyntaxKind {
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            AS_KW
                | IF_KW
                | ELSE_KW
                | EACH_KW
                | LET_KW
                | YIELD_KW
                | THIS_KW
                | TRUE_KW
                | FALSE_KW
                | NULL_KW
                | UNDEFINED_KW
        )
    }

    pub const fn is_trivia(self) -> bool {
        matches!(self, WHITESPACE | NEWLINE | COMMENT)
    }

    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            L_CURLY2
                | R_CURLY2
                | L_PAREN
                | R_PAREN
                | L_ANGLE
                | R_ANGLE
                | SLASH
                | DOT
                | PIPE
                | EQ
                | HASH
                | AT
        )
    }
}

impl biome_rowan::SyntaxKind for GlimmerSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            Self::GLIMMER_BOGUS | Self::GLIMMER_BOGUS_STATEMENT | Self::GLIMMER_BOGUS_EXPRESSION
        )
    }

    fn to_bogus(&self) -> Self {
        Self::GLIMMER_BOGUS
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
        matches!(self, Self::GLIMMER_ROOT)
    }

    fn is_list(&self) -> bool {
        GlimmerSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        GlimmerSyntaxKind::is_trivia(self)
    }

    fn to_string(&self) -> Option<&'static str> {
        GlimmerSyntaxKind::to_string(self)
    }
}
