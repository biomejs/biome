use super::parse_embedded_nodes::parse_snippet_code;
use super::{format, format_embeds_enabled, resolve_format_options};
use crate::WorkspaceError;
use crate::db::WorkspaceDb;
use crate::file_handlers::{
    ParsedOrigin, ParsedSnippetOrigin, css, graphql, grit, html, javascript, json, yaml,
};
use crate::settings::{ServiceLanguage, SettingsWithEditor};
use biome_css_formatter::CssFormatLanguage;
use biome_formatter::prelude::Document;
use biome_formatter::{FormatLanguage, FormatOptions, Printed, TrailingNewline};
use biome_fs::BiomePath;
use biome_graphql_formatter::GraphqlFormatLanguage;
use biome_grit_formatter::GritFormatLanguage;
use biome_html_formatter::HtmlFormatLanguage;
use biome_html_formatter::context::HtmlFormatOptions;
use biome_html_syntax::HtmlLanguage;
use biome_js_formatter::JsFormatLanguage;
use biome_json_formatter::JsonFormatLanguage;
use biome_languages::DocumentFileSource;
use biome_markdown_formatter::context::MdFormatOptions;
use biome_markdown_formatter::{MdFormatLanguage, format_node};
use biome_markdown_parser::{MarkdownParserOptions, parse_markdown};
use biome_markdown_syntax::{
    AnyMdBlock, AnyMdLeafBlock, MarkdownLanguage, MarkdownSyntaxKind, MarkdownSyntaxNode,
    MarkdownSyntaxToken, MdFencedCodeBlock,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, TextRange};
use biome_yaml_formatter::YamlFormatLanguage;
use rustc_hash::FxHashMap;

/// Formats a Markdown document together with the code of its snippets, such
/// as fenced code blocks, frontmatter, and HTML blocks.
///
/// Snippets stay as written when `markdown.formatter.formatEmbeds` is disabled
/// for the file, when they contain syntax errors, or when the formatter of
/// their language is disabled or unavailable.
pub(super) fn format_embedded(
    biome_path: &BiomePath,
    document_file_source: &DocumentFileSource,
    parse: ParsedOrigin,
    settings: &SettingsWithEditor,
    embedded_nodes: Vec<ParsedSnippetOrigin>,
    workspace_db: WorkspaceDb,
) -> Result<Printed, WorkspaceError> {
    if !format_embeds_enabled(settings.as_ref(), biome_path) {
        return format(
            biome_path,
            document_file_source,
            parse,
            settings,
            workspace_db,
        );
    }

    let options = resolve_format_options(biome_path, document_file_source, settings, &workspace_db);
    let tree = parse.syntax(&workspace_db);

    // The Markdown formatter writes a placeholder for each snippet, and the
    // closure below replaces it with the formatted code. A snippet that can't
    // be formatted keeps the Markdown formatter's output.
    let snippets: FxHashMap<TextRange, ParsedSnippetOrigin> = embedded_nodes
        .into_iter()
        .map(|snippet| (snippet.content_range(&workspace_db), snippet))
        .collect();
    let host_options = options.clone();
    let mut formatted = format_node(options, &tree, snippets.keys().copied().collect())?;
    formatted.format_embedded(move |range| {
        let snippet = snippets.get(&range)?;
        let file_source = snippet.file_source(&workspace_db)?;
        let host = SnippetHost {
            options: &host_options,
            is_html_block: tree
                .covering_element(range)
                .as_token()
                .is_some_and(|token| token.kind() == MarkdownSyntaxKind::MD_HTML_LITERAL),
        };
        let parse = match IndentedFenceCode::find(&tree, range) {
            Some(code) => parse_snippet_code(
                &code.without_indentation(),
                range.start(),
                file_source,
                biome_path,
                settings,
            )?,
            None => snippet.parsed_origin().parse(&workspace_db),
        };
        if parse.has_errors() {
            return None;
        }
        format_snippet(biome_path, file_source, parse, settings, &workspace_db, &host)
    });

    // Groups inside the inserted documents must propagate their expand flags.
    formatted.propagate_expand();

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

/// The code of a fenced code block whose opening fence is indented.
///
/// Markdown removes up to that many spaces from the start of each line of the
/// code, but the code of the snippet keeps them. The formatted code is written
/// without the indentation, so text that the formatter of the snippet's
/// language keeps as written, such as a multi-line template literal, must not
/// contain it either.
struct IndentedFenceCode {
    code: MarkdownSyntaxToken,
    indentation: usize,
}

impl IndentedFenceCode {
    /// Returns the code of the fenced code block whose code spans `range`, when
    /// the opening fence of the block is indented.
    fn find(root: &MarkdownSyntaxNode, range: TextRange) -> Option<Self> {
        let code = root.covering_element(range).into_token()?;
        let block = code.ancestors().find_map(MdFencedCodeBlock::cast)?;
        let indentation = block
            .indent()
            .iter()
            .map(|token| token.md_indent_char_token().map(|token| token.text().len()))
            .sum::<Result<usize, _>>()
            .ok()?;
        (indentation > 0).then_some(Self { code, indentation })
    }

    /// Returns the code as Markdown reads it, without the indentation of the
    /// fence. Its positions after the first line no longer match the file, so
    /// it's only suitable for formatting.
    fn without_indentation(&self) -> String {
        let text = self.code.text();
        let mut code = String::with_capacity(text.len());
        let mut rest = text;
        loop {
            let spaces = rest
                .bytes()
                .take(self.indentation)
                .take_while(|byte| *byte == b' ')
                .count();
            rest = &rest[spaces..];
            let Some(line_break) = rest.find(['\n', '\r']) else {
                code.push_str(rest);
                return code;
            };
            let line_end = if rest[line_break..].starts_with("\r\n") {
                line_break + 2
            } else {
                line_break + 1
            };
            code.push_str(&rest[..line_end]);
            rest = &rest[line_end..];
        }
    }
}

/// The Markdown document around a snippet.
struct SnippetHost<'a> {
    /// The formatter options of the Markdown document, whose printer prints
    /// the formatted snippet.
    options: &'a MdFormatOptions,
    /// Whether the snippet is the code of an HTML block.
    is_html_block: bool,
}

/// Formats the code of a snippet with the formatter of its language.
///
/// Returns `None` when that formatter is disabled for the Markdown file or the
/// language has no formatter. The document has no trailing line break, because
/// the Markdown formatter ends the snippet's last line.
fn format_snippet(
    path: &BiomePath,
    file_source: DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    workspace_db: &WorkspaceDb,
    host: &SnippetHost,
) -> Option<Document> {
    let trailing_newline = TrailingNewline::from(false);
    match file_source {
        DocumentFileSource::Js(_) => {
            let options =
                javascript::resolve_format_options(path, &file_source, settings, workspace_db)
                    .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, JsFormatLanguage::new(options))
        }
        DocumentFileSource::Json(_) => {
            let options = json::resolve_format_options(&file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, JsonFormatLanguage::new(options))
        }
        DocumentFileSource::Css(_) => {
            let options = css::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, CssFormatLanguage::new(options))
        }
        DocumentFileSource::Graphql(_) => {
            let options =
                graphql::resolve_format_options(path, &file_source, settings, workspace_db)
                    .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, GraphqlFormatLanguage::new(options))
        }
        DocumentFileSource::Html(_) => {
            let options = html::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            if host.is_html_block {
                format_html_block_snippet(path, settings, parse, options, host.options)
            } else {
                format_with_language(path, settings, parse, HtmlFormatLanguage::new(options))
            }
        }
        DocumentFileSource::Yaml(_) => {
            let options = yaml::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, YamlFormatLanguage::new(options))
        }
        DocumentFileSource::Markdown(_) => {
            let options = resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_markdown_snippet(path, settings, parse, MdFormatLanguage::new(options))
        }
        DocumentFileSource::Grit(_) => {
            let options = grit::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(path, settings, parse, GritFormatLanguage::new(options))
        }
        DocumentFileSource::Ignore | DocumentFileSource::Unknown => None,
    }
}

/// Formats `parse` with `language` when the formatter of that language is
/// enabled for the Markdown file at `path`.
fn format_with_language<F>(
    path: &BiomePath,
    settings: &SettingsWithEditor,
    parse: AnyParse,
    language: F,
) -> Option<Document>
where
    F: FormatLanguage,
    F::SyntaxLanguage: ServiceLanguage + 'static,
{
    if !settings.formatter_enabled_for_file_path::<F::SyntaxLanguage>(path) {
        return None;
    }
    let node = parse.embedded_syntax::<F::SyntaxLanguage>();
    let formatted = biome_formatter::format_node_with_offset(&node, language, false).ok()?;
    Some(formatted.into_document())
}

/// Formats Markdown written inside a fenced code block.
///
/// The fence of the enclosing block has at least three backticks and is longer
/// than the longest backtick sequence of the original code. The Markdown
/// formatter writes `~~~` fences with backticks and can lengthen fences, so the
/// formatted code can contain a backtick sequence that would close the
/// enclosing block. Such code stays as written.
fn format_markdown_snippet(
    path: &BiomePath,
    settings: &SettingsWithEditor,
    parse: AnyParse,
    language: MdFormatLanguage,
) -> Option<Document> {
    if !settings.formatter_enabled_for_file_path::<MarkdownLanguage>(path) {
        return None;
    }
    let node = parse.embedded_syntax::<MarkdownLanguage>();
    let fence_length =
        (longest_backtick_sequence(&node.node.text_with_trivia().to_string()) + 1).max(3);
    let formatted = biome_formatter::format_node_with_offset(&node, language, false).ok()?;
    let printed = formatted.print().ok()?;
    if longest_backtick_sequence(printed.as_code()) >= fence_length {
        return None;
    }
    Some(formatted.into_document())
}

/// Formats the HTML of an HTML block.
///
/// Markdown reads some HTML as a block only when its first line holds a lone
/// tag, such as `<span>`. The formatter can join such a tag with the text that
/// follows it, which turns the block into a paragraph. The HTML stays as written
/// when the formatted code isn't read as a single HTML block.
fn format_html_block_snippet(
    path: &BiomePath,
    settings: &SettingsWithEditor,
    parse: AnyParse,
    options: HtmlFormatOptions,
    host_options: &MdFormatOptions,
) -> Option<Document> {
    if !settings.formatter_enabled_for_file_path::<HtmlLanguage>(path) {
        return None;
    }
    // The printer of the Markdown document prints the formatted HTML, so the
    // check must break lines where that printer does.
    let options = options
        .with_indent_style(host_options.indent_style())
        .with_indent_width(host_options.indent_width())
        .with_line_width(host_options.line_width());
    let node = parse.embedded_syntax::<HtmlLanguage>();
    let formatted =
        biome_formatter::format_node_with_offset(&node, HtmlFormatLanguage::new(options), false)
            .ok()?;
    let printed = formatted.print().ok()?;
    let markdown = parse_markdown(printed.as_code(), MarkdownParserOptions::default()).tree();
    let mut blocks = markdown.value().iter();
    let is_html_block = matches!(
        (blocks.next(), blocks.next()),
        (
            Some(AnyMdBlock::AnyMdLeafBlock(AnyMdLeafBlock::MdHtmlBlock(_))),
            None
        )
    );
    is_html_block.then(|| formatted.into_document())
}

fn longest_backtick_sequence(text: &str) -> usize {
    text.split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or_default()
}
