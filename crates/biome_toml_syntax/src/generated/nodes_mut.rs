//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{TomlSyntaxToken as SyntaxToken, generated::nodes::*};
use biome_rowan::AstNode;
use std::iter::once;
impl TomlArray {
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_elements(self, element: TomlArrayElementList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl TomlArrayTable {
    pub fn with_l_double_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_key(self, element: TomlKey) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_double_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_entries(self, element: TomlKeyValueList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl TomlBareKey {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlBoolean {
    pub fn with_value_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlDateTime {
    pub fn with_value_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlFloat {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlInlineTable {
    pub fn with_l_curly_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_entries(self, element: TomlInlineTableEntryList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_curly_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl TomlInteger {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlKey {
    pub fn with_parts(self, element: TomlKeyPartList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl TomlKeyValue {
    pub fn with_key(self, element: TomlKey) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eq_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: AnyTomlValue) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl TomlQuotedKey {
    pub fn with_value_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlRoot {
    pub fn with_bom_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_items(self, element: TomlRootItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eof_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl TomlString {
    pub fn with_value_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl TomlTable {
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_key(self, element: TomlKey) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_entries(self, element: TomlKeyValueList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
