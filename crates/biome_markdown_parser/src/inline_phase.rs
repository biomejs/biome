//! Completes inline syntax after Markdown block parsing.
//!
//! CommonMark separates parsing into a block phase followed by an inline phase
//! because link reference definitions are document-global and are not all known
//! until block parsing finishes. See the
//! [CommonMark parsing strategy](https://spec.commonmark.org/0.31.2/#appendix-a-parsing-strategy).
//!
//! The block parser records each inline event subtree together with its source
//! range and the number of definitions known at that point. Inline subtrees that
//! precede later definitions are parsed again with the complete definition index.
//! Their replacement events, trivia, and diagnostics are merged before the
//! lossless tree sink constructs the CST, so source text is never rewritten.

use biome_markdown_syntax::MarkdownSyntaxKind::MD_ROOT;
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

/// Resolves deferred inline subtrees and returns an event stream ready for the
/// lossless tree sink.
///
/// The returned output contains no deferred inline records. Event replacement
/// requires the recorded event ranges to be ordered and non-overlapping.
pub(crate) fn parse_deferred_inlines(
    source: &str,
    options: &MarkdownParserOptions,
    mut output: MarkdownParserOutput,
) -> MarkdownParserOutput {
    debug_assert!(output.events.iter().all(|event| {
        !matches!(
            event,
            Event::Start {
                forward_parent: Some(_),
                ..
            }
        )
    }));

    let mut replacements = Vec::new();
    let mut previous_event_end = 0;

    for deferred in &output.deferred_inlines {
        if deferred.definitions_len == output.link_reference_definitions.len() {
            continue;
        }
        if deferred.event_range.start < previous_event_end
            || deferred.event_range.start > deferred.event_range.end
            || deferred.event_range.end > output.events.len()
        {
            continue;
        }

        let Some(fragment) = parse_inline_fragment(
            source,
            deferred,
            options.clone(),
            &output.link_reference_definitions,
        ) else {
            continue;
        };

        replacements.push((deferred, fragment));
        previous_event_end = deferred.event_range.end;
    }

    let mut events = Vec::with_capacity(output.events.len());
    let mut old_events = std::mem::take(&mut output.events).into_iter();
    let mut inline_trivia = Vec::new();
    let mut inline_diagnostics = Vec::new();
    let mut reparsed_ranges = Vec::with_capacity(replacements.len());
    let mut event_position = 0;

    for (deferred, fragment) in replacements {
        debug_assert!(event_position <= deferred.event_range.start);
        while event_position < deferred.event_range.start {
            let Some(event) = old_events.next() else {
                break;
            };
            events.push(event);
            event_position += 1;
        }
        while event_position < deferred.event_range.end {
            if old_events.next().is_none() {
                break;
            }
            event_position += 1;
        }
        events.extend(fragment.events);

        reparsed_ranges.push(deferred.source_range);
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
    output.diagnostics = merge_diagnostics(output.diagnostics, inline_diagnostics);
    output.deferred_inlines.clear();

    output
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
        TextRange::new(deferred.source_range.start(), source_end),
        options,
        deferred.context,
        definitions,
    )?;

    let wrapper = parser.start();

    match deferred.flavor {
        DeferredInlineFlavor::Paragraph => parse_inline_item_list(&mut parser),
        DeferredInlineFlavor::AtxParagraph => parse_header_content(&mut parser),
    }

    debug_assert_eq!(
        parser.cur_range().start(),
        deferred.source_range.end(),
        "inline fragment {:?} stopped at {:?} ({:?})",
        deferred.source_range,
        parser.cur_range(),
        (parser.cur(), &source[deferred.source_range])
    );
    wrapper.complete(&mut parser, MD_ROOT);
    let mut output = parser.finish();
    output.events.remove(0);
    output.events.pop();
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
