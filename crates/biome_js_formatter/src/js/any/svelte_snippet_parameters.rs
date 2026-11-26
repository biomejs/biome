//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsSvelteSnippetParameters;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsSvelteSnippetParameters;
impl FormatRule<AnyJsSvelteSnippetParameters> for FormatAnyJsSvelteSnippetParameters {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsSvelteSnippetParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsSvelteSnippetParameters::JsBogus(node) => node.format().fmt(f),
            AnyJsSvelteSnippetParameters::JsParameters(node) => node.format().fmt(f),
        }
    }
}
