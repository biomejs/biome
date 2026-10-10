use super::*;
use biome_analyze::RuleCategoriesBuilder;
use crate::settings::ModuleGraphResolutionKind;
use crate::test_utils::setup_workspace_and_open_project;
use crate::workspace::UpdateSettingsParams;
use biome_analyze::RuleCategoriesBuilder;
use biome_configuration::MarkdownConfiguration;
use biome_configuration::html::{HtmlConfiguration, HtmlFormatterConfiguration};
use biome_configuration::markdown::{MarkdownFormatterConfiguration, MarkdownParserConfiguration};
use biome_fs::MemoryFileSystem;
use biome_languages::{
    CssFileSource, DocumentFileSource, GraphqlFileSource, GritFileSource, HtmlFileSource,
    JsFileSource, JsonFileSource, MdFileSource, YamlFileSource,
};
use biome_parser::AnyParse;
use biome_rowan::TextSize;
use camino::Utf8Path;

const FILE_PATH: &str = "/project/file.md";

fn open_markdown_with_embeds(content: &str, frontmatter: bool) -> (LocalWorkspace, ProjectKey) {
    open_markdown(
        content,
        Configuration {
            markdown: Some(MarkdownConfiguration {
                parser: Some(MarkdownParserConfiguration {
                    frontmatter: Some(frontmatter.into()),
                    gfm: Some(true.into()),
                    cjk_friendly_emphasis: None,
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
}

fn open_markdown(content: &str, configuration: Configuration) -> (LocalWorkspace, ProjectKey) {
    let fs = MemoryFileSystem::default();
    fs.insert(Utf8PathBuf::from(FILE_PATH), content);

    let (workspace, project_key) = setup_workspace_and_open_project(fs, "/");
    workspace
        .update_settings(UpdateSettingsParams {
            project_key,
            workspace_directory: None,
            configuration,
            extended_configurations: vec![],
            module_graph_resolution_kind: ModuleGraphResolutionKind::None,
        })
        .unwrap();

    workspace
        .open_file(OpenFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            content: FileContent::FromServer,
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
            editor_features: None,
        })
        .unwrap();

    (workspace, project_key)
}

fn embeds_configuration(format_embeds: bool, analyze_embeds: bool) -> Configuration {
    Configuration {
        markdown: Some(MarkdownConfiguration {
            formatter: Some(MarkdownFormatterConfiguration {
                format_embeds: Some(format_embeds.into()),
                ..Default::default()
            }),
            analyze_embeds: Some(analyze_embeds.into()),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn format(workspace: &LocalWorkspace, project_key: ProjectKey) -> String {
    workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap()
        .into_code()
}

#[test]
fn parses_frontmatter_fenced_code_and_html_blocks() {
    const CONTENT: &str = r#"---
title: Biome
---

```js title=example
const value = 1;
```

```typescript
const value: number = 1;
```

```jsonc
{"value": true,}
```

```css
a { color: red; }
```

```gql
query Test { field }
```

```yml
items:
  - one
```

```html
<section>content</section>
```

```vue
<template><div>content</div></template>
```

```markdown
# Nested heading
```

```grit
`console.log($message)`
```

```rust
fn main() {}
```

<div>
  <span>content</span>
</div>
"#;

    let (workspace, _) = open_markdown_with_embeds(CONTENT, true);
    let db = workspace.get_db();
    let snippets = workspace.get_snippets(Utf8Path::new(FILE_PATH));
    let sources: Vec<_> = snippets
        .iter()
        .map(|snippet| {
            db.source_from_index(snippet.document_source_index(&db))
                .unwrap()
        })
        .collect();

    assert_eq!(
        sources,
        vec![
            DocumentFileSource::Yaml(YamlFileSource::yaml()),
            DocumentFileSource::Js(JsFileSource::js_module()),
            DocumentFileSource::Js(JsFileSource::ts()),
            DocumentFileSource::Json(JsonFileSource::json_allow_comments_and_trailing_commas(
                "jsonc",
            )),
            DocumentFileSource::Css(CssFileSource::css()),
            DocumentFileSource::Graphql(GraphqlFileSource::graphql()),
            DocumentFileSource::Yaml(YamlFileSource::yaml()),
            DocumentFileSource::Html(HtmlFileSource::html()),
            DocumentFileSource::Html(HtmlFileSource::vue()),
            DocumentFileSource::Markdown(MdFileSource::markdown()),
            DocumentFileSource::Grit(GritFileSource::grit()),
            DocumentFileSource::Html(HtmlFileSource::html()),
        ]
    );

    for snippet in snippets {
        let content_range = snippet.content_range(&db);
        let content_offset = snippet.content_offset(&db);
        assert_eq!(content_offset, content_range.start());
        assert!(
            !CONTENT[usize::from(content_range.start())..usize::from(content_range.end())]
                .is_empty()
        );

        let AnyParse::EmbeddedNode(parse) = snippet.parsed(&db) else {
            panic!("Markdown snippets must use offset-aware parses");
        };
        assert_eq!(parse.root().offset(), content_offset);
    }
}

#[test]
fn parses_html_block_at_document_start() {
    const CONTENT: &str = "<div>content</div>\n";
    let (workspace, project_key) = open_markdown_with_embeds(CONTENT, false);
    let db = workspace.get_db();
    let snippets = workspace.get_snippets(Utf8Path::new(FILE_PATH));

    assert_eq!(snippets.len(), 1);
    let snippet = snippets[0];
    assert_eq!(snippet.content_offset(&db), TextSize::from(0));
    assert_eq!(
        db.source_from_index(snippet.document_source_index(&db)),
        Some(DocumentFileSource::Html(HtmlFileSource::html()))
    );
    let AnyParse::EmbeddedNode(parse) = snippet.parsed(&db) else {
        panic!("HTML blocks must use offset-aware parses");
    };
    assert_eq!(parse.root().offset(), TextSize::from(0));

    let formatted = workspace
        .format_file(FormatFileParams {
            project_key,
            path: Utf8PathBuf::from(FILE_PATH).into(),
            inline_config: None,
        })
        .unwrap();
    assert_eq!(formatted.as_code(), CONTENT);
}

#[test]
fn frontmatter_requires_parser_option() {
    const CONTENT: &str = "---\ntitle: Biome\n---\n";

    let (workspace, _) = open_markdown_with_embeds(CONTENT, false);
    assert!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).is_empty());

    let (workspace, _) = open_markdown_with_embeds(CONTENT, true);
    assert_eq!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).len(), 1);
}

#[test]
fn skips_unsupported_nested_and_inline_embeds() {
    const CONTENT: &str = r#"```rust
fn main() {}
```

- item

    ```js
    const value = 1;
    ```

> ```json
> {"value": true}
> ```

Paragraph with <span>inline HTML</span>.
"#;

    let (workspace, _) = open_markdown_with_embeds(CONTENT, false);
    assert!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).is_empty());
}

#[test]
fn parses_embeds_only_when_formatted_or_analyzed() {
    const CONTENT: &str = "```js\nconst value = 1;\n```\n";

    let (workspace, _) = open_markdown(CONTENT, embeds_configuration(false, false));
    assert!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).is_empty());

    let (workspace, _) = open_markdown(CONTENT, embeds_configuration(true, false));
    assert_eq!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).len(), 1);

    let (workspace, _) = open_markdown(CONTENT, embeds_configuration(false, true));
    assert_eq!(workspace.get_snippets(Utf8Path::new(FILE_PATH)).len(), 1);
}

#[test]
fn skips_comment_only_html_blocks() {
    const CONTENT: &str =
        "<!-- biome-ignore lint/suspicious/noDebugger: example -->\n\n<div>content</div>\n";

    let (workspace, _) = open_markdown_with_embeds(CONTENT, false);
    let db = workspace.get_db();
    let snippets = workspace.get_snippets(Utf8Path::new(FILE_PATH));

    assert_eq!(snippets.len(), 1);
    let content_range = snippets[0].content_range(&db);
    assert_eq!(
        &CONTENT[usize::from(content_range.start())..usize::from(content_range.end())],
        "<div>content</div>"
    );
}

#[test]
fn formats_embeds_idempotently() {
    const CONTENT: &str = r#"---
title:    Biome
---

```js
const   value   =   {a:1,
b:2}
```

  ```css
  a{color:red}
  ```

  ```js
  const   template = `a
  b`;
  ```

```md
#   Nested
```

<div><p>aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb</p></div>

```js
const   unclosed = 1
"#;
    let configuration = || Configuration {
        markdown: Some(MarkdownConfiguration {
            parser: Some(MarkdownParserConfiguration {
                frontmatter: Some(true.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        html: Some(HtmlConfiguration {
            formatter: Some(HtmlFormatterConfiguration {
                enabled: Some(true.into()),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    };

    let (workspace, project_key) = open_markdown(CONTENT, configuration());
    let formatted = format(&workspace, project_key);
    assert!(
        formatted.contains("```js\nconst template = `a\nb`;\n```"),
        "{formatted}"
    );

    let (workspace, project_key) = open_markdown(&formatted, configuration());
    assert_eq!(format(&workspace, project_key), formatted);
}

#[test]
fn fix_file_fixes_and_formats_embeds() {
    const CONTENT: &str =
        "#   Embeds\n\n```js\ndebugger;\nconsole.log(   1)\n```\n\n```js\nfunction () {}\n```\n";

    let (workspace, project_key) = open_markdown(CONTENT, embeds_configuration(true, true));
    let result = workspace
        .fix_file(FixFileParams {
            project_key,
            path: BiomePath::new(FILE_PATH),
            fix_file_mode: FixFileMode::SafeAndUnsafeFixes,
            should_format: true,
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            rule_categories: RuleCategoriesBuilder::default().with_lint().build(),
            suppression_reason: None,
            inline_config: None,
        })
        .unwrap();

    assert_eq!(
        result.code,
        "# Embeds\n\n```js\nconsole.log(1);\n```\n\n```js\nfunction () {}\n```\n"
    );
}
