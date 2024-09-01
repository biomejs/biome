use std::{
    collections::{BTreeMap, BTreeSet},
    io, slice,
};

use biome_console::{fmt, markup, MarkupElement};
use biome_text_edit::{ChangeTag, CompressedOp, TextEdit};

use super::frame::{
    calculate_print_width, print_invisibles, text_width, IntoIter, OneIndexed,
    PrintInvisiblesOptions, CODE_FRAME_CONTEXT_LINES,
};

const MAX_PATCH_LINES: usize = 150;

pub(super) fn print_diff(fmt: &mut fmt::Formatter<'_>, diff: &TextEdit) -> io::Result<()> {
    // Before printing, we need to preprocess the list of DiffOps it's made of to classify them by line
    let mut modified_lines = BTreeSet::new();
    let mut inserted_lines = BTreeMap::new();
    let mut before_line_to_after = BTreeMap::new();

    let mut before_line = OneIndexed::MIN;
    let mut after_line = OneIndexed::MIN;

    process_diff_ops(
        diff,
        PushToLineState {
            modified_lines: &mut modified_lines,
            inserted_lines: &mut inserted_lines,
            before_line_to_after: &mut before_line_to_after,
        },
        &mut after_line,
        &mut before_line,
    );

    let before_line_count = before_line;
    let after_line_count = after_line;

    // If only a single line was modified, print a "short diff"
    let modified_line = if before_line_count == after_line_count {
        let mut iter = modified_lines.iter().filter_map(|key| {
            let line = inserted_lines.get(key)?;

            // A line has been modified if its diff list is empty (the line was
            // either fully inserted or fully removed) or if its diff list has
            // any delete or insert operation
            let has_edits = line.diffs.is_empty()
                || line.diffs.iter().any(|(tag, text)| {
                    matches!(tag, ChangeTag::Delete | ChangeTag::Insert) && !text.is_empty()
                });

            if has_edits {
                Some((key, line))
            } else {
                None
            }
        });

        iter.next().and_then(|(key, line)| {
            if iter.next().is_some() {
                return None;
            }

            // Disallow fully empty lines from being displayed in short mode
            if !line.diffs.is_empty() {
                Some((key, line))
            } else {
                None
            }
        })
    } else {
        None
    };

    if let Some((key, entry)) = modified_line {
        return print_short_diff(fmt, key, entry);
    }

    // Otherwise if multiple lines were modified we need to perform more preprocessing,
    // to merge identical line numbers and calculate how many context lines need to be rendered
    let mut diffs_by_line = Vec::new();
    let mut shown_line_indexes = BTreeSet::new();

    process_diff_lines(
        &mut inserted_lines,
        &mut before_line_to_after,
        &mut diffs_by_line,
        &mut shown_line_indexes,
        before_line_count,
        after_line_count,
    );

    // Finally when have a flat list of lines we can now print
    print_full_diff(
        fmt,
        &diffs_by_line,
        &shown_line_indexes,
        before_line_count,
        after_line_count,
    )
}

/// This function scans the list of DiffOps that make up the `diff` and derives
/// the following data structures:
/// - `modified_lines` is the set of [LineKey] that contain at least one insert
///     or delete operation
/// - `inserted_lines` maps a [LineKey] to the list of diff operations that
///     happen on the corresponding line
/// - `before_line_to_after` maps line numbers in the old revision of the text
///     to line numbers in the new revision
/// - `after_line` counts the number of lines in the new revision of the document
/// - `before_line` counts the number of lines in the old revision of the document
fn process_diff_ops<'diff>(
    diff: &'diff TextEdit,
    mut state: PushToLineState<'_, 'diff>,
    after_line: &mut OneIndexed,
    before_line: &mut OneIndexed,
) {
    for (op_index, op) in diff.iter().enumerate() {
        let op = match op {
            CompressedOp::DiffOp(op) => op,
            CompressedOp::EqualLines { line_count } => {
                let is_first_op = op_index == 0;
                for line_index in 0..=line_count.get() {
                    // Don't increment the first line if we are the first tuple marking the beginning of the file
                    if !(is_first_op && line_index == 0) {
                        *after_line = after_line.saturating_add(1);
                        *before_line = before_line.saturating_add(1);
                    }

                    state.before_line_to_after.insert(*before_line, *after_line);

                    push_to_line(&mut state, *before_line, *after_line, ChangeTag::Equal, "");
                }

                continue;
            }
        };

        let tag = op.tag();
        let text = op.text(diff);

        // Get all the lines
        let mut parts = text.split('\n');

        // Deconstruct each text chunk
        let current_line = parts.next();

        // The first chunk belongs to the current line
        if let Some(current_line) = current_line {
            push_to_line(&mut state, *before_line, *after_line, tag, current_line);
        }

        // Create unique lines for each other chunk
        for new_line in parts {
            match tag {
                ChangeTag::Equal => {
                    *after_line = after_line.saturating_add(1);
                    *before_line = before_line.saturating_add(1);
                }

                ChangeTag::Delete => {
                    *before_line = before_line.saturating_add(1);
                }
                ChangeTag::Insert => {
                    *after_line = after_line.saturating_add(1);
                }
            }

            state.before_line_to_after.insert(*before_line, *after_line);

            push_to_line(&mut state, *before_line, *after_line, tag, new_line);
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct LineKey {
    before_line: Option<OneIndexed>,
    after_line: Option<OneIndexed>,
}

impl LineKey {
    const fn before(before_line: OneIndexed) -> Self {
        Self {
            before_line: Some(before_line),
            after_line: None,
        }
    }

    const fn after(after_line: OneIndexed) -> Self {
        Self {
            before_line: None,
            after_line: Some(after_line),
        }
    }
}

#[derive(Debug, Clone)]
struct GroupDiffsLine<'a> {
    before_line: Option<OneIndexed>,
    after_line: Option<OneIndexed>,
    diffs: Vec<(ChangeTag, &'a str)>,
}

impl<'a> GroupDiffsLine<'a> {
    fn insert(
        inserted_lines: &mut BTreeMap<LineKey, Self>,
        key: LineKey,
        tag: ChangeTag,
        text: &'a str,
    ) {
        inserted_lines
            .entry(key)
            .and_modify(|line| {
                if !text.is_empty() {
                    line.diffs.push((tag, text));
                }
            })
            .or_insert_with_key(|key| GroupDiffsLine {
                before_line: key.before_line,
                after_line: key.after_line,
                diffs: if text.is_empty() {
                    Vec::new()
                } else {
                    vec![(tag, text)]
                },
            });
    }
}

struct PushToLineState<'a, 'b> {
    modified_lines: &'a mut BTreeSet<LineKey>,
    inserted_lines: &'a mut BTreeMap<LineKey, GroupDiffsLine<'b>>,
    before_line_to_after: &'a mut BTreeMap<OneIndexed, OneIndexed>,
}

fn push_to_line<'b>(
    state: &mut PushToLineState<'_, 'b>,
    before_line: OneIndexed,
    after_line: OneIndexed,
    tag: ChangeTag,
    text: &'b str,
) {
    let PushToLineState {
        modified_lines,
        inserted_lines,
        before_line_to_after,
    } = state;

    match tag {
        ChangeTag::Insert => {
            GroupDiffsLine::insert(inserted_lines, LineKey::after(after_line), tag, text);
            modified_lines.insert(LineKey::after(after_line));
        }
        ChangeTag::Delete => {
            GroupDiffsLine::insert(inserted_lines, LineKey::before(before_line), tag, text);
            modified_lines.insert(LineKey::before(before_line));
        }
        ChangeTag::Equal => {
            if before_line == OneIndexed::MIN && after_line == OneIndexed::MIN {
                before_line_to_after.insert(before_line, after_line);
            }

            GroupDiffsLine::insert(inserted_lines, LineKey::after(after_line), tag, text);
            GroupDiffsLine::insert(inserted_lines, LineKey::before(before_line), tag, text);
        }
    }
}

fn process_diff_lines<'lines, 'diff>(
    inserted_lines: &'lines mut BTreeMap<LineKey, GroupDiffsLine<'diff>>,
    before_line_to_after: &mut BTreeMap<OneIndexed, OneIndexed>,
    diffs_by_line: &mut Vec<&'lines GroupDiffsLine<'diff>>,
    shown_line_indexes: &mut BTreeSet<usize>,
    before_line_count: OneIndexed,
    after_line_count: OneIndexed,
) {
    // Merge identical lines
    for before_line in IntoIter::new(OneIndexed::MIN..=before_line_count) {
        let after_line = match before_line_to_after.get(&before_line) {
            Some(after_line) => *after_line,
            None => continue,
        };

        let inserted_before_line = inserted_lines.get(&LineKey::before(before_line));
        let inserted_after_line = inserted_lines.get(&LineKey::after(after_line));

        if let (Some(inserted_before_line), Some(inserted_after_line)) =
            (inserted_before_line, inserted_after_line)
        {
            if inserted_before_line.diffs == inserted_after_line.diffs {
                let line = inserted_lines
                    .remove(&LineKey::before(before_line))
                    .unwrap();

                inserted_lines.remove(&LineKey::after(after_line)).unwrap();

                inserted_lines.insert(
                    LineKey {
                        before_line: Some(before_line),
                        after_line: Some(after_line),
                    },
                    GroupDiffsLine {
                        before_line: Some(before_line),
                        after_line: Some(after_line),
                        diffs: line.diffs,
                    },
                );
            }
        }
    }

    let mut diffs_by_line_with_before_and_shared = Vec::new();

    // Print before lines, including those that are shared
    for before_line in IntoIter::new(OneIndexed::MIN..=before_line_count) {
        let line = inserted_lines.get(&LineKey::before(before_line));

        if let Some(line) = line {
            diffs_by_line_with_before_and_shared.push(line);
        }

        // If we have a shared line then add it
        if let Some(after_line) = before_line_to_after.get(&before_line) {
            let line = inserted_lines.get(&LineKey {
                before_line: Some(before_line),
                after_line: Some(*after_line),
            });

            if let Some(line) = line {
                diffs_by_line_with_before_and_shared.push(line);
            }
        }
    }

    // Calculate the parts of the diff we should show
    let mut last_printed_after = 0;

    for line in diffs_by_line_with_before_and_shared {
        if let Some(after_line) = line.after_line {
            catch_up_after(
                inserted_lines,
                diffs_by_line,
                shown_line_indexes,
                last_printed_after,
                after_line,
            );

            last_printed_after = after_line.get();
        }

        push_displayed_line(diffs_by_line, shown_line_indexes, line);
    }

    catch_up_after(
        inserted_lines,
        diffs_by_line,
        shown_line_indexes,
        last_printed_after,
        after_line_count,
    );
}

fn push_displayed_line<'input, 'group>(
    diffs_by_line: &mut Vec<&'group GroupDiffsLine<'input>>,
    shown_line_indexes: &mut BTreeSet<usize>,
    line: &'group GroupDiffsLine<'input>,
) {
    let i = diffs_by_line.len();
    diffs_by_line.push(line);

    if line.before_line.is_none() || line.after_line.is_none() {
        let first = i.saturating_sub(CODE_FRAME_CONTEXT_LINES.get());
        let last = i + CODE_FRAME_CONTEXT_LINES.get();
        shown_line_indexes.extend(first..=last);
    }
}

fn catch_up_after<'input, 'lines>(
    inserted_lines: &'lines BTreeMap<LineKey, GroupDiffsLine<'input>>,
    diffs_by_line: &mut Vec<&'lines GroupDiffsLine<'input>>,
    shown_line_indexes: &mut BTreeSet<usize>,
    last_printed_after: usize,
    after_line: OneIndexed,
) {
    let iter = IntoIter::new(OneIndexed::from_zero_indexed(last_printed_after)..=after_line);

    for i in iter {
        let key = LineKey::after(i);
        if let Some(line) = inserted_lines.get(&key) {
            push_displayed_line(diffs_by_line, shown_line_indexes, line);
        }
    }
}

fn print_short_diff(
    fmt: &mut fmt::Formatter<'_>,
    key: &LineKey,
    entry: &GroupDiffsLine<'_>,
) -> io::Result<()> {
    let index = match (key.before_line, key.after_line) {
        (None, Some(index)) | (Some(index), None) => index,
        (None, None) | (Some(_), Some(_)) => unreachable!(
            "the key of a modified line should have exactly one index in one of the two revisions"
        ),
    };

    fmt.write_markup(markup! {
        <Emphasis>
            {format_args!("  {} \u{2502} ", index.get())}
        </Emphasis>
    })?;

    let mut at_line_start = true;
    let last_index = entry.diffs.len().saturating_sub(1);

    for (i, (tag, text)) in entry.diffs.iter().enumerate() {
        let is_changed = *tag != ChangeTag::Equal;
        let options = PrintInvisiblesOptions {
            ignore_leading_tabs: false,
            ignore_lone_spaces: false,
            ignore_trailing_carriage_return: is_changed,
            at_line_start,
            at_line_end: i == last_index,
        };

        let element = match tag {
            ChangeTag::Equal => None,
            ChangeTag::Delete => Some(MarkupElement::Error),
            ChangeTag::Insert => Some(MarkupElement::Success),
        };

        let has_non_whitespace = if let Some(element) = element {
            let mut slot = None;
            let mut fmt = ElementWrapper::wrap(fmt, &mut slot, element);
            print_invisibles(&mut fmt, text, options)?
        } else {
            print_invisibles(fmt, text, options)?
        };

        if has_non_whitespace {
            at_line_start = false;
        }
    }

    fmt.write_str("\n")?;

    let no_length = calculate_print_width(index);
    fmt.write_markup(markup! {
        <Emphasis>
            {format_args!("  {: >1$} \u{2502} ", "", no_length.get())}
        </Emphasis>
    })?;

    for (tag, text) in &entry.diffs {
        let marker = match tag {
            ChangeTag::Equal => markup! { " " },
            ChangeTag::Delete => markup! { <Error>"-"</Error> },
            ChangeTag::Insert => markup! { <Success>"+"</Success> },
        };

        for _ in 0..text_width(text) {
            fmt.write_markup(marker)?;
        }
    }

    fmt.write_str("\n")
}

fn print_full_diff(
    fmt: &mut fmt::Formatter<'_>,
    diffs_by_line: &[&'_ GroupDiffsLine<'_>],
    shown_line_indexes: &BTreeSet<usize>,
    before_line_count: OneIndexed,
    after_line_count: OneIndexed,
) -> io::Result<()> {
    // Calculate width of line no column
    let before_no_length = calculate_print_width(before_line_count);
    let after_no_length = calculate_print_width(after_line_count);
    let line_no_length = before_no_length.get() + 1 + after_no_length.get();

    // Skip displaying the gutter if the file only has a single line
    let single_line = before_line_count == OneIndexed::MIN && after_line_count == OneIndexed::MIN;

    let mut displayed_lines = 0;
    let mut truncated = false;
    let mut last_displayed_line = None;

    // Print the actual frame
    for (i, line) in diffs_by_line.iter().enumerate() {
        if !shown_line_indexes.contains(&i) {
            continue;
        }

        displayed_lines += 1;

        if displayed_lines > MAX_PATCH_LINES {
            truncated = true;
            continue;
        }

        let mut line_type = ChangeTag::Equal;
        let mut marker = markup! { " " };

        if line.before_line.is_none() {
            marker = markup! { <Success>"+"</Success> };
            line_type = ChangeTag::Insert;
        }

        if line.after_line.is_none() {
            marker = markup! { <Error>"-"</Error> };
            line_type = ChangeTag::Delete;
        }

        if let Some(last_displayed_line) = last_displayed_line {
            if last_displayed_line + 1 != i {
                fmt.write_markup(markup! {
                    <Emphasis>"  "{"\u{b7}".repeat(line_no_length)}" \u{2502} \n"</Emphasis>
                })?;
            }
        }

        last_displayed_line = Some(i);

        if single_line {
            let line = FormatDiffLine {
                is_equal: line_type == ChangeTag::Equal,
                ops: &line.diffs,
            };

            match line_type {
                ChangeTag::Equal => fmt.write_markup(markup! {
                    "  "{line}"\n"
                })?,
                ChangeTag::Delete => fmt.write_markup(markup! {
                    {marker}" "<Error>{line}</Error>"\n"
                })?,
                ChangeTag::Insert => fmt.write_markup(markup! {
                    {marker}" "<Success>{line}</Success>"\n"
                })?,
            }
        } else {
            fmt.write_str("  ")?;

            if let Some(before_line) = line.before_line {
                fmt.write_markup(markup! {
                    <Emphasis>
                        {format_args!("{: >1$}", before_line.get(), before_no_length.get())}
                    </Emphasis>
                })?;
            } else {
                for _ in 0..before_no_length.get() {
                    fmt.write_str(" ")?;
                }
            }

            fmt.write_str(" ")?;

            if let Some(after_line) = line.after_line {
                fmt.write_markup(markup! {
                    <Emphasis>
                        {format_args!("{: >1$}", after_line.get(), after_no_length.get())}
                    </Emphasis>
                })?;
            } else {
                for _ in 0..after_no_length.get() {
                    fmt.write_str(" ")?;
                }
            }

            fmt.write_markup(markup! {
                <Emphasis>" \u{2502} "</Emphasis>{marker}' '
            })?;

            let line = FormatDiffLine {
                is_equal: line_type == ChangeTag::Equal,
                ops: &line.diffs,
            };

            match line_type {
                ChangeTag::Equal => fmt.write_markup(markup! {
                    {line}"\n"
                })?,
                ChangeTag::Delete => fmt.write_markup(markup! {
                    <Error>{line}</Error>"\n"
                })?,
                ChangeTag::Insert => fmt.write_markup(markup! {
                    <Success>{line}</Success>"\n"
                })?,
            }
        }
    }

    if truncated {
        fmt.write_markup(markup! {
            <Dim>{displayed_lines.saturating_sub(MAX_PATCH_LINES)}" more lines truncated\n"</Dim>
        })?;
    }

    fmt.write_str("\n")
}

struct FormatDiffLine<'a> {
    is_equal: bool,
    ops: &'a [(ChangeTag, &'a str)],
}

impl fmt::Display for FormatDiffLine<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> io::Result<()> {
        let mut at_line_start = true;
        let last_index = self.ops.len().saturating_sub(1);

        for (i, (tag, text)) in self.ops.iter().enumerate() {
            let is_changed = *tag != ChangeTag::Equal;
            let options = PrintInvisiblesOptions {
                ignore_leading_tabs: self.is_equal,
                ignore_lone_spaces: self.is_equal,
                ignore_trailing_carriage_return: is_changed,
                at_line_start,
                at_line_end: i == last_index,
            };

            let has_non_whitespace = if is_changed {
                let mut slot = None;
                let mut fmt = ElementWrapper::wrap(fmt, &mut slot, MarkupElement::Emphasis);
                print_invisibles(&mut fmt, text, options)?
            } else {
                print_invisibles(fmt, text, options)?
            };

            if has_non_whitespace {
                at_line_start = false;
            }
        }

        Ok(())
    }
}

struct ElementWrapper<'a, W: ?Sized>(&'a mut W, MarkupElement<'static>);

impl<'write> ElementWrapper<'write, dyn fmt::Write + 'write> {
    fn wrap<'slot, 'fmt: 'write + 'slot>(
        fmt: &'fmt mut fmt::Formatter<'_>,
        slot: &'slot mut Option<Self>,
        element: MarkupElement<'static>,
    ) -> fmt::Formatter<'slot> {
        fmt.wrap_writer(|writer| slot.get_or_insert(Self(writer, element)))
    }
}

impl<W: fmt::Write + ?Sized> fmt::Write for ElementWrapper<'_, W> {
    fn write_str(&mut self, elements: &fmt::MarkupElements<'_>, content: &str) -> io::Result<()> {
        let elements = fmt::MarkupElements::Node(elements, slice::from_ref(&self.1));
        self.0.write_str(&elements, content)
    }

    fn write_fmt(
        &mut self,
        elements: &fmt::MarkupElements<'_>,
        content: std::fmt::Arguments<'_>,
    ) -> io::Result<()> {
        let elements = fmt::MarkupElements::Node(elements, slice::from_ref(&self.1));
        self.0.write_fmt(&elements, content)
    }
}

#[cfg(test)]
mod tests {
    use super::print_diff;
    use biome_console::{fmt, markup, MarkupBuf};
    use biome_text_edit::TextEdit;
    use termcolor::Buffer;

    fn assert_eq_markup(actual: &MarkupBuf, expected: &MarkupBuf) {
        if actual != expected {
            let mut buffer = Buffer::ansi();
            let mut writer = fmt::Termcolor(&mut buffer);
            let mut output = fmt::Formatter::new(&mut writer);

            output
                .write_markup(markup! {
                    "assertion failed: (actual == expected)\n"
                    "actual:\n"
                    {actual}"\n"
                    {format_args!("{actual:#?}")}"\n"
                    "expected:\n"
                    {expected}"\n"
                    {format_args!("{expected:#?}")}"\n"
                })
                .unwrap();

            let buffer = buffer.into_inner();
            let buffer = String::from_utf8(buffer).unwrap();
            panic!("{buffer}");
        }
    }

    #[test]
    fn test_inline() {
        let diff = TextEdit::from_unicode_words("before", "after");

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            <Error>"-"</Error>" "<Error><Emphasis>"before"</Emphasis></Error>"\n"
            <Success>"+"</Success>" "<Success><Emphasis>"after"</Emphasis></Success>"\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn test_single_line() {
        let diff = TextEdit::from_unicode_words("start before end\n", "start after end \n");

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"start"</Error><Error><Dim>"·"</Dim></Error><Error><Emphasis>"before"</Emphasis></Error><Error><Dim>"·"</Dim></Error><Error>"end"</Error>"\n"
            "    "<Emphasis>"1 │ "</Emphasis><Success>"+"</Success>" "<Success>"start"</Success><Success><Dim>"·"</Dim></Success><Success><Emphasis>"after"</Emphasis></Success><Success><Dim>"·"</Dim></Success><Success>"end"</Success><Success><Dim><Emphasis>"·"</Emphasis></Dim></Success>"\n"
            "  "<Emphasis>"2"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn test_ellipsis() {
        const SOURCE_LEFT: &str = "Lorem
ipsum
dolor
sit
amet,
function
name(
    args
) {}
consectetur
adipiscing
elit,
sed
do
eiusmod

incididunt
function
name(
    args
) {}";

        const SOURCE_RIGHT: &str = "Lorem
ipsum
dolor
sit
amet,
function name(args) {
}
consectetur
adipiscing
elit,
sed
do
eiusmod

incididunt
function name(args) {
}";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>" 4"</Emphasis>" "<Emphasis>" 4 │ "</Emphasis>"  sit\n"
            "  "<Emphasis>" 5"</Emphasis>" "<Emphasis>" 5 │ "</Emphasis>"  amet,\n"
            "  "<Emphasis>" 6"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"function"</Error>"\n"
            "  "<Emphasis>" 7"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"name("</Error>"\n"
            "  "<Emphasis>" 8"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error><Dim><Emphasis>"····"</Emphasis></Dim></Error><Error>"args"</Error>"\n"
            "  "<Emphasis>" 9"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>")"</Error><Error><Dim>"·"</Dim></Error><Error>"{}"</Error>"\n"
            "     "<Emphasis>" 6 │ "</Emphasis><Success>"+"</Success>" "<Success>"function"</Success><Success><Dim><Emphasis>"·"</Emphasis></Dim></Success><Success>"name(args)"</Success><Success><Dim>"·"</Dim></Success><Success>"{"</Success>"\n"
            "     "<Emphasis>" 7 │ "</Emphasis><Success>"+"</Success>" "<Success>"}"</Success>"\n"
            "  "<Emphasis>"10"</Emphasis>" "<Emphasis>" 8 │ "</Emphasis>"  consectetur\n"
            "  "<Emphasis>"11"</Emphasis>" "<Emphasis>" 9 │ "</Emphasis>"  adipiscing\n"
            <Emphasis>"  ····· │ \n"
            </Emphasis>"  "<Emphasis>"16"</Emphasis>" "<Emphasis>"14 │ "</Emphasis>"  \n"
            "  "<Emphasis>"17"</Emphasis>" "<Emphasis>"15 │ "</Emphasis>"  incididunt\n"
            "  "<Emphasis>"18"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"function"</Error>"\n"
            "  "<Emphasis>"19"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"name("</Error>"\n"
            "  "<Emphasis>"20"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error><Dim><Emphasis>"····"</Emphasis></Dim></Error><Error>"args"</Error>"\n"
            "  "<Emphasis>"21"</Emphasis>"   "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>")"</Error><Error><Dim>"·"</Dim></Error><Error>"{}"</Error>"\n"
            "     "<Emphasis>"16 │ "</Emphasis><Success>"+"</Success>" "<Success>"function"</Success><Success><Dim><Emphasis>"·"</Emphasis></Dim></Success><Success>"name(args)"</Success><Success><Dim>"·"</Dim></Success><Success>"{"</Success>"\n"
            "     "<Emphasis>"17 │ "</Emphasis><Success>"+"</Success>" "<Success>"}"</Success>"\n"
            "\n"
        }.to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn remove_single_line() {
        const SOURCE_LEFT: &str = "declare module \"test\" {
	interface A {

		prop: string;
	}
}
";

        const SOURCE_RIGHT: &str = "declare module \"test\" {
	interface A {
		prop: string;
	}
}
";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>" "<Emphasis>"1 │ "</Emphasis>"  declare module \"test\" {\n"
            "  "<Emphasis>"2"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \tinterface A {\n"
            "  "<Emphasis>"3"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" \n"
            "  "<Emphasis>"4"</Emphasis>" "<Emphasis>"3 │ "</Emphasis>"  \t\tprop: string;\n"
            "  "<Emphasis>"5"</Emphasis>" "<Emphasis>"4 │ "</Emphasis>"  \t}\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn remove_many_lines() {
        const SOURCE_LEFT: &str = "declare module \"test\" {
	interface A {



		prop: string;
	}
}
";

        const SOURCE_RIGHT: &str = "declare module \"test\" {
	interface A {
		prop: string;
	}
}
";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>" "<Emphasis>"1 │ "</Emphasis>"  declare module \"test\" {\n"
            "  "<Emphasis>"2"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \tinterface A {\n"
            "  "<Emphasis>"3"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" \n"
            "  "<Emphasis>"4"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" \n"
            "  "<Emphasis>"5"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" \n"
            "  "<Emphasis>"6"</Emphasis>" "<Emphasis>"3 │ "</Emphasis>"  \t\tprop: string;\n"
            "  "<Emphasis>"7"</Emphasis>" "<Emphasis>"4 │ "</Emphasis>"  \t}\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn insert_single_line() {
        const SOURCE_LEFT: &str = "declare module \"test\" {
	interface A {
		prop: string;
	}
}
";

        const SOURCE_RIGHT: &str = "declare module \"test\" {
	interface A {

		prop: string;
	}
}
";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>" "<Emphasis>"1 │ "</Emphasis>"  declare module \"test\" {\n"
            "  "<Emphasis>"2"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \tinterface A {\n"
            "    "<Emphasis>"3 │ "</Emphasis><Success>"+"</Success>" \n"
            "  "<Emphasis>"3"</Emphasis>" "<Emphasis>"4 │ "</Emphasis>"  \t\tprop: string;\n"
            "  "<Emphasis>"4"</Emphasis>" "<Emphasis>"5 │ "</Emphasis>"  \t}\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn insert_many_lines() {
        const SOURCE_LEFT: &str = "declare module \"test\" {
	interface A {
		prop: string;
	}
}
";

        const SOURCE_RIGHT: &str = "declare module \"test\" {
	interface A {



		prop: string;
	}
}
";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>" "<Emphasis>"1 │ "</Emphasis>"  declare module \"test\" {\n"
            "  "<Emphasis>"2"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \tinterface A {\n"
            "    "<Emphasis>"3 │ "</Emphasis><Success>"+"</Success>" \n"
            "    "<Emphasis>"4 │ "</Emphasis><Success>"+"</Success>" \n"
            "    "<Emphasis>"5 │ "</Emphasis><Success>"+"</Success>" \n"
            "  "<Emphasis>"3"</Emphasis>" "<Emphasis>"6 │ "</Emphasis>"  \t\tprop: string;\n"
            "  "<Emphasis>"4"</Emphasis>" "<Emphasis>"7 │ "</Emphasis>"  \t}\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }

    #[test]
    fn remove_empty_line() {
        const SOURCE_LEFT: &str = "for (; ;) {
}

console.log(\"test\");
";

        const SOURCE_RIGHT: &str = "for (;;) {}

console.log(\"test\");
";

        let diff = TextEdit::from_unicode_words(SOURCE_LEFT, SOURCE_RIGHT);

        let mut output = MarkupBuf::default();
        print_diff(&mut fmt::Formatter::new(&mut output), &diff).unwrap();

        let expected = markup! {
            "  "<Emphasis>"1"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"for"</Error><Error><Dim>"·"</Dim></Error><Error>"(;"</Error><Error><Dim><Emphasis>"·"</Emphasis></Dim></Error><Error>";)"</Error><Error><Dim>"·"</Dim></Error><Error>"{"</Error>"\n"
            "  "<Emphasis>"2"</Emphasis>"  "<Emphasis>" │ "</Emphasis><Error>"-"</Error>" "<Error>"}"</Error>"\n"
            "    "<Emphasis>"1 │ "</Emphasis><Success>"+"</Success>" "<Success>"for"</Success><Success><Dim>"·"</Dim></Success><Success>"(;;)"</Success><Success><Dim>"·"</Dim></Success><Success>"{}"</Success>"\n"
            "  "<Emphasis>"3"</Emphasis>" "<Emphasis>"2 │ "</Emphasis>"  \n"
            "  "<Emphasis>"4"</Emphasis>" "<Emphasis>"3 │ "</Emphasis>"  console.log(\"test\");\n"
            "\n"
        }
        .to_owned();

        assert_eq_markup(&output, &expected);
    }
}
