use biome_configuration::{Configuration, HtmlConfiguration, html::HtmlFormatterConfiguration};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_html_syntax::HtmlFileSource;
use biome_service::workspace::DocumentFileSource;
use camino::Utf8Path;

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/html"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path) else {
        panic!("Failed to set up snapshot test");
    };

    let source_type: HtmlFileSource = test_file.input_file().as_path().try_into().unwrap();

    let config = Configuration {
        html: Some(HtmlConfiguration {
            formatter: Some(HtmlFormatterConfiguration {
                enabled: Some(true.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let snapshot = SpecSnapshot::new(test_file, test_directory, config)
        .with_document_file_source(DocumentFileSource::from(source_type));

    snapshot.test()
}
