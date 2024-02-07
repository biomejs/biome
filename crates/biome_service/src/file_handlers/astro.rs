use crate::file_handlers::{
    AnalyzerCapabilities, Capabilities, DebugCapabilities, ExtensionHandler, FormatterCapabilities,
    Language, Mime, ParserCapabilities,
};
use crate::settings::SettingsHandle;
use crate::WorkspaceError;
use biome_formatter::Printed;
use biome_fs::RomePath;
use biome_js_formatter::format_node;
use biome_js_parser::{parse_js_with_cache, JsParserOptions};
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_parser::AnyParse;
use biome_rowan::{FileSource, NodeCache};
use lazy_static::lazy_static;
use regex::{Regex, RegexBuilder};
use tracing::{debug, error, info};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct AstroFileHandler;

lazy_static! {
    pub static ref ASTRO_FENCE: Regex = RegexBuilder::new(r#"^---\s*$"#)
        .multi_line(true)
        .build()
        .unwrap();
}

impl ExtensionHandler for AstroFileHandler {
    fn language(&self) -> Language {
        Language::TypeScript
    }

    fn mime(&self) -> Mime {
        Mime::Javascript
    }

    fn capabilities(&self) -> Capabilities {
        Capabilities {
            parser: ParserCapabilities { parse: Some(parse) },
            debug: DebugCapabilities {
                debug_syntax_tree: None,
                debug_control_flow: None,
                debug_formatter_ir: None,
            },
            analyzer: AnalyzerCapabilities {
                lint: None,
                code_actions: None,
                rename: None,
                fix_all: None,
                organize_imports: None,
            },
            formatter: FormatterCapabilities {
                format: Some(format),
                format_range: None,
                format_on_type: None,
            },
        }
    }
}

fn parse(
    _rome_path: &RomePath,
    _language_hint: crate::file_handlers::Language,
    text: &str,
    _settings: SettingsHandle,
    cache: &mut NodeCache,
) -> AnyParse {
    let mut matches = ASTRO_FENCE.find_iter(text);
    let frontmatter = match (matches.next(), matches.next()) {
        (Some(start), Some(end)) => &text[start.end()..end.start()],
        _ => "",
    };
    let parse = parse_js_with_cache(
        frontmatter,
        JsFileSource::ts(),
        JsParserOptions::default(),
        cache,
    );
    let root = parse.syntax();
    let diagnostics = parse.into_diagnostics();

    AnyParse::new(
        // SAFETY: the parser should always return a root node
        root.as_send().unwrap(),
        diagnostics,
        JsFileSource::ts().as_any_file_source(),
    )
}

#[tracing::instrument(level = "trace", skip(parse, settings))]
fn format(
    rome_path: &RomePath,
    parse: AnyParse,
    settings: SettingsHandle,
) -> Result<Printed, WorkspaceError> {
    let options = settings.format_options::<JsLanguage>(rome_path);
    debug!("Options used for format: \n{}", options);

    let tree = parse.syntax();
    info!("Format file {}", rome_path.display());
    let formatted = format_node(options, &tree)?;
    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => {
            error!("The file {} couldn't be formatted", rome_path.display());
            Err(WorkspaceError::FormatError(error.into()))
        }
    }
}
