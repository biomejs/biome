//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyTsIdentifierBinding;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsIdentifierBinding;
impl FormatRule<AnyTsIdentifierBinding> for FormatAnyTsIdentifierBinding {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsIdentifierBinding, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsIdentifierBinding::JsMetavariable(node) => node.format().fmt(f),
            AnyTsIdentifierBinding::TsIdentifierBinding(node) => node.format().fmt(f),
        }
    }
}
