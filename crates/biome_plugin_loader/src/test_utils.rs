use std::path::Path;

use biome_js_formatter::{context::JsFormatOptions, format_node};
use biome_js_parser::{JsParserOptions, parse};
use biome_languages::JsFileSource;

pub(crate) fn snapshot_content(
    plugin_sources: &[(&str, &str)],
    input_sources: &[(&str, &str)],
    diagnostics: &str,
) -> String {
    let mut content = String::new();
    for (heading, sources, empty) in [
        ("Plugin", plugin_sources, "No plugin source was loaded."),
        ("Input", input_sources, "No input was analyzed."),
    ] {
        content.push_str(&format!("# {heading}\n\n"));
        if sources.is_empty() {
            content.push_str(empty);
            content.push_str("\n\n");
        }
        for (path, source) in sources {
            let path = path.replace('\\', "/");
            let language = Path::new(&path)
                .extension()
                .and_then(|extension| extension.to_str())
                .unwrap_or_default();
            content.push_str(&format!("## `{path}`\n\n```{language}\n"));
            let formatted = if heading == "Plugin" {
                JsFileSource::try_from_extension(language)
                    .ok()
                    .and_then(|file_source| {
                        let parsed = parse(source, file_source, JsParserOptions::default());
                        format_node(JsFormatOptions::new(file_source), &parsed.syntax(), vec![])
                            .ok()
                            .and_then(|formatted| formatted.print().ok())
                            .map(|printed| printed.into_code())
                    })
            } else {
                None
            };
            let source = formatted.as_deref().unwrap_or(source);
            content.push_str(source);
            if !source.ends_with('\n') {
                content.push('\n');
            }
            content.push_str("```\n\n");
        }
    }
    content.push_str("# Diagnostics\n\n```block\n");
    content.push_str(diagnostics.trim_end());
    content.push_str("\n```\n");
    content
}

#[test]
fn formats_only_plugin_sources() {
    let source = "import { ast } from 'api';\n            export const query = ast('JS_MODULE');";
    let content = snapshot_content(
        &[("plugin.js", source)],
        &[("input.js", "  call(1,2);  ")],
        "diagnostic",
    );
    assert!(
        content.contains("import { ast } from \"api\";\nexport const query = ast(\"JS_MODULE\");")
    );
    assert!(content.contains("```js\n  call(1,2);  \n```"));
}

#[test]
fn empty_sources() {
    assert_eq!(
        snapshot_content(&[], &[], "diagnostic\n"),
        "# Plugin\n\nNo plugin source was loaded.\n\n\
         # Input\n\nNo input was analyzed.\n\n\
         # Diagnostics\n\n```block\ndiagnostic\n```\n"
    );
}

#[test]
fn preserves_sources_and_diagnostics() {
    assert_eq!(
        snapshot_content(
            &[
                (r"rules\rule.grit", "`\\hello`\n"),
                ("biome-manifest.jsonc", "{}")
            ],
            &[
                (r"src\input.js", "\t'\\n';  \r\n\n"),
                ("input.ts", "let x: string;")
            ],
            "diagnostic \\path",
        ),
        "# Plugin\n\n## `rules/rule.grit`\n\n```grit\n`\\hello`\n```\n\n\
         ## `biome-manifest.jsonc`\n\n```jsonc\n{}\n```\n\n\
         # Input\n\n## `src/input.js`\n\n```js\n\t'\\n';  \r\n\n```\n\n\
         ## `input.ts`\n\n```ts\nlet x: string;\n```\n\n\
         # Diagnostics\n\n```block\ndiagnostic \\path\n```\n"
    );
}
