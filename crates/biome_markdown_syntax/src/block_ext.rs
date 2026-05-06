use crate::{AnyMdBlock, AnyMdCodeBlock, AnyMdLeafBlock};

impl AnyMdBlock {
    pub const fn is_fenced_block(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                AnyMdCodeBlock::MdFencedCodeBlock(_)
            ))
        )
    }

    /// Whether the block is a header or setext header.
    pub const fn is_any_header(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdHeader(_) | AnyMdLeafBlock::MdSetextHeader(_))
        )
    }

    pub const fn is_newline(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_)))
    }
}
