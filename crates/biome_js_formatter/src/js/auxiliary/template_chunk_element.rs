use crate::prelude::*;
use biome_formatter::FormatContext;
use biome_formatter::write;

use biome_js_syntax::{
    AnyJsExpression, JsCallArgumentList, JsCallArguments, JsCallExpression, JsSyntaxToken,
    JsTemplateChunkElement, JsTemplateExpression, TsTemplateChunkElement,
};
use biome_rowan::{AstNode, SyntaxResult, declare_node_union};
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
        if !f.context().should_delegate_fmt_embedded_nodes() {
            return None;
        }

        // Only mark template chunks that belong to a plausible embed candidate.
        // A template is a candidate when it has a tag (e.g. css``, gql``, styled.div``)
        // or is an argument to a simple call expression (e.g. graphql(`...`)).
        // Plain templates like console.log(`test`) must NOT be marked, otherwise
        // the formatter emits StartEmbedded/EndEmbedded tags that never get resolved
        // and corrupt the printer's tag stack.
        let template = node
            .syntax()
            .ancestors()
            .find_map(JsTemplateExpression::cast)?;

        if !is_plausible_embed_template(&template)? {
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

        Some(source_range)
    }
}

/// Known identifier tag names that produce embedded languages.
/// Must stay in sync with the `TemplateTag` entries in `JS_DETECTORS`.
const KNOWN_EMBED_TAGS: &[&str] = &["css", "gql", "graphql"];

/// Known object/callee names for member expressions (`styled.div```)
/// and call expressions (`styled(Comp)```, `graphql(``)`).
/// Must stay in sync with the `TemplateExpression` entries in `JS_DETECTORS`.
const KNOWN_EMBED_OBJECTS: &[&str] = &["styled", "graphql"];

/// Check whether a template expression is a known embed candidate.
///
/// Returns `Some(true)` only for templates whose tag or call pattern matches
/// one of the known embed detectors:
/// - `css```, `gql```, `graphql``` (identifier tag)
/// - `styled.div```, `styled(Comp)``` (member/call with known object)
/// - `graphql(`...`)` (untagged template as argument to known callee)
///
/// Returns `None` when the AST is malformed.
fn is_plausible_embed_template(expr: &JsTemplateExpression) -> Option<bool> {
    // The service currently only extracts embedded snippets from single-chunk
    // templates. Keep the formatter in sync so it doesn't emit StartEmbedded /
    // EndEmbedded tags for ranges that won't be resolved later.
    if expr.elements().len() != 1 {
        return Some(false);
    }

    if let Some(tag) = expr.tag() {
        return Some(match tag {
            // css``, gql``, graphql``
            AnyJsExpression::JsIdentifierExpression(ident) => {
                let name = ident.name().ok()?.value_token().ok()?;
                KNOWN_EMBED_TAGS
                    .iter()
                    .any(|known| name.text_trimmed() == *known)
            }
            // styled.div``
            AnyJsExpression::JsStaticMemberExpression(member) => {
                let AnyJsExpression::JsIdentifierExpression(ident) = member.object().ok()? else {
                    return Some(false);
                };
                let name = ident.name().ok()?.value_token().ok()?;
                KNOWN_EMBED_OBJECTS
                    .iter()
                    .any(|known| name.text_trimmed() == *known)
            }
            // styled(Component)``
            AnyJsExpression::JsCallExpression(call) => {
                let AnyJsExpression::JsIdentifierExpression(ident) = call.callee().ok()? else {
                    return Some(false);
                };
                let name = ident.name().ok()?.value_token().ok()?;
                KNOWN_EMBED_OBJECTS
                    .iter()
                    .any(|known| name.text_trimmed() == *known)
            }
            _ => false,
        });
    }

    // No tag — check if template is an argument to a known call expression.
    // e.g. graphql(`query { ... }`)
    let call = expr
        .parent::<JsCallArgumentList>()?
        .parent::<JsCallArguments>()?
        .parent::<JsCallExpression>()?;

    let AnyJsExpression::JsIdentifierExpression(ident) = call.callee().ok()? else {
        return Some(false);
    };
    let name = ident.name().ok()?.value_token().ok()?;
    Some(
        KNOWN_EMBED_OBJECTS
            .iter()
            .any(|known| name.text_trimmed() == *known),
    )
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
