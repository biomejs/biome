use super::*;
use crate::reporters::TestReporter;
use biome_diagnostics::console::fmt::{Formatter, Termcolor};
use biome_diagnostics::console::markup;
use biome_diagnostics::termcolor::Buffer;
use biome_diagnostics::Error;
use biome_diagnostics::PrintDiagnostic;
use biome_js_parser::{parse, JsParserOptions, Parse};
use biome_js_syntax::{AnyJsRoot, JsFileSource, JsSyntaxNode};
use biome_rowan::SyntaxKind;
use std::fmt::Debug;
use std::io;
use std::panic::RefUnwindSafe;
use std::path::Path;
use walkdir::WalkDir;
use yastl::Pool;

pub(crate) struct TestRunResult {
    pub(crate) outcome: TestRunOutcome,
    pub(crate) test_case: String,
}

pub(crate) enum TestRunOutcome {
    Passed(TestCaseFiles),
    IncorrectlyPassed(TestCaseFiles),
    IncorrectlyErrored {
        files: TestCaseFiles,
        errors: Vec<ParseDiagnostic>,
    },
    Panicked(Box<dyn Any + Send + 'static>),
}

impl TestRunOutcome {
    pub fn is_failed(&self) -> bool {
        !matches!(self, TestRunOutcome::Passed(_))
    }

    pub fn files(&self) -> Option<&TestCaseFiles> {
        match self {
            TestRunOutcome::Passed(files)
            | TestRunOutcome::IncorrectlyPassed(files)
            | TestRunOutcome::IncorrectlyErrored { files, .. } => Some(files),
            _ => None,
        }
    }

    pub fn panic_message(&self) -> Option<&str> {
        match self {
            TestRunOutcome::Panicked(panic) => panic
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| panic.downcast_ref::<&'static str>().copied()),

            _ => None,
        }
    }
}

impl From<TestRunOutcome> for Outcome {
    fn from(run_outcome: TestRunOutcome) -> Self {
        match run_outcome {
            TestRunOutcome::Passed(_) => Outcome::Passed,
            TestRunOutcome::IncorrectlyPassed(_) | TestRunOutcome::IncorrectlyErrored { .. } => {
                Outcome::Failed
            }
            TestRunOutcome::Panicked(_) => Outcome::Panicked,
        }
    }
}

/// A test case may parse multiple files. Represents a single file of a test case
#[derive(Debug, Clone)]
pub(crate) struct TestCaseFile {
    /// The (test case relative) name of the file
    name: String,

    /// The code of the file
    code: String,

    /// The source type used to parse the file
    source_type: JsFileSource,

    options: JsParserOptions,
}

impl TestCaseFile {
    pub(crate) fn parse(&self) -> Parse<AnyJsRoot> {
        parse(&self.code, self.source_type, self.options.clone())
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn code(&self) -> &str {
        &self.code
    }
}

pub(crate) fn create_bogus_node_in_tree_diagnostic(node: JsSyntaxNode) -> ParseDiagnostic {
    assert!(node.kind().is_bogus());
    ParseDiagnostic::new(
        "There are no parse errors but the parsed tree contains bogus nodes.",
        node.text_trimmed_range()
    )
    .with_hint( "This bogus node is present in the parsed tree but the parser didn't emit a diagnostic for it. Change the parser to either emit a diagnostic, fix the grammar, or the parsing.")
}

#[derive(Clone, Debug)]
pub(crate) struct TestCaseFiles {
    files: Vec<TestCaseFile>,
}

impl TestCaseFiles {
    pub(crate) fn single(
        name: String,
        code: String,
        source_type: JsFileSource,
        options: JsParserOptions,
    ) -> Self {
        Self {
            files: vec![TestCaseFile {
                name,
                code,
                source_type,
                options,
            }],
        }
    }

    pub(crate) fn new() -> Self {
        Self { files: vec![] }
    }

    pub(crate) fn add(
        &mut self,
        name: String,
        code: String,
        source_type: JsFileSource,
        options: JsParserOptions,
    ) {
        self.files.push(TestCaseFile {
            name,
            code,
            source_type,
            options,
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub(crate) fn emit_errors(&self, errors: &[Error], buffer: &mut Buffer) {
        for error in errors {
            if let Err(err) = Formatter::new(&mut Termcolor(&mut *buffer)).write_markup(markup! {
                {PrintDiagnostic::verbose(error)}
            }) {
                eprintln!("Failed to print diagnostic: {err}");
            }
        }
    }
}

impl<'a> IntoIterator for &'a TestCaseFiles {
    type Item = &'a TestCaseFile;
    type IntoIter = std::slice::Iter<'a, TestCaseFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}

pub(crate) trait TestCase: RefUnwindSafe + Send + Sync {
    /// Returns the (display) name of the test case. Used to uniquely identify the test case
    fn name(&self) -> &str;

    /// Runs the test case
    fn run(&self) -> TestRunOutcome;
}

pub(crate) trait TestSuite: Send + Sync {
    fn name(&self) -> &str;
    fn base_path(&self) -> &str;
    fn is_test(&self, path: &Path) -> bool;
    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>>;
    fn checkout(&self) -> io::Result<()>;
}

pub(crate) struct TestSuiteInstance {
    name: String,
    tests: Vec<Box<dyn TestCase>>,
}

impl TestSuiteInstance {
    pub fn new(suite: &dyn TestSuite, tests: Vec<Box<dyn TestCase>>) -> Self {
        Self {
            tests,
            name: suite.name().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.tests.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn TestCase>> {
        self.tests.iter()
    }
}

pub(crate) struct TestRunContext<'a> {
    pub(crate) filter: Option<String>,
    pub(crate) reporter: &'a mut dyn TestReporter,
    pub(crate) pool: &'a Pool,
}

pub(crate) fn run_test_suite(
    test_suite: &dyn TestSuite,
    context: &mut TestRunContext,
) -> TestResults {
    test_suite.checkout().expect("To checkout the repository");
    context.reporter.test_suite_started(test_suite);
    let instance = load_tests(test_suite, context);
    context.reporter.test_suite_run_started(&instance);

    std::panic::set_hook(Box::new(|info| {
        use std::io::Write;

        let backtrace = backtrace::Backtrace::default();
        let mut stacktrace = vec![];

        // Skip frames inside the backtrace lib
        for frame in backtrace.frames().iter().skip(6) {
            if let Some(s) = frame.symbols().first() {
                if let Some(file) = s.filename() {
                    // We don't care about std or cargo registry libs
                    let file_path = file.as_os_str().to_str().unwrap();
                    if file_path.starts_with("/rustc") || file_path.contains(".cargo") {
                        continue;
                    }

                    let _ = write!(stacktrace, "{}", file.display());
                } else if let Some(name) = s.name().and_then(|x| x.as_str()) {
                    let _ = write!(stacktrace, "{name}");
                } else {
                    let _ = write!(stacktrace, "<unknown>");
                }

                match (s.lineno(), s.colno()) {
                    (Some(line), Some(col)) => {
                        let _ = write!(stacktrace, " @ line {line} col {col}");
                    }
                    (Some(line), None) => {
                        let _ = write!(stacktrace, " @ line {line}");
                    }
                    (None, Some(col)) => {
                        let _ = write!(stacktrace, " @ col {col}");
                    }
                    _ => {}
                }

                let _ = writeln!(stacktrace);
            }
        }

        let stacktrace = String::from_utf8(stacktrace).unwrap();

        let mut msg = vec![];
        let _ = write!(msg, "{info}");
        let msg = String::from_utf8(msg).unwrap();

        tracing::error!(
            panic = msg.as_str(),
            stacktrace = stacktrace.as_str(),
            "Test panicked"
        );
    }));

    let mut test_results = TestResults::new();
    let (tx, rx) = std::sync::mpsc::channel();

    context.pool.scoped(|scope| {
        scope.execute(|| {
            let mut results: Vec<TestResult> = Vec::with_capacity(instance.len());
            for result in rx {
                context.reporter.test_completed(&result);
                results.push(TestResult {
                    test_case: result.test_case,
                    outcome: result.outcome.into(),
                });
            }
            test_results.store_results(results);
        });

        for test in instance.iter() {
            let tx = tx.clone();

            scope.execute(move || {
                let test_ref = test.as_ref();

                let outcome = match std::panic::catch_unwind(|| test_ref.run()) {
                    Ok(result) => result,
                    Err(panic) => {
                        let error = panic
                            .downcast_ref::<String>()
                            .map(|x| x.to_string())
                            .or_else(|| panic.downcast_ref::<&str>().map(|x| (*x).to_string()))
                            .unwrap_or_default();
                        tracing::error!(
                            panic = error.as_str(),
                            name = test.name(),
                            "Test panicked"
                        );
                        TestRunOutcome::Panicked(panic)
                    }
                };

                tx.send(TestRunResult {
                    test_case: test.name().to_owned(),
                    outcome,
                })
                .unwrap();
            });
        }

        drop(tx);
    });

    context
        .reporter
        .test_suite_completed(&instance, &test_results);

    let _ = std::panic::take_hook();

    test_results
}

fn load_tests(suite: &dyn TestSuite, context: &mut TestRunContext) -> TestSuiteInstance {
    let paths = WalkDir::new(suite.base_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|file| {
            let path = file.path();
            if !path.is_file() {
                return false;
            }

            if !suite.is_test(path) {
                return false;
            }

            if let Some(filter) = &context.filter {
                let normalized_path = path.to_string_lossy().replace('\\', "/");
                let normalized_query = filter.replace('\\', "/");
                normalized_path.contains(&normalized_query)
            } else {
                true
            }
        })
        .map(|file| file.path().to_owned())
        .collect::<Vec<_>>();

    context.reporter.tests_discovered(suite, paths.len());

    let (tx, rx) = std::sync::mpsc::channel();
    let mut tests: Vec<Box<dyn TestCase>> = Vec::with_capacity(paths.len());

    context.pool.scoped(|scope| {
        scope.execute(|| {
            for test in rx {
                context.reporter.test_loaded();
                if let Some(test) = test {
                    tests.push(test);
                }
            }
        });

        for path in paths {
            let tx = tx.clone();

            scope.execute(move || {
                tx.send(suite.load_test(&path)).unwrap();
            });
        }

        drop(tx);
    });

    TestSuiteInstance::new(suite, tests)
}
