use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::markup;
use biome_diagnostics::display::SourceFile;
use biome_diagnostics::{Error, Location, PrintDescription, Visit};
use biome_rowan::{TextRange, TextSize};
use biome_text_edit::{CompressedOp, DiffOp, TextEdit};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

pub(crate) struct RdJsonReporter<'a> {
    pub(crate) diagnostics_payload: &'a DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for RdJsonReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> std::io::Result<()> {
        visitor.report_diagnostics(
            writer,
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        Ok(())
    }
}

pub(crate) struct RdJsonReporterVisitor;

impl ReporterVisitor for RdJsonReporterVisitor {
    fn report_summary(
        &mut self,
        _writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        writer: &mut dyn ReporterWriter,
        _execution: &dyn Execution,
        payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> std::io::Result<()> {
        let rdjson_diagnostics: Vec<_> = payload
            .diagnostics
            .iter()
            .filter_map(|diagnostic| {
                if diagnostic.severity() >= payload.diagnostic_level {
                    if diagnostic.tags().is_verbose() {
                        if verbose {
                            diagnostic_to_rdjson(diagnostic)
                        } else {
                            None
                        }
                    } else {
                        diagnostic_to_rdjson(diagnostic)
                    }
                } else {
                    None
                }
            })
            .collect();

        let report = RdJsonReport {
            source: RdJsonSource {
                name: "Biome",
                url: "https://biomejs.dev",
            },
            diagnostics: rdjson_diagnostics,
        };

        let result = serde_json::to_string_pretty(&report)?;

        writer.log(markup! {
            {result}
        });

        Ok(())
    }
}

fn diagnostic_to_rdjson<'a>(diagnostic: &'a Error) -> Option<RdJsonDiagnostic<'a>> {
    let message = PrintDescription(diagnostic).to_string();
    let location = diagnostic.location();
    let location = to_rdjson_location(&location);

    let suggestions = to_rdjson_suggestions(diagnostic);
    let category = diagnostic.category()?;
    let code = RdJsonCode {
        url: category.link().map(String::from),
        value: category.name(),
    };

    Some(RdJsonDiagnostic {
        code,
        location,
        message,
        suggestions,
    })
}

fn to_rdjson_location(location: &Location<'_>) -> Option<RdJsonLocation> {
    let (Some(span), Some(source_code), Some(resource)) =
        (location.span, location.source_code, location.resource)
    else {
        return None;
    };
    let resource = resource.as_file()?;
    let source = SourceFile::new(source_code);
    let start = source.location(span.start()).ok()?;
    let end = source.location(span.end()).ok()?;
    Some(RdJsonLocation {
        path: resource.to_string(),
        range: Some(RdJsonRange {
            start: RdJsonLineColumn {
                column: start.column_number.get(),
                line: start.line_number.get(),
            },
            end: RdJsonLineColumn {
                column: end.column_number.get(),
                line: end.line_number.get(),
            },
        }),
    })
}

struct SuggestionsVisitor {
    suggestions: Vec<RdJsonSuggestion>,
}

impl Visit for SuggestionsVisitor {
    fn record_code_suggestion(
        &mut self,
        location: Location<'_>,
        diff: &TextEdit,
    ) -> std::io::Result<()> {
        let Some(source_code) = location.source_code else {
            return Ok(());
        };
        let Some(span) = changed_input_range(diff, source_code.text).or(location.span) else {
            return Ok(());
        };
        let Some(text) = suggestion_text(diff, source_code.text, span) else {
            return Ok(());
        };

        let source = SourceFile::new(source_code);
        let Ok(start) = source.location(span.start()) else {
            return Ok(());
        };
        let Ok(end) = source.location(span.end()) else {
            return Ok(());
        };
        let range = RdJsonRange {
            end: RdJsonLineColumn {
                line: end.line_number.get(),
                column: end.column_number.get(),
            },
            start: RdJsonLineColumn {
                line: start.line_number.get(),
                column: start.column_number.get(),
            },
        };

        self.suggestions.push(RdJsonSuggestion { text, range });

        Ok(())
    }
}

fn to_rdjson_suggestions(diagnostic: &Error) -> Vec<RdJsonSuggestion> {
    let mut visitor = SuggestionsVisitor {
        suggestions: vec![],
    };

    diagnostic.advices(&mut visitor).unwrap();

    visitor.suggestions
}

/// Computes the replacement text that should be applied to `range`.
fn suggestion_text(diff: &TextEdit, source: &str, range: TextRange) -> Option<String> {
    let mut output = String::new();
    let mut input_position = TextSize::from(0);

    for op in diff {
        match op {
            CompressedOp::DiffOp(DiffOp::Equal { range: diff_range }) => {
                let text = diff.get_text(*diff_range);
                append_overlap(&mut output, text, input_position, range)?;
                input_position += diff_range.len();
            }
            CompressedOp::DiffOp(DiffOp::Insert { range: diff_range }) => {
                if range.start() <= input_position && input_position <= range.end() {
                    output.push_str(diff.get_text(*diff_range));
                }
            }
            CompressedOp::DiffOp(DiffOp::Delete { range: diff_range }) => {
                input_position += diff_range.len();
            }
            CompressedOp::EqualLines { line_count } => {
                let text = equal_lines_text(source, input_position, line_count.get())?;
                append_overlap(&mut output, text, input_position, range)?;
                input_position += TextSize::of(text);
            }
        }
    }

    Some(output)
}

/// Returns the input range touched by `diff`.
fn changed_input_range(diff: &TextEdit, source: &str) -> Option<TextRange> {
    let mut input_position = TextSize::from(0);
    let mut start = None;
    let mut end = TextSize::from(0);

    for op in diff {
        match op {
            CompressedOp::DiffOp(DiffOp::Equal { range }) => {
                input_position += range.len();
            }
            CompressedOp::DiffOp(DiffOp::Insert { .. }) => {
                start.get_or_insert(input_position);
                end = input_position;
            }
            CompressedOp::DiffOp(DiffOp::Delete { range }) => {
                start.get_or_insert(input_position);
                input_position += range.len();
                end = input_position;
            }
            CompressedOp::EqualLines { line_count } => {
                let text = equal_lines_text(source, input_position, line_count.get())?;
                input_position += TextSize::of(text);
            }
        }
    }

    start.map(|start| TextRange::new(start, end))
}

/// Appends the part of `text` that overlaps with `range`.
fn append_overlap(
    output: &mut String,
    text: &str,
    text_start: TextSize,
    range: TextRange,
) -> Option<()> {
    let text_range = TextRange::at(text_start, TextSize::of(text));
    let Some(overlap) = text_range.intersect(range) else {
        return Some(());
    };

    let relative_range = overlap - text_start;
    let relative_range: std::ops::Range<usize> = relative_range.into();
    output.push_str(text.get(relative_range)?);
    Some(())
}

/// Replays the source text covered by a compressed equal-lines diff operation.
fn equal_lines_text(source: &str, start: TextSize, line_count: u32) -> Option<&str> {
    let input = source.get(usize::from(start)..)?;
    let mut length = TextSize::from(0);

    // Keep the same line-count semantics as TextEdit::new_string. Splitting on
    // '\n' preserves CRLF input because the preceding '\r' remains in each line.
    for line in input.split_inclusive('\n').take(line_count as usize + 1) {
        length += TextSize::of(line);
    }

    let range = TextRange::at(start, length);
    let range: std::ops::Range<usize> = range.into();
    debug_assert!(range.end <= source.len());
    source.get(range)
}

#[derive(Serialize)]
pub struct RdJsonReport<'a> {
    source: RdJsonSource,
    diagnostics: Vec<RdJsonDiagnostic<'a>>,
}

#[derive(Serialize)]
struct RdJsonSource {
    name: &'static str,
    url: &'static str,
}

#[derive(Serialize)]
struct RdJsonDiagnostic<'a> {
    code: RdJsonCode<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<RdJsonLocation>,
    message: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    suggestions: Vec<RdJsonSuggestion>,
}

#[derive(Serialize)]
struct RdJsonCode<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    value: &'a str,
}
#[derive(Serialize)]
struct RdJsonLocation {
    path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    range: Option<RdJsonRange>,
}

#[derive(Default, Serialize)]
struct RdJsonRange {
    end: RdJsonLineColumn,
    start: RdJsonLineColumn,
}

#[derive(Serialize)]
pub struct RdJsonSuggestion {
    range: RdJsonRange,
    text: String,
}

#[derive(Default, Serialize)]
pub struct RdJsonLineColumn {
    column: usize,
    line: usize,
}

#[cfg(test)]
mod tests {
    use super::{changed_input_range, equal_lines_text, suggestion_text};
    use biome_rowan::{TextRange, TextSize};
    use biome_text_edit::TextEdit;

    #[test]
    fn changed_input_range_uses_precise_replacement_range() {
        let source = "let f;\n";
        let diff = TextEdit::from_unicode_words(source, "let _f;\n");
        let range = changed_input_range(&diff, source).unwrap();

        assert_eq!(range, TextRange::new(TextSize::from(4), TextSize::from(5)));
        assert_eq!(suggestion_text(&diff, source, range), Some("_f".into()));
    }

    #[test]
    fn changed_input_range_handles_insertions() {
        let source = "let f;\n";
        let mut builder = TextEdit::builder();
        builder.equal("let ");
        builder.insert("_");
        builder.equal("f;\n");
        let diff = builder.finish();
        let range = changed_input_range(&diff, source).unwrap();

        assert_eq!(range, TextRange::empty(TextSize::from(4)));
        assert_eq!(suggestion_text(&diff, source, range), Some("_".into()));
    }

    #[test]
    fn equal_lines_text_preserves_crlf() {
        let source = "a\r\nb\r\nc\r\n";

        assert_eq!(
            equal_lines_text(source, TextSize::from(0), 2),
            Some("a\r\nb\r\nc\r\n")
        );
    }

    #[test]
    fn equal_lines_text_rejects_invalid_offsets() {
        assert_eq!(equal_lines_text("abc", TextSize::from(99), 1), None);
    }
}
