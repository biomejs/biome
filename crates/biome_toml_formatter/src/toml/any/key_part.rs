//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlKeyPart;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlKeyPart;
impl FormatRule<AnyTomlKeyPart> for FormatAnyTomlKeyPart {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlKeyPart, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlKeyPart::TomlBareKey(node) => node.format().fmt(f),
            AnyTomlKeyPart::TomlBogus(node) => node.format().fmt(f),
            AnyTomlKeyPart::TomlQuotedKey(node) => node.format().fmt(f),
        }
    }
}
