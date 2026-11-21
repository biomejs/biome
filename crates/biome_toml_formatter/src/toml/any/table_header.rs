//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlTableHeader;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlTableHeader;
impl FormatRule<AnyTomlTableHeader> for FormatAnyTomlTableHeader {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlTableHeader, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlTableHeader::TomlBogusTableHeader(node) => node.format().fmt(f),
            AnyTomlTableHeader::TomlTableHeader(node) => node.format().fmt(f),
        }
    }
}
