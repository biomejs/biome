use crate::reporter::{Reporter, ReporterVisitor};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{MarkupBuf, markup};
use biome_diagnostics::display::{SourceFile, markup_to_string};
use biome_diagnostics::{
    Category, Diagnostic, Error, Location, LogCategory, PrintDescription, Severity, Visit,
};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JsonReporterVisitor {
    summary: TraversalSummary,
    diagnostics: Vec<JsonReport>,
    command: String,
}

impl JsonReporterVisitor {
    pub(crate) fn new(summary: TraversalSummary) -> Self {
        Self {
            summary,
            diagnostics: vec![],
            command: String::new(),
        }
    }
}

impl biome_console::fmt::Display for JsonReporterVisitor {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        let content = serde_json::to_string(&self)?;
        fmt.write_str(content.as_str())
    }
}

pub struct JsonReporter<'a> {
    pub execution: &'a dyn Execution,
    pub diagnostics_payload: &'a DiagnosticsPayload,
    pub summary: TraversalSummary,
    pub verbose: bool,
    pub working_directory: Option<Utf8PathBuf>,
}

impl Reporter for JsonReporter<'_> {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_summary(self.execution, self.summary, self.verbose)?;
        visitor.report_diagnostics(
            self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;

        Ok(())
    }
}

impl ReporterVisitor for JsonReporterVisitor {
    fn report_summary(
        &mut self,
        execution: &dyn Execution,
        summary: TraversalSummary,
        _verbose: bool,
    ) -> std::io::Result<()> {
        self.summary = summary;
        self.command = execution.as_diagnostic_category().name().to_string();

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &dyn Execution,
        payload: &DiagnosticsPayload,
        verbose: bool,
        _working_directory: Option<&Utf8Path>,
    ) -> std::io::Result<()> {
        for diagnostic in &payload.diagnostics {
            if diagnostic.severity() >= payload.diagnostic_level {
                if diagnostic.tags().is_verbose() {
                    if verbose {
                        self.diagnostics.push(to_json_report(diagnostic))
                    }
                } else {
                    self.diagnostics.push(to_json_report(diagnostic))
                }
            }
        }
        Ok(())
    }
}

fn to_json_report(diagnostic: &biome_diagnostics::Error) -> JsonReport {
    let category = diagnostic.category();
    let severity = diagnostic.severity();
    let message = PrintDescription(diagnostic).to_string();
    let location = diagnostic.location();
    let location = to_location(&location).or_else(|| {
        let location = location.resource?;
        Some(LocationReport {
            path: location.to_string(),
            start: LocationSpan { column: 0, line: 0 },
            end: LocationSpan { column: 0, line: 0 },
        })
    });
    let advices = to_advices(diagnostic);

    JsonReport {
        category,
        message,
        severity,
        location,
        advices,
    }
}

fn to_advices(diagnostic: &Error) -> Vec<JsonSuggestion> {
    let mut visitor = SuggestionsVisitor {
        suggestions: vec![],
        last_diagnostic_length: 0,
        current_message: None,
    };
    diagnostic.advices(&mut visitor).unwrap();

    visitor.suggestions
}

fn to_location(location: &Location) -> Option<LocationReport> {
    let (Some(span), Some(source_code), Some(resource)) =
        (location.span, location.source_code, location.resource)
    else {
        return None;
    };
    let resource = resource.as_file()?;
    let source = SourceFile::new(source_code);
    let start = source.location(span.start()).ok()?;
    let end = source.location(span.end()).ok()?;
    Some(LocationReport {
        path: resource.to_string(),
        start: LocationSpan {
            column: start.column_number.get(),
            line: start.line_number.get(),
        },

        end: LocationSpan {
            column: end.column_number.get(),
            line: end.line_number.get(),
        },
    })
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonReport {
    category: Option<&'static Category>,
    severity: Severity,
    message: String,
    advices: Vec<JsonSuggestion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<LocationReport>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocationReport {
    path: String,
    start: LocationSpan,
    end: LocationSpan,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LocationSpan {
    column: usize,
    line: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonSuggestion {
    start: LocationSpan,
    end: LocationSpan,
    text: String,
}

struct SuggestionsVisitor {
    suggestions: Vec<JsonSuggestion>,
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
        if let (Some(span), Some(source_code)) = (location.span, location.source_code) {
            let source = SourceFile::new(source_code);
            let start = source.location(span.start()).expect("Invalid span");
            let end = source.location(span.end()).expect("Invalid span");

            self.suggestions.push(JsonSuggestion {
                end: LocationSpan {
                    line: end.line_number.get(),
                    column: end.column_number.get(),
                },
                start: LocationSpan {
                    line: start.line_number.get(),
                    column: start.column_number.get(),
                },
                text: self.current_message.take().unwrap_or_default(),
            })
        }

        Ok(())
    }
}
