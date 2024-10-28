use anyhow::{ensure, Context, Result};
use biome_analyze::ActionCategory;
use biome_console::fmt::Termcolor;
use biome_console::fmt::{self, Formatter};
use biome_console::MarkupBuf;
use biome_diagnostics::termcolor::NoColor;
use biome_diagnostics::{
    Applicability, {Diagnostic, DiagnosticTags, Location, PrintDescription, Severity, Visit},
};
use biome_lsp_converters::line_index::LineIndex;
use biome_lsp_converters::{from_proto, to_proto, PositionEncoding};
use biome_rowan::{TextRange, TextSize};
use biome_service::workspace::CodeAction;
use biome_text_edit::{CompressedOp, DiffOp, TextEdit};
use std::any::Any;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::ops::{Add, Range};
use std::{io, mem};
use tower_lsp::jsonrpc::Error as LspError;
use tower_lsp::lsp_types;
use tower_lsp::lsp_types::{self as lsp, CodeDescription, Url};
use tracing::error;

pub(crate) fn text_edit(
    line_index: &LineIndex,
    diff: TextEdit,
    position_encoding: PositionEncoding,
    offset: Option<u32>,
) -> Result<Vec<lsp::TextEdit>> {
    let mut result: Vec<lsp::TextEdit> = Vec::new();
    let mut offset = if let Some(offset) = offset {
        TextSize::from(offset)
    } else {
        TextSize::from(0)
    };

    for op in diff.iter() {
        match op {
            CompressedOp::DiffOp(DiffOp::Equal { range }) => {
                offset += range.len();
            }
            CompressedOp::DiffOp(DiffOp::Insert { range }) => {
                let start = to_proto::position(line_index, offset, position_encoding)?;

                // Merge with a previous delete operation if possible
                let last_edit = result.last_mut().filter(|text_edit| {
                    text_edit.range.end == start && text_edit.new_text.is_empty()
                });

                if let Some(last_edit) = last_edit {
                    last_edit.new_text = diff.get_text(*range).to_string();
                } else {
                    result.push(lsp::TextEdit {
                        range: lsp::Range::new(start, start),
                        new_text: diff.get_text(*range).to_string(),
                    });
                }
            }
            CompressedOp::DiffOp(DiffOp::Delete { range }) => {
                let start = to_proto::position(line_index, offset, position_encoding)?;
                offset += range.len();
                let end = to_proto::position(line_index, offset, position_encoding)?;

                result.push(lsp::TextEdit {
                    range: lsp::Range::new(start, end),
                    new_text: String::new(),
                });
            }

            CompressedOp::EqualLines { line_count } => {
                let mut line_col = line_index
                    .line_col(offset)
                    .expect("diff length is overflowing the line count in the original file");

                line_col.line += line_count.get() + 1;
                line_col.col = 0;

                // SAFETY: This should only happen if `line_index` wasn't built
                // from the same string as the old revision of `diff`
                let new_offset = line_index
                    .offset(line_col)
                    .expect("diff length is overflowing the line count in the original file");

                offset = new_offset;
            }
        }
    }

    Ok(result)
}

pub(crate) fn code_fix_to_lsp(
    url: &lsp::Url,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    diagnostics: &[lsp::Diagnostic],
    action: CodeAction,
    offset: Option<u32>,
) -> Result<lsp::CodeAction> {
    // Mark diagnostics emitted by the same rule as resolved by this action
    let diagnostics: Vec<_> = action
        .rule_name
        .as_ref()
        .filter(|_| action.category.matches("quickfix"))
        .map(|(group_name, rule_name)| {
            diagnostics
                .iter()
                .filter_map(|d| {
                    let code = d.code.as_ref()?;
                    let code = match code {
                        lsp::NumberOrString::String(code) => code.as_str(),
                        lsp::NumberOrString::Number(_) => return None,
                    };

                    let code = code.strip_prefix("lint/")?;
                    let code = code.strip_prefix(group_name.as_ref())?;
                    let code = code.strip_prefix('/')?;

                    if code == rule_name {
                        Some(d.clone())
                    } else {
                        None
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let kind = action.category.to_str().into_owned();
    let suggestion = action.suggestion;

    let mut changes = HashMap::new();
    let edits = text_edit(line_index, suggestion.suggestion, position_encoding, offset)?;

    changes.insert(url.clone(), edits);

    let edit = lsp::WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    let is_preferred = matches!(action.category, ActionCategory::Source(_))
        || matches!(suggestion.applicability, Applicability::Always)
            && !action.category.matches("quickfix.suppressRule");

    Ok(lsp::CodeAction {
        title: print_markup(&suggestion.msg),
        kind: Some(lsp::CodeActionKind::from(kind)),
        diagnostics: if !diagnostics.is_empty() {
            Some(diagnostics)
        } else {
            None
        },
        edit: Some(edit),
        command: None,
        is_preferred: is_preferred.then_some(true),
        disabled: None,
        data: None,
    })
}

/// Convert an [biome_diagnostics::Diagnostic] to a [lsp::Diagnostic], using the span
/// of the diagnostic's primary label as the diagnostic range.
/// Requires a [LineIndex] to convert a byte offset range to the line/col range
/// expected by LSP.
pub(crate) fn diagnostic_to_lsp<D: Diagnostic>(
    diagnostic: D,
    url: &lsp::Url,
    line_index: &LineIndex,
    position_encoding: PositionEncoding,
    offset: Option<u32>,
) -> Result<lsp::Diagnostic> {
    let location = diagnostic.location();

    let span = location.span.context("diagnostic location has no span")?;
    let span = if let Some(offset) = offset {
        TextRange::new(
            span.start().add(TextSize::from(offset)),
            span.end().add(TextSize::from(offset)),
        )
    } else {
        span
    };
    let span = to_proto::range(line_index, span, position_encoding)
        .context("failed to convert diagnostic span to LSP range")?;

    let severity = match diagnostic.severity() {
        Severity::Fatal | Severity::Error => lsp::DiagnosticSeverity::ERROR,
        Severity::Warning => lsp::DiagnosticSeverity::WARNING,
        Severity::Information => lsp::DiagnosticSeverity::INFORMATION,
        Severity::Hint => lsp::DiagnosticSeverity::HINT,
    };

    let code = diagnostic
        .category()
        .map(|category| lsp::NumberOrString::String(category.name().to_string()));

    let code_description = diagnostic
        .category()
        .and_then(|category| category.link())
        .and_then(|link| {
            let href = Url::parse(link).ok()?;
            Some(CodeDescription { href })
        });

    let message = PrintDescription(&diagnostic).to_string();
    ensure!(!message.is_empty(), "diagnostic description is empty");

    let mut related_information = None;
    let mut visitor = RelatedInformationVisitor {
        url,
        line_index,
        position_encoding,
        related_information: &mut related_information,
    };

    diagnostic.advices(&mut visitor).unwrap();

    let tags = diagnostic.tags();
    let tags = {
        let mut result = Vec::new();

        if tags.contains(DiagnosticTags::UNNECESSARY_CODE) {
            result.push(lsp::DiagnosticTag::UNNECESSARY);
        }

        if tags.contains(DiagnosticTags::DEPRECATED_CODE) {
            result.push(lsp::DiagnosticTag::DEPRECATED);
        }

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    };

    let mut diagnostic = lsp::Diagnostic::new(
        span,
        Some(severity),
        code,
        Some("biome".into()),
        message,
        related_information,
        tags,
    );
    diagnostic.code_description = code_description;
    Ok(diagnostic)
}

struct RelatedInformationVisitor<'a> {
    url: &'a lsp::Url,
    line_index: &'a LineIndex,
    position_encoding: PositionEncoding,
    related_information: &'a mut Option<Vec<lsp::DiagnosticRelatedInformation>>,
}

impl Visit for RelatedInformationVisitor<'_> {
    fn record_frame(&mut self, location: Location<'_>) -> io::Result<()> {
        let span = match location.span {
            Some(span) => span,
            None => return Ok(()),
        };

        let range = match to_proto::range(self.line_index, span, self.position_encoding) {
            Ok(range) => range,
            Err(_) => return Ok(()),
        };

        let related_information = self.related_information.get_or_insert_with(Vec::new);

        related_information.push(lsp::DiagnosticRelatedInformation {
            location: lsp::Location {
                uri: self.url.clone(),
                range,
            },
            message: String::new(),
        });

        Ok(())
    }
}

/// Convert a piece of markup into a String
fn print_markup(markup: &MarkupBuf) -> String {
    let mut message = Termcolor(NoColor::new(Vec::new()));
    fmt::Display::fmt(markup, &mut Formatter::new(&mut message))
        // SAFETY: Writing to a memory buffer should never fail
        .unwrap();

    // SAFETY: Printing uncolored markup never generates non UTF-8 byte sequences
    String::from_utf8(message.0.into_inner()).unwrap()
}

/// Helper to create a [tower_lsp::jsonrpc::Error] from a message
pub(crate) fn into_lsp_error(msg: impl Display + Debug) -> LspError {
    let mut error = LspError::internal_error();
    error!("Error: {}", msg);
    error.message = Cow::Owned(msg.to_string());
    error.data = Some(format!("{msg:?}").into());
    error
}

pub(crate) fn panic_to_lsp_error(err: Box<dyn Any + Send>) -> LspError {
    let mut error = LspError::internal_error();

    match err.downcast::<String>() {
        Ok(msg) => {
            error.message = Cow::Owned(msg.to_string());
        }
        Err(err) => match err.downcast::<&str>() {
            Ok(msg) => {
                error.message = Cow::Owned(msg.to_string());
            }
            Err(_) => {
                error.message = Cow::Owned(String::from("Biome encountered an unknown error"));
            }
        },
    }

    error
}

pub(crate) fn apply_document_changes(
    position_encoding: PositionEncoding,
    current_content: String,
    mut content_changes: Vec<lsp_types::TextDocumentContentChangeEvent>,
) -> String {
    // Skip to the last full document change, as it invalidates all previous changes anyways.
    let mut start = content_changes
        .iter()
        .rev()
        .position(|change| change.range.is_none())
        .map_or(0, |idx| content_changes.len() - idx - 1);

    let mut text: String = match content_changes.get_mut(start) {
        // peek at the first content change as an optimization
        Some(lsp_types::TextDocumentContentChangeEvent {
            range: None, text, ..
        }) => {
            let text = mem::take(text);
            start += 1;

            // The only change is a full document update
            if start == content_changes.len() {
                return text;
            }
            text
        }
        Some(_) => current_content,
        // we received no content changes
        None => return current_content,
    };

    let mut line_index = LineIndex::new(&text);

    // The changes we got must be applied sequentially, but can cross lines so we
    // have to keep our line index updated.
    // Some clients (e.g. Code) sort the ranges in reverse. As an optimization, we
    // remember the last valid line in the index and only rebuild it if needed.
    let mut index_valid = u32::MAX;
    for change in content_changes {
        // The None case can't happen as we have handled it above already
        if let Some(range) = change.range {
            if index_valid <= range.end.line {
                line_index = LineIndex::new(&text);
            }
            index_valid = range.start.line;
            if let Ok(range) = from_proto::text_range(&line_index, range, position_encoding) {
                text.replace_range(Range::<usize>::from(range), &change.text);
            }
        }
    }
    text
}

#[cfg(test)]
mod tests {
    use super::apply_document_changes;
    use biome_lsp_converters::line_index::LineIndex;
    use biome_lsp_converters::{PositionEncoding, WideEncoding};
    use biome_text_edit::TextEdit;
    use tower_lsp::lsp_types as lsp;
    use tower_lsp::lsp_types::{Position, Range, TextDocumentContentChangeEvent};

    #[test]
    fn test_diff_1() {
        const OLD: &str = "line 1 old
line 2
line 3
line 4
line 5
line 6
line 7 old";

        const NEW: &str = "line 1 new
line 2
line 3
line 4
line 5
line 6
line 7 new";

        let line_index = LineIndex::new(OLD);
        let diff = TextEdit::from_unicode_words(OLD, NEW);

        let text_edit = super::text_edit(&line_index, diff, PositionEncoding::Utf8, None).unwrap();

        assert_eq!(
            text_edit.as_slice(),
            &[
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 7,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 10,
                        },
                    },
                    new_text: String::from("new"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 6,
                            character: 7
                        },
                        end: lsp::Position {
                            line: 6,
                            character: 10
                        }
                    },
                    new_text: String::from("new"),
                },
            ]
        );
    }

    #[test]
    fn test_diff_2() {
        const OLD: &str = "console.log(\"Variable: \" + variable);";
        const NEW: &str = "console.log(`Variable: ${variable}`);";

        let line_index = LineIndex::new(OLD);
        let diff = TextEdit::from_unicode_words(OLD, NEW);

        let text_edit = super::text_edit(&line_index, diff, PositionEncoding::Utf8, None).unwrap();

        assert_eq!(
            text_edit.as_slice(),
            &[
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 12,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 13,
                        },
                    },
                    new_text: String::from("`"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 23
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 27
                        }
                    },
                    new_text: String::from("${"),
                },
                lsp::TextEdit {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 35
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 35
                        }
                    },
                    new_text: String::from("}`"),
                },
            ]
        );
    }

    #[test]
    fn test_range_formatting() {
        let encoding = PositionEncoding::Wide(WideEncoding::Utf16);
        let input = "(\"Jan 1, 2018\u{2009}–\u{2009}Jan 1, 2019\");\n(\"Jan 1, 2018\u{2009}–\u{2009}Jan 1, 2019\");\nisSpreadAssignment;\n".to_string();
        let change = TextDocumentContentChangeEvent {
            range: Some(Range::new(Position::new(0, 30), Position::new(1, 0))),
            range_length: Some(1),
            text: String::new(),
        };

        let output = apply_document_changes(encoding, input, vec![change]);
        let expected = "(\"Jan 1, 2018\u{2009}–\u{2009}Jan 1, 2019\");(\"Jan 1, 2018\u{2009}–\u{2009}Jan 1, 2019\");\nisSpreadAssignment;\n";

        assert_eq!(output, expected);
    }
}
