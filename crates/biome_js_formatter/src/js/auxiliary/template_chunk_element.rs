use crate::prelude::*;
use biome_formatter::FormatContext;
use biome_formatter::write;

use biome_js_syntax::{JsSyntaxToken, JsTemplateChunkElement, TsTemplateChunkElement};
use biome_rowan::{SyntaxResult, declare_node_union};
use biome_text_size::TextRange;

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

    fn embedded_node_range(
        &self,
        node: &JsTemplateChunkElement,
        f: &mut JsFormatter,
    ) -> Option<TextRange> {
        let embedded_node_ranges = f.context().embedded_node_ranges();
        if embedded_node_ranges.is_empty() {
            return None;
        }

        let transformed_range = node.template_chunk_token().ok()?.text_range();

        // Map the range back to the original source positions. The formatter works
        // with a transformed tree (parentheses removed by JsFormatSyntaxRewriter),
        // but the embedding service stores ranges from the original tree.
        let source_range = f
            .context()
            .source_map()
            .map_or(transformed_range, |map| map.source_range(transformed_range));

        // Only chunks whose range was registered by the embedding service are
        // delegated. Marking any other chunk would emit StartEmbedded/EndEmbedded
        // tags that never get resolved, losing the chunk's content.
        embedded_node_ranges
            .contains(&source_range)
            .then_some(source_range)
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
