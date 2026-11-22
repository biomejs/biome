//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlString;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlString;
impl FormatRule<AnyTomlString> for FormatAnyTomlString {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlString, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlString::TomlBasicString(node) => node.format().fmt(f),
            AnyTomlString::TomlLiteralString(node) => node.format().fmt(f),
            AnyTomlString::TomlMultilineBasicString(node) => node.format().fmt(f),
            AnyTomlString::TomlMultilineLiteralString(node) => node.format().fmt(f),
        }
    }
}
