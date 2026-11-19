use crate::kind::MarkdownSyntaxKind;
use crate::{MdInlineEmphasis, MdInlineItalic};
use biome_rowan::SyntaxResult;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MdEmphasisFence {
    DoubleStar,
    DoubleUnderscore,
}

impl MdEmphasisFence {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DoubleStar => "**",
            Self::DoubleUnderscore => "__",
        }
    }
}

impl MdInlineEmphasis {
    /// Returns the fence style used by this MdInlineEmphasis node.
    pub fn fence(&self) -> SyntaxResult<MdEmphasisFence> {
        let l_fence = self.l_fence()?;
        Ok(match l_fence.kind() {
            MarkdownSyntaxKind::DOUBLE_STAR => MdEmphasisFence::DoubleStar,
            MarkdownSyntaxKind::DOUBLE_UNDERSCORE => MdEmphasisFence::DoubleUnderscore,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum MdItalicFence {
    Star,
    Underscore,
}

impl MdInlineItalic {
    /// Returns the fence style used by this MdInlineItalic node.
    pub fn fence(&self) -> SyntaxResult<MdItalicFence> {
        let l_fence = self.l_fence()?;
        Ok(match l_fence.kind() {
            MarkdownSyntaxKind::STAR => MdItalicFence::Star,
            MarkdownSyntaxKind::UNDERSCORE => MdItalicFence::Underscore,
            _ => unreachable!(),
        })
    }
}
