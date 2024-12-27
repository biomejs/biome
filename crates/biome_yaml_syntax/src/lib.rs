#[macro_use]
mod generated;
mod file_source;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::RawSyntaxKind;
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

impl From<u16> for YamlSyntaxKind {
    fn from(d: u16) -> YamlSyntaxKind {
        assert!(d <= (YamlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, YamlSyntaxKind>(d) }
    }
}

impl From<YamlSyntaxKind> for u16 {
    fn from(k: YamlSyntaxKind) -> u16 {
        k as u16
    }
}

impl biome_rowan::SyntaxKind for YamlSyntaxKind {
    const TOMBSTONE: Self = YamlSyntaxKind::TOMBSTONE;
    const EOF: Self = YamlSyntaxKind::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            YamlSyntaxKind::YAML_BOGUS | YamlSyntaxKind::YAML_BOGUS_VALUE
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            YamlSyntaxKind::YAML_BOGUS_VALUE => YamlSyntaxKind::YAML_BOGUS_VALUE,
            _ => YamlSyntaxKind::YAML_BOGUS,
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
        matches!(self, YamlSyntaxKind::YAML_ROOT)
    }

    fn is_list(&self) -> bool {
        YamlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            YamlSyntaxKind::NEWLINE | YamlSyntaxKind::WHITESPACE | YamlSyntaxKind::COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        YamlSyntaxKind::to_string(self)
    }
}

impl YamlSyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, YamlSyntaxKind::NEWLINE | YamlSyntaxKind::WHITESPACE)
    }

    pub fn is_comments(self) -> bool {
        matches!(self, YamlSyntaxKind::COMMENT)
    }

    #[inline]
    pub const fn is_keyword(self) -> bool {
        matches!(self, T![null])
    }
}

impl TryFrom<YamlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: YamlSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                YamlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                YamlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                _ => unreachable!("Not Trivia"),
            }
        } else if value.is_comments() {
            match value {
                YamlSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Comment"),
            }
        } else {
            Err(())
        }
    }
}
