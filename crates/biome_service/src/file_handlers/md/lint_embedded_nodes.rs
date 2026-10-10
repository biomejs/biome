use crate::file_handlers::{
    LintParams, LintResults, LintSnippetAnalyzer, css, graphql, html, javascript, json, md, yaml,
};
use biome_analyze::{Never, SnippetAnalyzer};
use biome_languages::DocumentFileSource;

/// Returns the analyzers of the snippets of the Markdown file described by
/// `params`, such as fenced code blocks, frontmatter, and HTML blocks.
///
/// Snippets aren't analyzed when `markdown.analyzeEmbeds` is disabled for
/// the file, or when their language has no linter.
pub(super) fn snippet_analyzers<'a>(
    params: &LintParams<'a>,
) -> Vec<Box<dyn SnippetAnalyzer<Never, Output = LintResults> + 'a>> {
    let mut analyzers: Vec<Box<dyn SnippetAnalyzer<Never, Output = LintResults> + 'a>> = Vec::new();
    for snippet in params
        .parsed_source
        .snippets(&params.workspace_db)
        .for_analysis(
            &params.parsed_source,
            params.language,
            params.settings.as_ref(),
            params.path,
            &params.workspace_db,
        )
    {
        let Some(language) = snippet.file_source(&params.workspace_db) else {
            continue;
        };
        let offset = snippet.content_offset(&params.workspace_db);
        let snippet_params = params.for_snippet(&snippet, language);
        let analyzer = match language {
            DocumentFileSource::Js(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_js_analyze::METADATA,
                javascript::lint_with_inspector,
            ),
            DocumentFileSource::Json(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_json_analyze::METADATA,
                json::lint_with_inspector,
            ),
            DocumentFileSource::Css(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_css_analyze::METADATA,
                css::lint_with_inspector,
            ),
            DocumentFileSource::Graphql(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_graphql_analyze::METADATA,
                graphql::lint_with_inspector,
            ),
            DocumentFileSource::Html(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_html_analyze::METADATA,
                html::lint_with_inspector,
            ),
            DocumentFileSource::Yaml(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_yaml_analyze::METADATA,
                yaml::lint_with_inspector,
            ),
            DocumentFileSource::Markdown(_) => LintSnippetAnalyzer::new(
                snippet_params,
                offset,
                &biome_markdown_analyze::METADATA,
                md::lint_with_inspector,
            ),
            DocumentFileSource::Grit(_)
            | DocumentFileSource::Ignore
            | DocumentFileSource::Unknown => continue,
        };
        analyzers.push(Box::new(analyzer));
    }
    analyzers
}
