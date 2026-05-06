use crate::prelude::*;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_markdown_syntax::{
    MarkdownSyntaxKind, MdInlineItalic, MdInlineItalicFields, MdReferenceImage,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdInlineItalic {
    should_keep_fences: bool,
}

/// Determine the target delimiter for an italic node based on its context.
/// Prefers `_` but uses `*` when adjacent to alphanumeric characters
/// (per CommonMark spec: `a_b_c` won't parse as italic, but `a*b*c` will).
fn resolve_target_kind(node: &MdInlineItalic) -> MarkdownSyntaxKind {
    let prev_is_alphanum = node
        .l_fence()
        .ok()
        .and_then(|f| f.prev_token())
        .and_then(|t| t.text_trimmed().chars().last())
        .is_some_and(|c| c.is_alphanumeric());
    let next_is_alphanum = node
        .r_fence()
        .ok()
        .and_then(|f| f.next_token())
        .and_then(|t| t.text_trimmed().chars().next())
        .is_some_and(|c| c.is_alphanumeric());

    if prev_is_alphanum || next_is_alphanum {
        MarkdownSyntaxKind::STAR
    } else {
        MarkdownSyntaxKind::UNDERSCORE
    }
}

/// Check if any direct textual child contains the delimiter char for the given kind.
fn content_has_char(
    content: &biome_markdown_syntax::MdInlineItemList,
    kind: MarkdownSyntaxKind,
) -> bool {
    let char = if kind == MarkdownSyntaxKind::STAR {
        '*'
    } else {
        '_'
    };
    content.iter().any(|item| {
        matches!(&item, biome_markdown_syntax::AnyMdInline::MdTextual(t)
            if t.value_token().is_ok_and(|tok| tok.text().contains(char)))
    })
}

/// Check if this italic is nested inside another italic.
fn has_ancestor_italic(node: &MdInlineItalic) -> bool {
    node.syntax()
        .ancestors()
        .skip(1)
        .any(|a| MdInlineItalic::can_cast(a.kind()))
}

impl FormatNodeRule<MdInlineItalic> for FormatMdInlineItalic {
    fn fmt_fields(&self, node: &MdInlineItalic, f: &mut MarkdownFormatter) -> FormatResult<()> {
        let MdInlineItalicFields {
            l_fence,
            content,
            r_fence,
        } = node.as_fields();

        let l_fence = l_fence?;
        let r_fence = r_fence?;

        // Inside reference images the alt text doubles as the reference label.
        // Normalizing `*` → `_` would change the label and break reference resolution.
        // E.g. `![foo *bar*]` with `[foo *bar*]: url` must keep `*`.
        let inside_ref_image = node
            .syntax()
            .ancestors()
            .skip(1)
            .any(|a| MdReferenceImage::can_cast(a.kind()));

        if inside_ref_image || self.should_keep_fences {
            return write!(f, [l_fence.format(), content.format(), r_fence.format()]);
        }

        // Use `*` if inside another italic or near alphanumeric chars; otherwise `_`.
        let mut target_kind = if has_ancestor_italic(node) {
            MarkdownSyntaxKind::STAR
        } else {
            resolve_target_kind(node)
        };
        // If content has a `MdTextual` node containing the target char (e.g. a
        // plain-text `_` inside `*foo _bar*`), keep the original fence to avoid
        // that text being re-parsed as a delimiter.
        if content_has_char(&content, target_kind) {
            target_kind = l_fence.kind();
        }

        write_fence(&l_fence, target_kind, f)?;
        write!(f, [content.format()])?;
        write_fence(&r_fence, target_kind, f)
    }
}

/// Write a fence token, reusing it if it already matches the target kind.
fn write_fence(
    fence: &biome_markdown_syntax::MarkdownSyntaxToken,
    target_kind: MarkdownSyntaxKind,
    f: &mut MarkdownFormatter,
) -> FormatResult<()> {
    if fence.kind() == target_kind {
        write!(f, [fence.format()])
    } else {
        let text = if target_kind == MarkdownSyntaxKind::STAR {
            "*"
        } else {
            "_"
        };
        write!(f, [format_replaced(fence, &token(text))])
    }
}

pub(crate) struct FormatMdInlineItalicOptions {
    pub(crate) should_keep_fences: bool,
}

impl FormatRuleWithOptions<MdInlineItalic> for FormatMdInlineItalic {
    type Options = FormatMdInlineItalicOptions;
    fn with_options(mut self, options: Self::Options) -> Self {
        self.should_keep_fences = options.should_keep_fences;
        self
    }
}
