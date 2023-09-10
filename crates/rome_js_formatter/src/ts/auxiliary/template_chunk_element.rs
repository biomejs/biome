use crate::prelude::*;

use crate::js::auxiliary::template_chunk_element::AnyTemplateChunkElement;
use biome_js_syntax::TsTemplateChunkElement;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateChunkElement;

impl FormatNodeRule<TsTemplateChunkElement> for FormatTsTemplateChunkElement {
    fn fmt_fields(
        &self,
        node: &TsTemplateChunkElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyTemplateChunkElement::from(node.clone()).fmt(formatter)
    }
}
