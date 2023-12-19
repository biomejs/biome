use std::{env, path::Path};

use biome_css_formatter::context::CssFormatOptions;
use biome_formatter::IndentStyle;
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};

#[derive(serde::Serialize)]
struct TestInfo {
    test_file: String,
}

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/{css}/**/*.{css}", crate::test_snapshot, ""}

// TODO: Re-add CSS Prettier tests once we figure out if/how to handle PostCSS,
// and once the parser is sufficiently there to parse them all successfully.
#[allow(unused)]
fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let options = CssFormatOptions::default()
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(2.into());
    let language = language::CssTestFormatLanguage::default();
    let snapshot = PrettierSnapshot::new(test_file, language, options);

    snapshot.test()
}
