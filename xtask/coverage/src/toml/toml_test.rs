use crate::runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite};
use crate::util::checkout_repository;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{SyntaxKind, TextRange};
use biome_toml_parser::parse_toml;
use std::io;
use std::path::Path;
use xtask_glue::project_root;

const BASE_PATH: &str = "xtask/coverage/toml-test";
/// The commit of toml-test v2.2.0.
const REVISION: &str = "ce08da1ddb075d1c7596d663c7fcba9a2ae02c5c";
/// Lists the test files that apply to TOML 1.1.0, relative to `tests/`.
const FILE_LIST: &str = "tests/files-toml-1.1.0";

struct TomlTestCase {
    /// The path relative to `tests/`, such as `valid/array/array.toml`.
    name: String,
    /// `None` if the file isn't UTF-8, which only invalid cases use.
    source: Option<String>,
    should_fail: bool,
}

impl TomlTestCase {
    fn load(name: &str) -> Option<Self> {
        let path = project_root().join(BASE_PATH).join("tests").join(name);
        let bytes = std::fs::read(path).ok()?;

        Some(Self {
            name: name.to_string(),
            source: String::from_utf8(bytes).ok(),
            should_fail: name.starts_with("invalid/"),
        })
    }
}

impl TestCase for TomlTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let files = TestCaseFiles::new();

        let Some(source) = &self.source else {
            // Biome only parses UTF-8, so other encodings are rejected before
            // parsing.
            return if self.should_fail {
                TestRunOutcome::Passed(files)
            } else {
                TestRunOutcome::IncorrectlyErrored {
                    errors: vec![ParseDiagnostic::new(
                        "The file isn't valid UTF-8.",
                        None::<TextRange>,
                    )],
                    files,
                }
            };
        };

        let parsed = parse_toml(source);

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

/// The TOML 1.1.0 cases of [toml-test](https://github.com/toml-lang/toml-test).
///
/// Valid documents must parse without errors and invalid documents must
/// produce an error.
pub(crate) struct TomlTestSuite;

impl TestSuite for TomlTestSuite {
    fn name(&self) -> &str {
        "toml/toml-test"
    }

    fn base_path(&self) -> &str {
        BASE_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        path.extension()
            .is_some_and(|extension| extension == "toml")
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        let name = path
            .strip_prefix(Path::new(BASE_PATH).join("tests"))
            .ok()?
            .to_string_lossy()
            .replace('\\', "/");
        Some(Box::new(TomlTestCase::load(&name)?))
    }

    /// Loads the cases listed for TOML 1.1.0. The repository also contains
    /// cases that only apply to TOML 1.0.0.
    fn load_all(&self) -> Option<Vec<Box<dyn TestCase>>> {
        let list = std::fs::read_to_string(project_root().join(BASE_PATH).join(FILE_LIST)).ok()?;
        Some(
            list.lines()
                .filter(|name| name.ends_with(".toml"))
                .filter_map(|name| {
                    TomlTestCase::load(name).map(|test| Box::new(test) as Box<dyn TestCase>)
                })
                .collect(),
        )
    }

    fn checkout(&self) -> io::Result<()> {
        checkout_repository(
            "https://github.com/toml-lang/toml-test.git",
            REVISION,
            &project_root().join(BASE_PATH),
        )
    }
}
