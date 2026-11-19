use biome_configuration::css::CssFormatterConfiguration;
use biome_configuration::{Configuration, CssConfiguration};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_languages::{CssFileSource, DocumentFileSource};
use camino::Utf8Path;

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path) else {
        return;
    };

    let config = Configuration {
        css: Some(CssConfiguration {
            formatter: Some(CssFormatterConfiguration {
                enabled: Some(true.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let is_scss = test_file.file_name().ends_with(".scss");
    let mut snapshot = SpecSnapshot::new(test_file, test_directory, config);

    if is_scss {
        snapshot =
            snapshot.with_document_file_source(DocumentFileSource::Css(CssFileSource::scss()));
    }

    snapshot.test()
}
