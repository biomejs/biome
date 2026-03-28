use biome_configuration::Configuration;
use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_js_syntax::{JsFileSource, LanguageVariant, ModuleKind};
use biome_service::workspace::DocumentFileSource;
use camino::Utf8Path;

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path) else {
        return;
    };

    let mut source_type: JsFileSource = test_file.input_file().as_path().try_into().unwrap();
    if file_type != "module" {
        source_type = source_type.with_module_kind(ModuleKind::Script);
    }
    if !source_type.is_typescript() {
        source_type.set_variant(LanguageVariant::Jsx);
    }

    let snapshot = SpecSnapshot::new(test_file, test_directory, Configuration::default())
        .with_document_file_source(DocumentFileSource::from(source_type));

    snapshot.test()
}
