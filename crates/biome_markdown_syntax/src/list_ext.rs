use crate::MarkdownSyntaxKind::MD_ORDERED_LIST_MARKER;
use crate::{MdBulletList, MdListMarkerPrefix};
use crate::{MdBulletListItem, MdOrderedListItem};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug)]
pub enum ListMarker {
    Ordered,
    OrderedWithParen,
    Minus,
    Star,
    Plus,
    Unordered,
}

declare_node_union! {
    pub AnyListItem = MdBulletListItem | MdOrderedListItem
}

impl AnyListItem {
    pub fn list(&self) -> MdBulletList {
        match self {
            AnyListItem::MdBulletListItem(node) => node.md_bullet_list(),
            AnyListItem::MdOrderedListItem(node) => node.md_bullet_list(),
        }
    }
}

impl ListMarker {
    pub const fn is_ordered(&self) -> bool {
        matches!(self, ListMarker::Ordered | ListMarker::OrderedWithParen)
    }

    /// Ordered marker that uses parentheses e.g. `1)`
    pub const fn is_ordered_with_paren(&self) -> bool {
        matches!(self, ListMarker::OrderedWithParen)
    }

    pub const fn is_minus(&self) -> bool {
        matches!(self, ListMarker::Minus)
    }
}

impl MdListMarkerPrefix {
    pub fn list_marker(&self) -> SyntaxResult<ListMarker> {
        let marker = self.marker()?;
        if marker.kind() == MD_ORDERED_LIST_MARKER {
            if marker.text_trimmed().ends_with(')') {
                return Ok(ListMarker::OrderedWithParen);
            }
            Ok(ListMarker::Ordered)
        } else if marker.kind() == T![-] {
            Ok(ListMarker::Minus)
        } else if marker.kind() == T![*] {
            Ok(ListMarker::Star)
        } else if marker.kind() == T![+] {
            Ok(ListMarker::Plus)
        } else {
            Ok(ListMarker::Unordered)
        }
    }
}
