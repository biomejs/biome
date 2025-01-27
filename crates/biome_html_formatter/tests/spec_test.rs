use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_html_formatter::{context::HtmlFormatOptions, HtmlFormatLanguage};
use biome_html_syntax::HtmlFileSource;
use camino::Utf8Path;

mod language {
    include!("language.rs");
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p biome_js_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.json` and the output file name is `{spec_name}.json.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `json/null` -> input: `tests/specs/json/null.json`, expected output: `tests/specs/json/null.json.snap`
/// * `null` -> input: `tests/specs/null.json`, expected output: `tests/specs/null.json.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/html"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, |_| None) else {
        panic!("Failed to set up snapshot test");
    };

    let source_type: HtmlFileSource = test_file.input_file().as_path().try_into().unwrap();

    let options = HtmlFormatOptions::new(HtmlFileSource::html());
    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        HtmlFormatLanguage::new(options),
    );

    snapshot.test()
}
