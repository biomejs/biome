#![deny(clippy::use_self)]

mod file_source;
mod generated;
mod syntax_node;

pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::GlimmerFileSource;
pub use generated::*;
pub use syntax_node::*;

use biome_rowan::{RawSyntaxKind, SyntaxKind};

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
    pub fn is_comments(self) -> bool {
        matches!(self, Self::COMMENT | Self::MUSTACHE_COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(
            self,
            T![null] | T![true] | T![false] | T![undefined] | T![this]
        )
    }

    #[inline]
    pub const fn is_literal(self) -> bool {
        matches!(
            self,
            Self::GLIMMER_STRING_LITERAL
                | Self::GLIMMER_NUMBER_LITERAL
                | Self::GLIMMER_BOOLEAN_LITERAL
                | Self::GLIMMER_NULL_LITERAL
                | Self::GLIMMER_UNDEFINED_LITERAL
        )
    }
}

impl biome_rowan::SyntaxKind for GlimmerSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            Self::GLIMMER_BOGUS
                | Self::GLIMMER_BOGUS_EXPRESSION
                | Self::GLIMMER_BOGUS_STATEMENT
                | Self::GLIMMER_BOGUS_ATTRIBUTE
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if GlimmerExpression::can_cast(*kind) => Self::GLIMMER_BOGUS_EXPRESSION,
            kind if GlimmerStatement::can_cast(*kind) => Self::GLIMMER_BOGUS_STATEMENT,
            kind if GlimmerAttribute::can_cast(*kind) => Self::GLIMMER_BOGUS_ATTRIBUTE,
            _ => Self::GLIMMER_BOGUS,
        }
    }

    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

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
        matches!(
            self,
            Self::NEWLINE | Self::WHITESPACE | Self::COMMENT | Self::MUSTACHE_COMMENT
        )
    }

    fn from_keyword(keyword: &str) -> Option<Self> {
        let kw = match keyword {
            "null" => Self::NULL_KW,
            "true" => Self::TRUE_KW,
            "false" => Self::FALSE_KW,
            "undefined" => Self::UNDEFINED_KW,
            "this" => Self::THIS_KW,
            _ => return None,
        };

        Some(kw)
    }

    fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            Self::NULL_KW => "null",
            Self::TRUE_KW => "true",
            Self::FALSE_KW => "false",
            Self::UNDEFINED_KW => "undefined",
            Self::THIS_KW => "this",
            
            Self::L_CURLY2 => "{{",
            Self::R_CURLY2 => "}}",
            Self::HASH => "#",
            Self::SLASH => "/",
            Self::AT => "@",
            Self::DOT => ".",
            Self::L_PAREN => "(",
            Self::R_PAREN => ")",
            Self::EQ => "=",
            Self::L_ANGLE => "<",
            Self::R_ANGLE => ">",
            Self::BANG => "!",
            
            _ => return None,
        };

        Some(tok)
    }
}

/// Bogus nodes
pub use crate::generated::kind::GlimmerSyntaxKind::{
    GLIMMER_BOGUS, GLIMMER_BOGUS_ATTRIBUTE, GLIMMER_BOGUS_EXPRESSION, GLIMMER_BOGUS_STATEMENT,
};

// Re-export common types for convenience
pub type GlimmerSyntaxNode = biome_rowan::SyntaxNode<GlimmerLanguage>;
pub type GlimmerSyntaxToken = biome_rowan::SyntaxToken<GlimmerLanguage>;
pub type GlimmerSyntaxElement = biome_rowan::SyntaxElement<GlimmerLanguage>;
pub type GlimmerSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<GlimmerLanguage>;
pub type GlimmerSyntaxElementChildren = biome_rowan::SyntaxElementChildren<GlimmerLanguage>;
pub type GlimmerSyntaxList = biome_rowan::SyntaxList<GlimmerLanguage>;
