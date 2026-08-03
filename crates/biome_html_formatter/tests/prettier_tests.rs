use biome_configuration::{
    Configuration, HtmlConfiguration, formatter::FormatterConfiguration,
    html::HtmlFormatterConfiguration,
};
use biome_formatter::{IndentStyle, IndentWidth, Printed};
use biome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use biome_fs::{BiomePath, MemoryFileSystem};
use biome_html_formatter::context::SelfCloseVoidElements;
use biome_html_formatter::{HtmlFormatLanguage, context::HtmlFormatOptions};
use biome_languages::{DocumentFileSource, HtmlFileSource};
use biome_service::workspace::{
    FileContent, FormatFileParams, OpenFileParams, OpenProjectParams, OpenProjectResult,
    UpdateSettingsParams, server,
};
use camino::Utf8Path;
use std::env;
use std::sync::Arc;

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/**/*.{html,vue}", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Utf8Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let input_file = Utf8Path::new(input);
    let test_file = PrettierTestFile::new(input, root_path);
    let source_type: HtmlFileSource = input_file.try_into().unwrap();

    let options = HtmlFormatOptions::new(source_type)
        .with_indent_style(IndentStyle::Space)
        .with_indent_width(IndentWidth::default())
        .with_self_close_void_elements(SelfCloseVoidElements::Always);

    let language = language::HtmlTestFormatLanguage::new(source_type);

    let snapshot = PrettierSnapshot::new(test_file, language, HtmlFormatLanguage::new(options))
        .with_embed_formatter(Box::new(move |source| {
            format_through_workspace(source, source_type, input_file)
        }));

    snapshot.test()
}

/// Formats `source` through the workspace, which is the only caller that knows
/// how to hand the content of a `<script>` or a `<style>` to the formatter of
/// the language it is written in.
fn format_through_workspace(
    source: &str,
    source_type: HtmlFileSource,
    path: &Utf8Path,
) -> Result<Printed, String> {
    let workspace = server(Arc::new(MemoryFileSystem::default()), None);

    let OpenProjectResult { project_key } = workspace
        .open_project(OpenProjectParams {
            path: BiomePath::new(""),
            open_uninitialized: true,
        })
        .map_err(|err| err.to_string())?;

    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            configuration: configuration(),
            workspace_directory: None,
            extended_configurations: vec![],
            module_graph_resolution_kind: Default::default(),
        })
        .map_err(|err| err.to_string())?;

    let path = BiomePath::new(path);
    workspace
        .open_file(OpenFileParams {
            project_key,
            path: path.clone(),
            content: FileContent::FromClient {
                content: source.to_string(),
                version: 0,
            },
            document_file_source: Some(DocumentFileSource::from(source_type)),
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .map_err(|err| err.to_string())?;

    workspace
        .format_file(FormatFileParams {
            project_key,
            path,
            inline_config: None,
        })
        .map_err(|err| err.to_string())
}

/// The options a sample runs with, spelled the way the workspace takes them.
/// These have to say the same thing as the [`HtmlFormatOptions`] above, or the
/// host document and the snippets inside it come out formatted to two different
/// sets of options.
fn configuration() -> Configuration {
    Configuration {
        formatter: Some(FormatterConfiguration {
            indent_style: Some(IndentStyle::Space),
            indent_width: Some(IndentWidth::default()),
            // Plenty of the samples don't parse. The formatter reached through
            // `format_node` formats them anyway, so the workspace has to as
            // well, or the comparison stops where prettier's doesn't.
            format_with_errors: Some(true.into()),
            ..Default::default()
        }),
        html: Some(HtmlConfiguration {
            // Without this the workspace hands `.vue` to the JavaScript
            // handler, which knows nothing about the blocks a single-file
            // component is made of.
            experimental_full_support_enabled: Some(true.into()),
            formatter: Some(HtmlFormatterConfiguration {
                enabled: Some(true.into()),
                self_close_void_elements: Some(SelfCloseVoidElements::Always),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}
