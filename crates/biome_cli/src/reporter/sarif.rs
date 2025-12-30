use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::{
    Category, Error, Location, PrintDescription, Severity, display::SourceFile,
};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;

pub(crate) struct SarifReporter {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for SarifReporter {
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

pub(crate) struct SarifReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for SarifReporterVisitor<'_> {
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
        let sarif_rules: Vec<_> = payload
            .diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.category().map(category_to_sarif))
            .collect();

        let sarif_results: Vec<_> = payload
            .diagnostics
            .iter()
            .filter_map(|diagnostic| {
                if diagnostic.severity() >= payload.diagnostic_level {
                    if diagnostic.tags().is_verbose() {
                        if verbose {
                            diagnostic_to_sarif(diagnostic)
                        } else {
                            None
                        }
                    } else {
                        diagnostic_to_sarif(diagnostic)
                    }
                } else {
                    None
                }
            })
            .collect();

        let report = SarifReport {
            schema: "https://json.schemastore.org/sarif-2.1.0.json",
            version: "2.1.0",
            runs: Vec::from([SarifRun {
                tool: SarifTool {
                    driver: SarifDriver {
                        name: "Biome",
                        information_uri: "https://biomejs.dev",
                        rules: sarif_rules,
                    },
                },
                results: sarif_results,
            }]),
        };

        let result = serde_json::to_string_pretty(&report)?;

        self.0.log(markup! {
            {result}
        });

        Ok(())
    }
}

fn diagnostic_to_sarif<'a>(diagnostic: &'a Error) -> Option<SarifResult<'a>> {
    let message = SarifResultMessage {
        text: PrintDescription(diagnostic).to_string(),
    };
    let location = diagnostic.location();
    let location = to_sarif_result_location(location);

    Some(SarifResult {
        rule_id: diagnostic
            .category()
            .map(|category| category.name())
            .unwrap_or_default(),
        level: match diagnostic.severity() {
            Severity::Hint => "note",
            Severity::Information => "note",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "note",
        },
        message,
        locations: Vec::from([location]),
    })
}

fn category_to_sarif(category: &Category) -> SarifDriverRule {
    let name = category.name();
    let link = category.link().unwrap_or_default();

    SarifDriverRule {
        id: name,
        short_description: SarifDriverRuleDescription { text: link },
        full_description: SarifDriverRuleDescription { text: link },
        help: SarifDriverRuleDescription { text: link },
    }
}

fn to_sarif_result_location(location: Location) -> SarifResultLocation {
    if let (Some(span), Some(source_code), Some(resource)) =
        (location.span, location.source_code, location.resource)
    {
        let source = SourceFile::new(source_code);
        let start = source.location(span.start()).expect("Invalid span");
        let end = source.location(span.end()).expect("Invalid span");

        SarifResultLocation {
            physical_location: SarifResultLocationPhysicalLocation {
                artifact_location: SarifResultLocationPhysicalLocationArtifactLocation {
                    uri: if let Some(uri) = resource.as_file() {
                        uri.to_string()
                    } else {
                        String::new()
                    },
                },
                region: SarifResultLocationPhysicalLocationRegion {
                    start_line: start.line_number.get(),
                    start_column: start.column_number.get(),
                    end_line: end.line_number.get(),
                    end_column: end.column_number.get(),
                },
            },
        }
    } else {
        SarifResultLocation::default()
    }
}

#[derive(Serialize)]
pub struct SarifReport<'a> {
    #[serde(rename = "$schema")]
    schema: &'static str,
    version: &'static str,
    runs: Vec<SarifRun<'a>>,
}

#[derive(Serialize)]
struct SarifRun<'a> {
    tool: SarifTool,
    results: Vec<SarifResult<'a>>,
}

#[derive(Serialize)]
struct SarifTool {
    driver: SarifDriver,
}

#[derive(Serialize)]
struct SarifDriver {
    name: &'static str,
    #[serde(rename = "informationUri")]
    information_uri: &'static str,
    rules: Vec<SarifDriverRule>,
}

#[derive(Serialize)]
struct SarifDriverRule {
    id: &'static str,
    #[serde(rename = "shortDescription")]
    short_description: SarifDriverRuleDescription,
    #[serde(rename = "fullDescription")]
    full_description: SarifDriverRuleDescription,
    help: SarifDriverRuleDescription,
}

#[derive(Serialize)]
struct SarifDriverRuleDescription {
    text: &'static str,
}

#[derive(Serialize)]
struct SarifResult<'a> {
    #[serde(rename = "ruleId")]
    rule_id: &'a str,
    level: &'a str,
    message: SarifResultMessage,
    locations: Vec<SarifResultLocation>,
}

#[derive(Serialize)]
struct SarifResultMessage {
    text: String,
}

#[derive(Default, Serialize)]
struct SarifResultLocation {
    #[serde(rename = "physicalLocation")]
    physical_location: SarifResultLocationPhysicalLocation,
}

#[derive(Default, Serialize)]
struct SarifResultLocationPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    artifact_location: SarifResultLocationPhysicalLocationArtifactLocation,
    region: SarifResultLocationPhysicalLocationRegion,
}

#[derive(Default, Serialize)]
struct SarifResultLocationPhysicalLocationArtifactLocation {
    uri: String,
}

#[derive(Default, Serialize)]
struct SarifResultLocationPhysicalLocationRegion {
    #[serde(rename = "startLine")]
    start_line: usize,
    #[serde(rename = "startColumn")]
    start_column: usize,
    #[serde(rename = "endLine")]
    end_line: usize,
    #[serde(rename = "endColumn")]
    end_column: usize,
}
