#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod file_source;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::{AstNode, RawSyntaxKind};
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use syntax_node::*;

impl From<u16> for YamlSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<YamlSyntaxKind> for u16 {
    fn from(k: YamlSyntaxKind) -> Self {
        k as Self
    }
}

impl biome_rowan::SyntaxKind for YamlSyntaxKind {
    const TOMBSTONE: Self = Self::TOMBSTONE;
    const EOF: Self = Self::EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            Self::YAML_BOGUS
                | Self::YAML_BOGUS_BLOCK_NODE
                | Self::YAML_BOGUS_BLOCK_MAP_ENTRY
                | Self::YAML_BOGUS_BLOCK_HEADER
                | Self::YAML_BOGUS_FLOW_NODE
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyYamlBlockMapEntry::can_cast(*kind) => Self::YAML_BOGUS_BLOCK_MAP_ENTRY,
            kind if AnyYamlBlockNode::can_cast(*kind) => Self::YAML_BOGUS_BLOCK_NODE,
            kind if AnyYamlBlockHeader::can_cast(*kind) => Self::YAML_BOGUS_BLOCK_HEADER,
            kind if AnyYamlFlowNode::can_cast(*kind) => Self::YAML_BOGUS_FLOW_NODE,
            _ => Self::YAML_BOGUS,
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
        matches!(self, Self::YAML_ROOT)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(self, Self::WHITESPACE | Self::COMMENT)
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<YamlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: YamlSyntaxKind) -> Result<Self, Self::Error> {
        match value {
            YamlSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
            YamlSyntaxKind::COMMENT => Ok(Self::SingleLineComment),
            YamlSyntaxKind::NEWLINE => Ok(Self::Newline),
            _ => Err(()),
        }
    }
}
