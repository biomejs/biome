#![deny(clippy::use_self)]

#[macro_use]
mod generated;
pub mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

use biome_rowan::RawSyntaxKind;

impl From<u16> for TomlSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<TomlSyntaxKind> for u16 {
    fn from(k: TomlSyntaxKind) -> Self {
        k as Self
    }
}

impl TomlSyntaxKind {
    pub fn is_comments(self) -> bool {
        matches!(self, Self::COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![true] | T![false])
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
            Self::TOML_STRING
            | Self::TOML_INTEGER
            | Self::TOML_FLOAT
            | Self::TOML_BOOLEAN
            | Self::TOML_DATE_TIME
            | Self::TOML_ARRAY
            | Self::TOML_INLINE_TABLE
            | Self::TOML_BOGUS_VALUE => Self::TOML_BOGUS_VALUE,
            _ => Self::TOML_BOGUS,
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
        matches!(self, Self::TOML_ROOT)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, Self::NEWLINE | Self::WHITESPACE | Self::COMMENT)
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
