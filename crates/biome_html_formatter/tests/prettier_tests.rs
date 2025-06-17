use biome_formatter::{IndentStyle, IndentWidth};
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{HtmlFormatLanguage, context::HtmlFormatOptions};
use biome_html_syntax::HtmlFileSource;
use camino::Utf8Path;
use std::env;

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/**/*.html", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Utf8Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let source_type = HtmlFileSource::html();

    let options = HtmlFormatOptions::new(HtmlFileSource::html())
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(IndentWidth::default())
        .with_self_close_void_elements(SelfCloseVoidElements::Always);

    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = PrettierSnapshot::new(test_file, language, HtmlFormatLanguage::new(options));

    snapshot.test()
}
