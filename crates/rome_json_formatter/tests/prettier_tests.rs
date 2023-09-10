use std::{env, path::Path};

use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use rome_formatter::IndentStyle;
use rome_json_formatter::context::JsonFormatOptions;

#[derive(serde::Serialize)]
struct TestInfo {
    test_file: String,
}

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/{json}/**/*.{json}", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let options = JsonFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(2.into());
    let language = language::JsonTestFormatLanguage::default();
    let snapshot = PrettierSnapshot::new(test_file, language, options);

    snapshot.test()
}
