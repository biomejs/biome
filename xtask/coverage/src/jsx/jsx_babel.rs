use crate::runner::create_bogus_node_in_tree_diagnostic;
use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::{JsFileSource, ModuleKind};
use biome_rowan::SyntaxKind;
use std::io;
use std::path::Path;
use std::process::Command;
use xtask::project_root;

const OK_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/jsx/basic";

struct BabelJsxTestCase {
    name: String,
    code: String,
}

impl BabelJsxTestCase {
    fn new(path: &Path, code: String) -> Self {
        let name = path
            .components()
            .rev()
            .nth(1)
            .and_then(|x| x.as_os_str().to_str())
            .unwrap_or("")
            .to_string();

        Self { name, code }
    }
}

impl TestCase for BabelJsxTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let source_type = JsFileSource::jsx().with_module_kind(ModuleKind::Script);
        let options = JsParserOptions::default().with_parse_class_parameter_decorators();

        let files = TestCaseFiles::single(
            self.name().to_string(),
            self.code.clone(),
            source_type,
            options.clone(),
        );
        let result = parse(&self.code, source_type, options);

        if result.diagnostics().is_empty() {
            if let Some(bogus) = result
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
        } else {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: result.diagnostics().to_vec(),
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct BabelJsxTestSuite;

impl TestSuite for BabelJsxTestSuite {
    fn name(&self) -> &str {
        "jsx/babel"
    }

    fn base_path(&self) -> &str {
        OK_PATH
    }

    fn is_test(&self, path: &std::path::Path) -> bool {
        path.extension().is_some_and(|x| x == "js")
    }

    fn load_test(&self, path: &std::path::Path) -> Option<Box<dyn crate::runner::TestCase>> {
        let code = check_file_encoding(path)?;
        Some(Box::new(BabelJsxTestCase::new(path, code)))
    }
    fn checkout(&self) -> io::Result<()> {
        let base_path = project_root().join("xtask/coverage/babel");
        let mut command = Command::new("git");
        command
            .arg("clone")
            .arg("https://github.com/babel/babel.git")
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
}
