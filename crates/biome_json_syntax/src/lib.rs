#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod file_source;
pub mod member_ext;
pub mod object_ext;
pub mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::{JsonFileSource, JsonFileVariant};
pub use syntax_node::*;

use biome_rowan::{RawSyntaxKind, SyntaxKind, TokenText};

impl From<u16> for JsonSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<JsonSyntaxKind> for u16 {
    fn from(k: JsonSyntaxKind) -> Self {
        k as Self
    }
}

impl JsonSyntaxKind {
    pub fn is_comments(self) -> bool {
        matches!(self, Self::COMMENT | Self::MULTILINE_COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![null] | T![true] | T![false])
    }
}

impl biome_rowan::SyntaxKind for JsonSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, Self::JSON_BOGUS | Self::JSON_BOGUS_VALUE)
    }

    fn to_bogus(&self) -> Self {
        match self {
            Self::JSON_NUMBER_VALUE
            | Self::JSON_STRING_VALUE
            | Self::JSON_BOOLEAN_VALUE
            | Self::JSON_NULL_VALUE
            | Self::JSON_ARRAY_VALUE
            | Self::JSON_OBJECT_VALUE
            | Self::JSON_BOGUS_VALUE => Self::JSON_BOGUS_VALUE,
            _ => Self::JSON_BOGUS,
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
        matches!(self, Self::JSON_ROOT)
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

impl TryFrom<JsonSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: JsonSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsonSyntaxKind::NEWLINE => Ok(Self::Newline),
                JsonSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else if value.is_comments() {
            match value {
                JsonSyntaxKind::COMMENT => Ok(Self::SingleLineComment),
                JsonSyntaxKind::MULTILINE_COMMENT => Ok(Self::MultiLineComment),
                _ => unreachable!("Not Comment"),
            }
        } else {
            Err(())
        }
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &JsonSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == JsonSyntaxKind::JSON_STRING_LITERAL {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
