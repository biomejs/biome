//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlKey;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlKey;
impl FormatRule<AnyTomlKey> for FormatAnyTomlKey {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlKey, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlKey::TomlBogusKey(node) => node.format().fmt(f),
            AnyTomlKey::TomlKey(node) => node.format().fmt(f),
        }
    }
}
