use crate::{
    Access, AstroSemanticModel, AstroSnippet, Namespace, OpenReason, Origin, RefSite, Resolution,
    SymbolKind, astro_model,
};
use biome_html_parser::{HtmlParserOptions, parse_html};
use biome_html_syntax::{
    AstroEmbeddedContent, HtmlAttributeSingleTextExpression, HtmlElement, HtmlEmbeddedContent,
    HtmlRoot, HtmlSingleTextExpression, HtmlSpreadAttribute,
};
use biome_js_parser::JsParserOptions;
use biome_js_semantic::{SemanticModelOptions, semantic_model};
use biome_js_syntax::AnyJsRoot;
use biome_languages::javascript::JsEmbeddingKind;
use biome_languages::{HtmlFileSource, JsFileSource};
use biome_rowan::{AstNode, TextRange, TextSize};
use std::fmt::Write;

struct FoundSnippet {
    text: String,
    range: TextRange,
    offset: TextSize,
    source: JsFileSource,
}

fn semantic_snapshot(source: &str, model: &AstroSemanticModel) -> String {
    let mut snapshot = String::new();
    writeln!(snapshot, "## Input\n\n```astro").unwrap();
    snapshot.push_str(source);
    if !source.ends_with('\n') {
        snapshot.push('\n');
    }
    writeln!(snapshot, "```\n\n## Semantic model\n\n```text").unwrap();
    write!(snapshot, "{model}").unwrap();
    if !snapshot.ends_with('\n') {
        snapshot.push('\n');
    }
    writeln!(snapshot, "```").unwrap();
    snapshot
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
        } else if let Some(spread) = HtmlSpreadAttribute::cast_ref(&node)
            && let Ok(expression) = spread.argument()
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
        }
    }

    let parsed: Vec<_> = found
        .iter()
        .map(|snippet| {
            let parsed =
                biome_js_parser::parse(&snippet.text, snippet.source, JsParserOptions::default());
            let has_errors = parsed.has_errors();
            let root: AnyJsRoot = parsed.tree();
            let mut options = SemanticModelOptions::from(&snippet.source);
            options.globals.insert("console".into());
            let js = semantic_model(&root, options);
            (root, js, has_errors)
        })
        .collect();
    let snippets: Vec<_> = parsed
        .iter()
        .zip(&found)
        .map(|((root, js, has_errors), found)| AstroSnippet {
            root,
            js,
            source: found.source,
            content_range: found.range,
            content_offset: found.offset,
            has_errors: *has_errors,
        })
        .collect();

    astro_model(&host, &snippets)
}

#[test]
fn malformed_frontmatter_opens_server_semantics() {
    let model = model(
        r#"---
const visible = 1;
const broken =
---
<div>{visible}</div>
<Missing />
"#,
    );
    let scope = model
        .blocks()
        .find(|block| block.kind() == crate::BlockKind::Frontmatter)
        .and_then(|block| block.scope())
        .expect("frontmatter scope");

    for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
        assert!(
            scope
                .open_reasons(namespace)
                .contains(OpenReason::ParseError)
        );
    }
    assert!(model.references().any(|reference| {
        reference.name() == "Missing" && reference.resolution() == Resolution::Unknowable
    }));
}

#[test]
fn malformed_template_expression_is_a_local_unknown_island() {
    let model = model(
        r#"---
const known = 1;
---
<div>{maybe + }</div>
<div>{missing}</div>
"#,
    );
    let expressions: Vec<_> = model
        .snippets()
        .filter(|snippet| snippet.host() == crate::SnippetHost::Expression)
        .collect();

    assert_eq!(expressions.len(), 2);
    let malformed_scope = expressions[0].scope().expect("malformed expression scope");
    assert!(
        malformed_scope
            .open_reasons(Namespace::Value)
            .contains(OpenReason::ParseError)
    );
    assert!(
        !expressions[1]
            .scope()
            .expect("valid expression scope")
            .is_open(Namespace::Value)
    );
    assert!(model.references().any(|reference| {
        reference.name() == "maybe" && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "missing" && reference.resolution() == Resolution::Unresolved
    }));
}

#[test]
fn records_astro_document_structure() {
    let source = r#"---
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
"#;
    let model = model(source);

    insta::assert_snapshot!(semantic_snapshot(source, &model));
}

#[test]
fn resolves_frontmatter_declarations_from_template_references() {
    let source = r#"---
import Card from "./Card.astro";
import * as UI from "./ui";
import type { Thing } from "./types";
import type Legacy = require("./legacy");
import LegacyValue = require("./legacy-value");
const title = "Hello";
let count = 0;
const items = [1];
const rest = { id: "hero" };
const escaped = 1;
const upper = title.toUpperCase();
const currentUrl = Astro.url;
interface TypeOnlyComponent {}
interface Merged {}
const Merged = Card;
interface OnlyTypeNamespace {}
interface MergedNamespace {}
namespace MergedNamespace { export type Member = string }
type ValueQuery = typeof MissingValue;
type MissingAlias = MissingType;
module LegacyModule { export type Member = string }
type ModuleMember = LegacyModule.Member;
const moduleValue = LegacyModule;
type ModuleValueMember = typeof moduleValue.member;
type ConfiguredGlobalType = console;
type ConfiguredGlobalMember = console.Member;
type ConfiguredGlobalValueMember = typeof console.log;
class ComponentClass {}
enum Status { Ready }
type ComponentClassType = ComponentClass;
type StatusType = Status;
type StatusMember = Status.Ready;
type LegacyValueType = LegacyValue;
type LegacyValueMember = LegacyValue.Member;
interface Props { name: string }
---
<Card label={title} />
<UI.Card />
<TypeOnlyComponent />
<Legacy />
<LegacyValue />
<ComponentClass />
<Merged />
<div>{Status.Ready}</div>
<div>{title as Legacy}</div>
<div>{title as Merged}</div>
<div>{null as MergedNamespace.Member}</div>
<div>{null as OnlyTypeNamespace.Member}</div>
<div>{items.map((title) => title + count)}</div>
<div>{missing}</div>
<div>{console.log(title)}</div>
<div>{Astro.url}</div>
<div>{esc\u0061ped}</div>
<div>{Astr\u006f.url}</div>
<div>{title as Thing}</div>
<Card {title} {...rest} class:list={{ active: count > 0 }} />
"#;
    let model = model(source);

    let title = model
        .symbols()
        .find(|symbol| symbol.name() == "title")
        .expect("frontmatter title symbol");
    assert_eq!(title.kind(), SymbolKind::Local);
    assert_eq!(title.uses().count(), 7);

    let props = model
        .symbols()
        .find(|symbol| symbol.name() == "Props")
        .expect("frontmatter Props symbol");
    assert!(props.is_type_only());
    assert_eq!(props.uses().count(), 1);

    let astro = model
        .symbols()
        .find(|symbol| symbol.origin() == Origin::AstroBuiltin)
        .expect("Astro builtin");
    assert_eq!(astro.uses().count(), 3);
    assert!(model.references().any(|reference| {
        reference.name() == "Astro" && reference.resolution() == Resolution::Symbol(astro.id())
    }));
    let escaped = model
        .symbols()
        .find(|symbol| symbol.name() == "escaped")
        .expect("escaped frontmatter symbol");
    assert_eq!(escaped.uses().count(), 1);
    assert!(model.references().any(|reference| {
        reference.name() == "console" && reference.resolution() == Resolution::Global
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "missing" && reference.resolution() == Resolution::Unresolved
    }));
    assert!(
        model
            .references()
            .any(|reference| { reference.name() == "Thing" && reference.access() == Access::Type })
    );
    assert_eq!(
        model
            .references()
            .filter(|reference| {
                reference.name() == "Card" && reference.site() == RefSite::TagName
            })
            .count(),
        2
    );

    let merged_value = model
        .symbols()
        .find(|symbol| symbol.name() == "Merged" && symbol.declares_value())
        .expect("Merged value declaration");
    let merged_type = model
        .symbols()
        .find(|symbol| symbol.name() == "Merged" && symbol.is_type_only())
        .expect("Merged type declaration");
    assert!(model.references().any(|reference| {
        reference.name() == "Merged"
            && reference.site() == RefSite::TagName
            && reference.resolution() == Resolution::Symbol(merged_value.id())
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "Merged"
            && reference.access() == Access::Type
            && reference.resolution() == Resolution::Symbol(merged_type.id())
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "TypeOnlyComponent"
            && reference.site() == RefSite::TagName
            && reference.resolution() == Resolution::Unresolved
    }));

    let merged_namespace = model
        .symbols()
        .find(|symbol| symbol.name() == "MergedNamespace" && symbol.declares_value())
        .expect("MergedNamespace namespace declaration");
    assert!(model.references().any(|reference| {
        reference.name() == "MergedNamespace"
            && reference.resolution() == Resolution::Symbol(merged_namespace.id())
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "OnlyTypeNamespace" && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "MissingValue" && reference.resolution() == Resolution::Unresolved
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "MissingType" && reference.resolution() == Resolution::Unknowable
    }));

    let legacy_module = model
        .symbols()
        .find(|symbol| symbol.name() == "LegacyModule")
        .expect("LegacyModule declaration");
    assert!(legacy_module.declares_value());
    assert!(legacy_module.declares_namespace());
    assert_eq!(legacy_module.uses().count(), 2);
    let module_value = model
        .symbols()
        .find(|symbol| symbol.name() == "moduleValue")
        .expect("moduleValue declaration");
    assert_eq!(module_value.uses().count(), 1);
    assert!(model.references().any(|reference| {
        reference.name() == "console"
            && reference.namespace() == Namespace::Type
            && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "console"
            && reference.namespace() == Namespace::Namespace
            && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "console"
            && reference.namespace() == Namespace::Value
            && reference.access() == Access::Type
            && reference.resolution() == Resolution::Global
    }));

    let component_class = model
        .symbols()
        .find(|symbol| symbol.name() == "ComponentClass")
        .expect("ComponentClass declaration");
    assert!(component_class.declares_value());
    assert!(component_class.declares_type());
    assert!(!component_class.declares_namespace());
    assert_eq!(component_class.uses().count(), 2);

    let status = model
        .symbols()
        .find(|symbol| symbol.name() == "Status")
        .expect("Status declaration");
    assert!(status.declares_value());
    assert!(status.declares_type());
    assert!(status.declares_namespace());
    assert_eq!(status.uses().count(), 3);

    let legacy_value = model
        .symbols()
        .find(|symbol| symbol.name() == "LegacyValue")
        .expect("LegacyValue import-equals declaration");
    assert_eq!(legacy_value.kind(), SymbolKind::Import);
    assert!(legacy_value.declares_value());
    assert!(legacy_value.declares_type());
    assert!(legacy_value.declares_namespace());
    assert_eq!(legacy_value.uses().count(), 3);

    let legacy = model
        .symbols()
        .find(|symbol| symbol.name() == "Legacy")
        .expect("Legacy import-equals declaration");
    assert_eq!(legacy.kind(), SymbolKind::Import);
    assert!(legacy.is_type_only());
    assert!(model.references().any(|reference| {
        reference.name() == "Legacy"
            && reference.site() == RefSite::TagName
            && reference.resolution() == Resolution::Unresolved
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "Legacy"
            && reference.access() == Access::Type
            && reference.resolution() == Resolution::Symbol(legacy.id())
    }));

    let ranges: Vec<_> = model
        .references()
        .map(|reference| reference.host_range())
        .collect();
    assert!(ranges.windows(2).all(|ranges| ranges[0] <= ranges[1]));

    insta::assert_snapshot!(semantic_snapshot(source, &model));
}
