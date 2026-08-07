//! Completes inline syntax after Markdown block parsing.
//!
//! CommonMark separates parsing into a block phase followed by an inline phase
//! because link reference definitions are document-global and are not all known
//! until block parsing finishes. See the
//! [CommonMark parsing strategy](https://spec.commonmark.org/0.31.2/#appendix-a-parsing-strategy).
//!
//! The initial document parse records each inline event subtree together with its
//! source range and the number of definitions known at that point. Inline subtrees
//! that precede later definitions are parsed again with the complete definition index.
//! Their replacement events, trivia, and diagnostics are merged before the
//! lossless tree sink constructs the CST, so source text is never rewritten.

use biome_markdown_syntax::MarkdownSyntaxKind;
use biome_markdown_syntax::MarkdownSyntaxKind::{MD_INLINE_ITEM_LIST, MD_PARAGRAPH, MD_ROOT};
use biome_parser::Parser;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::token_source::Trivia;
use biome_rowan::{TextRange, TextSize};

use crate::MarkdownParserOptions;
use crate::parser::{
    DeferredInline, DeferredInlineFlavor, LinkReferenceDefinitions, MarkdownParser,
    MarkdownParserOutput,
};
use crate::syntax::header::parse_header_content;
use crate::syntax::parse_inline_item_list;

/// Resolves deferred inline subtrees into an event stream ready for the
/// lossless tree sink.
///
/// Debug builds validate recorded event and source range invariants. Returns
/// `false` without changing the output if reparsing produces a mismatched fragment.
pub(crate) fn parse_deferred_inlines(
    source: &str,
    options: &MarkdownParserOptions,
    output: &mut MarkdownParserOutput,
) -> bool {
    debug_assert!(
        validate_deferred_inlines(source, output),
        "Deferred inlines are invalid, reparsing may produce a mismatched fragment."
    );

    let mut replacements = Vec::new();

    for deferred in &output.deferred_inlines {
        if deferred.definitions_len() == output.link_reference_definitions.len() {
            continue;
        }

        let Some(fragment) = parse_inline_fragment(
            source,
            deferred,
            options.clone(),
            &output.link_reference_definitions,
        ) else {
            return false;
        };

        replacements.push((
            deferred.event_range().clone(),
            deferred.source_range(),
            fragment,
        ));
    }

    let mut events = Vec::with_capacity(output.events.len());
    let mut old_events = std::mem::take(&mut output.events).into_iter();
    let mut inline_trivia = Vec::new();
    let mut inline_diagnostics = Vec::new();
    let mut reparsed_ranges = Vec::with_capacity(replacements.len());
    let mut event_position = 0;

    for (event_range, source_range, fragment) in replacements {
        events.extend(old_events.by_ref().take(event_range.start - event_position));
        old_events
            .by_ref()
            .take(event_range.end - event_range.start)
            .for_each(drop);
        event_position = event_range.end;
        events.extend(fragment.events);

        reparsed_ranges.push(source_range);
        inline_trivia.extend(fragment.trivia);
        inline_diagnostics.extend(fragment.diagnostics);
    }
    events.extend(old_events);
    output.events = events;

    let mut range_index = 0;
    output.trivia.retain(|trivia| {
        !overlaps_ordered_ranges(trivia.text_range(), &reparsed_ranges, &mut range_index)
    });
    output.trivia.extend(inline_trivia);
    output.trivia.sort_by_key(Trivia::offset);

    range_index = 0;
    output.diagnostics.retain(|diagnostic| {
        diagnostic
            .span()
            .is_none_or(|span| !overlaps_ordered_ranges(span, &reparsed_ranges, &mut range_index))
    });
    output.diagnostics =
        merge_diagnostics(std::mem::take(&mut output.diagnostics), inline_diagnostics);
    output.deferred_inlines.clear();

    true
}

fn validate_deferred_inlines(source: &str, output: &MarkdownParserOutput) -> bool {
    let mut previous_event_end = 0;
    let mut previous_source_end = TextSize::from(0);
    let mut previous_token_end = TextSize::from(0);
    let definitions_len = output.link_reference_definitions.len();

    for deferred in &output.deferred_inlines {
        let event_range = deferred.event_range();
        let source_range = deferred.source_range();
        let expected_kind = match deferred.flavor() {
            DeferredInlineFlavor::Paragraph => MD_INLINE_ITEM_LIST,
            DeferredInlineFlavor::AtxParagraph => MD_PARAGRAPH,
        };
        if event_range.start < previous_event_end {
            return false;
        }
        let Some(events) = output.events.get(event_range.clone()) else {
            return false;
        };
        for event in &output.events[previous_event_end..event_range.start] {
            if let Event::Token { end, .. } = event {
                previous_token_end = *end;
            }
        }
        let source_start = usize::from(source_range.start());
        let source_end = usize::from(source_range.end());
        let Some(first_token_end) = events.iter().find_map(|event| match event {
            Event::Token { end, .. } => Some(*end),
            _ => None,
        }) else {
            return false;
        };
        let Some(last_token_end) = events.iter().rev().find_map(|event| match event {
            Event::Token { end, .. } => Some(*end),
            _ => None,
        }) else {
            return false;
        };
        let event_source_start =
            skip_contiguous_trivia(previous_token_end, first_token_end, &output.trivia);
        let event_source_end =
            skip_contiguous_trivia(last_token_end, source_range.end(), &output.trivia);

        if deferred.definitions_len() > definitions_len
            || source_range.start() < previous_source_end
            || source.get(source_start..source_end).is_none()
            || event_source_start != source_range.start()
            || event_source_end != source_range.end()
            || !is_complete_subtree(events)
            || !matches!(
                events,
                [
                    Event::Start {
                        kind,
                        forward_parent: None,
                    },
                    ..,
                    Event::Finish,
                ] if *kind == expected_kind
            )
        {
            return false;
        }

        previous_event_end = event_range.end;
        previous_source_end = source_range.end();
        previous_token_end = last_token_end;
    }

    true
}

fn skip_contiguous_trivia(mut offset: TextSize, end: TextSize, trivia: &[Trivia]) -> TextSize {
    let mut index = trivia.partition_point(|piece| piece.offset() < offset);
    while let Some(piece) = trivia.get(index)
        && piece.offset() == offset
        && piece.text_range().end() <= end
    {
        offset = piece.text_range().end();
        index += 1;
    }
    offset
}

fn is_complete_subtree(events: &[Event<MarkdownSyntaxKind>]) -> bool {
    let mut depth = 0usize;
    for (index, event) in events.iter().enumerate() {
        match event {
            Event::Start {
                forward_parent: None,
                ..
            } => depth += 1,
            Event::Start {
                forward_parent: Some(_),
                ..
            } => return false,
            Event::Finish => {
                let Some(next_depth) = depth.checked_sub(1) else {
                    return false;
                };
                depth = next_depth;
                if depth == 0 && index + 1 != events.len() {
                    return false;
                }
            }
            Event::Token { .. } if depth == 0 => return false,
            Event::Token { .. } => {}
        }
    }

    depth == 0
}

fn parse_inline_fragment<'source>(
    source: &'source str,
    deferred: &DeferredInline,
    options: MarkdownParserOptions,
    definitions: &'source LinkReferenceDefinitions,
) -> Option<MarkdownParserOutput> {
    let source_end = TextSize::try_from(source.len()).ok()?;
    let mut parser = MarkdownParser::new_range(
        source,
        TextRange::new(deferred.source_range().start(), source_end),
        options,
        deferred.context(),
        definitions,
    )?;

    let wrapper = parser.start();

    match deferred.flavor() {
        DeferredInlineFlavor::Paragraph => parse_inline_item_list(&mut parser),
        DeferredInlineFlavor::AtxParagraph => parse_header_content(&mut parser),
    }

    if parser.cur_range().start() != deferred.source_range().end() {
        wrapper.abandon(&mut parser);
        return None;
    }
    wrapper.complete(&mut parser, MD_ROOT);
    let mut output = parser.finish();
    let mut events = output.events.into_iter();
    if !matches!(
        events.next(),
        Some(Event::Start {
            kind: MD_ROOT,
            forward_parent: None,
        })
    ) || !matches!(events.next_back(), Some(Event::Finish))
    {
        return None;
    }
    output.events = events.collect();
    Some(output)
}
fn overlaps_ordered_ranges(
    range: TextRange,
    ranges: &[TextRange],
    range_index: &mut usize,
) -> bool {
    while ranges
        .get(*range_index)
        .is_some_and(|candidate| candidate.end() <= range.start())
    {
        *range_index += 1;
    }

    ranges.get(*range_index).is_some_and(|candidate| {
        candidate
            .intersect(range)
            .is_some_and(|intersection| !intersection.is_empty())
    })
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use biome_markdown_syntax::MarkdownSyntaxKind;

    use super::*;
    use crate::parser::InlineContainerContext;

    fn context() -> InlineContainerContext {
        MarkdownParser::new("", MarkdownParserOptions::default()).inline_container_context()
    }

    fn output(
        events: Vec<Event<MarkdownSyntaxKind>>,
        deferred_inlines: Vec<DeferredInline>,
    ) -> MarkdownParserOutput {
        MarkdownParserOutput {
            events,
            diagnostics: Vec::new(),
            trivia: Vec::new(),
            deferred_inlines,
            link_reference_definitions: LinkReferenceDefinitions::default(),
            list_tightness: Vec::new(),
            list_item_indents: Vec::new(),
            quote_indents: Vec::new(),
        }
    }

    fn paragraph_events(end: TextSize) -> Vec<Event<MarkdownSyntaxKind>> {
        vec![
            Event::Start {
                kind: MD_INLINE_ITEM_LIST,
                forward_parent: None,
            },
            Event::Token {
                kind: MarkdownSyntaxKind::MD_TEXTUAL_LITERAL,
                end,
            },
            Event::Finish,
        ]
    }

    fn deferred(event_range: Range<usize>, source_range: TextRange) -> DeferredInline {
        DeferredInline::for_test(
            event_range,
            source_range,
            DeferredInlineFlavor::Paragraph,
            context(),
            0,
        )
    }

    #[test]
    fn invalid_deferred_range_leaves_output_unchanged() {
        let output = output(
            paragraph_events(2.into()),
            vec![deferred(0..3, TextRange::new(1.into(), 2.into()))],
        );

        assert!(!validate_deferred_inlines("é", &output));
        assert_eq!(output.events.len(), 3);
        assert_eq!(output.deferred_inlines.len(), 1);
    }

    #[test]
    fn source_start_must_match_the_event_position() {
        let output = output(
            paragraph_events(2.into()),
            vec![deferred(0..3, TextRange::new(1.into(), 2.into()))],
        );

        assert!(!validate_deferred_inlines("ab", &output));
        assert_eq!(output.events.len(), 3);
        assert_eq!(output.deferred_inlines.len(), 1);
    }

    #[test]
    fn unchanged_record_still_participates_in_overlap_validation() {
        let mut events = paragraph_events(1.into());
        events.extend(paragraph_events(2.into()));
        let output = output(
            events,
            vec![
                deferred(0..3, TextRange::new(0.into(), 1.into())),
                deferred(2..6, TextRange::new(1.into(), 2.into())),
            ],
        );

        assert!(!validate_deferred_inlines("ab", &output));
        assert_eq!(output.events.len(), 6);
        assert_eq!(output.deferred_inlines.len(), 2);
    }

    #[test]
    fn deferred_event_range_must_contain_a_complete_subtree() {
        let events = vec![
            Event::Start {
                kind: MD_INLINE_ITEM_LIST,
                forward_parent: None,
            },
            Event::Start {
                kind: MarkdownSyntaxKind::MD_TEXTUAL,
                forward_parent: None,
            },
            Event::Token {
                kind: MarkdownSyntaxKind::MD_TEXTUAL_LITERAL,
                end: 1.into(),
            },
            Event::Finish,
            Event::Finish,
        ];
        let output = output(
            events,
            vec![deferred(0..4, TextRange::new(0.into(), 1.into()))],
        );

        assert!(!validate_deferred_inlines("a", &output));
        assert_eq!(output.events.len(), 5);
        assert_eq!(output.deferred_inlines.len(), 1);
    }

    #[test]
    fn fragment_must_end_at_the_recorded_source_boundary() {
        let deferred = deferred(0..3, TextRange::new(0.into(), 1.into()));

        assert!(
            parse_inline_fragment(
                "text\n",
                &deferred,
                MarkdownParserOptions::default(),
                &LinkReferenceDefinitions::default(),
            )
            .is_none()
        );
    }
}
