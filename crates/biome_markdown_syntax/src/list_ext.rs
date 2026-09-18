use crate::MarkdownSyntaxKind::MD_ORDERED_LIST_MARKER;
use crate::{MdBullet, MdBulletList, MdListMarkerPrefix};
use crate::{MdBulletListItem, MdOrderedListItem};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ListMarker {
    /// `1.` or `1)`
    Ordered,
    /// Only markers with `1)`
    OrderedWithParen,
    /// `- 1`
    Minus,
    /// `* 1`
    Star,
    /// `+ 1`
    Plus,
    /// Any other marker
    Unordered,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum OrderedListDelimiter {
    /// The marker is written with a dot, for example `1.`.
    Dot,
    /// The marker is written with a closing parenthesis, for example `1)`.
    Paren,
}

declare_node_union! {
    pub AnyListItem = MdBulletListItem | MdOrderedListItem
}

impl AnyListItem {
    pub fn list(&self) -> MdBulletList {
        match self {
            Self::MdBulletListItem(node) => node.md_bullet_list(),
            Self::MdOrderedListItem(node) => node.md_bullet_list(),
        }
    }
}

impl ListMarker {
    /// `true` if [ListMarker::Ordered] or [ListMarker::OrderedWithParen]
    pub const fn is_ordered(&self) -> bool {
        matches!(self, Self::Ordered | Self::OrderedWithParen)
    }

    /// Ordered marker that uses parentheses e.g. `1)`
    pub const fn is_ordered_with_paren(&self) -> bool {
        matches!(self, Self::OrderedWithParen)
    }

    pub const fn is_minus(&self) -> bool {
        matches!(self, Self::Minus)
    }

    /// Returns the marker text for unordered list markers.
    ///
    /// Ordered markers include a number, so they do not have a fixed text value.
    pub const fn unordered_marker_text(&self) -> Option<&'static str> {
        match self {
            Self::Minus => Some("-"),
            Self::Star => Some("*"),
            Self::Plus => Some("+"),
            _ => None,
        }
    }
}

impl OrderedListDelimiter {
    /// Returns the delimiter text that appears after the ordered-list number.
    pub const fn marker_text(self) -> &'static str {
        match self {
            Self::Dot => ".",
            Self::Paren => ")",
        }
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

    pub fn post_marker_len(&self) -> Option<usize> {
        Some(self.post_marker_space_token()?.text_trimmed().len())
    }

    /// Reads the number from an ordered list marker.
    ///
    /// This accepts both ordered marker styles: `1.` and `1)`. It strips the
    /// final delimiter and parses the remaining text as a number.
    pub fn ordered_marker_number(&self) -> Option<usize> {
        if !self.list_marker().ok()?.is_ordered() {
            return None;
        }

        let marker = self.marker().ok()?;
        let marker_text = marker.text_trimmed();
        let number_text = marker_text
            .strip_suffix('.')
            .or_else(|| marker_text.strip_suffix(')'))?;

        number_text.parse().ok()
    }
}

impl MdBullet {
    /// Reads the number from this bullet's ordered list marker.
    ///
    /// This returns `None` when the bullet is unordered, when the marker is
    /// missing, or when the marker text cannot be parsed as a number.
    pub fn ordered_marker_number(&self) -> Option<usize> {
        self.prefix().ok()?.ordered_marker_number()
    }
}
