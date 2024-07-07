use crate::runner::{TestCaseFiles, TestRunOutcome, TestRunResult, TestSuite, TestSuiteInstance};
use crate::{Summary, TestResults};
use ascii_table::{Align, AsciiTable};
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::{DiagnosticExt, Error};
use colored::Colorize;
use indicatif::ProgressBar;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{stdout, IsTerminal, Write};
use std::str::FromStr;
use std::time::Instant;

pub(crate) trait TestReporter: Send + Sync {
    /// Called at the beginning of processing a test suite
    fn test_suite_started(&mut self, _suite: &dyn TestSuite) {}
    /// Called after all potential tests have been discovered for a test suite
    fn tests_discovered(&mut self, _suite: &dyn TestSuite, _count: usize) {}
    /// Called after loading a single test of a test suite
    fn test_loaded(&mut self) {}
    /// Called after all tests for a test suite have been loaded, right before the runner executes the tests
    fn test_suite_run_started(&mut self, _suite: &TestSuiteInstance) {}
    /// A test run completed
    fn test_completed(&mut self, _result: &TestRunResult) {}
    /// A test suite completed
    fn test_suite_completed(&mut self, _suite: &TestSuiteInstance, _result: &TestResults) {}

    /// Called when all test suites have completed
    fn run_completed(&mut self) {}
}

/// Prints a progress bar showing which tests are executed
pub(crate) struct DefaultReporter {
    pb: ProgressBar,
    start: Instant,
}

impl Default for DefaultReporter {
    fn default() -> Self {
        Self {
            pb: ProgressBar::hidden(),
            start: Instant::now(),
        }
    }
}

impl TestReporter for DefaultReporter {
    fn test_suite_started(&mut self, _suite: &dyn TestSuite) {
        self.start = Instant::now();
    }

    fn tests_discovered(&mut self, _suite: &dyn TestSuite, count: usize) {
        self.pb.finish_and_clear();

        let pb = ProgressBar::new(count as u64);
        pb.set_message(format!("{} test files", "Loading".bold().cyan()));
        pb.set_style(
            indicatif::ProgressStyle::with_template("{msg} [{bar:40}]")
                .unwrap()
                .progress_chars("=> "),
        );
        self.pb = pb;
    }

    fn test_loaded(&mut self) {
        self.pb.inc(1);
    }

    fn test_suite_run_started(&mut self, suite: &TestSuiteInstance) {
        self.pb.finish_and_clear();
        self.pb.println(format!(
            "{} {} test files in {:.2}s",
            "Loaded".bold().bright_green(),
            suite.len(),
            self.start.elapsed().as_secs_f32()
        ));

        self.pb = ProgressBar::new(suite.len() as u64)
            .with_message(format!("{} tests", "Running".bold().cyan()));

        self.start = Instant::now();
    }

    fn test_completed(&mut self, result: &TestRunResult) {
        self.pb.inc(1);

        let label = match &result.outcome {
            TestRunOutcome::Passed(_) => "PASS".bold().green(),
            TestRunOutcome::IncorrectlyPassed(_)
            | TestRunOutcome::IncorrectlyErrored { .. }
            | TestRunOutcome::Panicked(_) => "FAIL".bold().red(),
        };

        self.pb
            .println(format!("{} {}", label, result.test_case.blue()));
    }

    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, _results: &TestResults) {
        self.pb.finish_and_clear();
        self.pb.println(format!(
            "{}: {} {} tests in {:.2}s",
            suite.name(),
            "Ran".bold().bright_green(),
            suite.len(),
            self.start.elapsed().as_secs_f32()
        ));
    }
}

pub enum SummaryDetailLevel {
    /// Only prints the coverage table
    Coverage,
    /// Prints the coverage table as well as all failing tests with their diagnostics
    Failing,

    /// Prints the RAST of the parsed syntax and the diagnostics for all tests (including tests that pass with expected diagnostics).
    Debug,
}

impl FromStr for SummaryDetailLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "coverage" => SummaryDetailLevel::Coverage,
            "failing" => SummaryDetailLevel::Failing,
            "debug" => SummaryDetailLevel::Debug,
            _ => return Err(String::from(
                "Unknown summary detail level. Valid values are: 'coverage', 'failing, and 'rast'.",
            )),
        })
    }
}

impl SummaryDetailLevel {
    fn is_coverage_only(&self) -> bool {
        matches!(self, SummaryDetailLevel::Coverage)
    }

    fn is_debug(&self) -> bool {
        matches!(self, SummaryDetailLevel::Debug)
    }
}

/// Reporter that prints a summary for each phase (tests loaded, test suite completed) to the console output
pub(crate) struct SummaryReporter {
    /// Buffer to store the detailed output of tests
    buffer: Buffer,
    /// The results of the ran test suites
    results: HashMap<String, Summary>,
    output_target: OutputTarget,
    detail_level: SummaryDetailLevel,
}

pub(crate) enum OutputTarget {
    Stdout(std::io::Stdout),
    Stderr(std::io::Stderr),
}

impl OutputTarget {
    pub fn stdout() -> Self {
        OutputTarget::Stdout(std::io::stdout())
    }

    pub fn stderr() -> Self {
        OutputTarget::Stderr(std::io::stderr())
    }
}

impl Write for OutputTarget {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            OutputTarget::Stderr(stderr) => stderr.write(buf),
            OutputTarget::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            OutputTarget::Stderr(stderr) => stderr.flush(),
            OutputTarget::Stdout(stdout) => stdout.flush(),
        }
    }
}

impl SummaryReporter {
    pub fn new(detail_level: SummaryDetailLevel, output_target: OutputTarget) -> Self {
        let buffer = if stdout().is_terminal() {
            Buffer::ansi()
        } else {
            // piping to a file
            Buffer::no_color()
        };

        Self {
            results: HashMap::default(),
            buffer,
            output_target,
            detail_level,
        }
    }

    fn writeln(&mut self, msg: &str) {
        writeln!(self.buffer, "{msg}").unwrap();
    }

    fn summary_table(results: HashMap<String, Summary>) -> String {
        let mut table = AsciiTable::default();
        let has_multiple_test_suites = results.len() > 1;

        let offset = if has_multiple_test_suites {
            table
                .column(0)
                .set_header("Test suite")
                .set_align(Align::Left);
            1
        } else {
            0
        };

        table
            .column(offset)
            .set_header("Tests ran".to_string())
            .set_align(Align::Right);
        table
            .column(offset + 1)
            .set_header("Passed")
            .set_align(Align::Right);
        table
            .column(offset + 2)
            .set_header("Failed")
            .set_align(Align::Right);
        table
            .column(offset + 3)
            .set_header("Panics")
            .set_align(Align::Right);
        table
            .column(offset + 4)
            .set_header("Coverage")
            .set_align(Align::Right);

        let mut results: Vec<_> = results.into_iter().collect();
        results.sort_by(|(l, _), (r, _)| l.cmp(r));

        let rows = results.into_iter().map(|(suite, summary)| {
            let panicked = summary.panics;
            let errored = summary.failed;
            let passed = summary.passed;
            let coverage = if summary.coverage.is_nan() {
                "\u{221E}".to_string()
            } else {
                format!("{:.2}", summary.coverage)
            };

            let total = panicked + errored + passed;

            let mut values = if has_multiple_test_suites {
                vec![suite]
            } else {
                Vec::default()
            };

            values.extend([
                total.to_string(),
                passed.to_string(),
                errored.to_string(),
                panicked.to_string(),
                coverage,
            ]);

            values
        });

        table.format(rows)
    }

    fn write_errors(&mut self, errors: &[Error], files: &TestCaseFiles) {
        files.emit_errors(errors, &mut self.buffer);
        self.writeln("");
    }
}

impl TestReporter for SummaryReporter {
    fn test_completed(&mut self, result: &TestRunResult) {
        if self.detail_level.is_coverage_only() {
            return;
        }

        match &result.outcome {
            TestRunOutcome::Passed(files) => {
                if self.detail_level.is_debug() {
                    self.writeln(&format!("{} {}", "[PASS]".bold().green(), result.test_case));

                    let mut all_errors = Vec::new();
                    for file in files {
                        if let Some(errors) = file.parse().ok().err() {
                            all_errors.extend(errors.into_iter().map(|error| {
                                error
                                    .with_file_path(file.name())
                                    .with_file_source_code(file.code())
                            }));
                        }
                    }

                    if !all_errors.is_empty() {
                        self.write_errors(&all_errors, files);
                    }
                }
            }
            TestRunOutcome::Panicked(_) => {
                let panic = result.outcome.panic_message();
                self.writeln(&format!(
                    "{} {}: {}",
                    "[PANIC]".bold().red(),
                    result.test_case,
                    panic.unwrap_or("Unknown panic reason")
                ));
            }
            TestRunOutcome::IncorrectlyPassed(_) => {
                self.writeln(&format!(
                    "{} {}: Incorrectly passed",
                    "[FAIL]".bold().red(),
                    result.test_case
                ));
            }
            TestRunOutcome::IncorrectlyErrored { errors, files } => {
                self.writeln(&format!(
                    "{} {}: Incorrectly errored:",
                    "[FAIL]".bold().red(),
                    result.test_case
                ));
                let errors: Vec<_> = errors
                    .iter()
                    .map(|diagnostic| Error::from(diagnostic.clone()))
                    .collect();
                self.write_errors(&errors, files);
            }
        }

        if self.detail_level.is_debug() {
            if let Some(files) = result.outcome.files() {
                for file in files {
                    let program = file.parse();
                    self.writeln(&format!(
                        "RAST Output for {}:\n{:#?}\n",
                        &file.name().bold(),
                        program.syntax()
                    ));
                }
            }
        }
    }

    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, results: &TestResults) {
        self.results
            .insert(suite.name().to_string(), results.summary.clone());
    }

    fn run_completed(&mut self) {
        let results = std::mem::take(&mut self.results);
        let table = Self::summary_table(results);

        self.output_target
            .write_all(self.buffer.as_slice())
            .unwrap();

        writeln!(self.output_target, "{table}").unwrap();
    }
}

#[derive(Default)]
pub(crate) struct JsonReporter {
    results: HashMap<String, Value>,
}

impl TestReporter for JsonReporter {
    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, result: &TestResults) {
        self.results.insert(
            suite.name().to_string(),
            serde_json::to_value(result).unwrap(),
        );
    }

    fn run_completed(&mut self) {
        let results = std::mem::take(&mut self.results);
        println!("{}", serde_json::to_string(&results).unwrap());
    }
}

/// Test reporter that forwards the event to multiple reporters.
/// Allows composing different reporters for a single test run
pub(crate) struct MulticastTestReporter(Vec<Box<dyn TestReporter>>);

impl MulticastTestReporter {
    pub fn new(reporter: Box<dyn TestReporter>) -> Self {
        Self(vec![reporter])
    }

    pub fn add(&mut self, reporter: Box<dyn TestReporter>) {
        self.0.push(reporter);
    }
}

impl TestReporter for MulticastTestReporter {
    fn test_suite_started(&mut self, test_suite: &dyn TestSuite) {
        for reporter in &mut self.0 {
            reporter.test_suite_started(test_suite);
        }
    }

    fn tests_discovered(&mut self, test_suite: &dyn TestSuite, count: usize) {
        for reporter in &mut self.0 {
            reporter.tests_discovered(test_suite, count);
        }
    }

    fn test_loaded(&mut self) {
        for reporter in &mut self.0 {
            reporter.test_loaded();
        }
    }

    fn test_suite_run_started(&mut self, suite: &TestSuiteInstance) {
        for reporter in &mut self.0 {
            reporter.test_suite_run_started(suite);
        }
    }

    fn test_completed(&mut self, result: &TestRunResult) {
        for reporter in &mut self.0 {
            reporter.test_completed(result);
        }
    }

    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, result: &TestResults) {
        for reporter in &mut self.0 {
            reporter.test_suite_completed(suite, result);
        }
    }

    fn run_completed(&mut self) {
        for reporter in &mut self.0 {
            reporter.run_completed();
        }
    }
}
