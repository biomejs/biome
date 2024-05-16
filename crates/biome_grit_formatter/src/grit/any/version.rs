//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritVersion;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritVersion;
impl FormatRule<AnyGritVersion> for FormatAnyGritVersion {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritVersion, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritVersion::GritBogusVersion(node) => node.format().fmt(f),
            AnyGritVersion::GritVersion(node) => node.format().fmt(f),
        }
    }
}
