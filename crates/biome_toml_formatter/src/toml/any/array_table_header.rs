//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlArrayTableHeader;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlArrayTableHeader;
impl FormatRule<AnyTomlArrayTableHeader> for FormatAnyTomlArrayTableHeader {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlArrayTableHeader, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlArrayTableHeader::TomlArrayTableHeader(node) => node.format().fmt(f),
            AnyTomlArrayTableHeader::TomlBogusTableHeader(node) => node.format().fmt(f),
        }
    }
}
