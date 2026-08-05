//! Runs the imported `prettier-plugin-svelte` suite.
//!
//! The suite is laid out like our other Prettier comparisons: an input, and a
//! `.prettier-snap` holding what the plugin produces for it. A `.snap` appears
//! next to any sample we do not yet reproduce, and records the difference.
//!
//! See `scripts/update-svelte-plugin-tests.mjs` for how the suite is imported.

use biome_formatter::{AttributePosition, IndentStyle, IndentWidth, LineWidth};
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use biome_html_formatter::context::{
    IndentScriptAndStyle, SelfCloseVoidElements, WhitespaceSensitivity,
};
use biome_html_formatter::{HtmlFormatLanguage, context::HtmlFormatOptions};
use biome_languages::HtmlFileSource;
use camino::Utf8Path;
use std::env;
use std::fs::read_to_string;

mod language;

tests_macros::gen_tests! {"tests/specs/svelte-plugin/**/*.svelte", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Utf8Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/svelte-plugin/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let source_type: HtmlFileSource = test_file.input_file().try_into().unwrap();

    let options = plugin_options(test_file.input_file());
    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = PrettierSnapshot::new(test_file, language, HtmlFormatLanguage::new(options));

    snapshot.test()
}

/// Builds the options a sample runs with.
///
/// The plugin's own test drivers pass `tabWidth: 4` and leave everything else
/// at its default, then merge in the sample's `options.json`. Options we have
/// no equivalent for, such as `svelteSortOrder`, are ignored; the sample then
/// shows up as a difference rather than silently passing for the wrong reason.
fn plugin_options(input_file: &Utf8Path) -> HtmlFormatOptions {
    let mut options = HtmlFormatOptions::new(HtmlFileSource::svelte())
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(IndentWidth::try_from(4).expect("4 is a valid indent width"))
        .with_self_close_void_elements(SelfCloseVoidElements::Always)
        .with_indent_script_and_style(IndentScriptAndStyle::from(true));

    let Some(overrides) = read_sample_options(input_file) else {
        return options;
    };

    if let Some(tab_width) = overrides["tabWidth"].as_u64()
        && let Ok(indent_width) = IndentWidth::try_from(tab_width as u8)
    {
        options = options.with_indent_width(indent_width);
    }
    if let Some(use_tabs) = overrides["useTabs"].as_bool() {
        options = options.with_indent_style(if use_tabs {
            IndentStyle::Tab
        } else {
            IndentStyle::Space
        });
    }
    if let Some(print_width) = overrides["printWidth"].as_u64()
        && let Ok(line_width) = LineWidth::try_from(print_width as u16)
    {
        options = options.with_line_width(line_width);
    }
    if let Some(bracket_same_line) = overrides["bracketSameLine"].as_bool() {
        options = options.with_bracket_same_line(bracket_same_line.into());
    }
    if let Some(true) = overrides["singleAttributePerLine"].as_bool() {
        options = options.with_attribute_position(AttributePosition::Multiline);
    }
    if let Some(sensitivity) = overrides["htmlWhitespaceSensitivity"].as_str()
        && let Ok(sensitivity) = sensitivity.parse::<WhitespaceSensitivity>()
    {
        options = options.with_whitespace_sensitivity(sensitivity);
    }
    if let Some(indent) = overrides["svelteIndentScriptAndStyle"].as_bool() {
        options = options.with_indent_script_and_style(IndentScriptAndStyle::from(indent));
    }

    options
}

fn read_sample_options(input_file: &Utf8Path) -> Option<serde_json::Value> {
    let options_file = input_file.with_extension("options.json");
    let contents = read_to_string(options_file).ok()?;
    serde_json::from_str(&contents).ok()
}
