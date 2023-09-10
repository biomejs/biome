use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::{TsIndexSignatureParameter, TsIndexSignatureParameterFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexSignatureParameter;

impl FormatNodeRule<TsIndexSignatureParameter> for FormatTsIndexSignatureParameter {
    fn fmt_fields(
        &self,
        node: &TsIndexSignatureParameter,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsIndexSignatureParameterFields {
            binding,
            type_annotation,
        } = node.as_fields();
        let binding = binding.format();
        let type_annotation = type_annotation.format();

        write![f, [binding, type_annotation]]
    }
}
