use crate::runner::create_bogus_node_in_tree_diagnostic;
use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use biome_js_parser::JsParserOptions;
use biome_js_syntax::{JsFileSource, LanguageVariant};
use biome_rowan::SyntaxKind;
use std::io;
use std::path::Path;
use std::process::Command;
use xtask::project_root;

const CASES_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/typescript";

struct BabelTypescriptTestCase {
    name: String,
    expected_to_fail: bool,
    code: String,
    variant: LanguageVariant,
}

impl BabelTypescriptTestCase {
    fn new(path: &Path, code: String, expected_to_fail: bool, variant: LanguageVariant) -> Self {
        let name = path
            .parent()
            .unwrap()
            .strip_prefix(CASES_PATH)
            .unwrap()
            .display()
            .to_string();

        Self {
            name,
            code,
            expected_to_fail,
            variant,
        }
    }
}

impl TestCase for BabelTypescriptTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let source_type = JsFileSource::ts().with_variant(self.variant);
        let options = JsParserOptions::default().with_parse_class_parameter_decorators();
        let files = TestCaseFiles::single(
            self.name().to_string(),
            self.code.clone(),
            source_type,
            options.clone(),
        );

        let result = biome_js_parser::parse(&self.code, source_type, options);

        if self.expected_to_fail && result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyPassed(files)
        } else if self.expected_to_fail {
            TestRunOutcome::Passed(files)
        } else if !result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: result.diagnostics().to_vec(),
            }
        } else if let Some(bogus) = result
            .syntax()
            .descendants()
            .find(|descendant| descendant.kind().is_bogus())
        {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: vec![create_bogus_node_in_tree_diagnostic(bogus)],
            }
        } else {
            TestRunOutcome::Passed(files)
        }
    }
}

#[derive(Default)]
pub(crate) struct BabelTypescriptTestSuite;

impl TestSuite for BabelTypescriptTestSuite {
    fn name(&self) -> &str {
        "ts/babel"
    }

    fn base_path(&self) -> &str {
        CASES_PATH
    }

    fn checkout(&self) -> io::Result<()> {
        let base_path = project_root().join("xtask/coverage/babel");
        let mut command = Command::new("git");
        command
            .arg("clone")
            .arg("https://github.com/babel/babel.git")
            .arg("--depth")
            .arg("1")
            .arg(base_path.display().to_string());
        command.output()?;
        let mut command = Command::new("git");
        command
            .arg("reset")
            .arg("--hard")
            .arg("33a6be4e56b149647c15fd6c0157c1413456851d");
        command.output()?;

        Ok(())
    }

    fn is_test(&self, path: &std::path::Path) -> bool {
        path.extension().is_some_and(|x| x == "ts")
    }

    fn load_test(&self, path: &std::path::Path) -> Option<Box<dyn crate::runner::TestCase>> {
        let code = check_file_encoding(path)?;

        let output_json_path = path.with_file_name("output.json");
        let options_path = path.with_file_name("options.json");

        let mut should_fail = false;
        let mut variant = LanguageVariant::Standard;

        if output_json_path.exists() {
            if let Some(content) = check_file_encoding(&output_json_path) {
                should_fail = content.contains("\"errors\":");
            }
        }

        if options_path.exists() {
            if let Some(content) = check_file_encoding(&options_path) {
                should_fail = should_fail || content.contains("\"throws\":");

                if content.contains("jsx") {
                    variant = LanguageVariant::Jsx;
                }
            }
        };

        Some(Box::new(BabelTypescriptTestCase::new(
            path,
            code,
            should_fail,
            variant,
        )))
    }
}
