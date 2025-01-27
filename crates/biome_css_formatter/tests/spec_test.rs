use biome_configuration::css::CssFormatterConfiguration;
use biome_configuration::{Configuration, CssConfiguration};
use biome_css_formatter::{context::CssFormatOptions, CssFormatLanguage};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_service::workspace::UpdateSettingsParams;
use camino::Utf8Path;

mod language {
    include!("language.rs");
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p biome_css_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.css` and the output file name is `{spec_name}.css.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `css/null` -> input: `tests/specs/css/null.css`, expected output: `tests/specs/css/null.css.snap`
/// * `null` -> input: `tests/specs/null.css`, expected output: `tests/specs/null.css.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));
    let settings = |project_key| {
        Some(UpdateSettingsParams {
            project_key,
            configuration: Configuration {
                css: Some(CssConfiguration {
                    formatter: Some(CssFormatterConfiguration {
                        enabled: Some(true.into()),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            vcs_base_path: None,
            gitignore_matches: vec![],
            workspace_directory: None,
        })
    };

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, settings) else {
        return;
    };

    let options = CssFormatOptions::default();
    let language = language::CssTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        CssFormatLanguage::new(options),
    );

    snapshot.test()
}
