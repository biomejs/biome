//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlValue;
impl FormatRule<AnyTomlValue> for FormatAnyTomlValue {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlValue, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlValue::TomlArray(node) => node.format().fmt(f),
            AnyTomlValue::TomlBogusValue(node) => node.format().fmt(f),
            AnyTomlValue::TomlBoolean(node) => node.format().fmt(f),
            AnyTomlValue::TomlDateTime(node) => node.format().fmt(f),
            AnyTomlValue::TomlFloat(node) => node.format().fmt(f),
            AnyTomlValue::TomlInlineTable(node) => node.format().fmt(f),
            AnyTomlValue::TomlInteger(node) => node.format().fmt(f),
            AnyTomlValue::TomlString(node) => node.format().fmt(f),
        }
    }
}
