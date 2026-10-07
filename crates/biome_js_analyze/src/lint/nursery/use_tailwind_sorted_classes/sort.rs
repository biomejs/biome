use biome_js_syntax::{JsTemplateChunkElement, JsTemplateElement};
use biome_rowan::{AstNode, NodeCache, TextRange, TextSize, TokenText};
use biome_tailwind_logic::syntax_service::TailwindSyntax;
use biome_tailwind_logic::use_tailwind_sorted_classes::{TailwindDesignSystem, sort_class_list};
use biome_tailwind_parser::parse_tailwind_with_options;

use crate::tailwind::AnyTailwindClassString;

/// Sorts the classes of the queried class string. Returns `None` when the
/// classes can't be parsed.
pub(crate) fn sort_classes(
    query: &TailwindSyntax<AnyTailwindClassString>,
    class_name: &TokenText,
    template_literal_space_context: Option<&TemplateLiteralSpaceContext>,
    design: &TailwindDesignSystem,
) -> Option<String> {
    let (ignore_prefix, ignore_postfix) =
        template_literal_space_context.map_or((false, false), |ctx| ctx.get_ignore_flags());

    let mut result = if ignore_prefix || ignore_postfix {
        // A class glued to an interpolation can be a fragment, such as `bg-` in
        // `` `bg-${color}` ``, so it stays in place and only the classes between
        // the glued ones are parsed and sorted.
        let mut classes = class_name.split_whitespace();
        let prefix = if ignore_prefix { classes.next() } else { None };
        let postfix = if ignore_postfix {
            classes.next_back()
        } else {
            None
        };
        let middle = classes.collect::<Vec<_>>().join(" ");
        let parse = parse_tailwind_with_options(
            &middle,
            &mut NodeCache::default(),
            design.parser_options(),
        );
        if parse.has_errors() {
            return None;
        }
        let sorted_middle = sort_class_list(&parse.tree(), design);
        prefix
            .into_iter()
            .chain((!sorted_middle.is_empty()).then_some(sorted_middle.as_str()))
            .chain(postfix)
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        if query.tailwind_has_errors() {
            return None;
        }
        sort_class_list(&query.tailwind_root(), design)
    };

    // Edge space handling for template literals only
    if let Some(ctx) = template_literal_space_context {
        if ctx.keep_leading() {
            result.insert(0, ' ');
        }
        if ctx.keep_trailing() {
            result.push(' ');
        }
    }

    Some(result)
}

// Get the range of the class name to be sorted.
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

    /// Skip first class from sorting when it's connected to a variable: `${var}px-2 m-4`
    #[inline]
    pub(crate) fn ignore_prefix(&self) -> bool {
        self.prefix_is_var && !self.leading_space
    }
    /// Skip last class from sorting when it's connected to a variable: `p-2 m-4${var}`
    #[inline]
    pub(crate) fn ignore_postfix(&self) -> bool {
        self.postfix_is_var && !self.trailing_space
    }
    /// Preserve leading space to maintain variable boundary: `${var} p-2 m-4`
    #[inline]
    pub(crate) fn keep_leading(&self) -> bool {
        self.prefix_is_var && self.leading_space
    }
    /// Preserve trailing space to maintain variable boundary: `p-2 m-4 ${var}`
    #[inline]
    pub(crate) fn keep_trailing(&self) -> bool {
        self.postfix_is_var && self.trailing_space
    }

    /// Returns (ignore_prefix, ignore_postfix) for sorting
    #[inline]
    pub(crate) fn get_ignore_flags(&self) -> (bool, bool) {
        (self.ignore_prefix(), self.ignore_postfix())
    }
}

/// Returns the template space context for the given node
pub(crate) fn get_template_literal_space_context(
    node: &AnyTailwindClassString,
) -> Option<TemplateLiteralSpaceContext> {
    match node {
        AnyTailwindClassString::JsTemplateChunkElement(chunk) => {
            TemplateLiteralSpaceContext::from_chunk(chunk)
        }
        _ => None,
    }
}
