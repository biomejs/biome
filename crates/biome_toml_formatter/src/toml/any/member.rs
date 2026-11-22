//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlMember;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlMember;
impl FormatRule<AnyTomlMember> for FormatAnyTomlMember {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlMember, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlMember::TomlBogusMember(node) => node.format().fmt(f),
            AnyTomlMember::TomlMember(node) => node.format().fmt(f),
        }
    }
}
