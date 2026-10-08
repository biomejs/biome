use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::{JsSyntaxToken, JsTemplateChunkElement, TsTemplateChunkElement};
use biome_rowan::{SyntaxResult, declare_node_union};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsTemplateChunkElement;

impl FormatNodeRule<JsTemplateChunkElement> for FormatJsTemplateChunkElement {
    fn fmt_fields(
        &self,
        node: &JsTemplateChunkElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyTemplateChunkElement::from(node.clone()).fmt(formatter)
    }
}

declare_node_union! {
    pub(crate) AnyTemplateChunkElement = JsTemplateChunkElement | TsTemplateChunkElement
}

impl AnyTemplateChunkElement {
    pub(crate) fn template_chunk_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            Self::JsTemplateChunkElement(chunk) => chunk.template_chunk_token(),
            Self::TsTemplateChunkElement(chunk) => chunk.template_chunk_token(),
        }
    }
}

impl Format<JsFormatContext> for AnyTemplateChunkElement {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let chunk = self.template_chunk_token()?;

        write!(
            f,
            [format_replaced(
                &chunk,
                &syntax_token_cow_slice(
                    // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
                    // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'
                    normalize_newlines(chunk.text_trimmed(), ['\r']),
                    &chunk,
                    chunk.text_trimmed_range().start(),
                )
            )]
        )
    }
}
