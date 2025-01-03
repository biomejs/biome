use biome_formatter_test::spec::{SpecSnapshot, SpecTestFile};
use biome_grit_formatter::{context::GritFormatOptions, GritFormatLanguage};
use camino::Utf8Path;

mod language {
    include!("language.rs");
}

pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, _file_type: &str) {
    let root_path = Utf8Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/specs/"));

    let Some(test_file) = SpecTestFile::try_from_file(spec_input_file, root_path, |_| None) else {
        panic!("Failed to set up snapshot test");
    };

    let options = GritFormatOptions::default();
    let language = language::GritTestFormatLanguage;

    let snapshot = SpecSnapshot::new(
        test_file,
        test_directory,
        language,
        GritFormatLanguage::new(options),
    );

    snapshot.test()
}
