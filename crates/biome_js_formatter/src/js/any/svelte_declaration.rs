//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsSvelteDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsSvelteDeclaration;
impl FormatRule<AnyJsSvelteDeclaration> for FormatAnyJsSvelteDeclaration {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsSvelteDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsSvelteDeclaration::JsBogusVariableDeclaration(node) => node.format().fmt(f),
            AnyJsSvelteDeclaration::JsVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
