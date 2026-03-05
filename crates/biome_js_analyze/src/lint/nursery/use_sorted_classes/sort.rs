//! JS-specific wrappers around the language-agnostic sorting in `biome_tailwind_sort`.
//!
//! This module handles JS template literal boundary detection and delegates
//! the actual sorting to `biome_tailwind_sort::sort`.

use biome_js_syntax::{JsTemplateChunkElement, JsTemplateElement};
use biome_rowan::{AstNode, TextRange, TextSize, TokenText};
use biome_tailwind_sort::sort::{self, SortContext};
use biome_tailwind_sort::sort_config::SortConfig;

use crate::shared::any_class_string_like::AnyClassStringLike;

/// JS-specific template literal space context.
///
/// Determines whether edge classes in a template chunk should be excluded from
/// sorting because they're glued to a template expression (e.g. `${var}px-2`).
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

    /// Convert to the language-agnostic `SortContext`.
    fn to_sort_context(self) -> SortContext {
        SortContext {
            ignore_prefix: self.ignore_prefix(),
            ignore_suffix: self.ignore_postfix(),
            keep_leading_space: self.keep_leading(),
            keep_trailing_space: self.keep_trailing(),
        }
    }

    /// Skip first class from sorting when it's connected to a variable: `${var}px-2 m-4`
    #[inline]
    fn ignore_prefix(&self) -> bool {
        self.prefix_is_var && !self.leading_space
    }
    /// Skip last class from sorting when it's connected to a variable: `p-2 m-4${var}`
    #[inline]
    fn ignore_postfix(&self) -> bool {
        self.postfix_is_var && !self.trailing_space
    }
    /// Preserve leading space to maintain variable boundary: `${var} p-2 m-4`
    #[inline]
    fn keep_leading(&self) -> bool {
        self.prefix_is_var && self.leading_space
    }
    /// Preserve trailing space to maintain variable boundary: `p-2 m-4 ${var}`
    #[inline]
    fn keep_trailing(&self) -> bool {
        self.postfix_is_var && self.trailing_space
    }
}

/// Returns the template space context for the given node.
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

/// Sort the given class string, handling JS-specific template literal context.
pub(crate) fn sort_class_name(
    class_name: &TokenText,
    sort_config: &SortConfig,
    template_ctx: &Option<TemplateLiteralSpaceContext>,
) -> String {
    let sort_context = template_ctx.as_ref().map(|ctx| ctx.to_sort_context());
    sort::sort_class_name(class_name.text(), sort_config, &sort_context)
}

/// Get the range of the class name that was actually sorted.
///
/// The `range` parameter is the full range of the node (including quotes for strings).
/// The offsets are computed from the class text to determine which edge classes are
/// excluded, then applied to the original range.
pub(crate) fn get_sort_class_name_range(
    class_name: &TokenText,
    range: &TextRange,
    template_ctx: &Option<TemplateLiteralSpaceContext>,
) -> Option<TextRange> {
    let sort_context = template_ctx.as_ref().map(|ctx| ctx.to_sort_context());

    let mut class_iter = class_name.split_whitespace();
    let first_class_len = class_iter.next().map_or(0, |s| s.len()) as u32;
    let last_class_len = class_iter.next_back().map_or(0, |s| s.len()) as u32;

    let (ignore_prefix, ignore_suffix) = sort_context
        .as_ref()
        .map_or((false, false), |ctx| (ctx.ignore_prefix, ctx.ignore_suffix));
    let offset_prefix = if ignore_prefix { first_class_len } else { 0 };
    let offset_suffix = if ignore_suffix { last_class_len } else { 0 };

    let start = range.start() + TextSize::from(offset_prefix);
    let end = range.end() - TextSize::from(offset_suffix);

    if end < start {
        return None;
    }

    Some(TextRange::new(start, end))
}
