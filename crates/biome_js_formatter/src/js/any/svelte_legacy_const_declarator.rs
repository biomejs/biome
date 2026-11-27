//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsSvelteLegacyConstDeclarator;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsSvelteLegacyConstDeclarator;
impl FormatRule<AnyJsSvelteLegacyConstDeclarator> for FormatAnyJsSvelteLegacyConstDeclarator {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &AnyJsSvelteLegacyConstDeclarator,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyJsSvelteLegacyConstDeclarator::JsBogus(node) => node.format().fmt(f),
            AnyJsSvelteLegacyConstDeclarator::JsVariableDeclarator(node) => node.format().fmt(f),
        }
    }
}
