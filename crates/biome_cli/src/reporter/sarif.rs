use crate::reporter::{Reporter, ReporterVisitor, ReporterWriter};
use crate::runner::execution::Execution;
use crate::{DiagnosticsPayload, TraversalSummary};
use biome_analyze::{GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup};
use biome_console::markup;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{Error, Location, PrintDescription, Severity, display::SourceFile};
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_syntax::HtmlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_rowan::Language;
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
use std::collections::{BTreeMap, HashSet};

pub(crate) struct SarifReporter<'a> {
    pub(crate) diagnostics_payload: &'a DiagnosticsPayload,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) verbose: bool,
    pub(crate) working_directory: Option<Utf8PathBuf>,
}

impl Reporter for SarifReporter<'_> {
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

pub(crate) struct SarifReporterVisitor<'a> {
    rule_descriptions: BTreeMap<&'static str, &'a str>,
}

impl<'a> SarifReporterVisitor<'a> {
    pub fn new() -> Self {
        let mut visitor = Self {
            rule_descriptions: BTreeMap::new(),
        };

        biome_graphql_analyze::visit_registry(&mut visitor);
        biome_html_analyze::visit_registry(&mut visitor);
        biome_css_analyze::visit_registry(&mut visitor);
        biome_json_analyze::visit_registry(&mut visitor);
        biome_js_analyze::visit_registry(&mut visitor);

        visitor
    }

    fn store_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let category = <R::Group as RuleGroup>::Category::CATEGORY;
        if matches!(
            category,
            RuleCategory::Syntax | RuleCategory::Lint | RuleCategory::Action
        ) {
            let first_line: &'static str =
                R::METADATA.docs.lines().next().unwrap_or_default().trim();
            self.rule_descriptions.insert(R::METADATA.name, first_line);
        }
    }
}

impl RegistryVisitor<JsLanguage> for SarifReporterVisitor<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.store_rule::<R, JsLanguage>();
    }
}

impl RegistryVisitor<JsonLanguage> for SarifReporterVisitor<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, JsonLanguage>();
    }
}

impl RegistryVisitor<CssLanguage> for SarifReporterVisitor<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, CssLanguage>();
    }
}

impl RegistryVisitor<GraphqlLanguage> for SarifReporterVisitor<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, GraphqlLanguage>();
    }
}

impl RegistryVisitor<HtmlLanguage> for SarifReporterVisitor<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.store_rule::<R, HtmlLanguage>();
    }
}

impl ReporterVisitor for SarifReporterVisitor<'_> {
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
                            if let Some(driver_rule) =
                                to_sarif_driver_rule(diagnostic, &self.rule_descriptions)
                            {
                                sarif_rules.insert(driver_rule);
                            }
                            to_sarif_result(diagnostic, working_directory)
                        } else {
                            None
                        }
                    } else {
                        if let Some(driver_rule) =
                            to_sarif_driver_rule(diagnostic, &self.rule_descriptions)
                        {
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
                            // Make sure to maintain same order every run
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

        writer.log(markup! {
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
    let location = to_sarif_result_location(location, working_directory)?;

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

fn to_sarif_driver_rule<'a>(
    diagnostic: &'a Error,
    rule_descriptions: &BTreeMap<&'static str, &'a str>,
) -> Option<SarifDriverRule<'a>> {
    let category = diagnostic.category()?;

    let name = category.name();
    let link = if name == "format" {
        "https://biomejs.dev/formatter/"
    } else {
        category.link().unwrap_or_default()
    };

    let description: &'a str = if name == "format" {
        "Follow a consistent style—handling things like spacing, indentation, line breaks, and punctuation to make code easier to read and maintain."
    } else if let Some(description) =
        rule_descriptions.get(name.split('/').next_back().unwrap_or_default())
    {
        description
    } else {
        ""
    };

    Some(SarifDriverRule {
        id: name,
        short_description: SarifDriverRuleDescription { text: description },
        help_uri: link,
    })
}

fn to_sarif_result_location(
    location: Location,
    working_directory: Option<&Utf8Path>,
) -> Option<SarifResultLocation> {
    let artifact_location =
        to_sarif_result_location_artifact_location(location, working_directory)?;
    let region = to_sarif_result_location_region(location);

    Some(SarifResultLocation {
        physical_location: SarifResultLocationPhysicalLocation {
            artifact_location,
            region,
        },
    })
}

fn to_sarif_result_location_artifact_location(
    location: Location,
    working_directory: Option<&Utf8Path>,
) -> Option<SarifResultLocationPhysicalLocationArtifactLocation> {
    let resource = location.resource?;
    let file = resource.as_file()?;
    let absolute_path = working_directory
        .as_ref()
        .map(|wd| wd.join(file))
        .unwrap_or(file.into());

    Some(SarifResultLocationPhysicalLocationArtifactLocation {
        // Transform Windows' backslash paths to UNIX's forward slash (SARIF spec follows RFC 3986 for consistency)
        uri: absolute_path.as_str().replace('\\', "/"),
    })
}

fn to_sarif_result_location_region(
    location: Location,
) -> Option<SarifResultLocationPhysicalLocationRegion> {
    let source_code = location.source_code?;
    let span = location.span?;

    let source = SourceFile::new(source_code);
    let start = source.location(span.start()).ok()?;
    let end = source.location(span.end()).ok()?;

    Some(SarifResultLocationPhysicalLocationRegion {
        start_line: start.line_number.get(),
        start_column: start.column_number.get(),
        end_line: end.line_number.get(),
        end_column: end.column_number.get(),
    })
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
#[serde(rename_all = "camelCase")]
struct SarifDriver<'a> {
    name: &'static str,
    information_uri: &'static str,
    rules: Vec<SarifDriverRule<'a>>,
}

#[derive(Serialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
struct SarifDriverRule<'a> {
    id: &'static str,
    short_description: SarifDriverRuleDescription<'a>,
    help_uri: &'a str,
}

#[derive(Serialize, Eq, PartialEq, Hash)]
struct SarifDriverRuleDescription<'a> {
    text: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResult<'a> {
    rule_id: &'a str,
    level: &'a str,
    message: SarifResultMessage,
    locations: Vec<SarifResultLocation>,
}

#[derive(Serialize)]
struct SarifResultMessage {
    text: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResultLocation {
    physical_location: SarifResultLocationPhysicalLocation,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResultLocationPhysicalLocation {
    artifact_location: SarifResultLocationPhysicalLocationArtifactLocation,
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<SarifResultLocationPhysicalLocationRegion>,
}

#[derive(Serialize)]
struct SarifResultLocationPhysicalLocationArtifactLocation {
    uri: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SarifResultLocationPhysicalLocationRegion {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
}
