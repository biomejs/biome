use crate::{
    Access, AstroSemanticModel, AstroSemanticModelOptions, AstroSnippet, BlockKind, Namespace,
    OpenReason, Origin, RefSite, Resolution, SymbolKind, astro_model_with_options,
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
    model_with_globals(source, &["console"])
}

fn model_with_globals(source: &str, globals: &[&str]) -> AstroSemanticModel {
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
            let js = semantic_model(&root, SemanticModelOptions::from(&snippet.source));
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

    astro_model_with_options(
        &host,
        &snippets,
        AstroSemanticModelOptions {
            globals: globals.iter().copied().map(Into::into).collect(),
        },
    )
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

#[test]
fn isolates_client_scripts_and_bridges_define_vars() {
    let source = r#"---
const serverOnly = "server";
const theme = "dark";
const dynamic = { fromSpread: 1 };
---
<script define:vars={{ theme, count: 1, count: 2, local: 1, foo: 1, "\x66oo": 2, bar: 1, [`\x62ar`]: 2, zero: 1, "\u{0000007A}ero": 2, "😀": 3, "\uD83D\uDE00": 4, "\\u0066oo": 5, "\u{10400}": 6, "\u{D801}\u{DC00}": 7, "": 8, [``]: 9, [(``)]: 10 }}>
console.log(theme, count, local, foo, \u0066oo, bar, zero, serverOnly, Astro);
const local = theme;
</script>
<script>
console.log(theme, local, serverOnly, Astro);
const own = 1;
</script>
<script define:vars={{ ...dynamic }}>
console.log(maybeInjected);
</script>
<script define:vars={dynamic}>
console.log(unknownBinding);
</script>
<script define:vars={{ ["literalKey"]: 1, [theme]: 2 }}>
console.log(literalKey, maybeComputed);
</script>
<script define:vars>
console.log(missingInitializer);
</script>
<script define:vars={{ broken:, known: { nested: } }}>
console.log(broken, known, malformedUnknown);
</script>
<script define:vars={{ "\01": 1 }}>
console.log(octalUnknown);
</script>
<script type="application/json" define:vars={{ ignoredJson: serverOnly }}>{"ok":true}</script>
<script define:vars={{ selfClosing: 1 }} />
<script define:vars />
<script src="/ignored.js" define:vars={{ externalInjected: serverOnly }}>
const ignored = 1;
console.log(ignored);
</script>
<script src="/external.js" />
"#;
    let model = model(source);
    assert_eq!(
        model
            .blocks()
            .filter(|block| block.kind() == BlockKind::Script)
            .count(),
        13
    );
    let scripts: Vec<_> = model
        .blocks()
        .filter(|block| block.kind() == BlockKind::Script && block.scope().is_some())
        .collect();
    assert_eq!(scripts.len(), 12);
    let scopes: Vec<_> = scripts
        .iter()
        .map(|block| block.scope().expect("client script scope"))
        .collect();
    assert!(scopes.iter().all(|scope| scope.parent().is_none()));

    let theme_binding = model
        .symbols()
        .find(|symbol| {
            symbol.name() == "theme"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("static define:vars binding");
    let count_binding = model
        .symbols()
        .find(|symbol| {
            symbol.name() == "count"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("named define:vars binding");
    assert_eq!(theme_binding.uses().count(), 2);
    assert_eq!(count_binding.uses().count(), 1);
    assert_eq!(
        model
            .symbols()
            .filter(|symbol| {
                symbol.name() == "count"
                    && symbol.origin() == Origin::DefineVars
                    && symbol.scope() == scopes[0]
            })
            .count(),
        1
    );
    let foo = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "foo"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("escaped define:vars binding");
    assert_eq!(foo.name(), r"\x66oo");
    assert_eq!(foo.uses().count(), 2);
    assert_eq!(
        model
            .symbols()
            .filter(|symbol| {
                symbol.semantic_name() == "foo"
                    && symbol.origin() == Origin::DefineVars
                    && symbol.scope() == scopes[0]
            })
            .count(),
        1
    );
    let bar = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "bar"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("template-key define:vars binding");
    assert_eq!(bar.name(), r"\x62ar");
    assert_eq!(bar.uses().count(), 1);
    assert_eq!(
        model
            .symbols()
            .filter(|symbol| {
                symbol.semantic_name() == "bar"
                    && symbol.origin() == Origin::DefineVars
                    && symbol.scope() == scopes[0]
            })
            .count(),
        1
    );
    let zero = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "zero"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("leading-zero code-point binding");
    assert_eq!(zero.name(), r"\u{0000007A}ero");
    assert_eq!(zero.uses().count(), 1);
    let emoji = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "😀"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("surrogate-pair define:vars binding");
    assert_eq!(emoji.name(), r"\uD83D\uDE00");
    let literal_escape = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == r"\u0066oo"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("literal-backslash define:vars binding");
    assert_eq!(literal_escape.name(), r"\\u0066oo");
    assert_eq!(literal_escape.uses().count(), 0);
    let deseret = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "𐐀"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("braced-surrogate define:vars binding");
    assert_eq!(deseret.name(), r"\u{D801}\u{DC00}");
    assert_eq!(
        model
            .symbols()
            .filter(|symbol| {
                symbol.semantic_name() == "𐐀"
                    && symbol.origin() == Origin::DefineVars
                    && symbol.scope() == scopes[0]
            })
            .count(),
        1
    );
    let empty = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name().is_empty()
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("empty-template define:vars binding");
    assert!(empty.name().is_empty());
    assert_eq!(
        model
            .symbols()
            .filter(|symbol| {
                symbol.semantic_name().is_empty()
                    && symbol.origin() == Origin::DefineVars
                    && symbol.scope() == scopes[0]
            })
            .count(),
        1
    );
    let local = model
        .symbols()
        .find(|symbol| {
            symbol.name() == "local"
                && symbol.kind() == SymbolKind::Local
                && symbol.scope() == scopes[0]
        })
        .expect("client-local declaration");
    let injected_local = model
        .symbols()
        .find(|symbol| {
            symbol.name() == "local"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[0]
        })
        .expect("shadowed define:vars binding");
    assert_eq!(local.uses().count(), 1);
    assert_eq!(injected_local.uses().count(), 0);
    assert_eq!(
        scopes[0]
            .lookup(Namespace::Value, "local")
            .expect("local lookup")
            .id(),
        local.id()
    );
    assert!(scopes[1].lookup(Namespace::Value, "theme").is_none());
    assert!(model.references().any(|reference| {
        reference.name() == "serverOnly"
            && reference.scope() == Some(scopes[0].clone())
            && reference.resolution() == Resolution::Unresolved
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "Astro"
            && reference.scope() == Some(scopes[0].clone())
            && reference.resolution() == Resolution::Unresolved
    }));
    assert!(
        scopes[2]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::Spread)
    );
    assert!(
        scopes[3]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::NonLiteral)
    );
    assert!(model.references().any(|reference| {
        reference.name() == "maybeInjected"
            && reference.scope() == Some(scopes[2].clone())
            && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "console"
            && reference.scope() == Some(scopes[2].clone())
            && reference.resolution() == Resolution::Unknowable
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "unknownBinding"
            && reference.scope() == Some(scopes[3].clone())
            && reference.resolution() == Resolution::Unknowable
    }));
    let literal_key = model
        .symbols()
        .find(|symbol| {
            symbol.name() == "literalKey"
                && symbol.origin() == Origin::DefineVars
                && symbol.scope() == scopes[4]
        })
        .expect("literal computed define:vars binding");
    assert_eq!(literal_key.uses().count(), 1);
    assert!(
        scopes[4]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::Computed)
    );
    assert!(model.references().any(|reference| {
        reference.name() == "maybeComputed"
            && reference.scope() == Some(scopes[4].clone())
            && reference.resolution() == Resolution::Unknowable
    }));
    assert!(
        scopes[5]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::ParseError)
    );
    assert!(
        scopes[6]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::ParseError)
    );
    assert!(
        scopes[7]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::ParseError)
    );
    for (name, scope) in [
        ("missingInitializer", &scopes[5]),
        ("broken", &scopes[6]),
        ("known", &scopes[6]),
        ("malformedUnknown", &scopes[6]),
        ("octalUnknown", &scopes[7]),
    ] {
        assert!(
            model.references().any(|reference| {
                reference.name() == name
                    && reference.scope() == Some(scope.clone())
                    && reference.resolution() == Resolution::Unknowable
            }),
            "{name}"
        );
    }
    assert!(model.symbols().all(|symbol| symbol.name() != "broken"));
    assert!(model.symbols().all(|symbol| symbol.name() != "known"));
    assert!(
        model
            .symbols()
            .all(|symbol| symbol.name() != "externalInjected")
    );
    assert!(model.symbols().all(|symbol| symbol.name() != "ignoredJson"));
    let self_closing = scopes[8]
        .lookup(Namespace::Value, "selfClosing")
        .expect("self-closing define:vars binding");
    assert_eq!(self_closing.origin(), Origin::DefineVars);
    let server_only = model
        .symbols()
        .find(|symbol| symbol.name() == "serverOnly")
        .expect("server-only declaration");
    assert_eq!(server_only.uses().count(), 0);
    assert!(model.symbols().all(|symbol| symbol.name() != "ignored"));
    assert!(
        scopes[9]
            .open_reasons(Namespace::Value)
            .contains(OpenReason::ParseError)
    );
    for scope in [&scopes[10], &scopes[11]] {
        for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
            assert!(
                scope
                    .open_reasons(namespace)
                    .contains(OpenReason::ExternalSource)
            );
        }
    }

    insta::assert_snapshot!(semantic_snapshot(source, &model));
}

#[test]
fn applies_configured_globals_to_server_and_client_snippets() {
    let model = model_with_globals(
        r#"---
configuredGlobal;
---
<div>{configuredGlobal}</div>
<script>configuredGlobal; console; Astro;</script>
"#,
        &["configuredGlobal", "console"],
    );
    let script_scope = model
        .blocks()
        .find(|block| block.kind() == BlockKind::Script)
        .and_then(|block| block.scope())
        .expect("client script scope");

    assert!(model.references().any(|reference| {
        reference.name() == "configuredGlobal"
            && reference.scope() == Some(script_scope.clone())
            && reference.resolution() == Resolution::Global
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "console"
            && reference.scope() == Some(script_scope.clone())
            && reference.resolution() == Resolution::Global
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "Astro"
            && reference.scope() == Some(script_scope.clone())
            && reference.resolution() == Resolution::Unresolved
    }));
    assert!(model.references().any(|reference| {
        reference.name() == "configuredGlobal"
            && reference.scope() != Some(script_scope.clone())
            && reference.resolution() == Resolution::Global
    }));
}

#[test]
fn excludes_define_vars_proto_setters() {
    let source = r#"---
---
<script define:vars={{ __proto__: null }}>
console.log(__proto__);
</script>
<script define:vars={{ ["__proto__"]: 1 }}>
console.log(__proto__);
</script>
<style define:vars={{ __proto__: null }}>
.card { color: var(--__proto__); }
</style>
<style define:vars={{ ["__proto__"]: 1 }}>
.card { color: var(--__proto__); }
</style>
"#;
    let model = model(source);
    let scripts: Vec<_> = model
        .blocks()
        .filter(|block| block.kind() == BlockKind::Script)
        .collect();
    let first_scope = scripts[0].scope().expect("first script scope");
    let second_scope = scripts[1].scope().expect("second script scope");
    assert!(first_scope.lookup(Namespace::Value, "__proto__").is_none());
    assert!(model.references().any(|reference| {
        reference.name() == "__proto__"
            && reference.scope() == Some(first_scope.clone())
            && reference.resolution() == Resolution::Unresolved
    }));
    assert_eq!(
        second_scope
            .lookup(Namespace::Value, "__proto__")
            .expect("computed __proto__ binding")
            .origin(),
        Origin::DefineVars
    );

    let styles: Vec<_> = model
        .blocks()
        .filter(|block| block.kind() == BlockKind::Style)
        .collect();
    assert!(styles[0].define_vars().next().is_none());
    assert!(styles[0].define_vars_open_reasons().is_empty());
    assert_eq!(
        styles[1]
            .define_vars()
            .next()
            .expect("computed __proto__ style key")
            .semantic_name()
            .text(),
        "__proto__"
    );

    insta::assert_snapshot!(semantic_snapshot(source, &model));
}

#[test]
fn records_style_define_vars() {
    let source = r#"---
const foreground = "red";
const vars = { dynamic: "blue" };
---
<style define:vars={{ foreground, accent: "blue", "\x66oo": 1, 1e2: "a", 0x64: "b", [0x64n]: "c", 0x100000000000000000000000000000000: "large", [-1]: "negative", [-1n]: "negativeBigint" }}>
.card { color: var(--foreground); border-color: var(--100); outline-color: var(---1); }
</style>
<style define:vars={{ ...vars }}>
.card { color: var(--maybe); }
</style>
<style define:vars={vars}>
.card { color: var(--unknown); }
</style>
<style define:vars>
.card { color: var(--missing); }
</style>
<style define:vars={{ [/x/]: "regex" }}>
.card { color: var(--regex); }
</style>
<style is:raw define:vars={{ ignored: foreground }}>
.card { color: var(--ignored); }
</style>
"#;
    let model = model(source);
    let styles: Vec<_> = model
        .blocks()
        .filter(|block| block.kind() == BlockKind::Style)
        .collect();
    assert_eq!(styles.len(), 6);
    let names = styles[0]
        .define_vars()
        .map(|variable| variable.semantic_name().to_string())
        .collect::<Vec<_>>();
    assert_eq!(&names[..4], ["foreground", "accent", "foo", "100"]);
    assert_eq!(names.len(), 6);
    let numeric = styles[0]
        .define_vars()
        .find(|variable| variable.semantic_name().text() == "100")
        .expect("numeric define:vars key");
    assert_eq!(numeric.name(), "0x64n");
    let large = styles[0]
        .define_vars()
        .find(|variable| variable.name() == "0x100000000000000000000000000000000")
        .expect("large numeric define:vars key");
    assert_eq!(large.semantic_name().text(), "3.402823669209385e+38");
    let negative = styles[0]
        .define_vars()
        .find(|variable| variable.semantic_name().text() == "-1")
        .expect("negative define:vars key");
    assert_eq!(negative.name(), "-1n");
    assert!(styles[0].define_vars_open_reasons().is_empty());
    assert!(
        styles[1]
            .define_vars_open_reasons()
            .contains(OpenReason::Spread)
    );
    assert!(
        styles[2]
            .define_vars_open_reasons()
            .contains(OpenReason::NonLiteral)
    );
    assert!(
        styles[3]
            .define_vars_open_reasons()
            .contains(OpenReason::ParseError)
    );
    assert!(
        styles[4]
            .define_vars_open_reasons()
            .contains(OpenReason::Computed)
    );
    assert!(styles[5].define_vars().next().is_none());
    assert!(styles[5].define_vars_open_reasons().is_empty());

    let foreground = model
        .symbols()
        .find(|symbol| symbol.name() == "foreground")
        .expect("foreground declaration");
    let vars = model
        .symbols()
        .find(|symbol| symbol.name() == "vars")
        .expect("vars declaration");
    assert_eq!(foreground.uses().count(), 1);
    assert_eq!(vars.uses().count(), 2);

    insta::assert_snapshot!(semantic_snapshot(source, &model));
}

#[test]
fn handles_long_numeric_define_vars_key() {
    let digits = "f".repeat(20_000);
    let source = format!("---\n---\n<script define:vars={{{{ 0x{digits}: 1 }}}}></script>\n");
    let model = model(&source);
    let block = model
        .blocks()
        .find(|block| block.kind() == BlockKind::Script)
        .expect("script block");
    let binding = block
        .scope()
        .and_then(|scope| scope.lookup(Namespace::Value, "Infinity"))
        .expect("infinite numeric key");
    assert_eq!(binding.origin(), Origin::DefineVars);
    assert!(
        block
            .scope()
            .expect("script scope")
            .open_reasons(Namespace::Value)
            .is_empty()
    );
}

#[test]
fn classifies_external_and_inline_src_scripts() {
    let model = model(
        r#"<script src></script>
<script src=""></script>
<script src="/inline.js" is:inline>const inlineValue = 1;</script>
"#,
    );
    let scripts: Vec<_> = model
        .blocks()
        .filter(|block| block.kind() == BlockKind::Script)
        .collect();

    assert_eq!(scripts.len(), 3);
    assert!(scripts[0].has_external_source());
    assert!(scripts[1].has_external_source());
    assert!(!scripts[2].has_external_source());

    let inline_scope = scripts[2].scope().expect("inline script scope");
    assert!(
        model
            .symbols()
            .any(|symbol| { symbol.name() == "inlineValue" && symbol.scope() == inline_scope })
    );
}

#[test]
fn normalizes_raw_constant_template_line_endings() {
    let source = "---\n---\n<script define:vars={{ [`line\r\nbreak`]: 1, [`line\nbreak`]: 2, [`line\\rbreak`]: 3 }}>\n</script>\n";
    let model = model(source);
    let scope = model
        .blocks()
        .find(|block| block.kind() == BlockKind::Script)
        .and_then(|block| block.scope())
        .expect("client script scope");
    let names: Vec<_> = model
        .symbols()
        .filter(|symbol| {
            symbol.semantic_name() == "line\nbreak" && symbol.origin() == Origin::DefineVars
        })
        .collect();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0].name(), "line\nbreak");
    let escaped = model
        .symbols()
        .find(|symbol| {
            symbol.semantic_name() == "line\rbreak" && symbol.origin() == Origin::DefineVars
        })
        .expect("escaped carriage-return binding");
    assert_eq!(escaped.name(), r"line\rbreak");
    assert!(scope.open_reasons(Namespace::Value).is_empty());
}
