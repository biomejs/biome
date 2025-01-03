use crate::runner::{
    create_bogus_node_in_tree_diagnostic, TestCase, TestCaseFiles, TestRunOutcome, TestSuite,
};
use biome_js_parser::{parse, JsParserOptions};
use biome_js_syntax::JsFileSource;
use biome_rowan::syntax::SyntaxKind;
use biome_rowan::AstNode;
use regex::Regex;
use serde::Deserialize;
use std::io;
use std::path::Path;
use std::process::Command;
use xtask::project_root;

const BASE_PATH: &str = "xtask/coverage/test262/test";

/// Representation of the YAML metadata in Test262 tests.
// taken from the boa project
#[derive(Debug, Clone, Deserialize)]
pub struct MetaData {
    pub description: Box<str>,
    pub esid: Option<Box<str>>,
    pub es5id: Option<Box<str>>,
    pub es6id: Option<Box<str>>,
    #[serde(default)]
    pub info: Box<str>,
    #[serde(default)]
    pub features: Box<[Box<str>]>,
    #[serde(default)]
    pub includes: Box<[Box<str>]>,
    #[serde(default)]
    pub flags: Box<[TestFlag]>,
    #[serde(default)]
    pub negative: Option<Negative>,
    #[serde(default)]
    pub locale: Box<[Box<str>]>,
}

/// Negative test information structure.
#[derive(Debug, Clone, Deserialize)]
pub struct Negative {
    pub phase: Phase,
    #[serde(rename = "type")]
    pub error_type: Box<str>,
}

/// Individual test flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TestFlag {
    OnlyStrict,
    NoStrict,
    Module,
    Raw,
    Async,
    Generated,
    #[serde(rename = "CanBlockIsFalse")]
    CanBlockIsFalse,
    #[serde(rename = "CanBlockIsTrue")]
    CanBlockIsTrue,
    #[serde(rename = "non-deterministic")]
    NonDeterministic,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Phase {
    Parse,
    Early,
    Resolution,
    Runtime,
}

#[derive(Debug)]
struct Test262TestCase {
    name: String,
    code: String,
    meta: MetaData,
}

impl Test262TestCase {
    fn new(path: &Path, code: String, meta: MetaData) -> Self {
        let name = path.strip_prefix(BASE_PATH).unwrap().display().to_string();

        Self { name, code, meta }
    }

    fn execute_test(&self, append_use_strict: bool, source_type: JsFileSource) -> TestRunOutcome {
        let code = if append_use_strict {
            format!("\"use strict\";\n{}", self.code)
        } else {
            self.code.clone()
        };

        let should_fail = self
            .meta
            .negative
            .as_ref()
            .filter(|neg| neg.phase == Phase::Parse)
            .is_some();

        let options = JsParserOptions::default().with_parse_class_parameter_decorators();
        let files = TestCaseFiles::single(
            self.name.clone(),
            self.code.clone(),
            source_type,
            options.clone(),
        );

        match parse(&code, source_type, options).ok() {
            Ok(root) if !should_fail => {
                if let Some(bogus) = root
                    .syntax()
                    .descendants()
                    .find(|descendant| descendant.kind().is_bogus())
                {
                    TestRunOutcome::IncorrectlyErrored {
                        errors: vec![create_bogus_node_in_tree_diagnostic(bogus)],
                        files,
                    }
                } else {
                    TestRunOutcome::Passed(files)
                }
            }
            Err(_) if should_fail => TestRunOutcome::Passed(files),
            Ok(_) if should_fail => TestRunOutcome::IncorrectlyPassed(files),
            Err(errors) if !should_fail => TestRunOutcome::IncorrectlyErrored { errors, files },
            _ => unreachable!(),
        }
    }
}

impl TestCase for Test262TestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let meta = &self.meta;
        if meta.flags.contains(&TestFlag::OnlyStrict) {
            self.execute_test(true, JsFileSource::js_script())
        } else if meta.flags.contains(&TestFlag::Module) {
            self.execute_test(false, JsFileSource::js_module())
        } else if meta.flags.contains(&TestFlag::NoStrict) || meta.flags.contains(&TestFlag::Raw) {
            self.execute_test(false, JsFileSource::js_script())
        } else {
            let l = self.execute_test(false, JsFileSource::js_script());
            let r = self.execute_test(true, JsFileSource::js_script());
            merge_outcomes(l, r)
        }
    }
}

pub(crate) struct Test262TestSuite;

impl TestSuite for Test262TestSuite {
    fn name(&self) -> &str {
        "js/262"
    }

    fn base_path(&self) -> &str {
        BASE_PATH
    }

    fn checkout(&self) -> io::Result<()> {
        let base_path = project_root().join(BASE_PATH);
        let mut command = Command::new("git");
        command
            .arg("clone")
            .arg("https://github.com/tc39/test262.git")
            .arg("--depth")
            .arg("1")
            .arg(base_path.display().to_string());
        command.output()?;
        let mut command = Command::new("git");
        command
            .arg("reset")
            .arg("--hard")
            .arg("715dd1073bc060f4ee221e2e74770f5728e7b8a0");
        command.output()?;
        Ok(())
    }

    fn is_test(&self, path: &Path) -> bool {
        match path.extension() {
            None => false,
            Some(ext) => ext == "js",
        }
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        let code = std::fs::read_to_string(path).ok()?;

        let meta = read_metadata(&code).ok()?;

        if !meta
            .negative
            .as_ref()
            .map_or(true, |negative| negative.phase == Phase::Parse)
        {
            None
        } else {
            Some(Box::new(Test262TestCase::new(path, code, meta)))
        }
    }
}

fn read_metadata(code: &str) -> io::Result<MetaData> {
    // Regular expression to retrieve the metadata of a test.
    let meta_regex = Regex::new(r"/\*\-{3}((?:.|\n)*)\-{3}\*/")
        .expect("could not compile metadata regular expression");

    let yaml = meta_regex
        .captures(code)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no metadata found"))?
        .get(1)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "no metadata found"))?
        .as_str();

    serde_yaml::from_str(yaml).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn merge_outcomes(l: TestRunOutcome, r: TestRunOutcome) -> TestRunOutcome {
    if l.is_failed() {
        l
    } else {
        r
    }
}
