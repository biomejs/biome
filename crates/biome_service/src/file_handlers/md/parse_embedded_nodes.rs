use crate::embed::EmbedContent;
use crate::embed::markdown::{EmbedCandidate, EmbedDetectorsRegistry, EmbedMatch};
use crate::file_handlers::{ParseEmbedResult, ParseEmbeddedParams};
use crate::settings::SettingsWithEditor;
#[cfg(feature = "lang_css")]
use biome_css_parser::parse_css_with_offset_and_cache;
#[cfg(feature = "lang_css")]
use biome_css_syntax::CssLanguage;
use biome_fs::BiomePath;
#[cfg(feature = "lang_graphql")]
use biome_graphql_parser::parse_graphql_with_offset_and_cache;
#[cfg(feature = "lang_grit")]
use biome_grit_parser::parse_grit_with_offset_and_cache;
#[cfg(feature = "lang_html")]
use biome_html_parser::parse_html_with_offset_and_cache;
#[cfg(feature = "lang_html")]
use biome_html_syntax::HtmlLanguage;
#[cfg(feature = "lang_js")]
use biome_js_parser::parse_js_with_offset_and_cache;
#[cfg(feature = "lang_js")]
use biome_js_syntax::JsLanguage;
use biome_json_parser::parse_json_with_offset_and_cache;
use biome_json_syntax::JsonLanguage;
use biome_languages::DocumentFileSource;
use biome_markdown_parser::parse_markdown_with_offset_and_cache;
use biome_markdown_syntax::{
    MarkdownLanguage, MarkdownSyntaxToken, MdFencedCodeBlock, MdFrontmatter, MdHtmlBlock, MdRoot,
};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, AstNodeList, NodeCache, TextRange};
#[cfg(feature = "lang_yaml")]
use biome_yaml_parser::parse_yaml_with_offset_and_cache;

struct EmbedParseContext<'a, 'settings> {
    cache: &'a mut NodeCache,
    path: &'a BiomePath,
    settings: &'a SettingsWithEditor<'settings>,
}

pub(crate) fn parse_embedded_nodes(params: ParseEmbeddedParams) -> ParseEmbedResult {
    let ParseEmbeddedParams {
        any_parse,
        path,
        file_source,
        settings,
        node_cache,
    } = params;

    if file_source.to_markdown_file_source().is_none() {
        return ParseEmbedResult::default();
    }

    let root: MdRoot = any_parse.tree();
    let mut nodes = Vec::new();
    let mut context = EmbedParseContext {
        cache: node_cache,
        path,
        settings,
    };

    if let Some(frontmatter) = root.frontmatter()
        && let Some(candidate) = build_frontmatter_candidate(&frontmatter)
    {
        parse_and_push(&candidate, &mut context, &mut nodes);
    }

    for fenced_code_block in root
        .syntax()
        .descendants()
        .filter_map(MdFencedCodeBlock::cast)
    {
        if let Some(candidate) = build_fenced_code_block_candidate(&fenced_code_block) {
            parse_and_push(&candidate, &mut context, &mut nodes);
        }
    }

    for html_block in root.syntax().descendants().filter_map(MdHtmlBlock::cast) {
        if let Some(candidate) = build_html_block_candidate(&html_block) {
            parse_and_push(&candidate, &mut context, &mut nodes);
        }
    }

    ParseEmbedResult { nodes }
}

fn parse_and_push(
    candidate: &EmbedCandidate,
    context: &mut EmbedParseContext,
    nodes: &mut Vec<(AnyParse, EmbedContent, DocumentFileSource)>,
) {
    let Some(embed_match) = EmbedDetectorsRegistry::detect_match(candidate) else {
        return;
    };
    if let Some(parsed) = parse_matched_embed(candidate, &embed_match, context) {
        nodes.push(parsed);
    }
}

fn build_frontmatter_candidate(frontmatter: &MdFrontmatter) -> Option<EmbedCandidate> {
    let token = frontmatter.content().ok()?.value_token().ok()?;
    Some(EmbedCandidate::Frontmatter {
        content: embed_content(frontmatter.range(), token),
    })
}

fn build_fenced_code_block_candidate(
    fenced_code_block: &MdFencedCodeBlock,
) -> Option<EmbedCandidate> {
    let content_list = fenced_code_block.content();
    if content_list.len() != 1 {
        // Container prefixes split nested fences into multiple nodes. Parsing them as one snippet
        // would make guest ranges diverge from their positions in the Markdown source.
        return None;
    }

    let content = content_list.first()?.as_md_code_content()?.clone();
    let token = content.value_token().ok()?;
    let info_string = fenced_code_block
        .code_list()
        .syntax()
        .text_with_trivia()
        .into_text();

    Some(EmbedCandidate::CodeBlock {
        content: embed_content(fenced_code_block.range(), token),
        info_string,
    })
}

fn build_html_block_candidate(html_block: &MdHtmlBlock) -> Option<EmbedCandidate> {
    let token = html_block.content().ok()?.value_token().ok()?;
    Some(EmbedCandidate::HtmlBlock {
        content: embed_content(html_block.range(), token),
    })
}

fn embed_content(element_range: TextRange, token: MarkdownSyntaxToken) -> EmbedContent {
    let content_range = token.text_range();
    EmbedContent {
        element_range,
        content_range,
        content_offset: content_range.start(),
        text: token.token_text(),
    }
}

fn parse_matched_embed(
    candidate: &EmbedCandidate,
    embed_match: &EmbedMatch,
    context: &mut EmbedParseContext,
) -> Option<(AnyParse, EmbedContent, DocumentFileSource)> {
    let content = candidate.content();
    let file_source = embed_match.file_source;

    let parse = match file_source {
        #[cfg(feature = "lang_js")]
        DocumentFileSource::Js(js_source) => {
            let options = context
                .settings
                .parse_options::<JsLanguage>(context.path, &file_source);
            parse_js_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                js_source,
                options,
                context.cache,
            )
            .into()
        }
        DocumentFileSource::Json(_) => {
            let options = context
                .settings
                .parse_options::<JsonLanguage>(context.path, &file_source);
            parse_json_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                context.cache,
                options,
            )
            .into()
        }
        #[cfg(feature = "lang_css")]
        DocumentFileSource::Css(css_source) => {
            let options = context
                .settings
                .parse_options::<CssLanguage>(context.path, &file_source);
            parse_css_with_offset_and_cache(
                content.text.text(),
                css_source,
                content.content_offset,
                context.cache,
                options,
            )
            .into()
        }
        #[cfg(feature = "lang_graphql")]
        DocumentFileSource::Graphql(_) => parse_graphql_with_offset_and_cache(
            content.text.text(),
            content.content_offset,
            context.cache,
        )
        .into(),
        #[cfg(feature = "lang_html")]
        DocumentFileSource::Html(_) => {
            let options = context
                .settings
                .parse_options::<HtmlLanguage>(context.path, &file_source);
            parse_html_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                context.cache,
                options,
            )
            .into()
        }
        #[cfg(feature = "lang_grit")]
        DocumentFileSource::Grit(_) => parse_grit_with_offset_and_cache(
            content.text.text(),
            content.content_offset,
            context.cache,
        )
        .into(),
        DocumentFileSource::Markdown(_) => {
            let options = context
                .settings
                .parse_options::<MarkdownLanguage>(context.path, &file_source);
            parse_markdown_with_offset_and_cache(
                content.text.text(),
                content.content_offset,
                context.cache,
                options,
            )
            .into()
        }
        #[cfg(feature = "lang_yaml")]
        DocumentFileSource::Yaml(_) => parse_yaml_with_offset_and_cache(
            content.text.text(),
            content.content_offset,
            context.cache,
        )
        .into(),
        DocumentFileSource::Ignore | DocumentFileSource::Unknown => return None,
    };

    Some((parse, content, file_source))
}
