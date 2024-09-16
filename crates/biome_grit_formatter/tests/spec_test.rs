use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use std::path::Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, Some(settings))
    else {
        return;
    };

    let language = language::GritTestFormatLanguage::default();

    let snapshot = SpecSnapshot::new(test_file, test_directory, language, ());

    snapshot.test()
}
