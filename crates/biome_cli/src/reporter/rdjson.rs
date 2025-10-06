use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Console, ConsoleExt, MarkupBuf, markup};
use biome_diagnostics::display::{SourceFile, markup_to_string};
use biome_diagnostics::{Error, Location, LogCategory, PrintDescription, Visit};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

pub(crate) struct RdJsonReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for RdJsonReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_diagnostics(
            &self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        Ok(())
    }
}

pub(crate) struct RdJsonReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for RdJsonReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        _execution: &Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &Execution,
        payload: DiagnosticsPayload,
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

        self.0.log(markup! {
            {result}
        });

        Ok(())
    }
}

fn diagnostic_to_rdjson<'a>(diagnostic: &'a Error) -> Option<RdJsonDiagnostic<'a>> {
    let message = PrintDescription(diagnostic).to_string();
    let location = diagnostic.location();
    let location = to_rdjson_location(&location);

    let suggestions = to_rdjson_suggetions(diagnostic);
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
    current_message: Option<String>,
    last_diagnostic_length: usize,
}

impl Visit for SuggestionsVisitor {
    fn record_log(&mut self, _category: LogCategory, text: &dyn Display) -> std::io::Result<()> {
        let message = {
            let mut message = MarkupBuf::default();
            let mut fmt = Formatter::new(&mut message);
            fmt.write_markup(markup!({ { text } }))?;
            markup_to_string(&message).expect("Invalid markup")
        };
        let current_diagnostic_length = self.suggestions.len();

        if self.last_diagnostic_length != current_diagnostic_length {
            let last_suggestion = self
                .suggestions
                .last_mut()
                .expect("No suggestions to append to");
            last_suggestion.text = message;
        } else if let Some(current_message) = self.current_message.as_mut() {
            current_message.push_str(&message);
        } else {
            self.current_message = Some(message);
        }

        Ok(())
    }

    fn record_frame(&mut self, location: Location<'_>) -> std::io::Result<()> {
        let range = if let (Some(span), Some(source_code)) = (location.span, location.source_code) {
            let source = SourceFile::new(source_code);
            let start = source.location(span.start()).expect("Invalid span");
            let end = source.location(span.end()).expect("Invalid span");

            RdJsonRange {
                end: RdJsonLineColumn {
                    line: end.line_number.get(),
                    column: end.column_number.get(),
                },
                start: RdJsonLineColumn {
                    line: start.line_number.get(),
                    column: start.column_number.get(),
                },
            }
        } else {
            RdJsonRange::default()
        };

        self.last_diagnostic_length = self.suggestions.len();
        self.suggestions.push(RdJsonSuggestion {
            text: self.current_message.take().unwrap_or_default(),
            range,
        });

        Ok(())
    }
}

fn to_rdjson_suggetions(diagnostic: &Error) -> Vec<RdJsonSuggestion> {
    let mut visitor = SuggestionsVisitor {
        suggestions: vec![],
        last_diagnostic_length: 0,
        current_message: None,
    };

    diagnostic.advices(&mut visitor).unwrap();

    visitor.suggestions
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
