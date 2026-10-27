//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlKeyValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlKeyValue;
impl FormatRule<AnyTomlKeyValue> for FormatAnyTomlKeyValue {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlKeyValue, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlKeyValue::TomlBogus(node) => node.format().fmt(f),
            AnyTomlKeyValue::TomlKeyValue(node) => node.format().fmt(f),
        }
    }
}
