use crate::reporter::terminal::ConsoleTraversalSummary;
use crate::reporter::{EvaluatedPathsDiagnostic, FixedPathsDiagnostic};
use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Console, ConsoleExt, markup};
use biome_diagnostics::advice::ListAdvice;
use biome_diagnostics::{
    Advices, Category, Diagnostic, MessageAndDescription, PrintDiagnostic, Resource, Severity,
    Visit, category,
};
use biome_fs::BiomePath;
use camino::Utf8PathBuf;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::io;

pub(crate) struct SummaryReporter {
    pub(crate) summary: TraversalSummary,
    pub(crate) diagnostics_payload: DiagnosticsPayload,
    pub(crate) execution: Execution,
    pub(crate) evaluated_paths: BTreeSet<BiomePath>,
    pub(crate) working_directory: Option<Utf8PathBuf>,
    pub(crate) verbose: bool,
    pub(crate) minimal: bool,
}

impl Reporter for SummaryReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(
            &self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.minimal,
        )?;
        if self.verbose {
            visitor.report_handled_paths(self.evaluated_paths, self.working_directory)?;
        }
        visitor.report_summary(&self.execution, self.summary, self.verbose, self.minimal)?;
        Ok(())
    }
}

pub(crate) struct SummaryReporterVisitor<'a>(pub(crate) &'a mut dyn Console);

impl ReporterVisitor for SummaryReporterVisitor<'_> {
    fn report_summary(
        &mut self,
        execution: &Execution,
        summary: TraversalSummary,
        verbose: bool,
        _minimal: bool,
    ) -> io::Result<()> {
        if execution.is_check() && summary.suggested_fixes_skipped > 0 {
            self.0.log(markup! {
                <Warn>"Skipped "{summary.suggested_fixes_skipped}" suggested fixes.\n"</Warn>
                <Info>"If you wish to apply the suggested (unsafe) fixes, use the command "<Emphasis>"biome check --write --unsafe\n"</Emphasis></Info>
            })
        }

        if !execution.is_ci() && summary.diagnostics_not_printed > 0 {
            self.0.log(markup! {
                <Warn>"The number of diagnostics exceeds the limit allowed. Use "<Emphasis>"--max-diagnostics"</Emphasis>" to increase it.\n"</Warn>
                <Info>"Diagnostics not shown: "</Info><Emphasis>{summary.diagnostics_not_printed}</Emphasis><Info>"."</Info>
            })
        }

        self.0.log(markup! {
            {ConsoleTraversalSummary(execution.traversal_mode(), &summary, verbose)}
        });

        Ok(())
    }

    fn report_diagnostics(
        &mut self,
        execution: &Execution,
        diagnostics_payload: DiagnosticsPayload,
        verbose: bool,
        _minimal: bool,
    ) -> io::Result<()> {
        let mut files_to_diagnostics = FileToDiagnostics::default();

        let iter = diagnostics_payload.diagnostics.iter().rev();
        for diagnostic in iter {
            let location = diagnostic.location().resource.and_then(|r| match r {
                Resource::File(p) => Some(p),
                _ => None,
            });
            let Some(location) = location else {
                continue;
            };

            let category = diagnostic.category();
            let severity = &diagnostic.severity();

            if diagnostic.severity() >= diagnostics_payload.diagnostic_level {
                if diagnostic.tags().is_verbose() {
                    if verbose {
                        if execution.is_check() || execution.is_lint() {
                            if let Some(category) = category {
                                if category.name().starts_with("lint/") {
                                    files_to_diagnostics.insert_rule(category.name(), severity);
                                }
                            }
                        }
                    } else {
                        continue;
                    }
                }

                if let Some(category) = category {
                    if category.name() == "parse" {
                        files_to_diagnostics.insert_parse(location);
                    }
                }

                if execution.is_check() || execution.is_lint() || execution.is_ci() {
                    if let Some(category) = category {
                        if category.name().starts_with("lint/")
                            || category.name().starts_with("suppressions/")
                            || category.name().starts_with("assist/")
                        {
                            files_to_diagnostics.insert_rule(category.name(), severity);
                        }
                    }
                }

                if execution.is_check() || execution.is_format() || execution.is_ci() {
                    if let Some(category) = category {
                        if category.name() == "format" {
                            files_to_diagnostics.insert_format(location);
                        }
                    }
                }
            }
        }

        self.0.log(markup! {{files_to_diagnostics}});

        Ok(())
    }

    fn report_handled_paths(
        &mut self,
        evaluated_paths: BTreeSet<BiomePath>,
        working_directory: Option<Utf8PathBuf>,
    ) -> io::Result<()> {
        let evaluated_paths_diagnostic = EvaluatedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths
                    .iter()
                    .map(|p| {
                        working_directory
                            .as_ref()
                            .and_then(|wd| {
                                p.strip_prefix(wd.as_str())
                                    .map(|path| path.to_string())
                                    .ok()
                            })
                            .unwrap_or(p.to_string())
                    })
                    .collect(),
            },
        };

        let fixed_paths_diagnostic = FixedPathsDiagnostic {
            advice: ListAdvice {
                list: evaluated_paths
                    .iter()
                    .filter(|p| p.was_written())
                    .map(|p| {
                        working_directory
                            .as_ref()
                            .and_then(|wd| {
                                p.strip_prefix(wd.as_str())
                                    .map(|path| path.to_string())
                                    .ok()
                            })
                            .unwrap_or(p.to_string())
                    })
                    .collect(),
            },
        };

        self.0.log(markup! {
            {PrintDiagnostic::verbose(&evaluated_paths_diagnostic)}
        });
        self.0.log(markup! {
            {PrintDiagnostic::verbose(&fixed_paths_diagnostic)}
        });

        Ok(())
    }
}

#[derive(Debug, Default)]
struct FileToDiagnostics {
    formats: BTreeSet<String>,
    rules: RulesByCategory,
    parse: BTreeSet<String>,
}

impl FileToDiagnostics {
    fn insert_rule(&mut self, rule_name: impl Into<RuleName>, severity: &Severity) {
        let rule_name = rule_name.into();
        self.rules.insert(rule_name, severity);
    }

    fn insert_format(&mut self, location: &str) {
        self.formats.insert(location.into());
    }

    fn insert_parse(&mut self, location: &str) {
        self.parse.insert(location.into());
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    severity = Information
)]
struct SummaryListDiagnostic<'a> {
    #[category]
    category: &'static Category,

    #[message]
    message: MessageAndDescription,

    #[advice]
    list: SummaryListAdvice<'a>,
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    severity = Information,
    category = "reporter/violations",
    message = "Some lint rules or assist actions reported some violations."
)]
struct LintSummaryDiagnostic<'a> {
    #[advice]
    tables: &'a RulesByCategory,
}

#[derive(Debug)]
struct SummaryListAdvice<'a>(&'a BTreeSet<String>);

impl Advices for SummaryListAdvice<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let list: Vec<_> = self.0.iter().map(|s| s as &dyn Display).collect();
        visitor.record_list(&list)
    }
}

impl Display for FileToDiagnostics {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if !self.parse.is_empty() {
            let diagnostic = SummaryListDiagnostic {
                message: MessageAndDescription::from(
                    markup! {
                        <Warn>"The following files have parsing errors."</Warn>
                    }
                    .to_owned(),
                ),
                list: SummaryListAdvice(&self.parse),
                category: category!("reporter/parse"),
            };
            fmt.write_markup(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            })?;
        }

        if !self.formats.is_empty() {
            let diagnostic = SummaryListDiagnostic {
                message: MessageAndDescription::from(
                    markup! {
                        <Warn>"The following files needs to be formatted."</Warn>
                    }
                    .to_owned(),
                ),
                list: SummaryListAdvice(&self.formats),
                category: category!("reporter/format"),
            };
            fmt.write_markup(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            })?;
        }

        if !self.rules.0.is_empty() {
            let diagnostic = LintSummaryDiagnostic {
                tables: &self.rules,
            };
            fmt.write_markup(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            })?;
        }
        Ok(())
    }
}

#[derive(Debug, Default)]
struct RulesByCategory(BTreeMap<RuleName, DiagnosticsBySeverity>);

impl RulesByCategory {
    fn insert(&mut self, rule: RuleName, severity: &Severity) {
        if let Some(value) = self.0.get_mut(&rule) {
            value.track_severity(severity);
        } else {
            let mut diagnostics_by_severity = DiagnosticsBySeverity::default();
            diagnostics_by_severity.track_severity(severity);
            self.0.insert(rule, diagnostics_by_severity);
        }
    }
}

impl Advices for &RulesByCategory {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let headers = &[
            markup!("Rule Name").to_owned(),
            markup!("Diagnostics").to_owned(),
        ];
        let (first, second): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .rev()
            .map(|(rule_name, diagnostic)| {
                (
                    markup! {{rule_name}}.to_owned(),
                    markup! {{diagnostic}}.to_owned(),
                )
            })
            .unzip();
        let array = [first.as_slice(), second.as_slice()];
        visitor.record_table(15usize, headers, &array)
    }
}

#[derive(Debug, Default)]
struct RuleName(&'static str);

impl AsRef<str> for RuleName {
    fn as_ref(&self) -> &'static str {
        self.0
    }
}

impl From<&'static str> for RuleName {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl Eq for RuleName {}

impl PartialEq<Self> for RuleName {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd<Self> for RuleName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RuleName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.len().cmp(&other.0.len())
    }
}
impl Display for RuleName {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        fmt.write_markup(markup!(
            <Emphasis>{self.0}</Emphasis>
        ))
    }
}

#[derive(Debug, Default)]
struct DiagnosticsBySeverity {
    errors: usize,
    warnings: usize,
    info: usize,
}

impl DiagnosticsBySeverity {
    fn track_severity(&mut self, severity: &Severity) {
        match severity {
            Severity::Information => self.info += 1,
            Severity::Warning => {
                self.warnings += 1;
            }
            Severity::Error => {
                self.errors += 1;
            }
            // not used for now inside the linter
            Severity::Hint | Severity::Fatal => {}
        }
    }
}

impl Display for DiagnosticsBySeverity {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let total = self.warnings + self.info + self.errors;
        fmt.write_str(&format!("{total}"))?;
        fmt.write_str(" ")?;
        fmt.write_str("(")?;
        fmt.write_markup(markup! {
            <Error>{self.errors}" error(s), "</Error>
        })?;
        fmt.write_markup(markup! {
            <Warn>{self.warnings}" warning(s), "</Warn>
        })?;
        fmt.write_markup(markup! {
            <Info>{self.info}" info(s)"</Info>
        })?;
        fmt.write_str(")")?;

        Ok(())
    }
}
