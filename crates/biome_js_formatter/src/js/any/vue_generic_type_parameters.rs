//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsVueGenericTypeParameters;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsVueGenericTypeParameters;
impl FormatRule<AnyJsVueGenericTypeParameters> for FormatAnyJsVueGenericTypeParameters {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsVueGenericTypeParameters, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsVueGenericTypeParameters::JsBogus(node) => node.format().fmt(f),
            AnyJsVueGenericTypeParameters::TsTypeParameterList(node) => node.format().fmt(f),
        }
    }
}
