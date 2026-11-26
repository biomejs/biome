use crate::shared::any_class_string_like::AnyClassStringLike;
use biome_analyze::options::PreferredQuote;
use biome_js_factory::make::{
    js_literal_member_name, js_string_literal, js_string_literal_expression,
    js_string_literal_single_quotes, js_template_chunk, js_template_chunk_element, jsx_string,
};
use biome_js_syntax::{
    JsBinaryExpression, JsConditionalExpression, JsLanguage, JsLogicalExpression,
    JsParenthesizedExpression, JsSyntaxKind, JsTemplateElement, JsTemplateElementList,
    JsTemplateExpression, TsAsExpression, TsNonNullAssertionExpression, TsSatisfiesExpression,
};
use biome_rowan::{AstNode, BatchMutation, SyntaxKindSet, TextRange, TextSize};

pub(crate) type AnyTailwindClassString = AnyClassStringLike;

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

/// Returns whether the start and the end of `node` touch other text of the
/// same class string: another operand of a `+` expression, or the chunks around
/// a template substitution. Both do for `"b"` in `a + "b" + c`.
pub(crate) fn concatenated_edges(node: &AnyTailwindClassString) -> (bool, bool) {
    let mut leading = false;
    let mut trailing = false;
    let mut child = node.syntax().clone();
    for ancestor in node.syntax().ancestors().skip(1) {
        if let Some(binary) = JsBinaryExpression::cast_ref(&ancestor) {
            if binary.left().is_ok_and(|left| left.syntax() == &child) {
                trailing = true;
            } else {
                leading = true;
            }
        } else if JsTemplateElement::can_cast(ancestor.kind()) {
            return (true, true);
        } else if !CONCATENATION_WRAPPER_KINDS.matches(ancestor.kind()) {
            break;
        }
        child = ancestor;
    }
    (leading, trailing)
}

/// The nodes a class string can sit in while still being an operand of an outer
/// `+` expression.
const CONCATENATION_WRAPPER_KINDS: SyntaxKindSet<JsLanguage> = JsParenthesizedExpression::KIND_SET
    .union(JsConditionalExpression::KIND_SET)
    .union(JsLogicalExpression::KIND_SET)
    .union(JsTemplateExpression::KIND_SET)
    .union(JsTemplateElementList::KIND_SET)
    .union(TsAsExpression::KIND_SET)
    .union(TsSatisfiesExpression::KIND_SET)
    .union(TsNonNullAssertionExpression::KIND_SET);

fn inner_text_start(node: &AnyTailwindClassString) -> Option<TextSize> {
    match node {
        AnyTailwindClassString::JsStringLiteralExpression(node) => {
            Some(node.value_token().ok()?.text_trimmed_range().start() + TextSize::from(1))
        }
        AnyTailwindClassString::JsLiteralMemberName(node) => {
            let value = node.value().ok()?;
            // Only string literal keys have an opening quote to skip.
            let quote_len = if value.kind() == JsSyntaxKind::JS_STRING_LITERAL {
                TextSize::from(1)
            } else {
                TextSize::from(0)
            };
            Some(value.text_trimmed_range().start() + quote_len)
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
