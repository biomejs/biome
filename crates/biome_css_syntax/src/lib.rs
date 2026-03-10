#![deny(clippy::use_self)]

#[macro_use]
mod file_source;
mod generated;
mod import_ext;
mod scss_ext;
pub mod selector_ext;
pub mod stmt_ext;
mod string_ext;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{
    SyntaxNodeText, TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent,
};
pub use file_source::{CssFileLanguage, CssFileSource, CssVariant, EmbeddingKind};
pub use syntax_node::*;

use crate::CssSyntaxKind::*;
use biome_rowan::{RawSyntaxKind, SyntaxKind, TokenText};

impl From<u16> for CssSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<CssSyntaxKind> for u16 {
    fn from(k: CssSyntaxKind) -> Self {
        k as Self
    }
}

impl CssSyntaxKind {
    /// Returns `true` for any contextual or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        true
    }

    /// Returns `true` for contextual keywords
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u16) >= (MEDIA_KW as u16) && (self as u16) <= (FONT_FACE_KW as u16)
    }

    /// Returns `true` for css-wide keywords
    #[inline]
    pub const fn is_css_wide_keyword(self) -> bool {
        (self as u16) >= (INITIAL_KW as u16) && (self as u16) <= (DEFAULT_KW as u16)
    }

    /// Returns `true` for contextual attribute modifier keywords
    #[inline]
    pub const fn is_attribute_modifier_keyword(self) -> bool {
        let k = self as u16;
        k == (I_KW as u16) || k == (S_KW as u16)
    }

    /// Returns true for all non-contextual keywords (includes future reserved keywords)
    #[inline]
    pub const fn is_non_contextual_keyword(self) -> bool {
        self.is_keyword() && !self.is_contextual_keyword()
    }

    /// Returns true for all _known_ dimension units.
    ///
    /// Note that dimensions allow any identifier as the unit value, but only
    /// these known units will be parsed as a `CssRegularDimension`. All others
    /// will be parsed as `CssUnknownDimension` instead.
    #[inline]
    pub const fn is_known_dimension_unit(self) -> bool {
        (self as u16) >= (EM_KW as u16) && (self as u16) <= (FR_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for CssSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(self, CSS_BOGUS)
    }

    fn to_bogus(&self) -> Self {
        CSS_BOGUS
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
        matches!(self, CSS_ROOT)
    }

    #[inline]
    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::NEWLINE | Self::WHITESPACE | Self::COMMENT | Self::MULTILINE_COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<CssSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: CssSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                CssSyntaxKind::NEWLINE => Ok(Self::Newline),
                CssSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
                CssSyntaxKind::COMMENT => Ok(Self::SingleLineComment),
                CssSyntaxKind::MULTILINE_COMMENT => Ok(Self::MultiLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &CssSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == CssSyntaxKind::CSS_STRING_LITERAL {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
