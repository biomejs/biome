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

    pub const fn is_newline(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdNewline(_)))
    }
}
