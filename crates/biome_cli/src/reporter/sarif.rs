use std::collections::HashSet;

use crate::reporter::{Reporter, ReporterVisitor};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::{Error, Location, PrintDescription, Severity, display::SourceFile};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
pub(crate) struct SarifReporter<'a> {
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for SarifReporter<'_> {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> std::io::Result<()> {
        visitor.report_diagnostics(
            self.execution,
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
        _execution: &dyn Execution,
        _summary: TraversalSummary,
        _verbose: bool,
    ) -> std::io::Result<()> {
        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        _execution: &dyn Execution,
        payload: DiagnosticsPayload,
        verbose: bool,
        working_directory: Option<&Utf8Path>,
    ) -> std::io::Result<()> {
        let mut sarif_rules: HashSet<_> = HashSet::new();

        let sarif_results: Vec<_> = payload
            .diagnostics
            .iter()
            .filter_map(|diagnostic| {
                if diagnostic.severity() >= payload.diagnostic_level {
                    if diagnostic.tags().is_verbose() {
                        if verbose {
                            if let Some(driver_rule) = to_sarif_driver_rule(diagnostic) {
                                sarif_rules.insert(driver_rule);
                            }
                            to_sarif_result(diagnostic, working_directory)
                        } else {
                            None
                        }
                    } else {
                        if let Some(driver_rule) = to_sarif_driver_rule(diagnostic) {
                            sarif_rules.insert(driver_rule);
                        }
                        to_sarif_result(diagnostic, working_directory)
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
                        rules: {
                            // Make to sure to maintain same order every run
                            let mut sarif_rules_vec = sarif_rules.into_iter().collect::<Vec<_>>();
                            sarif_rules_vec.sort_by(|a, b| a.id.cmp(b.id));
                            sarif_rules_vec
                        },
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

fn to_sarif_result<'a>(
    diagnostic: &'a Error,
    working_directory: Option<&Utf8Path>,
) -> Option<SarifResult<'a>> {
    let category = diagnostic.category()?;

    let message = SarifResultMessage {
        text: PrintDescription(diagnostic).to_string(),
    };

    let location = diagnostic.location();
    let location = to_sarif_result_location(location, working_directory);

    Some(SarifResult {
        rule_id: category.name(),
        level: match diagnostic.severity() {
            Severity::Hint => "note",
            Severity::Information => "note",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "error",
        },
        message,
        locations: Vec::from([location]),
    })
}

fn to_sarif_driver_rule<'a>(diagnostic: &'a Error) -> Option<SarifDriverRule<'a>> {
    let category = diagnostic.category()?;

    let name = category.name();
    let link = category.link().unwrap_or_default();

    Some(SarifDriverRule {
        id: name,
        short_description: SarifDriverRuleDescription { text: "" },
        help_uri: if name == "format" {
            "https://biomejs.dev/formatter/"
        } else {
            link
        },
    })
}

fn to_sarif_result_location(
    location: Location,
    working_directory: Option<&Utf8Path>,
) -> SarifResultLocation {
    SarifResultLocation {
        physical_location: SarifResultLocationPhysicalLocation {
            artifact_location: to_sarif_result_location_artifact_location(
                location,
                working_directory,
            ),
            region: to_sarif_result_location_region(location),
        },
    }
}

fn to_sarif_result_location_artifact_location(
    location: Location,
    working_directory: Option<&Utf8Path>,
) -> SarifResultLocationPhysicalLocationArtifactLocation {
    if let Some(resource) = location.resource {
        let Some(file) = resource.as_file() else {
            return SarifResultLocationPhysicalLocationArtifactLocation::default();
        };
        let absolute_path = working_directory
            .as_ref()
            .map(|wd| wd.join(file))
            .unwrap_or(file.into());
        let absolute_path = format!("file://{}", absolute_path.as_str());
        SarifResultLocationPhysicalLocationArtifactLocation { uri: absolute_path }
    } else {
        SarifResultLocationPhysicalLocationArtifactLocation::default()
    }
}

fn to_sarif_result_location_region(
    location: Location,
) -> SarifResultLocationPhysicalLocationRegion {
    let Some(source_code) = location.source_code else {
        return SarifResultLocationPhysicalLocationRegion::default();
    };
    let Some(span) = location.span else {
        return SarifResultLocationPhysicalLocationRegion::default();
    };
    let source = SourceFile::new(source_code);
    let start = source.location(span.start()).ok();
    let end = source.location(span.end()).ok();

    SarifResultLocationPhysicalLocationRegion {
        start_line: if let Some(start) = start {
            start.line_number.get()
        } else {
            0
        },
        start_column: if let Some(start) = start {
            start.column_number.get()
        } else {
            0
        },
        end_line: if let Some(end) = end {
            end.line_number.get()
        } else {
            0
        },
        end_column: if let Some(end) = end {
            end.column_number.get()
        } else {
            0
        },
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
    tool: SarifTool<'a>,
    results: Vec<SarifResult<'a>>,
}

#[derive(Serialize)]
struct SarifTool<'a> {
    driver: SarifDriver<'a>,
}

#[derive(Serialize)]
struct SarifDriver<'a> {
    name: &'static str,
    #[serde(rename = "informationUri")]
    information_uri: &'static str,
    rules: Vec<SarifDriverRule<'a>>,
}

#[derive(Serialize, Eq, PartialEq, Hash)]
struct SarifDriverRule<'a> {
    id: &'static str,
    #[serde(rename = "shortDescription")]
    short_description: SarifDriverRuleDescription,
    #[serde(rename = "helpUri")]
    help_uri: &'a str,
}

#[derive(Serialize, Eq, PartialEq, Hash)]
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
