use crate::reporter::terminal::ConsoleTraversalSummary;
use crate::reporter::{EvaluatedPathsDiagnostic, FixedPathsDiagnostic};
use crate::{DiagnosticsPayload, Execution, Reporter, ReporterVisitor, TraversalSummary};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Console, ConsoleExt, MarkupBuf, markup};
use biome_diagnostics::advice::ListAdvice;
use biome_diagnostics::{
    Advices, Category, Diagnostic, LogCategory, PrintDiagnostic, Resource, Severity, Visit,
    category,
};
use biome_fs::BiomePath;
use camino::{Utf8Path, Utf8PathBuf};
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
}

impl Reporter for SummaryReporter {
    fn write(self, visitor: &mut dyn ReporterVisitor) -> io::Result<()> {
        visitor.report_diagnostics(
            &self.execution,
            self.diagnostics_payload,
            self.verbose,
            self.working_directory.as_deref(),
        )?;
        if self.verbose {
            visitor
                .report_handled_paths(self.evaluated_paths, self.working_directory.as_deref())?;
        }
        visitor.report_summary(&self.execution, self.summary, self.verbose)?;
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
        working_directory: Option<&Utf8Path>,
    ) -> io::Result<()> {
        let mut files_to_diagnostics =
            FileToDiagnostics::default().with_working_directory(working_directory);

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
                        if (execution.is_check() || execution.is_lint())
                            && let Some(category) = category
                            && category.name().starts_with("lint/")
                        {
                            files_to_diagnostics.insert_rule_for_file(
                                category.name(),
                                severity,
                                location,
                            );
                        }
                    } else {
                        continue;
                    }
                }

                if let Some(category) = category
                    && category.name() == "parse"
                {
                    files_to_diagnostics.insert_parse(location);
                }

                if (execution.is_check() || execution.is_lint() || execution.is_ci())
                    && let Some(category) = category
                    && (category.name().starts_with("lint/")
                        || category.name().starts_with("suppressions/")
                        || category.name().starts_with("assist/")
                        || category.name().starts_with("plugin"))
                {
                    files_to_diagnostics.insert_rule_for_file(category.name(), severity, location);
                }

                if (execution.is_check() || execution.is_format() || execution.is_ci())
                    && let Some(category) = category
                    && category.name() == "format"
                {
                    files_to_diagnostics.insert_format(location);
                }
            }
        }

        self.0.log(markup! {{files_to_diagnostics}});

        Ok(())
    }

    fn report_handled_paths(
        &mut self,
        evaluated_paths: BTreeSet<BiomePath>,
        working_directory: Option<&Utf8Path>,
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
struct FileToDiagnostics<'a> {
    formats: BTreeSet<String>,
    rules: RulesByCategory,
    parse: BTreeSet<String>,
    violation_file_counts: BTreeMap<&'a str, DiagnosticsBySeverity>,
    working_directory: Option<&'a Utf8Path>,
}

impl<'a> FileToDiagnostics<'a> {
    fn with_working_directory(mut self, working_directory: Option<&'a Utf8Path>) -> Self {
        self.working_directory = working_directory;
        self
    }

    fn insert_rule_for_file(
        &mut self,
        rule_name: impl Into<RuleName>,
        severity: &Severity,
        location: &'a str,
    ) {
        let rule_name = rule_name.into();
        self.rules.insert(rule_name, severity);

        // Track file-specific diagnostics
        let entry = self.violation_file_counts.entry(location).or_default();
        entry.track_severity(severity);
    }

    fn insert_format(&mut self, location: &'a str) {
        self.formats.insert(location.into());
    }

    fn insert_parse(&mut self, location: &'a str) {
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
    message: MarkupBuf,

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
    file_counts: FileDiagnosticCounts<'a>,

    #[advice]
    tables: &'a RulesByCategory,
}

#[derive(Debug)]
struct SummaryListAdvice<'a>(&'a [MarkupBuf]);

impl Advices for SummaryListAdvice<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let list: Vec<_> = self.0.iter().map(|s| s as &dyn Display).collect();
        visitor.record_list(&list)
    }
}

#[derive(Debug)]
struct FileDiagnosticCounts<'a>(
    &'a BTreeMap<&'a str, DiagnosticsBySeverity>,
    Option<&'a Utf8Path>,
);

impl Display for FileDiagnosticCounts<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if !self.0.is_empty() {
            fmt.write_markup(markup! { "\n" })?;
        }
        Ok(())
    }
}

impl Advices for FileDiagnosticCounts<'_> {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        if !self.0.is_empty() {
            visitor.record_log(LogCategory::Info, &"The following files have violations:")?;
            let mut list = Vec::new();
            for (file, counts) in self.0 {
                let absolute_path = self.1.map(|wd| wd.join(file)).map_or_else(
                    || (*file).to_string(),
                    |file| format!("file://{}", file.as_str()),
                );

                let count = CounterLine(counts.errors, counts.warnings, counts.info);

                list.push(
                    markup! {
                        <Hyperlink href={absolute_path.as_str()}>{*file}</Hyperlink>" ("{count}")"
                    }
                    .to_owned(),
                );
            }
            let list: Vec<_> = list.iter().map(|s| s as &dyn Display).collect();
            visitor.record_list(list.as_slice())
        } else {
            Ok(())
        }
    }
}

impl<'a> Display for FileToDiagnostics<'a> {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        if !self.parse.is_empty() {
            let parse_files_with_counts: Vec<_> = self
                .parse
                .iter()
                .map(|file| {
                    let absolute_path = self
                        .working_directory
                        .as_ref()
                        .map(|wd| wd.join(file))
                        .unwrap_or(file.into());
                    let absolute_path = format!("file://{}", absolute_path.as_str());
                    markup! {
                        <Hyperlink href={absolute_path}>{file}</Hyperlink>
                    }
                    .to_owned()
                })
                .collect();

            let diagnostic = SummaryListDiagnostic {
                message: markup! {
                    <Info>"The following files have parsing errors:"</Info>
                }
                .to_owned(),

                list: SummaryListAdvice(&parse_files_with_counts),
                category: category!("reporter/parse"),
            };
            fmt.write_markup(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            })?;
        }

        if !self.formats.is_empty() {
            let format_files_with_counts: Vec<_> = self
                .formats
                .iter()
                .map(|file| {
                    let absolute_path = self
                        .working_directory
                        .as_ref()
                        .map(|wd| wd.join(file))
                        .unwrap_or(file.into());
                    let absolute_path = format!("file://{}", absolute_path.as_str());
                    markup! {
                        <Hyperlink href={absolute_path}>{file}</Hyperlink>
                    }
                    .to_owned()
                })
                .collect();

            let diagnostic = SummaryListDiagnostic {
                message: markup! {
                    <Info>"The following files need to be formatted:"</Info>
                }
                .to_owned(),

                list: SummaryListAdvice(&format_files_with_counts),
                category: category!("reporter/format"),
            };
            fmt.write_markup(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            })?;
        }

        if !self.rules.0.is_empty() {
            let diagnostic = LintSummaryDiagnostic {
                file_counts: FileDiagnosticCounts(
                    &self.violation_file_counts,
                    self.working_directory,
                ),
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
        visitor.record_log(
            LogCategory::Info,
            &"The following lint rules have violations:",
        )?;
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
        let count = CounterLine(self.errors, self.warnings, self.info);
        fmt.write_str(&format!("{total}"))?;
        fmt.write_str(" ")?;
        fmt.write_markup(markup! {
            "("{count}")"
        })?;
        Ok(())
    }
}

struct CounterLine(usize, usize, usize);

impl Display for CounterLine {
    fn fmt(&self, fmt: &mut Formatter) -> io::Result<()> {
        let errors = self.0;
        let warnings = self.1;
        let info = self.2;

        if errors > 0 {
            fmt.write_markup(markup! {
                <Error>{errors} " " {if errors == 1 { "error" } else { "errors" }}</Error>
            })?;
        }

        if warnings > 0 {
            if errors > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_markup(markup! {
                <Warn>{warnings} " " {if warnings == 1 { "warning" } else { "warnings" }}</Warn>
            })?;
        }

        if info > 0 {
            if errors > 0 || warnings > 0 {
                fmt.write_str(", ")?;
            }
            fmt.write_markup(markup! {
                <Info>{info} " " {if info == 1 { "info" } else { "infos" }}</Info>
            })?;
        }

        Ok(())
    }
}
