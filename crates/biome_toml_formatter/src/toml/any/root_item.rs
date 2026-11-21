//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlRootItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlRootItem;
impl FormatRule<AnyTomlRootItem> for FormatAnyTomlRootItem {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlRootItem, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlRootItem::TomlArrayTable(node) => node.format().fmt(f),
            AnyTomlRootItem::TomlBogus(node) => node.format().fmt(f),
            AnyTomlRootItem::TomlKeyValue(node) => node.format().fmt(f),
            AnyTomlRootItem::TomlTable(node) => node.format().fmt(f),
        }
    }
}
