use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{MarkupBuf, markup};
use biome_diagnostics::display::{SourceFile, markup_to_string};
use biome_diagnostics::{
    Category, Error, Location, LogCategory, PrintDescription, Severity, Visit,
};
use biome_json_factory::make::*;
use biome_json_syntax::{AnyJsonMemberName, AnyJsonValue, JsonRoot, JsonSyntaxKind, T};
use camino::{Utf8Path, Utf8PathBuf};

#[derive(Debug, Default)]
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

    pub(crate) fn to_json(&self) -> JsonRoot {
        let diagnostics_elements: Vec<AnyJsonValue> =
            self.diagnostics.iter().map(report_to_json).collect();

        let diagnostics_separators =
            vec![token(T![,]); diagnostics_elements.len().saturating_sub(1)];

        let root_members = vec![
            self.summary.json_member(),
            json_member(
                AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal(
                    "diagnostics",
                ))),
                token(T![:]),
                AnyJsonValue::JsonArrayValue(json_array_value(
                    token(T!['[']),
                    json_array_element_list(diagnostics_elements, diagnostics_separators),
                    token(T![']']),
                )),
            ),
            json_member(
                AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("command"))),
                token(T![:]),
                AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(
                    &self.command,
                ))),
            ),
        ];

        let root_separators = vec![token(T![,]); root_members.len() - 1];

        json_root(
            AnyJsonValue::JsonObjectValue(json_object_value(
                token(T!['{']),
                json_member_list(root_members, root_separators),
                token(T!['}']),
            )),
            token(JsonSyntaxKind::EOF),
        )
        .build()
    }
}

fn location_span_to_json(span: &LocationSpan) -> AnyJsonValue {
    let members = vec![
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("line"))),
            token(T![:]),
            AnyJsonValue::JsonNumberValue(json_number_value(json_number_literal(span.line))),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("column"))),
            token(T![:]),
            AnyJsonValue::JsonNumberValue(json_number_value(json_number_literal(span.column))),
        ),
    ];
    let separators = vec![token(T![,])];

    AnyJsonValue::JsonObjectValue(json_object_value(
        token(T!['{']),
        json_member_list(members, separators),
        token(T!['}']),
    ))
}

fn location_report_to_json(location: &LocationReport) -> AnyJsonValue {
    let members = vec![
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("path"))),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(&location.path))),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("start"))),
            token(T![:]),
            location_span_to_json(&location.start),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("end"))),
            token(T![:]),
            location_span_to_json(&location.end),
        ),
    ];
    let separators = vec![token(T![,]); members.len() - 1];

    AnyJsonValue::JsonObjectValue(json_object_value(
        token(T!['{']),
        json_member_list(members, separators),
        token(T!['}']),
    ))
}

fn suggestion_to_json(suggestion: &JsonSuggestion) -> AnyJsonValue {
    let members = vec![
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("start"))),
            token(T![:]),
            location_span_to_json(&suggestion.start),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("end"))),
            token(T![:]),
            location_span_to_json(&suggestion.end),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("text"))),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(&suggestion.text))),
        ),
    ];
    let separators = vec![token(T![,]); members.len() - 1];

    AnyJsonValue::JsonObjectValue(json_object_value(
        token(T!['{']),
        json_member_list(members, separators),
        token(T!['}']),
    ))
}

fn report_to_json(report: &JsonReport) -> AnyJsonValue {
    let severity_str = match report.severity {
        Severity::Hint => "hint",
        Severity::Information => "info",
        Severity::Warning => "warning",
        Severity::Error => "error",
        Severity::Fatal => "fatal",
    };

    let mut members = vec![
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("severity"))),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(severity_str))),
        ),
        json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("message"))),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(&report.message))),
        ),
    ];

    // Add category if present
    if let Some(category) = report.category {
        members.push(json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("category"))),
            token(T![:]),
            AnyJsonValue::JsonStringValue(json_string_value(json_string_literal(category.name()))),
        ));
    }

    // Add location if present
    if let Some(ref location) = report.location {
        members.push(json_member(
            AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("location"))),
            token(T![:]),
            location_report_to_json(location),
        ));
    }

    // Add advices array
    let advice_elements: Vec<AnyJsonValue> =
        report.advices.iter().map(suggestion_to_json).collect();
    let advice_separators = vec![token(T![,]); advice_elements.len().saturating_sub(1)];

    members.push(json_member(
        AnyJsonMemberName::JsonMemberName(json_member_name(json_string_literal("advices"))),
        token(T![:]),
        AnyJsonValue::JsonArrayValue(json_array_value(
            token(T!['[']),
            json_array_element_list(advice_elements, advice_separators),
            token(T![']']),
        )),
    ));

    let separators = vec![token(T![,]); members.len() - 1];

    AnyJsonValue::JsonObjectValue(json_object_value(
        token(T!['{']),
        json_member_list(members, separators),
        token(T!['}']),
    ))
}

pub struct JsonReporter<'a> {
    pub execution: &'a dyn Execution,
    pub diagnostics_payload: &'a DiagnosticsPayload,
    pub summary: TraversalSummary,
    pub verbose: bool,
    pub working_directory: Option<Utf8PathBuf>,
}

impl Reporter for JsonReporter<'_> {
    fn write(
        self,
        writer: &mut dyn ReporterWriter,
        visitor: &mut dyn ReporterVisitor,
    ) -> std::io::Result<()> {
        visitor.report_summary(writer, self.execution, self.summary, self.verbose)?;
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

impl ReporterVisitor for JsonReporterVisitor {
    fn report_summary(
        &mut self,
        _writer: &mut dyn ReporterWriter,
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
        _writer: &mut dyn ReporterWriter,
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
        let location = location
            .resource
            .and_then(|location| location.as_file().map(|f| f.to_string()))?;
        Some(LocationReport {
            path: location,
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

#[derive(Debug)]
struct JsonReport {
    category: Option<&'static Category>,
    severity: Severity,
    message: String,
    advices: Vec<JsonSuggestion>,
    location: Option<LocationReport>,
}

#[derive(Debug)]
struct LocationReport {
    path: String,
    start: LocationSpan,
    end: LocationSpan,
}

#[derive(Debug)]
struct LocationSpan {
    column: usize,
    line: usize,
}

#[derive(Debug)]
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
