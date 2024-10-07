#[macro_use]
mod generated;
mod file_source;
mod syntax_node;

pub use self::generated::*;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::HtmlFileSource;
pub use syntax_node::*;

use crate::HtmlSyntaxKind::{
    HTML_BOGUS, HTML_BOGUS_ATTRIBUTE, HTML_BOGUS_ELEMENT, HTML_CLOSING_ELEMENT,
};
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind, TokenText};

impl From<u16> for HtmlSyntaxKind {
    fn from(d: u16) -> HtmlSyntaxKind {
        assert!(d <= (HtmlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, HtmlSyntaxKind>(d) }
    }
}

impl From<HtmlSyntaxKind> for u16 {
    fn from(k: HtmlSyntaxKind) -> u16 {
        k as u16
    }
}

impl HtmlSyntaxKind {
    pub fn is_comments(self) -> bool {
        matches!(self, HtmlSyntaxKind::HTML_COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![null] | T![true] | T![false])
    }
}

impl biome_rowan::SyntaxKind for HtmlSyntaxKind {
    const TOMBSTONE: Self = HtmlSyntaxKind::TOMBSTONE;
    const EOF: Self = HtmlSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            HtmlSyntaxKind::HTML_BOGUS
                | HtmlSyntaxKind::HTML_BOGUS_ATTRIBUTE
                | HtmlSyntaxKind::HTML_BOGUS_ELEMENT
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyHtmlAttribute::can_cast(*kind) => HTML_BOGUS_ATTRIBUTE,
            kind if AnyHtmlElement::can_cast(*kind) => HTML_BOGUS_ELEMENT,
            HTML_CLOSING_ELEMENT => HTML_BOGUS_ELEMENT,

            _ => HTML_BOGUS,
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
        matches!(self, HtmlSyntaxKind::HTML_ROOT)
    }

    fn is_list(&self) -> bool {
        HtmlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, HtmlSyntaxKind::NEWLINE | HtmlSyntaxKind::WHITESPACE)
    }

    fn to_string(&self) -> Option<&'static str> {
        HtmlSyntaxKind::to_string(self)
    }
}

impl TryFrom<HtmlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: HtmlSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                HtmlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                HtmlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else if value.is_comments() {
            match value {
                HtmlSyntaxKind::HTML_COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Comment"),
            }
        } else {
            Err(())
        }
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &HtmlSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if token.kind() == HtmlSyntaxKind::HTML_STRING_LITERAL {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}
