use super::parse_embedded_nodes::parse_snippet_code;
use super::{format, format_embeds_enabled, resolve_format_options};
use crate::WorkspaceError;
use crate::db::WorkspaceDb;
use crate::file_handlers::{
    ParsedOrigin, ParsedSnippetOrigin, css, graphql, grit, html, javascript, json, yaml,
};
use crate::settings::{ServiceLanguage, SettingsWithEditor};
use biome_css_formatter::CssFormatLanguage;
use biome_formatter::rowan::Printed;
use biome_formatter::rowan::prelude::{
    Document, Formatter, RowanSource, literal_line_break_without_parent, text,
};
use biome_formatter::{
    Buffer, Format, FormatContext, FormatLanguage, FormatResult, SimpleFormatContext,
    SimpleFormatOptions, TrailingNewline, normalize_newlines, write,
};
use biome_fs::BiomePath;
use biome_graphql_formatter::GraphqlFormatLanguage;
use biome_grit_formatter::GritFormatLanguage;
use biome_html_formatter::HtmlFormatLanguage;
use biome_js_formatter::JsFormatLanguage;
use biome_json_formatter::JsonFormatLanguage;
use biome_languages::DocumentFileSource;
use biome_markdown_formatter::{MdFormatLanguage, format_node};
use biome_markdown_syntax::{
    MarkdownLanguage, MarkdownSyntaxKind, MarkdownSyntaxNode, MarkdownSyntaxToken,
    MdFencedCodeBlock,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, TextRange, TextSize};
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
    let snippets: FxHashMap<_, _> = embedded_nodes
        .into_iter()
        .filter(|snippet| !is_multiline_mdx_snippet_in_container(&tree, snippet.content_range()))
        .map(|snippet| (snippet.content_range(), snippet))
        .collect();
    let mut formatted = format_node(options, &tree, snippets.keys().copied().collect())?;
    formatted.format_embedded(move |range| {
        let snippet = snippets.get(&range)?;
        let file_source = snippet.file_source(&workspace_db)?;
        // This is needed due to the parsed representation of code blocks (mostly rowan), so we need
        // to allocate a new string to replace the previous code.
        let parse = if matches!(file_source, DocumentFileSource::Markdown(_)) {
            parse_markdown_fence_code(&tree, range, file_source, biome_path, settings)?
        } else {
            match IndentedFenceCode::from_range(&tree, range) {
                Some(code) => parse_snippet_code(
                    &code.without_indentation(),
                    range.start(),
                    file_source,
                    biome_path,
                    settings,
                )?,
                None => snippet.parsed_origin().parse(&workspace_db),
            }
        };
        if parse.has_errors() {
            return None;
        }
        let single_line = requires_single_line_output(&tree, range);
        format_snippet(
            biome_path,
            file_source,
            parse,
            settings,
            &workspace_db,
            single_line,
        )
    });

    match formatted.print() {
        Ok(printed) => Ok(printed),
        Err(error) => Err(WorkspaceError::FormatError(error.into())),
    }
}

/// Returns whether the formatted code of the snippet at `range` must fit on
/// one line: the JavaScript of an inline MDX construct, or of any MDX
/// construct inside a list item or a block quote. A line break would split a
/// paragraph, or start a line without the indentation or `>` markers of its
/// container.
fn requires_single_line_output(root: &MarkdownSyntaxNode, range: TextRange) -> bool {
    let Some(token) = root.token_at_offset(range.start()).right_biased() else {
        return false;
    };
    let is_mdx_literal = matches!(
        token.kind(),
        MarkdownSyntaxKind::MDX_EXPRESSION_LITERAL | MarkdownSyntaxKind::MDX_JSX_TAG_LITERAL
    );
    is_mdx_literal
        && token.ancestors().any(|node| {
            matches!(
                node.kind(),
                MarkdownSyntaxKind::MDX_INLINE_EXPRESSION
                    | MarkdownSyntaxKind::MDX_JSX_INLINE_TAG
                    | MarkdownSyntaxKind::MD_QUOTE
                    | MarkdownSyntaxKind::MD_BULLET_LIST_ITEM
                    | MarkdownSyntaxKind::MD_ORDERED_LIST_ITEM
            )
        })
}

/// Returns whether the MDX literal at `range` spans lines inside a list item
/// or a block quote. Its lines carry the indentation or the `>` markers of
/// their container, which the formatted JavaScript would lose after its
/// first line, so such a literal stays as written.
fn is_multiline_mdx_snippet_in_container(root: &MarkdownSyntaxNode, range: TextRange) -> bool {
    let Some(token) = root.token_at_offset(range.start()).right_biased() else {
        return false;
    };
    let is_mdx_literal = matches!(
        token.kind(),
        MarkdownSyntaxKind::MDX_ESM_LITERAL
            | MarkdownSyntaxKind::MDX_EXPRESSION_LITERAL
            | MarkdownSyntaxKind::MDX_JSX_TAG_LITERAL
    );
    is_mdx_literal
        && token.text_trimmed().contains(['\n', '\r'])
        && token.ancestors().any(|node| {
            matches!(
                node.kind(),
                MarkdownSyntaxKind::MD_QUOTE
                    | MarkdownSyntaxKind::MD_BULLET_LIST_ITEM
                    | MarkdownSyntaxKind::MD_ORDERED_LIST_ITEM
            )
        })
}

/// Parses the Markdown code of the fenced code block of `root` whose code
/// spans `range`, without the line break that ends the opening fence line.
///
/// The snippet of the block is parsed from code that starts with that line
/// break. Frontmatter must start on the first line of a document, so the
/// snippet reads frontmatter at the start of the code as a thematic break and a
/// paragraph, and formatting it would remove the indentation of the YAML.
///
/// When the opening fence is indented, positions in the parsed code don't
/// match the file. Use it only for formatting.
fn parse_markdown_fence_code(
    root: &MarkdownSyntaxNode,
    range: TextRange,
    file_source: DocumentFileSource,
    path: &BiomePath,
    settings: &SettingsWithEditor,
) -> Option<AnyParse> {
    let code = match IndentedFenceCode::from_range(root, range) {
        Some(code) => code.without_indentation(),
        None => root
            .covering_element(range)
            .into_token()?
            .text()
            .to_string(),
    };
    let opening_line_end = code.find(['\n', '\r']).map_or(0, |line_break| {
        if code[line_break..].starts_with("\r\n") {
            line_break + 2
        } else {
            line_break + 1
        }
    });
    parse_snippet_code(
        &code[opening_line_end..],
        range.start() + TextSize::from(opening_line_end as u32),
        file_source,
        path,
        settings,
    )
}

/// A fenced code block whose opening fence is indented.
///
/// The CommonMark spec removes up to the fence's indentation from each line of
/// the code when the block is rendered. This block renders as
/// `<pre><code class="language-js">let a = 1;\n</code></pre>`, without leading
/// spaces:
///
/// ````md
///   ```js
///   let a = 1;
///   ```
/// ````
///
/// The block's snippet is parsed from the lines as written, spaces included.
/// Formatting the snippet would keep those spaces in text that formatters don't
/// change, such as multi-line template literals, so the code returned by
/// [Self::without_indentation] is formatted instead.
struct IndentedFenceCode {
    code: MarkdownSyntaxToken,
    indentation: usize,
}

impl IndentedFenceCode {
    /// Creates the fenced code block of `root` whose code spans `range`, if its
    /// opening fence is indented.
    fn from_range(root: &MarkdownSyntaxNode, range: TextRange) -> Option<Self> {
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

    /// Returns the code as it's rendered, without the fence's indentation.
    ///
    /// After the first line, positions in the returned code don't match the
    /// file. Use it only for formatting.
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

/// Formats the code of a snippet with the formatter of its language.
///
/// Returns `None` when that formatter is disabled for the Markdown file or the
/// language has no formatter. The document has no trailing line break, because
/// the Markdown formatter ends the snippet's last line.
///
/// With `single_line`, a snippet whose formatted code spans lines keeps its
/// source instead.
fn format_snippet(
    path: &BiomePath,
    file_source: DocumentFileSource,
    parse: AnyParse,
    settings: &SettingsWithEditor,
    workspace_db: &WorkspaceDb,
    single_line: bool,
) -> Option<Document> {
    let trailing_newline = TrailingNewline::from(false);
    match file_source {
        DocumentFileSource::Js(_) => {
            let options =
                javascript::resolve_format_options(path, &file_source, settings, workspace_db)
                    .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                JsFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Json(_) => {
            let options = json::resolve_format_options(&file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                JsonFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Css(_) => {
            let options = css::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                CssFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Graphql(_) => {
            let options =
                graphql::resolve_format_options(path, &file_source, settings, workspace_db)
                    .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                GraphqlFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Html(_) => {
            let options = html::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                HtmlFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Yaml(_) => {
            let options = yaml::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                YamlFormatLanguage::new(options),
                single_line,
            )
        }
        DocumentFileSource::Markdown(_) => {
            let options = resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_markdown_snippet(path, settings, parse, MdFormatLanguage::new(options))
        }
        DocumentFileSource::Grit(_) => {
            let options = grit::resolve_format_options(path, &file_source, settings, workspace_db)
                .with_trailing_newline(trailing_newline);
            format_with_language(
                path,
                settings,
                parse,
                GritFormatLanguage::new(options),
                single_line,
            )
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
    single_line: bool,
) -> Option<Document>
where
    F: FormatLanguage,
    F::SyntaxLanguage: ServiceLanguage + 'static,
{
    if !settings.formatter_enabled_for_file_path::<F::SyntaxLanguage>(path) {
        return None;
    }
    let node = parse.embedded_syntax::<F::SyntaxLanguage>();
    let printed = biome_formatter::format_node_with_offset(&node, language, false)
        .ok()?
        .print()
        .ok()?;
    if single_line && printed.as_code().contains('\n') {
        return None;
    }
    printed_document(printed.as_code())
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
    let printed = biome_formatter::format_node_with_offset(&node, language, false)
        .ok()?
        .print()
        .ok()?;
    if longest_backtick_sequence(printed.as_code()) >= fence_length {
        return None;
    }
    printed_document(printed.as_code())
}

/// Returns a document holding `code`, a snippet printed with the options of its
/// language.
fn printed_document(code: &str) -> Option<Document> {
    let context = SimpleFormatContext::<RowanSource>::from_options(SimpleFormatOptions::default());
    let formatted = biome_formatter::format!(context, [PrintedCode(code)]).ok()?;
    Some(formatted.into_document())
}

/// The lines of a snippet printed with the options of its language.
///
/// The lines are joined with literal line breaks, which the Markdown printer
/// writes with the line ending of the Markdown file and without adding
/// indentation, so the snippet keeps its own indentation. The code can use any
/// line ending, because the formatter of its language prints it with its own
/// `lineEnding` option.
struct PrintedCode<'a>(&'a str);

impl<Context: FormatContext> Format<Context> for PrintedCode<'_> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let code = normalize_newlines(self.0, ['\r']);
        for (index, line) in code.lines().enumerate() {
            if index > 0 {
                write!(f, [literal_line_break_without_parent()])?;
            }
            write!(f, [text(line, None)])?;
        }
        Ok(())
    }
}

fn longest_backtick_sequence(text: &str) -> usize {
    text.split(|character| character != '`')
        .map(str::len)
        .max()
        .unwrap_or_default()
}
