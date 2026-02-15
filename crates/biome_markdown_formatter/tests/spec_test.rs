use biome_configuration::{
    Configuration,
    markdown::{MarkdownConfiguration, MarkdownFormatterConfiguration},
};
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_fs::BiomePath;
use biome_markdown_formatter::{MarkdownFormatLanguage, context::MarkdownFormatOptions};
use biome_service::workspace::UpdateSettingsParams;
use camino::Utf8Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, |project_key| {
        Some(UpdateSettingsParams {
            configuration: Configuration {
                markdown: Some(MarkdownConfiguration {
                    formatter: Some(MarkdownFormatterConfiguration {
                        enabled: Some(true.into()),
                        ..Default::default()
                    }),
                }),
                ..Default::default()
            },
            project_key,
            workspace_directory: Some(BiomePath::new(test_directory)),
            extended_configurations: vec![],
            module_graph_resolution_kind: Default::default(),
        })
    }) else {
        return;
    };

    let options = MarkdownFormatOptions::default();
    let language = language::MarkdownTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        MarkdownFormatLanguage::new(options),
    );

    snapshot.test()
}
