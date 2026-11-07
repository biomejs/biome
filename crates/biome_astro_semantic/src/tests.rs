use crate::{AstroSemanticModel, AstroSnippet, astro_model};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::{
    AstroEmbeddedContent, HtmlAttributeSingleTextExpression, HtmlElement, HtmlEmbeddedContent,
    HtmlRoot, HtmlSingleTextExpression,
};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::javascript::JsEmbeddingKind;
use biome_languages::{HtmlFileSource, JsFileSource};
use biome_rowan::{AstNode, TextRange, TextSize};

struct FoundSnippet {
    text: String,
    range: TextRange,
    offset: TextSize,
    source: JsFileSource,
}

fn model(source: &str) -> AstroSemanticModel {
    let parsed = parse_html(source, HtmlParserOptions::from(&HtmlFileSource::astro()));
    let host: HtmlRoot = parsed.tree();
    let mut found = Vec::new();

    for node in host.syntax().descendants() {
        if let Some(content) = AstroEmbeddedContent::cast_ref(&node)
            && let Some(token) = content.content_token()
        {
            found.push(FoundSnippet {
                text: token.text_trimmed().to_string(),
                range: token.text_trimmed_range(),
                offset: token.text_trimmed_range().start(),
                source: JsFileSource::astro(),
            });
        } else if let Some(element) = HtmlElement::cast_ref(&node)
            && element.is_javascript_tag()
            && !element.has_astro_is_raw()
            && let Some(token) = element
                .children()
                .syntax()
                .descendants()
                .find_map(HtmlEmbeddedContent::cast)
                .and_then(|content| content.value_token().ok())
        {
            found.push(FoundSnippet {
                text: token.text().to_string(),
                range: token.text_range(),
                offset: token.text_range().start(),
                source: JsFileSource::ts(),
            });
        } else if let Some(expression) = HtmlAttributeSingleTextExpression::cast_ref(&node)
            && let Ok(expression) = expression.expression()
            && let Ok(token) = expression.html_literal_token()
        {
            found.push(FoundSnippet {
                text: token.text().to_string(),
                range: token.text_range(),
                offset: token.text_range().start(),
                source: JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
                    frontmatter: false,
                    is_class_attribute: false,
                }),
            });
        } else if let Some(expression) = HtmlSingleTextExpression::cast_ref(&node)
            && let Some(token) = expression
                .expression()
                .and_then(|expression| expression.html_literal_token().ok())
        {
            found.push(FoundSnippet {
                text: token.text().to_string(),
                range: token.text_range(),
                offset: token.text_range().start(),
                source: JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
                    frontmatter: false,
                    is_class_attribute: false,
                }),
            });
        }
    }

    let parsed: Vec<_> = found
        .iter()
        .map(|snippet| {
            let root: AnyJsRoot =
                biome_js_parser::parse(&snippet.text, snippet.source, JsParserOptions::default())
                    .tree();
            let js = semantic_model(&root, SemanticModelOptions::from(&snippet.source));
            (root, js)
        })
        .collect();
    let snippets: Vec<_> = parsed
        .iter()
        .zip(&found)
        .map(|((root, js), found)| AstroSnippet {
            root,
            js,
            source: found.source,
            content_range: found.range,
            content_offset: found.offset,
        })
        .collect();

    astro_model(&host, &snippets)
}

#[test]
fn records_astro_document_structure() {
    let model = model(
        r#"---
const title = "Hello";
---
<h1>{title}</h1>
<div><script define:vars={{ theme: "dark" }}>console.log("ready")</script></div>
<Script />
<Style />
<script src="/client.js" />
<script type="application/json">{"name":"Biome"}</script>
<script type="importmap">{"imports":{}}</script>
<script is:raw>not JavaScript</script>
<script type="application/json" />
<script is:raw src="/raw.js" />
<style>h1 { color: red; }</style>
"#,
    );

    insta::assert_snapshot!(model.to_string());
}
