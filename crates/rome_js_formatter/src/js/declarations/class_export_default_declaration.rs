use crate::prelude::*;
use crate::utils::format_class::FormatClass;

use biome_js_syntax::JsClassExportDefaultDeclaration;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsClassExportDefaultDeclaration;

impl FormatNodeRule<JsClassExportDefaultDeclaration> for FormatJsClassExportDefaultDeclaration {
    fn fmt_fields(
        &self,
        node: &JsClassExportDefaultDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        FormatClass::from(&node.clone().into()).fmt(f)
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsClassExportDefaultDeclaration,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        /* Formatted as part of `FormatClass` */
        Ok(())
    }
}
