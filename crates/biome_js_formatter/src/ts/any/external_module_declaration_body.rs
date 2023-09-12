//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsExternalModuleDeclarationBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsExternalModuleDeclarationBody;
impl FormatRule<AnyTsExternalModuleDeclarationBody> for FormatAnyTsExternalModuleDeclarationBody {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &AnyTsExternalModuleDeclarationBody,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            AnyTsExternalModuleDeclarationBody::TsEmptyExternalModuleDeclarationBody(node) => {
                node.format().fmt(f)
            }
            AnyTsExternalModuleDeclarationBody::TsModuleBlock(node) => node.format().fmt(f),
        }
    }
}
