use biome_formatter::{IndentStyle, IndentWidth};
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{HtmlFormatLanguage, context::HtmlFormatOptions};
use biome_languages::{DocumentFileSource, HtmlFileSource};
use camino::Utf8Path;
use std::env;

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/**/*.{html,vue}", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Utf8Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let source_type: HtmlFileSource = test_file.input_file().try_into().unwrap();

    let options = HtmlFormatOptions::new(source_type)
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(IndentWidth::default())
        .with_self_close_void_elements(SelfCloseVoidElements::Always)
        // Prettier always indents in vanilla HTML
        .with_indent_script_and_style(source_type.is_html().into());

    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = PrettierSnapshot::new(test_file, language, HtmlFormatLanguage::new(options))
        .with_document_file_source(DocumentFileSource::from(source_type));

    snapshot.test()
}
