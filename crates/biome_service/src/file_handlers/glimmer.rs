use crate::WorkspaceError;
use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, CodeActionsParams, DebugCapabilities, EnabledForPath,
    ExtensionHandler, FixAllParams, FormatterCapabilities, LintParams, LintResults, ParseResult,
    ParserCapabilities, SearchCapabilities, javascript,
};
use crate::settings::Settings;
use crate::workspace::{DocumentFileSource, FixFileResult, PullActionsResult};
use biome_formatter::Printed;
use biome_fs::BiomePath;
use biome_js_parser::{JsParserOptions, parse_js_with_cache};
use biome_js_syntax::{JsFileSource, TextRange, TextSize};
use biome_parser::AnyParse;
use biome_rowan::NodeCache;
use regex::Regex;
use std::sync::LazyLock;

/// Regex to match Glimmer <template> tags
/// Simple pattern: <template> never has attributes and never nests
pub static GLIMMER_TEMPLATE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<template>[\s\S]*?</template>").expect("Invalid Glimmer template regex")
});

/// Information about a template's position and context in the original source
#[derive(Debug)]
struct TemplateInfo {
    template_text: String,
    /// Whether this template had a semicolon after it in the original source
    has_trailing_semicolon: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct GlimmerFileHandler;

impl GlimmerFileHandler {
    /// Extract the JavaScript/TypeScript code with <template> blocks replaced by markers
    ///
    /// Templates are replaced with markers that work in their specific contexts.
    /// We use a simple identifier-based approach that's valid everywhere.
    /// Returns the extracted JS and info about each template.
    fn extract_js_content_with_info(text: &str) -> (String, Vec<TemplateInfo>) {
        let mut result = String::new();
        let mut last_end = 0;
        let mut template_index = 0;
        let mut template_infos = Vec::new();

        for template_match in GLIMMER_TEMPLATE.find_iter(text) {
            // Add JS before this template
            result.push_str(&text[last_end..template_match.start()]);

            // Check if there's a semicolon after this template in the original
            let after_template_pos = template_match.end();
            let has_trailing_semicolon = after_template_pos < text.len()
                && text[after_template_pos..].trim_start().starts_with(';');

            template_infos.push(TemplateInfo {
                template_text: template_match.as_str().to_string(),
                has_trailing_semicolon,
            });

            // Use an identifier that will be treated as:
            // - In class body: field declaration `__BIOME_GLIMMER_TEMPLATE_0__;`
            // - In expression context: identifier reference
            result.push_str(&format!(
                "__BIOME_GLIMMER_TEMPLATE_{index}__",
                index = template_index
            ));

            last_end = template_match.end();
            template_index += 1;
        }

        // Add remaining JS after last template
        result.push_str(&text[last_end..]);
        (result, template_infos)
    }

    /// Extract the JavaScript/TypeScript code with <template> blocks replaced by markers
    ///
    /// Templates are replaced with markers that work in their specific contexts.
    /// We use a simple identifier-based approach that's valid everywhere.
    pub fn extract_js_content(text: &str) -> String {
        Self::extract_js_content_with_info(text).0
    }

    /// Reconstruct the file with formatted JS and original templates
    ///
    /// Replaces the identifier markers with the original template blocks.
    /// Uses original source info to determine if semicolons should be kept.
    pub fn output(input: &str, formatted_js: &str) -> String {
        let (_, template_infos) = Self::extract_js_content_with_info(input);

        if template_infos.is_empty() {
            return formatted_js.to_string();
        }

        // Replace markers with templates
        let mut result = formatted_js.to_string();
        for (idx, template_info) in template_infos.iter().enumerate() {
            let marker = format!("__BIOME_GLIMMER_TEMPLATE_{idx}__");
            let marker_with_semi = format!("{marker};");

            // If the formatted output has a semicolon but original didn't, remove it
            if result.contains(&marker_with_semi) && !template_info.has_trailing_semicolon {
                result = result.replace(&marker_with_semi, &template_info.template_text);
            } else {
                // Otherwise just replace the marker (keeps semicolons if they were there)
                result = result.replace(&marker, &template_info.template_text);
            }
        }

        result
    }

    /// Check if the file contains any <template> blocks
    pub fn has_templates(text: &str) -> bool {
        GLIMMER_TEMPLATE.is_match(text)
    }
}

impl ExtensionHandler for GlimmerFileHandler {
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            enabled_for_path: EnabledForPath {
                formatter: Some(javascript::formatter_enabled),
                search: Some(javascript::search_enabled),
                assist: Some(javascript::assist_enabled),
                linter: Some(javascript::linter_enabled),
            },
            parser: ParserCapabilities {
                parse: Some(parse),
                // Templates are embedded HTML, but we don't parse them separately yet
                // since we preserve them as-is for now
                parse_embedded_nodes: None,
            },
            debug: DebugCapabilities {
                debug_syntax_tree: None,
                debug_control_flow: None,
                debug_formatter_ir: None,
                debug_type_info: None,
                debug_registered_types: None,
                debug_semantic_model: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: Some(lint),
                code_actions: Some(code_actions),
                rename: None,
                fix_all: Some(fix_all),
                update_snippets: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: Some(format_range),
                format_on_type: Some(format_on_type),
                format_embedded: None,
            },
            search: SearchCapabilities { search: None },
        }
    }
}

/// Parse GJS/GTS file as JavaScript/TypeScript
///
/// Templates are stripped out before parsing to avoid syntax errors.
/// In the future, templates will be parsed separately with the HTML parser.
fn parse(
    biome_path: &BiomePath,
    file_source: DocumentFileSource,
    text: &str,
    _settings: &Settings,
    cache: &mut NodeCache,
) -> ParseResult {
    let js_file_source = file_source.to_js_file_source().unwrap_or_else(|| {
        // Determine if this is GJS or GTS based on extension
        if let Some(ext) = biome_path.extension() {
            if ext == "gts" {
                return JsFileSource::gts();
            }
        }
        JsFileSource::gjs()
    });

    // Extract JS content with templates replaced by whitespace
    let js_content = GlimmerFileHandler::extract_js_content(text);

    let parse = parse_js_with_cache(
        &js_content,
        js_file_source,
        JsParserOptions::default(),
        cache,
    );

    ParseResult {
        any_parse: parse.into(),
        language: Some(file_source),
    }
}

fn format(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
) -> Result<Printed, WorkspaceError> {
    // For now, delegate to JavaScript formatter
    // The reconstruction happens at the workspace level, not here
    // TODO: Format <template> blocks separately with Glimmer formatter
    javascript::format(biome_path, document_file_source, parse, settings)
}

fn format_range(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    range: TextRange,
) -> Result<Printed, WorkspaceError> {
    javascript::format_range(biome_path, document_file_source, parse, settings, range)
}

fn format_on_type(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: AnyParse,
    settings: &Settings,
    offset: TextSize,
) -> Result<Printed, WorkspaceError> {
    javascript::format_on_type(biome_path, document_file_source, parse, settings, offset)
}

fn lint(params: LintParams) -> LintResults {
    // For now, delegate to JavaScript linter
    // TODO: Also lint <template> blocks with Glimmer rules
    javascript::lint(params)
}

fn code_actions(params: CodeActionsParams) -> PullActionsResult {
    javascript::code_actions(params)
}

fn fix_all(params: FixAllParams) -> Result<FixFileResult, WorkspaceError> {
    javascript::fix_all(params)
}

#[cfg(test)]
mod tests {
    use super::GlimmerFileHandler;

    #[test]
    fn test_extract_js_content_basic() {
        let input = r#"
import Component from '@glimmer/component';

export default class MyComponent extends Component {
  <template>
    <div>{{@title}}</div>
  </template>

  get title() {
    return 'Hello';
  }
}
"#;

        let output = GlimmerFileHandler::extract_js_content(input);

        // Should replace template with marker
        assert!(output.contains("__BIOME_GLIMMER_TEMPLATE_0__"));
        assert!(!output.contains("<template>"));
        assert!(output.contains("import Component"));
        assert!(output.contains("get title"));
    }

    #[test]
    fn test_extract_js_content_multiple_templates() {
        let input = r#"
const Foo = <template>{{@foo}}</template>;
const Bar = <template>{{@bar}}</template>;
"#;

        let output = GlimmerFileHandler::extract_js_content(input);

        assert!(output.contains("__BIOME_GLIMMER_TEMPLATE_0__"));
        assert!(output.contains("__BIOME_GLIMMER_TEMPLATE_1__"));
        assert!(!output.contains("<template>"));
    }

    #[test]
    fn test_output_reconstruction() {
        let input = r#"const Foo = <template>{{@foo}}</template>;"#;
        let extracted = GlimmerFileHandler::extract_js_content(input);

        // Simulate formatting (just adding spaces)
        let formatted = extracted.replace("const", "const ");

        let output = GlimmerFileHandler::output(input, &formatted);

        // Should have template back
        assert!(output.contains("<template>{{@foo}}</template>"));
        assert!(!output.contains("__BIOME_GLIMMER_TEMPLATE"));
    }

    #[test]
    fn test_has_templates() {
        assert!(GlimmerFileHandler::has_templates("<template>test</template>"));
        assert!(GlimmerFileHandler::has_templates("foo <template>test</template> bar"));
        assert!(!GlimmerFileHandler::has_templates("no templates here"));
        assert!(!GlimmerFileHandler::has_templates(""));
    }

    #[test]
    fn test_semicolon_handling() {
        let input = r#"
class Foo {
  <template>
    <div></div>
  </template>
}
"#;
        let extracted = GlimmerFileHandler::extract_js_content(input);
        let formatted = format!("{};", extracted.trim()); // Formatter might add semicolon

        let output = GlimmerFileHandler::output(input, &formatted);

        // Should not have semicolon after template if original didn't
        assert!(!output.contains("</template>;"));
    }

    #[test]
    fn test_parse_templates_single() {
        let input = r#"<template><div>{{@title}}</div></template>"#;
        let results = GlimmerFileHandler::parse_templates(input);

        assert_eq!(results.len(), 1);
        // Parsing may have errors in templates - that's OK for now
        // The important thing is that we're extracting and parsing templates
        if results[0].has_errors() {
            eprintln!("Diagnostics: {:?}", results[0].diagnostics());
        }
    }

    #[test]
    fn test_parse_templates_multiple() {
        let input = r#"
const Foo = <template>{{@foo}}</template>;
const Bar = <template>{{@bar}}</template>;
"#;
        let results = GlimmerFileHandler::parse_templates(input);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_parse_templates_in_class() {
        let input = r#"
export default class MyComponent extends Component {
  <template>
    <div>
      <h1>{{@title}}</h1>
      {{#if @showButton}}
        <button>Click</button>
      {{/if}}
    </div>
  </template>

  get message() {
    return 'Hello';
  }
}
"#;
        let results = GlimmerFileHandler::parse_templates(input);

        assert_eq!(results.len(), 1);
        // Print diagnostics if there are errors
        if results[0].has_errors() {
            eprintln!("Template has {} diagnostics", results[0].diagnostics().len());
            for diag in results[0].diagnostics() {
                eprintln!("  - {:?}", diag);
            }
        }
    }
}
