use super::{
    ValueListLayout, has_value_boundary_comments, is_comma_delimiter,
    is_comma_separated_declaration_value_list, is_value_boundary_comment,
};
use crate::prelude::*;
use biome_css_syntax::{CssGenericProperty, CssIdentifier, CssLanguage, CssSyntaxNode};
use biome_formatter::comments::SourceComment;
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::{FormatResult, RemoveSoftLinesBuffer, format_args, write};
use biome_rowan::{SyntaxNodeChildren, TextSize};
use std::{iter::Peekable, slice};

/// Returns grouped formatting when a declaration value's layout or comments
/// require comma groups to break independently.
///
/// ```css
/// a { box-shadow: 1px /* boundary */ 1px red, 2px 2px blue; }
/// ```
pub(super) fn format_if_applicable<'a>(
    node: &'a CssSyntaxNode,
    layout: ValueListLayout,
    comments: &'a [SourceComment<CssLanguage>],
) -> Option<impl Format<CssFormatContext> + 'a> {
    let is_applicable = match layout {
        ValueListLayout::Fill
            if node
                .parent()
                .is_some_and(|parent| CssGenericProperty::can_cast(parent.kind())) =>
        {
            true
        }
        ValueListLayout::PreserveInline | ValueListLayout::OnePerLine => true,
        _ => has_value_boundary_comments(comments),
    };

    if !is_applicable || !is_comma_separated_declaration_value_list(node) {
        return None;
    }

    Some(format_with(move |f| {
        CommaGroupsWriter {
            cursor: CommaGroupCursor::new(node, comments),
            layout,
        }
        .write(f)
    }))
}

/// Merges list elements and dangling comments in source order.
struct CommaGroupCursor<'a> {
    elements: Peekable<SyntaxNodeChildren<CssLanguage>>,
    comments: Peekable<slice::Iter<'a, SourceComment<CssLanguage>>>,
    list_end: TextSize,
}

impl<'a> CommaGroupCursor<'a> {
    fn new(node: &CssSyntaxNode, comments: &'a [SourceComment<CssLanguage>]) -> Self {
        Self {
            elements: node.children().peekable(),
            comments: comments.iter().peekable(),
            list_end: node.text_trimmed_range().end(),
        }
    }

    fn preview_group(&mut self) -> Option<CommaGroupPreview> {
        let starts_on_new_line = self.elements.peek()?.has_leading_newline();
        let has_value_boundary_comment = if self.comments.peek().is_none() {
            false
        } else {
            let group_end = self
                .elements
                .clone()
                .find(is_comma_delimiter)
                .map_or_else(|| self.list_end, |comma| comma.text_trimmed_range().end());

            self.comments
                .clone()
                .take_while(|comment| comment.piece().text_range().end() <= group_end)
                .any(is_value_boundary_comment)
        };

        Some(CommaGroupPreview {
            starts_on_new_line,
            has_value_boundary_comment,
        })
    }

    fn has_remaining_comments(&mut self) -> bool {
        self.comments.peek().is_some()
    }
}

impl<'a> Iterator for CommaGroupCursor<'a> {
    type Item = CommaGroupEntry<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let element_start = self.elements.peek()?.text_trimmed_range().start();

        if let Some(comment) = self
            .comments
            .next_if(|comment| comment.piece().text_range().end() <= element_start)
        {
            Some(CommaGroupEntry::Comment(comment))
        } else {
            self.elements.next().map(CommaGroupEntry::Element)
        }
    }
}

enum CommaGroupEntry<'a> {
    Comment(&'a SourceComment<CssLanguage>),
    Element(CssSyntaxNode),
}

struct CommaGroupPreview {
    starts_on_new_line: bool,
    has_value_boundary_comment: bool,
}

/// Streams comma-separated declaration groups through the formatter.
///
/// ```css
/// a { margin: 1px/* boundary */2px, 3px; }
/// ```
struct CommaGroupsWriter<'a> {
    cursor: CommaGroupCursor<'a>,
    layout: ValueListLayout,
}

impl CommaGroupsWriter<'_> {
    fn write(mut self, f: &mut Formatter<'_, CssFormatContext>) -> FormatResult<()> {
        let mut groups = f.fill();

        while let Some(info) = self.cursor.preview_group() {
            let layout = self.layout;
            let group_separator =
                format_once(move |f| layout.fmt_group_separator(info.starts_on_new_line, f));
            let formatted_group = format_once(|f| self.write_next_group(f));

            if info.has_value_boundary_comment {
                let formatted_group = formatted_group.memoized();
                let flat_group = format_with(|f| {
                    let mut flat = RemoveSoftLinesBuffer::new(f);
                    write!(flat, [&formatted_group])
                });
                let expanded_group = group(&formatted_group);

                // Comma groups choose their own layout even when the outer value
                // list breaks between groups.
                groups.entry(
                    &group_separator,
                    &best_fitting!(&flat_group, &expanded_group),
                );
            } else {
                groups.entry(&group_separator, &group(&formatted_group));
            }
        }

        groups.finish()?;
        debug_assert!(
            !self.cursor.has_remaining_comments(),
            "all comma-group comments must be formatted"
        );
        Ok(())
    }

    fn write_next_group(&mut self, f: &mut Formatter<'_, CssFormatContext>) -> FormatResult<()> {
        let mut values = f.fill();
        let mut state = CommaGroupValueState::Empty;
        let layout = self.layout;

        for entry in self.cursor.by_ref() {
            match entry {
                CommaGroupEntry::Comment(comment) => {
                    state = state.after_comment(is_value_boundary_comment(comment));
                    let formatted_comment = format_dangling_comment(comment);

                    if state == CommaGroupValueState::BoundaryStart {
                        values.entry(
                            &format_once(|_| Ok(())),
                            &soft_line_indent_or_space(&formatted_comment),
                        );
                    } else {
                        values.entry(&soft_line_break_or_space(), &formatted_comment);
                    }
                }
                CommaGroupEntry::Element(element) => {
                    let is_comma = is_comma_delimiter(&element);
                    let starts_on_new_line = element.has_leading_newline();
                    let has_source_hard_separator =
                        !is_comma && starts_on_new_line && layout.is_source_break_preserving();
                    let separator = format_once(move |f| {
                        if is_comma {
                            Ok(())
                        } else {
                            layout.fmt_value_separator(starts_on_new_line, f)
                        }
                    });
                    let formatted_element = format_element(element);

                    match state {
                        CommaGroupValueState::BoundaryStart if !is_comma => {
                            values.entry(
                                &format_once(|_| Ok(())),
                                &soft_line_indent_or_space(&formatted_element),
                            );
                        }
                        CommaGroupValueState::BoundaryContinuation
                            if !is_comma && !has_source_hard_separator =>
                        {
                            values.entry(
                                &format_once(|_| Ok(())),
                                &indent(&format_args![&separator, &formatted_element]),
                            );
                        }
                        _ => {
                            values.entry(&separator, &formatted_element);
                        }
                    }

                    if !is_comma {
                        state = state.after_value(has_source_hard_separator);
                    } else {
                        break;
                    }
                }
            }
        }

        values.finish()
    }
}

/// Formats a type-erased comma-group element.
///
/// Direct identifiers need an explicit preserve policy because SCSS expression
/// item lists also reach this path.
fn format_element(element: CssSyntaxNode) -> impl Format<CssFormatContext> {
    format_with(move |f| {
        if let Some(identifier) = CssIdentifier::cast_ref(&element) {
            write!(f, [identifier.format().with_text_case(CssCase::Preserve)])
        } else {
            write!(f, [element.format()])
        }
    })
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum CommaGroupValueState {
    /// No value has been formatted in the current comma group.
    Empty,
    /// No boundary-comment continuation is active.
    Normal,
    /// A boundary comment was emitted and the following value starts its indented segment.
    BoundaryStart,
    /// Values remain in the boundary segment until a source-preserved break.
    BoundaryContinuation,
}

impl CommaGroupValueState {
    fn after_comment(self, is_boundary: bool) -> Self {
        match (self, is_boundary) {
            (Self::Empty, _) => Self::Empty,
            (_, true) => Self::BoundaryStart,
            _ => Self::Normal,
        }
    }

    fn after_value(self, has_source_hard_separator: bool) -> Self {
        match self {
            Self::BoundaryStart => Self::BoundaryContinuation,
            Self::BoundaryContinuation if !has_source_hard_separator => Self::BoundaryContinuation,
            _ => Self::Normal,
        }
    }
}
