use crate::shared::any_class_string_like::{AnyClassStringLike, ClassStringLikeOptions};
use biome_analyze::options::PreferredQuote;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_js_syntax::JsLanguage;
use biome_rowan::{BatchMutation, TextRange, TextSize, TokenText};

pub(crate) type AnyTailwindClassString = AnyClassStringLike;

pub(crate) fn class_string(
    node: &AnyTailwindClassString,
    options: &impl ClassStringLikeOptions,
) -> Option<TokenText> {
    node.should_visit(options)
        .unwrap_or(false)
        .then(|| node.value())?
}

pub(crate) fn host_range(node: &AnyTailwindClassString, range: TextRange) -> Option<TextRange> {
    let start = inner_text_start(node)?;
    Some(TextRange::new(start + range.start(), start + range.end()))
}

pub(crate) fn apply_fixed_class_string(
    mutation: &mut BatchMutation<JsLanguage>,
    node: &AnyTailwindClassString,
    fixed: &str,
    preferred_quote: PreferredQuote,
    preferred_jsx_quote: PreferredQuote,
) {
    match node {
        AnyTailwindClassString::JsStringLiteralExpression(string_literal) => {
            let is_double_quote = string_literal
                .value_token()
                .map_or(preferred_quote.is_double(), |token| {
                    token.text_trimmed().starts_with('"')
                });
            let replacement = js_string_literal_expression(if is_double_quote {
                js_string_literal(fixed)
            } else {
                js_string_literal_single_quotes(fixed)
            });
            mutation.replace_node(string_literal.clone(), replacement);
        }
        AnyTailwindClassString::JsLiteralMemberName(string_literal) => {
            let is_double_quote = string_literal
                .value()
                .map_or(preferred_quote.is_double(), |token| {
                    token.text_trimmed().starts_with('"')
                });
            let replacement = js_literal_member_name(if is_double_quote {
                js_string_literal(fixed)
            } else {
                js_string_literal_single_quotes(fixed)
            });
            mutation.replace_node(string_literal.clone(), replacement);
        }
        AnyTailwindClassString::JsxString(jsx_string_node) => {
            let is_double_quote = jsx_string_node
                .value_token()
                .map_or(preferred_jsx_quote.is_double(), |token| {
                    token.text_trimmed().starts_with('"')
                });
            let replacement = jsx_string(if is_double_quote {
                js_string_literal(fixed)
            } else {
                js_string_literal_single_quotes(fixed)
            });
            mutation.replace_node(jsx_string_node.clone(), replacement);
        }
        AnyTailwindClassString::JsTemplateChunkElement(chunk) => {
            let replacement = js_template_chunk_element(js_template_chunk(fixed));
            mutation.replace_node(chunk.clone(), replacement);
        }
    }
}

fn inner_text_start(node: &AnyTailwindClassString) -> Option<TextSize> {
    match node {
        AnyTailwindClassString::JsStringLiteralExpression(node) => {
            Some(node.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1))
        }
        AnyTailwindClassString::JsLiteralMemberName(node) => {
            Some(node.value().ok()?.text_trimmed_range().start() + TextSize::from(1))
        }
        AnyTailwindClassString::JsxString(node) => {
            Some(node.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1))
        }
        AnyTailwindClassString::JsTemplateChunkElement(node) => Some(
            node.template_chunk_token()
                .ok()?
                .text_trimmed_range()
                .start(),
        ),
    }
}
