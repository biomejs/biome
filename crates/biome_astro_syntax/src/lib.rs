#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod file_source;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::{AstroFileSource, AstroVariant};
pub use syntax_node::*;

use crate::AstroSyntaxKind::{
    ASTRO_BOGUS, ASTRO_BOGUS_ATTRIBUTE, ASTRO_BOGUS_ELEMENT, ASTRO_BOGUS_EXPRESSION,
};
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind, TokenText};

impl From<u16> for AstroSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<AstroSyntaxKind> for u16 {
    fn from(k: AstroSyntaxKind) -> Self {
        k as Self
    }
}

impl AstroSyntaxKind {
    pub fn is_comments(self) -> bool {
        matches!(self, Self::ASTRO_COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![null] | T![true] | T![false])
    }

    pub fn is_expression(self) -> bool {
        matches!(self, Self::ASTRO_EXPRESSION)
    }

    pub fn is_frontmatter(self) -> bool {
        matches!(self, Self::ASTRO_FRONTMATTER)
    }

    pub fn is_element(self) -> bool {
        matches!(
            self,
            Self::ASTRO_ELEMENT | Self::ASTRO_SELF_CLOSING_ELEMENT
        )
    }

    pub fn is_component(self) -> bool {
        matches!(
            self,
            Self::ASTRO_COMPONENT | Self::ASTRO_SELF_CLOSING_COMPONENT
        )
    }

    pub fn is_attribute(self) -> bool {
        matches!(
            self,
            Self::ASTRO_ATTRIBUTE
                | Self::ASTRO_SHORTHAND_ATTRIBUTE
                | Self::ASTRO_SPREAD_ATTRIBUTE
                | Self::ASTRO_EXPRESSION_ATTRIBUTE
                | Self::ASTRO_TEMPLATE_LITERAL_ATTRIBUTE
        )
    }
}

impl biome_rowan::SyntaxKind for AstroSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            Self::ASTRO_BOGUS
                | Self::ASTRO_BOGUS_ATTRIBUTE
                | Self::ASTRO_BOGUS_ELEMENT
                | Self::ASTRO_BOGUS_EXPRESSION
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyAstroAttribute::can_cast(*kind) => ASTRO_BOGUS_ATTRIBUTE,
            kind if AnyAstroElement::can_cast(*kind) => ASTRO_BOGUS_ELEMENT,
            kind if self.is_expression() => ASTRO_BOGUS_EXPRESSION,
            _ => ASTRO_BOGUS,
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
        matches!(self, Self::ASTRO_ROOT)
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

impl TryFrom<AstroSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: AstroSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                AstroSyntaxKind::NEWLINE => Ok(Self::Newline),
                AstroSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else if value.is_comments() {
            match value {
                AstroSyntaxKind::ASTRO_COMMENT => Ok(Self::MultiLineComment),
                _ => unreachable!("Not Comment"),
            }
        } else {
            Err(())
        }
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &AstroSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == AstroSyntaxKind::ASTRO_STRING_LITERAL {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}