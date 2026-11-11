use crate::bindings::EmbeddedBinding;
use biome_db::ParsedSource;
use biome_js_syntax::{
    AnyJsExpression, AnyJsModuleItem, AnyJsRoot, JsFunctionDeclaration, JsIdentifierAssignment,
    JsIdentifierBinding, JsImport, JsSyntaxKind, JsVariableDeclarator, inner_string_text,
    unescape_js_identifier,
};
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::AnyParse;
use biome_rowan::{AstNode, Text, TextRange, TextSize, TokenText, WalkEvent};
use camino::Utf8Path;

/// Astro component imports and function-valued template expressions, in host offsets.
/// Only direct component imports and locally declared functions are classified.
#[derive(Debug, Default, PartialEq)]
pub struct AstroData {
    component_imports: Vec<EmbeddedBinding>,
    function_values: Vec<TextRange>,
}

impl AstroData {
    /// Finds a default `.astro` import or a named `Image` or `Picture` import from
    /// `astro:assets` in the frontmatter. Namespace imports are not classified.
    pub fn component_import(&self, name: &str) -> Option<&EmbeddedBinding> {
        self.component_imports
            .iter()
            .find(|binding| unescape_js_identifier(binding.text.text()) == name)
    }

    /// Returns a function-valued template expression contained in `range`.
    /// Calls, imported values, and indirect assignments are not classified.
    pub fn function_value(&self, range: TextRange) -> Option<TextRange> {
        self.function_values
            .iter()
            .copied()
            .find(|value| range.contains_range(*value))
    }
}

/// Returns the local names of directly imported Astro components.
/// Ranges are relative to the JavaScript snippet, before its host offset is applied.
pub fn component_imports(import: &JsImport) -> Vec<EmbeddedBinding> {
    let Ok(clause) = import.import_clause() else {
        return Vec::new();
    };
    if clause.type_token().is_some() {
        return Vec::new();
    }
    let Ok(source) = import.source_text() else {
        return Vec::new();
    };
    let is_astro = source.text().ends_with(".astro");
    if !is_astro && source.text() != "astro:assets" {
        return Vec::new();
    }
    let mut bindings = Vec::new();
    if is_astro
        && let Some(specifier) = clause.default_specifiers()
        && let Ok(local) = specifier.local_name()
        && let Some(local) = local.as_js_identifier_binding()
        && let Ok(token) = local.name_token()
    {
        bindings.push(EmbeddedBinding {
            range: local.range(),
            text: token.token_text_trimmed(),
            source: Some(source.clone()),
        });
    }
    if let Some(specifiers) = clause.named_specifiers() {
        for specifier in specifiers.specifiers().into_iter().flatten() {
            if specifier.type_token().is_some() {
                continue;
            }
            let Some(imported) = specifier.imported_name() else {
                continue;
            };
            let imported = inner_string_text(&imported);
            let imported = unescape_js_identifier(imported.text());
            if !(is_astro && imported == "default"
                || !is_astro && matches!(imported.as_ref(), "Image" | "Picture"))
            {
                continue;
            }
            let Some(local) = specifier.local_name() else {
                continue;
            };
            let Some(local) = local.as_js_identifier_binding() else {
                continue;
            };
            let Ok(token) = local.name_token() else {
                continue;
            };
            bindings.push(EmbeddedBinding {
                range: local.range(),
                text: token.token_text_trimmed(),
                source: Some(source.clone()),
            });
        }
    }
    bindings
}

pub(crate) fn collect_astro_data<'a>(
    snippets: impl Iterator<Item = (&'a AnyParse, DocumentFileSource, TextSize)> + Clone,
) -> AstroData {
    let mut data = AstroData::default();
    let mut functions = Vec::new();
    let mut writes = Vec::new();
    for (parse, source, content_offset) in snippets.clone() {
        let Some(source) = source.to_js_file_source() else {
            continue;
        };
        let kind = source.as_embedding_kind();
        if !kind.is_astro_frontmatter() && !kind.is_astro_template() {
            continue;
        }
        let root = parse.tree::<AnyJsRoot>();
        let offset = host_offset(parse, content_offset);
        if let Some(module) = root.as_js_module() {
            for item in module.items() {
                if let AnyJsModuleItem::JsImport(import) = item {
                    data.component_imports
                        .extend(component_imports(&import).into_iter().map(|mut binding| {
                            binding.range += offset;
                            binding
                        }));
                }
            }
        }
        let kinds = if kind.is_astro_frontmatter() {
            JsFunctionDeclaration::KIND_SET
                .union(JsVariableDeclarator::KIND_SET)
                .union(JsIdentifierAssignment::KIND_SET)
        } else {
            JsIdentifierAssignment::KIND_SET
        };
        for event in root.syntax().preorder_of_kinds(kinds) {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            if let Some(assignment) = JsIdentifierAssignment::cast_ref(&node) {
                if let Ok(name) = assignment.name_token() {
                    writes.push(identifier_name(name.token_text_trimmed()));
                }
                continue;
            }
            if let Some(variable) = JsVariableDeclarator::cast_ref(&node)
                && !variable
                    .declaration()
                    .is_some_and(|declaration| declaration.is_const())
            {
                if let Ok(id) = variable.id() {
                    writes.extend(
                        id.syntax()
                            .preorder_of_kinds(JsIdentifierBinding::KIND_SET)
                            .filter_map(|event| {
                                let WalkEvent::Enter(node) = event else {
                                    return None;
                                };
                                let name = JsIdentifierBinding::cast(node)?.name_token().ok()?;
                                Some(identifier_name(name.token_text_trimmed()))
                            }),
                    );
                }
                continue;
            }
            if !node.ancestors().skip(1).all(|parent| {
                matches!(
                    parent.kind(),
                    JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST
                        | JsSyntaxKind::JS_VARIABLE_DECLARATION
                        | JsSyntaxKind::JS_VARIABLE_STATEMENT
                        | JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE
                        | JsSyntaxKind::JS_EXPORT
                        | JsSyntaxKind::JS_MODULE_ITEM_LIST
                        | JsSyntaxKind::JS_MODULE
                )
            }) {
                continue;
            }
            let id = if let Some(function) = JsFunctionDeclaration::cast_ref(&node) {
                function
                    .id()
                    .ok()
                    .and_then(|id| id.as_js_identifier_binding().cloned())
            } else if let Some(variable) = JsVariableDeclarator::cast(node) {
                if !variable
                    .declaration()
                    .is_some_and(|declaration| declaration.is_const())
                {
                    continue;
                }
                let Some(value) = variable
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                else {
                    continue;
                };
                if !is_function(&value) {
                    continue;
                }
                variable
                    .id()
                    .ok()
                    .and_then(|id| id.as_any_js_binding()?.as_js_identifier_binding().cloned())
            } else {
                None
            };
            if let Some(id) = id
                && let Ok(name) = id.name_token()
            {
                functions.push(identifier_name(name.token_text_trimmed()));
            }
        }
    }
    functions.retain(|name| !writes.contains(name));
    for (parse, source, content_offset) in snippets {
        if !source
            .to_js_file_source()
            .is_some_and(|source| source.as_embedding_kind().is_astro_template())
        {
            continue;
        }
        let root = parse.tree::<AnyJsRoot>();
        let Some(expression) = root
            .as_js_expression_template_root()
            .and_then(|snippet| snippet.expression())
        else {
            continue;
        };
        let Some(expression) = expression.omit_type_wrappers() else {
            continue;
        };
        let is_known_function = expression
            .as_js_identifier_expression()
            .and_then(|identifier| identifier.name().ok())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| {
                let reference = unescape_js_identifier(token.text_trimmed());
                functions
                    .iter()
                    .any(|name| name.text() == reference.as_ref())
            });
        if is_function(&expression) || is_known_function {
            data.function_values
                .push(expression.range() + host_offset(parse, content_offset));
        }
    }
    data
}

fn identifier_name(text: TokenText) -> Text {
    let decoded = match unescape_js_identifier(text.text()) {
        std::borrow::Cow::Borrowed(_) => None,
        std::borrow::Cow::Owned(decoded) => Some(decoded),
    };
    decoded.map_or_else(|| text.into(), Into::into)
}

fn is_function(expression: &AnyJsExpression) -> bool {
    matches!(
        expression.clone().omit_type_wrappers(),
        Some(
            AnyJsExpression::JsArrowFunctionExpression(_)
                | AnyJsExpression::JsFunctionExpression(_)
        )
    )
}

fn host_offset(parse: &AnyParse, content_offset: TextSize) -> TextSize {
    match parse {
        AnyParse::EmbeddedNode(parse) => parse.root.offset(),
        AnyParse::Node(_) => content_offset,
    }
}

#[salsa::tracked(returns(ref))]
fn astro_data_from_source(db: &dyn LanguageDb, file: ParsedSource) -> AstroData {
    collect_astro_data(file.snippets(db).iter().filter_map(|snippet| {
        Some((
            snippet.parsed(db),
            db.source_from_index(snippet.document_source_index(db))?,
            snippet.content_offset(db),
        ))
    }))
}

/// Returns collected Astro facts, or `None` if the file is absent from the workspace.
pub fn astro_data<'db>(db: &'db dyn LanguageDb, path: &Utf8Path) -> Option<&'db AstroData> {
    Some(astro_data_from_source(db, db.parsed_source_for_path(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EmbeddedSnippet, collect_embedded_data};
    use biome_html_parser::parse_html;
    use biome_js_parser::{JsParserOptions, parse_js_with_offset};
    use biome_languages::{HtmlFileSource, JsFileSource, javascript::JsEmbeddingKind};

    #[test]
    fn collects_astro_imports_and_function_values_in_host_offsets() {
        let frontmatter = "\nimport \\u0043ard from './Card.astro';\nimport { Pict\\u0075re as \\u0050hoto, getImage } from 'astro:assets';\nimport type TypeOnly from './Card.astro';\nimport { helper } from './Card.astro';\nconst click = () => {};\nfunction declared() {}\nfunction changed() {}\nchanged = 'text';\nfunction changedInTemplate() {}\nfunction outer() { const nested = () => {}; }\n";
        let expressions = [
            "click",
            "declared",
            "changed",
            "nested",
            "() => {}",
            "factory(() => {})",
            "changedInTemplate = null",
            "changedInTemplate",
        ];
        let mut host = format!("---{frontmatter}---\r\n<p>日本語</p>\r\n");
        let mut expression_offsets = Vec::new();
        for expression in expressions {
            host.push_str("<Button client:load onClick={");
            expression_offsets.push(TextSize::of(&host));
            host.push_str(expression);
            host.push_str("} />\r\n");
        }
        let template_source = JsFileSource::tsx().with_embedding_kind(JsEmbeddingKind::Astro {
            content_offset: TextSize::default(),
            frontmatter: false,
            is_class_attribute: false,
            is_class_list_attribute: false,
        });
        let mut snippets = vec![(frontmatter, TextSize::from(3), JsFileSource::astro())];
        snippets.extend(
            expressions
                .into_iter()
                .zip(expression_offsets)
                .map(|(text, offset)| (text, offset, template_source)),
        );
        let parses: Vec<_> = snippets
            .iter()
            .map(|(text, offset, source)| {
                let parse =
                    parse_js_with_offset(text, *offset, *source, JsParserOptions::default());
                assert!(!parse.has_errors(), "{:#?}", parse.diagnostics());
                AnyParse::from(parse)
            })
            .collect();
        let host_parse = parse_html(&host, (&HtmlFileSource::astro()).into()).into();
        let data = collect_embedded_data(
            HtmlFileSource::astro().into(),
            &host_parse,
            snippets
                .iter()
                .zip(&parses)
                .map(|((text, offset, source), parse)| {
                    EmbeddedSnippet::new(
                        parse,
                        TextRange::at(*offset, TextSize::of(*text)),
                        (*source).into(),
                    )
                })
                .collect(),
        );
        let astro = data.astro();
        assert_eq!(astro.component_imports.len(), 2);
        let card = astro.component_import("Card").unwrap();
        assert_eq!(&host[card.range], r"\u0043ard");
        assert!(astro.component_import("Photo").is_some());
        assert!(astro.component_import("helper").is_none());
        for (index, (text, offset, _)) in snippets.iter().skip(1).enumerate() {
            let range = TextRange::at(*offset, TextSize::of(*text));
            assert_eq!(
                astro.function_value(range),
                matches!(index, 0 | 1 | 4).then_some(range),
                "{text}"
            );
        }
    }
}
