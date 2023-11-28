use crate::prelude::*;

use crate::js::bindings::parameters::FormatAnyJsParameters;
use biome_js_syntax::JsConstructorParameters;

use super::parameters::{AnyJsParameters, FormatJsParametersOptions};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsConstructorParameters;

impl FormatNodeRule<JsConstructorParameters> for FormatJsConstructorParameters {
    fn fmt_fields(&self, node: &JsConstructorParameters, f: &mut JsFormatter) -> FormatResult<()> {
        FormatAnyJsParameters::new(
            AnyJsParameters::JsConstructorParameters(node.clone()),
            FormatJsParametersOptions::default(),
        )
        .fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsConstructorParameters,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `FormatJsAnyParameters
        Ok(())
    }
}
