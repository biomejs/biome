use biome_js_syntax::{JsTemplateChunkElement, JsTemplateElement};
use biome_rowan::{AstNode, TextRange, TextSize, TokenText};
use crate::shared::any_class_string_like::AnyClassStringLike;

pub fn get_sort_class_name_range(
    class_name: &TokenText,
    range: &TextRange,
    template_literal_space_context: &Option<TemplateLiteralSpaceContext>,
) -> Option<TextRange> {
    let mut class_iter = class_name.split_whitespace();
    let first_class_len = class_iter.next().map_or(0, |s| s.len()) as u32;
    let last_class_len = class_iter.next_back().map_or(0, |s| s.len()) as u32;

    let (ignore_prefix, ignore_postfix) = template_literal_space_context
        .as_ref()
        .map_or((false, false), |ctx| ctx.get_ignore_flags());
    let offset_prefix = if ignore_prefix { first_class_len } else { 0 };
    let offset_postfix = if ignore_postfix { last_class_len } else { 0 };

    let start = range.start() + TextSize::from(offset_prefix);
    let end = range.end() - TextSize::from(offset_postfix);

    if end < start {
        return None;
    }

    Some(TextRange::new(start, end))
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TemplateLiteralSpaceContext {
    pub(crate) prefix_is_var: bool,
    pub(crate) postfix_is_var: bool,
    pub(crate) leading_space: bool,
    pub(crate) trailing_space: bool,
}

impl TemplateLiteralSpaceContext {
    pub(crate) fn from_chunk(chunk: &JsTemplateChunkElement) -> Option<Self> {
        let token = chunk.template_chunk_token().ok()?;
        let value = token.text_trimmed();
        if value.trim().is_empty() {
            return None;
        }

        let syntax = chunk.syntax();
        let prefix_is_var = syntax
            .prev_sibling()
            .is_some_and(|s| JsTemplateElement::can_cast(s.kind()));
        let postfix_is_var = syntax
            .next_sibling()
            .is_some_and(|s| JsTemplateElement::can_cast(s.kind()));

        Some(Self {
            prefix_is_var,
            postfix_is_var,
            leading_space: value.starts_with(' '),
            trailing_space: value.ends_with(' '),
        })
    }

    #[inline]
    pub(crate) fn ignore_prefix(&self) -> bool {
        self.prefix_is_var && !self.leading_space
    }
    #[inline]
    pub(crate) fn ignore_postfix(&self) -> bool {
        self.postfix_is_var && !self.trailing_space
    }
    #[inline]
    pub(crate) fn keep_leading(&self) -> bool {
        self.prefix_is_var && self.leading_space
    }
    #[inline]
    pub(crate) fn keep_trailing(&self) -> bool {
        self.postfix_is_var && self.trailing_space
    }

    #[inline]
    pub(crate) fn get_ignore_flags(&self) -> (bool, bool) {
        (self.ignore_prefix(), self.ignore_postfix())
    }
}

pub(crate) fn get_template_literal_space_context(
    node: &AnyClassStringLike,
) -> Option<TemplateLiteralSpaceContext> {
    match node {
        AnyClassStringLike::JsTemplateChunkElement(chunk) => {
            TemplateLiteralSpaceContext::from_chunk(chunk)
        }
        _ => None,
    }
}
