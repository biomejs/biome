use crate::list_ext::AnyListItem;
use crate::{AnyMdBlock, AnyMdCodeBlock, AnyMdContainerBlock, AnyMdLeafBlock, MdParagraph};
use biome_rowan::AstNodeList;

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
            Self::AnyMdContainerBlock(AnyMdContainerBlock::MdBulletListItem(item)) => {
                Some(AnyListItem::MdBulletListItem(item.clone()))
            }
            Self::AnyMdContainerBlock(AnyMdContainerBlock::MdOrderedListItem(item)) => {
                Some(AnyListItem::MdOrderedListItem(item.clone()))
            }
            _ => None,
        }
    }
}

impl MdParagraph {
    pub fn ends_with_newline(&self) -> bool {
        self.list().last().is_some_and(|item| {
            item.as_md_textual()
                .is_some_and(|textual| textual.is_newline().unwrap_or_default())
        })
    }

    pub fn ends_with_double_newline(&self) -> bool {
        let mut iter = self.list().iter();
        let last = iter.next_back();
        let penultimate = iter.next_back();

        match (last, penultimate) {
            (Some(last), Some(penultimate)) => {
                last.as_md_textual()
                    .is_some_and(|textual| textual.is_newline().unwrap_or_default())
                    && penultimate
                        .as_md_textual()
                        .is_some_and(|textual| textual.is_newline().unwrap_or_default())
            }
            _ => false,
        }
    }
}
