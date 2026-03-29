use crate::prelude::*;

use crate::js::auxiliary::template_chunk_element::{
    AnyTemplateChunkElement, embedded_template_chunk_range,
};
use biome_js_syntax::TsTemplateChunkElement;
use biome_text_size::TextRange;

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

    fn embedded_node_range(
        &self,
        node: &TsTemplateChunkElement,
        f: &mut JsFormatter,
    ) -> Option<TextRange> {
        embedded_template_chunk_range(&AnyTemplateChunkElement::from(node.clone()), f)
    }
}
