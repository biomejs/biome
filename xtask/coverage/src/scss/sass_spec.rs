use crate::runner::{
    TestCase, TestCaseFiles, TestRunOutcome, TestSuite, create_bogus_node_in_tree_diagnostic,
};
use crate::util::checkout_repository;
use biome_css_parser::{CssParserOptions, parse_css};
use biome_languages::CssFileSource;
use biome_rowan::SyntaxKind;
use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use xtask_glue::project_root;

const CHECKOUT_PATH: &str = "xtask/coverage/sass-spec";
const BASE_PATH: &str = "xtask/coverage/sass-spec/spec";
const REVISION: &str = "49bf57edf18c4938599b3afd53dd826ee2f4e0e6";

#[derive(Debug)]
struct SassSpecTestCase {
    name: String,
    source: String,
}

impl SassSpecTestCase {
    fn new(name: String, source: String) -> Self {
        Self { name, source }
    }
}

impl TestCase for SassSpecTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let parsed = parse_css(
            &self.source,
            CssFileSource::scss(),
            CssParserOptions::default(),
        );
        let files = TestCaseFiles::new();

        if !parsed.diagnostics().is_empty() {
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
                errors: vec![create_bogus_node_in_tree_diagnostic(bogus)],
                files,
            };
        }

        TestRunOutcome::Passed(files)
    }
}

pub(crate) struct SassSpecTestSuite;

impl TestSuite for SassSpecTestSuite {
    fn name(&self) -> &str {
        "scss/sass-spec"
    }

    fn base_path(&self) -> &str {
        BASE_PATH
    }

    fn is_test(&self, _path: &Path) -> bool {
        false
    }

    fn load_test(&self, _path: &Path) -> Option<Box<dyn TestCase>> {
        None
    }

    fn checkout(&self) -> io::Result<()> {
        checkout_repository(
            "https://github.com/sass/sass-spec.git",
            REVISION,
            &project_root().join(CHECKOUT_PATH),
        )
    }

    fn load_all(&self) -> Option<Vec<Box<dyn TestCase>>> {
        Some(
            load_sass_spec_cases(Path::new(BASE_PATH))
                .expect("failed to load Sass specification tests")
                .into_iter()
                .map(|case| Box::new(case) as Box<dyn TestCase>)
                .collect(),
        )
    }
}

fn load_sass_spec_cases(base_path: &Path) -> io::Result<Vec<SassSpecTestCase>> {
    let virtual_files = load_hrx_files(base_path)?;
    let mut cases = collect_virtual_cases(virtual_files);

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.file_name().is_none_or(|name| name != "input.scss")
            || !has_success_expectation_on_disk(path)
        {
            continue;
        }

        let source = read_sass_source(path)?;
        let Some(name) = path
            .parent()
            .and_then(|parent| parent.strip_prefix(base_path).ok())
            .map(normalize_path)
        else {
            continue;
        };

        cases.insert(name.clone(), SassSpecTestCase::new(name, source));
    }

    Ok(cases.into_values().collect())
}

fn read_sass_source(path: &Path) -> io::Result<String> {
    let bytes = std::fs::read(path)?;
    decode_sass_source(&bytes)
}

fn decode_sass_source(bytes: &[u8]) -> io::Result<String> {
    if bytes.starts_with(&[0xff, 0xfe]) {
        let (pairs, remainder) = bytes.as_chunks::<2>();
        if !remainder.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "UTF-16LE source has an odd byte length",
            ));
        }
        let units = pairs
            .iter()
            .copied()
            .map(u16::from_le_bytes)
            .collect::<Vec<_>>();
        String::from_utf16(&units)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    } else if bytes.starts_with(&[0xfe, 0xff]) {
        let (pairs, remainder) = bytes.as_chunks::<2>();
        if !remainder.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "UTF-16BE source has an odd byte length",
            ));
        }
        let units = pairs
            .iter()
            .copied()
            .map(u16::from_be_bytes)
            .collect::<Vec<_>>();
        String::from_utf16(&units)
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    } else {
        String::from_utf8(bytes.to_vec())
            .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }
}

fn load_hrx_files(base_path: &Path) -> io::Result<BTreeMap<PathBuf, String>> {
    let mut files = BTreeMap::new();

    for entry in WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "hrx") {
            continue;
        }

        let source = std::fs::read_to_string(path)?;
        let mut archive_root = path
            .strip_prefix(base_path)
            .expect("HRX archive must be inside the Sass spec directory")
            .to_path_buf();
        archive_root.set_extension("");

        insert_hrx_files(&mut files, &archive_root, &source)?;
    }

    Ok(files)
}

fn insert_hrx_files(
    files: &mut BTreeMap<PathBuf, String>,
    archive_root: &Path,
    source: &str,
) -> io::Result<()> {
    for file in parse_hrx(source) {
        if file.path.ends_with('/') {
            continue;
        }

        let path = archive_root.join(file.path);
        if files.insert(path.clone(), file.body.to_string()).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("duplicate Sass spec file: {}", path.display()),
            ));
        }
    }

    Ok(())
}

fn collect_virtual_cases(
    mut files: BTreeMap<PathBuf, String>,
) -> BTreeMap<String, SassSpecTestCase> {
    let inputs = files
        .keys()
        .filter(|path| path.file_name().is_some_and(|name| name == "input.scss"))
        .cloned()
        .collect::<Vec<_>>();
    let mut cases = BTreeMap::new();

    for input in inputs {
        if !has_success_expectation(&files, &input) {
            continue;
        }

        let Some(name) = input.parent().map(normalize_path) else {
            continue;
        };
        let source = files
            .remove(&input)
            .expect("collected Sass spec input must exist");
        cases.insert(name.clone(), SassSpecTestCase::new(name, source));
    }

    cases
}

fn has_success_expectation(files: &BTreeMap<PathBuf, String>, input: &Path) -> bool {
    let Some(parent) = input.parent() else {
        return false;
    };

    if files.contains_key(&parent.join("error-dart-sass")) {
        false
    } else {
        files.contains_key(&parent.join("output-dart-sass.css"))
            || files.contains_key(&parent.join("output.css"))
    }
}

fn has_success_expectation_on_disk(input: &Path) -> bool {
    let Some(parent) = input.parent() else {
        return false;
    };

    if parent.join("error-dart-sass").is_file() {
        false
    } else {
        parent.join("output-dart-sass.css").is_file() || parent.join("output.css").is_file()
    }
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

#[derive(Debug, Eq, PartialEq)]
struct HrxFile<'a> {
    path: &'a str,
    body: &'a str,
}

#[derive(Debug)]
struct HrxBoundary<'a> {
    start: usize,
    body_start: usize,
    path: Option<&'a str>,
}

fn parse_hrx(source: &str) -> Vec<HrxFile<'_>> {
    let mut boundaries = Vec::new();
    let mut offset = 0;

    for line in source.split_inclusive('\n') {
        let start = offset;
        offset += line.len();

        let header = line.trim_end_matches(['\r', '\n']);
        let Some(rest) = header.strip_prefix("<===>") else {
            continue;
        };
        let path = rest
            .strip_prefix(' ')
            .map(str::trim_start)
            .filter(|path| !path.is_empty());

        boundaries.push(HrxBoundary {
            start,
            body_start: offset,
            path,
        });
    }

    let mut files = Vec::new();
    for (index, boundary) in boundaries.iter().enumerate() {
        let Some(path) = boundary.path else {
            continue;
        };

        let mut body_end = boundaries
            .get(index + 1)
            .map_or(source.len(), |next| next.start);
        if boundaries.get(index + 1).is_some() && source[..body_end].ends_with('\n') {
            body_end -= 1;
            if source[..body_end].ends_with('\r') {
                body_end -= 1;
            }
        }

        files.push(HrxFile {
            path,
            body: &source[boundary.body_start..body_end],
        });
    }

    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_hrx_files_and_comments() {
        let archive = "<===> case/input.scss\na {b: c}\n\n<===>\ncomment\n<===> case/output.css\na {\n  b: c;\n}\n";
        let files = parse_hrx(archive);

        assert_eq!(
            files,
            vec![
                HrxFile {
                    path: "case/input.scss",
                    body: "a {b: c}\n",
                },
                HrxFile {
                    path: "case/output.css",
                    body: "a {\n  b: c;\n}\n",
                },
            ]
        );
    }

    #[test]
    fn collects_only_successful_scss_cases() {
        let archive = "<===> success/input.scss\na {b: c}\n<===> success/output.css\na {b: c}\n<===> error/input.scss\na {b:}\n<===> error/error\nError\n<===> dart-error/input.scss\na {b: c}\n<===> dart-error/output.css\na {b: c}\n<===> dart-error/error-dart-sass\nError\n<===> sass/input.sass\na\n  b: c\n<===> sass/output.css\na {b: c}\n";
        let mut files = BTreeMap::new();
        insert_hrx_files(&mut files, Path::new("suite"), archive).unwrap();

        let cases = collect_virtual_cases(files);
        assert_eq!(cases.keys().collect::<Vec<_>>(), vec!["suite/success"]);
        assert_eq!(cases["suite/success"].source, "a {b: c}");
    }

    #[test]
    fn preserves_byte_order_marks() {
        assert_eq!(
            decode_sass_source(&[0xef, 0xbb, 0xbf, b'a']).unwrap(),
            "\u{feff}a"
        );
        assert_eq!(
            decode_sass_source(&[0xff, 0xfe, b'a', 0]).unwrap(),
            "\u{feff}a"
        );
        assert_eq!(
            decode_sass_source(&[0xfe, 0xff, 0, b'a']).unwrap(),
            "\u{feff}a"
        );
        assert!(decode_sass_source(&[0xff, 0xfe, b'a']).is_err());
        assert!(decode_sass_source(&[0xfe, 0xff, 0]).is_err());
    }

    #[test]
    fn valid_case_passes() {
        let case = SassSpecTestCase::new("valid".to_string(), "$x: 1;".to_string());
        assert!(matches!(case.run(), TestRunOutcome::Passed(_)));
    }

    #[test]
    fn invalid_case_fails() {
        let case = SassSpecTestCase::new("invalid".to_string(), "a { color: ; }".to_string());
        assert!(matches!(
            case.run(),
            TestRunOutcome::IncorrectlyErrored { .. }
        ));
    }
}
