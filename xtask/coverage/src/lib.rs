pub mod compare;
pub mod js;
pub mod jsx;
mod reporters;
pub mod results;
mod runner;
pub mod symbols;
pub mod ts;
mod util;

pub use crate::reporters::SummaryDetailLevel;

use crate::js::test262::Test262TestSuite;
use crate::reporters::{
    DefaultReporter, JsonReporter, MulticastTestReporter, OutputTarget, SummaryReporter,
    TestReporter,
};
use crate::runner::{run_test_suite, TestRunContext, TestSuite};
use biome_parser::diagnostic::ParseDiagnostic;
use biome_string_case::StrOnlyExtension;
use jsx::jsx_babel::BabelJsxTestSuite;
use serde::{Deserialize, Serialize};
use std::any::Any;
use symbols::msts::SymbolsMicrosoftTestSuite;
use ts::ts_babel::BabelTypescriptTestSuite;
use ts::ts_microsoft::MicrosoftTypescriptTestSuite;
use util::decode_maybe_utf16_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    #[serde(rename = "o")]
    pub outcome: Outcome,
    #[serde(rename = "h")]
    pub test_case: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub enum Outcome {
    Passed,
    Failed,
    Panicked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
    #[serde(rename = "s")]
    pub summary: Summary,
    #[serde(rename = "p")]
    pub details: Vec<TestResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Summary {
    #[serde(rename = "a")]
    pub tests_ran: u32,
    #[serde(rename = "pa")]
    pub passed: u32,
    #[serde(rename = "f")]
    pub failed: u32,
    #[serde(rename = "pc")]
    pub panics: u32,
    #[serde(rename = "c")]
    pub coverage: f64,
}

impl Default for TestResults {
    fn default() -> Self {
        Self {
            summary: Summary {
                tests_ran: 0,
                passed: 0,
                failed: 0,
                panics: 0,
                coverage: 0.0,
            },
            details: vec![],
        }
    }
}

impl TestResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store_results(&mut self, results: Vec<TestResult>) {
        self.details = results;
        let passed = self.passed_tests() as u32;
        let tests_ran = self.details.len();
        let coverage = (f64::from(passed) / tests_ran as f64) * 100.0;
        self.summary = Summary {
            tests_ran: self.details.len() as u32,
            passed,
            failed: self.errored_tests() as u32,
            panics: self.panicked_tests() as u32,
            coverage,
        };
    }

    pub fn panicked_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| matches!(res.outcome, Outcome::Panicked))
            .count()
    }

    pub fn errored_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| matches!(res.outcome, Outcome::Failed))
            .count()
    }

    pub fn passed_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| res.outcome == Outcome::Passed)
            .count()
    }
}

pub fn run(
    suites: Option<&str>,
    filter: Option<&str>,
    json: bool,
    detail_level: SummaryDetailLevel,
) {
    let mut reporters = MulticastTestReporter::new(Box::<DefaultReporter>::default());

    let output_target = if json {
        reporters.add(Box::<JsonReporter>::default());
        OutputTarget::stderr()
    } else {
        OutputTarget::stdout()
    };

    reporters.add(Box::new(SummaryReporter::new(detail_level, output_target)));

    let mut context = TestRunContext {
        filter: filter.map(|s| s.to_string()),
        reporter: &mut reporters,
        pool: &yastl::Pool::new(
            std::thread::available_parallelism()
                .map_or(2, usize::from)
                .max(2),
        ),
    };

    let mut ran_any_tests = false;
    for test_suite in get_test_suites(suites) {
        let result = run_test_suite(test_suite.as_ref(), &mut context);
        ran_any_tests = ran_any_tests || result.summary.tests_ran > 0
    }

    reporters.run_completed();

    if !ran_any_tests {
        std::process::exit(1);
    }
}

const ALL_SUITES: &str = "*";
const ALL_JS_SUITES: &str = "js";
const ALL_TS_SUITES: &str = "ts";
const ALL_JSX_SUITES: &str = "jsx";
const ALL_SYMBOLS_SUITES: &str = "symbols";

fn get_test_suites(suites: Option<&str>) -> Vec<Box<dyn TestSuite>> {
    let suites = suites.unwrap_or("*").to_lowercase_cow();
    let mut ids: Vec<_> = suites.split(',').collect();

    let mut suites: Vec<Box<dyn TestSuite>> = vec![];

    while let Some(id) = ids.pop() {
        match id {
            ALL_JS_SUITES | "javascript" => ids.extend(["js/262"]),
            ALL_TS_SUITES | "typescript" => ids.extend(["ts/microsoft", "ts/babel"]),
            ALL_JSX_SUITES => ids.extend(["jsx/babel"]),
            ALL_SYMBOLS_SUITES => ids.extend(["symbols/microsoft"]),
            ALL_SUITES => ids.extend(["js", "ts", "jsx", "symbols"]),

            "js/262" => suites.push(Box::new(Test262TestSuite)),
            "ts/microsoft" => suites.push(Box::new(MicrosoftTypescriptTestSuite)),
            "ts/babel" => suites.push(Box::new(BabelTypescriptTestSuite)),
            "jsx/babel" => suites.push(Box::new(BabelJsxTestSuite)),
            "symbols/microsoft" => suites.push(Box::new(SymbolsMicrosoftTestSuite)),

            _ => {}
        }
    }

    suites
}

fn check_file_encoding(path: &std::path::Path) -> Option<String> {
    let buffer = std::fs::read(path).unwrap();
    decode_maybe_utf16_string(&buffer)
        .ok()
        .map(|decoded| decoded.to_string())
}
