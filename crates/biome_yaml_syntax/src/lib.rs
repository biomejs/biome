#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod anchor_ext;
mod block_ext;
mod flow_ext;
mod syntax_node;

pub use self::generated::*;
use biome_rowan::{AstNode, RawSyntaxKind, TokenText};
pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use block_ext::{AnyYamlBlockScalar, AnyYamlEntryValue};
pub use flow_ext::AnyYamlFlowScalar;
pub use syntax_node::*;

/// Text of `token`, excluding all trivia and removing quotes if `token` is a quoted scalar.
///
/// An unterminated quoted scalar only loses its opening quote. The text of a plain scalar
/// excludes the blanks that end it, which its token includes when a line break follows them.
pub fn inner_string_text(token: &YamlSyntaxToken) -> TokenText {
    let text = token.token_text_trimmed();
    let range = match token.kind() {
        YamlSyntaxKind::SINGLE_QUOTED_LITERAL => range_without_quotes(&text, '\''),
        YamlSyntaxKind::DOUBLE_QUOTED_LITERAL => range_without_quotes(&text, '"'),
        YamlSyntaxKind::PLAIN_LITERAL => {
            TextRange::up_to(text.trim_end_matches([' ', '\t']).text_len())
        }
        _ => return text,
    };
    text.slice(range)
}

/// Returns the range of `text` without its opening quote, and without its closing `quote` when
/// it has one.
fn range_without_quotes(text: &str, quote: char) -> TextRange {
    let len = text.text_len();
    let end = if len > TextSize::from(1) && text.ends_with(quote) {
        len - TextSize::from(1)
    } else {
        len
    };
    TextRange::new(TextSize::from(1), end)
}

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
