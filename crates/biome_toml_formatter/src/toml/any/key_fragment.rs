//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_toml_syntax::AnyTomlKeyFragment;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTomlKeyFragment;
impl FormatRule<AnyTomlKeyFragment> for FormatAnyTomlKeyFragment {
    type Context = TomlFormatContext;
    fn fmt(&self, node: &AnyTomlKeyFragment, f: &mut TomlFormatter) -> FormatResult<()> {
        match node {
            AnyTomlKeyFragment::TomlBareKey(node) => node.format().fmt(f),
            AnyTomlKeyFragment::TomlBogusKeyFragment(node) => node.format().fmt(f),
            AnyTomlKeyFragment::TomlQuotedKey(node) => node.format().fmt(f),
        }
    }
}
