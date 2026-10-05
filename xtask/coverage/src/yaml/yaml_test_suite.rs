use crate::runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite};
use crate::util::checkout_repository;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::SyntaxKind;
use biome_yaml_parser::parse_yaml;
use std::io;
use std::path::Path;
use xtask_glue::project_root;

const BASE_PATH: &str = "xtask/coverage/yaml-test-suite";
const REVISION: &str = "6e6c296ae9c9d2d5c4134b4b64d01b29ac19ff6f";

struct YamlTestCase {
    name: String,
    source: String,
    should_fail: bool,
}

impl YamlTestCase {
    fn from_path(path: &Path, source: String, should_fail: bool) -> Option<Self> {
        let name = path
            .parent()?
            .strip_prefix(BASE_PATH)
            .ok()?
            .to_string_lossy()
            .replace('\\', "/");

        Some(Self {
            name,
            source,
            should_fail,
        })
    }
}

impl TestCase for YamlTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let parsed = parse_yaml(&self.source);
        let files = TestCaseFiles::new();

        if self.should_fail {
            return if parsed.has_errors() {
                TestRunOutcome::Passed(files)
            } else {
                TestRunOutcome::IncorrectlyPassed(files)
            };
        }

        if parsed.has_errors() {
            return TestRunOutcome::IncorrectlyErrored {
                errors: parsed.into_diagnostics(),
                files,
            };
        }

        if let Some(bogus) = parsed
            .syntax()
            .descendants()
            .find(|node| node.kind().is_bogus())
        {
            return TestRunOutcome::IncorrectlyErrored {
                errors: vec![ParseDiagnostic::new(
                    "There are no parse errors but the parsed tree contains bogus nodes.",
                    bogus.text_trimmed_range(),
                )],
                files,
            };
        }

        TestRunOutcome::Passed(files)
    }
}

pub(crate) struct YamlTestSuite;

impl TestSuite for YamlTestSuite {
    fn name(&self) -> &str {
        "yaml/yaml-test-suite"
    }

    fn base_path(&self) -> &str {
        BASE_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        path.file_name().is_some_and(|name| name == "in.yaml")
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        let source = std::fs::read_to_string(path).ok()?;
        let should_fail = path.parent()?.join("error").is_file();
        Some(Box::new(YamlTestCase::from_path(
            path,
            source,
            should_fail,
        )?))
    }

    fn checkout(&self) -> io::Result<()> {
        checkout_repository(
            "https://github.com/yaml/yaml-test-suite.git",
            REVISION,
            &project_root().join(BASE_PATH),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_case(source: &str, should_fail: bool) -> YamlTestCase {
        YamlTestCase {
            name: "test".to_string(),
            source: source.to_string(),
            should_fail,
        }
    }

    #[test]
    fn uses_upstream_case_id_as_name() {
        let path = Path::new(BASE_PATH).join("3RLN/00/in.yaml");
        let case = YamlTestCase::from_path(&path, String::new(), false).unwrap();
        assert_eq!(case.name(), "3RLN/00");
    }

    #[test]
    fn recognizes_suite_inputs() {
        let suite = YamlTestSuite;
        assert!(suite.is_test(Path::new("229Q/in.yaml")));
        assert!(!suite.is_test(Path::new("229Q/test.event")));
    }

    #[test]
    fn valid_case_passes() {
        assert!(matches!(
            test_case("key: value\n", false).run(),
            TestRunOutcome::Passed(_)
        ));
    }

    #[test]
    fn expected_error_passes() {
        assert!(matches!(
            test_case("[", true).run(),
            TestRunOutcome::Passed(_)
        ));
    }

    #[test]
    fn unexpected_error_fails() {
        assert!(matches!(
            test_case("[", false).run(),
            TestRunOutcome::IncorrectlyErrored { .. }
        ));
    }

    #[test]
    fn missing_expected_error_fails() {
        assert!(matches!(
            test_case("key: value\n", true).run(),
            TestRunOutcome::IncorrectlyPassed(_)
        ));
    }

    #[test]
    fn preserves_utf8_bom() {
        assert!(matches!(
            test_case("\u{feff}key: value\n", false).run(),
            TestRunOutcome::Passed(_)
        ));
    }
}
