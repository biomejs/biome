use biome_css_formatter::context::CssFormatOptions;
use biome_css_parser::{parse_css, CssParserOptions};
use biome_formatter::FormatError;
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_js_formatter::{
    context::JsFormatOptions, JsForeignLanguage, JsForeignLanguageFormatter, JsFormatLanguage,
};
use biome_js_syntax::{JsFileSource, ModuleKind};
use std::path::Path;

mod language {
    include!("language.rs");
}

#[derive(Debug, Clone)]
struct MultiLanguageFormatter;

impl JsForeignLanguageFormatter for MultiLanguageFormatter {
    fn format(
        &self,
        language: biome_js_formatter::JsForeignLanguage,
        source: &str,
    ) -> biome_formatter::FormatResult<biome_formatter::prelude::Document> {
        let css_parse_options = CssParserOptions::default().allow_grit_metavariables();
        let css_format_options = CssFormatOptions::default();
        match language {
            JsForeignLanguage::Css => {
                let parse = parse_css(source, css_parse_options);
                if parse.has_errors() {
                    return Err(FormatError::SyntaxError);
                }
                biome_css_formatter::format_node(css_format_options, &parse.syntax())
                    .map(|formatted| formatted.into_document())
            }
        }
    }
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
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, file_type: &str) {
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, None) else {
        return;
    };

    let mut source_type: JsFileSource = test_file.input_file().as_path().try_into().unwrap();
    if file_type != "module" {
        source_type = source_type.with_module_kind(ModuleKind::Script);
    }

    let options = JsFormatOptions::new(source_type);
    let language = language::JsTestFormatLanguage::new(source_type);

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        JsFormatLanguage::new(options, MultiLanguageFormatter),
    );

    snapshot.test()
}
