//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_json_syntax::AnyJsonMemberName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsonMemberName;
impl FormatRule<AnyJsonMemberName> for FormatAnyJsonMemberName {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &AnyJsonMemberName, f: &mut JsonFormatter) -> FormatResult<()> {
        match node {
            AnyJsonMemberName::JsonBogusName(node) => node.format().fmt(f),
            AnyJsonMemberName::JsonMemberName(node) => node.format().fmt(f),
            AnyJsonMemberName::JsonMetavariable(node) => node.format().fmt(f),
        }
    }
}
