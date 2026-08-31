#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::{AstNode, RawSyntaxKind};
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

impl From<u16> for TomlSyntaxKind {
    fn from(value: u16) -> Self {
        assert!(value <= Self::__LAST as u16);
        unsafe { std::mem::transmute::<u16, Self>(value) }
    }
}

impl From<TomlSyntaxKind> for u16 {
    fn from(kind: TomlSyntaxKind) -> Self {
        kind as Self
    }
}

impl biome_rowan::SyntaxKind for TomlSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, Self::TOML_BOGUS | Self::TOML_BOGUS_VALUE)
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyTomlValue::can_cast(*kind) => Self::TOML_BOGUS_VALUE,
            _ => Self::TOML_BOGUS,
        }
    }

    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        matches!(self, Self::TOML_ROOT)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, Self::NEWLINE | Self::WHITESPACE)
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<TomlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: TomlSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            TomlSyntaxKind::NEWLINE => Ok(Self::Newline),
            TomlSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
            TomlSyntaxKind::COMMENT => Ok(Self::SingleLineComment),
            _ => Err(()),
        }
    }
}
