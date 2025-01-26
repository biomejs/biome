use biome_configuration::PartialConfiguration;
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_fs::BiomePath;
use biome_deserialize::json::deserialize_from_str;
use biome_diagnostics::print_diagnostic_to_string;
use biome_html_formatter::{context::HtmlFormatOptions, HtmlFormatLanguage};
use biome_html_syntax::HtmlFileSource;
use biome_service::settings::Settings;
use std::path::Path;

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
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/html"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, None) else {
        panic!("Failed to set up snapshot test");
    };

    let source_type: HtmlFileSource = test_file.input_file().as_path().try_into().unwrap();

    let mut options = HtmlFormatOptions::new(HtmlFileSource::html());

    let options_path = Path::new(test_directory).join("options.json");
    if options_path.exists() {
        let mut options_path = BiomePath::new(&options_path);

        let mut settings = Settings::default();
        let (test_options, diagnostics) = deserialize_from_str::<PartialConfiguration>(
            options_path.get_buffer_from_file().as_str(),
        )
        .consume();

        settings.merge_with_configuration(test_options.unwrap_or_default(), None, None, &[]).unwrap();

        let settings = settings.formatter;

        if let Some(attribute_position) = settings.attribute_position {
            options = options.with_attribute_position(attribute_position);
        }

        if !diagnostics.is_empty() {
            for diagnostic in diagnostics {
                println!("{:?}", print_diagnostic_to_string(&diagnostic));
            }

            panic!("Configuration is invalid");
        }
    }

    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        HtmlFormatLanguage::new(options),
    );

    snapshot.test()
}
