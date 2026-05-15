use crate::MarkdownSyntaxKind::MD_ORDERED_LIST_MARKER;
use crate::{MdBulletList, MdListMarkerPrefix};
use crate::{MdBulletListItem, MdOrderedListItem};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug)]
pub enum ListMarker {
    Ordered,
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
    pub fn is_ordered(&self) -> bool {
        matches!(self, ListMarker::Ordered)
    }
}

impl MdListMarkerPrefix {
    pub fn list_marker(&self) -> SyntaxResult<ListMarker> {
        let marker = self.marker()?;
        if marker.kind() == MD_ORDERED_LIST_MARKER {
            Ok(ListMarker::Ordered)
        } else {
            Ok(ListMarker::Unordered)
        }
    }
}
