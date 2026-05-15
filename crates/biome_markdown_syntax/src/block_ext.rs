use crate::list_ext::AnyListItem;
use crate::{AnyMdBlock, AnyMdCodeBlock, AnyMdContainerBlock, AnyMdLeafBlock};

impl AnyMdBlock {
    pub const fn is_fenced_block(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::AnyMdCodeBlock(
                AnyMdCodeBlock::MdFencedCodeBlock(_)
            ))
        )
    }

    pub const fn is_list(&self) -> bool {
        matches!(
            self,
            Self::AnyMdContainerBlock(
                AnyMdContainerBlock::MdBulletListItem(_)
                    | AnyMdContainerBlock::MdOrderedListItem(_)
            )
        )
    }

    pub const fn is_continuation_indent(&self) -> bool {
        matches!(
            self,
            Self::AnyMdLeafBlock(AnyMdLeafBlock::MdContinuationIndent(_))
        )
    }

    pub const fn is_paragraph(&self) -> bool {
        matches!(self, Self::AnyMdLeafBlock(AnyMdLeafBlock::MdParagraph(_)))
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

    pub fn as_any_list_item(&self) -> Option<AnyListItem> {
        match self {
            AnyMdBlock::AnyMdContainerBlock(AnyMdContainerBlock::MdBulletListItem(item)) => {
                Some(AnyListItem::MdBulletListItem(item.clone()))
            }
            AnyMdBlock::AnyMdContainerBlock(AnyMdContainerBlock::MdOrderedListItem(item)) => {
                Some(AnyListItem::MdOrderedListItem(item.clone()))
            }
            _ => None,
        }
    }
}
