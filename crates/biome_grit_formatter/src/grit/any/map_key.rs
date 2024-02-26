//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::GritMapKey;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritMapKey;
impl FormatRule<GritMapKey> for FormatGritMapKey {
    type Context = GritFormatContext;
    fn fmt(&self, node: &GritMapKey, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            GritMapKey::GritName(node) => node.format().fmt(f),
            GritMapKey::GritVariable(node) => node.format().fmt(f),
        }
    }
}
