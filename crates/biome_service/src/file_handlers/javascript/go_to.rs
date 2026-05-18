use crate::file_handlers::{ResolveBindingParams, ResolveDefinitionParams};
use crate::workspace::{DefinitionReference, GoToDefinitionResult};
use biome_css_syntax::{TextRange, TextSize};
use biome_fs::BiomePath;
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsRoot, AnyJsxAttributeValue, JsImport, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode,
    JsVariableDeclarator, JsxAttribute, JsxReferenceIdentifier, JsxString,
};
use biome_module_graph::{JsOwnExport, ModuleDb, ModuleInfoKind};
use biome_rowan::{AstNode, AstSeparatedList, TokenAtOffset, TokenText};
use camino::Utf8Path;
use std::ops::Add;

/// Source-side capability: given a cursor position, identify what binding the user clicked on.
pub(crate) fn resolve_binding(params: ResolveBindingParams) -> Option<DefinitionReference> {
    let root: AnyJsRoot = params.parse.tree();

    let token = match root.syntax().token_at_offset(params.cursor_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, right) => right,
        TokenAtOffset::None => return None,
    };

    for ancestor in token.ancestors() {
        if let Some(jsx_attribute) = JsxAttribute::cast_ref(&ancestor) {
            // Check if cursor is inside a JSX className/class attribute string.
            // This doesn't need the semantic model, so try it before the model check.
            if let Some(class_name) =
                extract_css_class_at_offset(jsx_attribute, params.cursor_offset)
            {
                return Some(DefinitionReference::CssClass { class_name });
            }
        }

        if let Some(reference) = JsReferenceIdentifier::cast_ref(&ancestor)
            && let Some(semantic_model) = params.services.as_js_services()?.semantic_model.as_ref()
            && let Some(binding) = semantic_model.binding(&reference)
        {
            let binding_syntax = binding.syntax();
            if let Some(specifier) = is_under_import_clause(&binding_syntax) {
                let name = binding.syntax().text_trimmed().to_string();
                return Some(DefinitionReference::Import {
                    local_name: name,
                    specifier: specifier.to_string(),
                });
            }
            if let Some(result) = retrieve_reference_under_dynamic_import(&binding_syntax) {
                return Some(result);
            }
            return Some(DefinitionReference::Local {
                range: binding.syntax().text_trimmed_range(),
            });
        }

        if let Some(reference) = JsxReferenceIdentifier::cast_ref(&ancestor)
            && let Some(semantic_model) = params.services.as_js_services()?.semantic_model.as_ref()
            && let Some(binding) = semantic_model.binding(&reference)
        {
            let binding_syntax = binding.syntax();
            if let Some(specifier) = is_under_import_clause(&binding_syntax) {
                let name = binding_syntax.text_trimmed().to_string();
                return Some(DefinitionReference::Import {
                    local_name: name,
                    specifier: specifier.to_string(),
                });
            }
            if let Some(result) = retrieve_reference_under_dynamic_import(&binding_syntax) {
                return Some(result);
            }
            return Some(DefinitionReference::Local {
                range: binding_syntax.text_trimmed_range(),
            });
        }

        // Try to resolve when cursor is directly on an import binding name.
        // E.g., cursor on `foo` in `import { foo } from './utils'`
        if let Some(binding_node) = AnyJsIdentifierBinding::cast_ref(&ancestor) {
            let binding_range = binding_node.name_token().ok()?.text_trimmed_range();
            let binding_text = binding_node.name_token().ok()?.text_trimmed().to_string();

            if let Some(specifier) = is_under_import_clause(binding_node.syntax()) {
                return Some(DefinitionReference::Import {
                    local_name: binding_text,
                    specifier: specifier.to_string(),
                });
            }
            if let Some(result) = retrieve_reference_under_dynamic_import(binding_node.syntax()) {
                return Some(result);
            }

            return Some(DefinitionReference::Local {
                range: binding_range,
            });
        }
    }

    None
}

/// Checks if a syntax node is under an import clause.
fn is_under_import_clause(node: &JsSyntaxNode) -> Option<TokenText> {
    if !node.ancestors().skip(1).any(|ancestor| {
        matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE
                | JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE
                | JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE
                | JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE
        )
    }) {
        return None;
    }

    let js_import = node.ancestors().skip(1).find_map(JsImport::cast)?;

    js_import.source_text().ok()
}

fn retrieve_reference_under_dynamic_import(
    identifier: &JsSyntaxNode,
) -> Option<DefinitionReference> {
    let declarator = identifier
        .ancestors()
        .skip(1)
        .find_map(JsVariableDeclarator::cast)?;

    let initializer_is_dynamic_import = declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
        .and_then(|expr| expr.as_js_await_expression().cloned())
        .and_then(|expr| expr.argument().ok())
        .and_then(|expr| expr.as_js_import_call_expression().cloned())?;

    let argument = initializer_is_dynamic_import
        .arguments()
        .iter()
        .next()?
        .args()
        .iter()
        .next()?
        .ok()?;
    let argument = argument
        .as_any_js_expression()?
        .as_any_js_literal_expression()?
        .as_js_string_literal_expression()?;

    Some(DefinitionReference::Import {
        local_name: identifier.text_trimmed().to_string(),
        specifier: argument.inner_string_text().ok()?.to_string(),
    })
}

/// Destination-side capability: given a binding reference, resolve the definition location.
pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    let mut result = GoToDefinitionResult::default();
    match params.definition_ref {
        DefinitionReference::Local { range } => {
            result.store(params.path.clone(), *range);
        }
        DefinitionReference::Import {
            local_name,
            specifier,
        } => {
            resolve_import_definition(
                local_name,
                specifier,
                params.path.as_path(),
                params.module_db,
                &mut result,
            );
        }
        DefinitionReference::HtmlComponent { local_name, source } => {
            resolve_import_definition(
                local_name,
                source,
                params.path.as_path(),
                params.module_db,
                &mut result,
            );
        }
        DefinitionReference::LocalEmbedded { range, .. } => {
            if let Some(offset) = params.offset {
                result.store(params.path.clone(), range.add(offset))
            }
        }
        // CssClass is routed to the CSS handler by the orchestrator
        _ => return None,
    };

    Some(result)
}

/// Resolves an imported symbol to its definition in the target module.
fn resolve_import_definition(
    local_name: &str,
    specifier: &str,
    current_path: &Utf8Path,
    module_db: &dyn ModuleDb,
    result: &mut GoToDefinitionResult,
) -> Option<()> {
    let module_info = module_db.module_info_for_path(current_path)?;
    match module_info {
        ModuleInfoKind::Js(module_info) => {
            let import_path = module_info
                .static_import_paths
                .get(specifier)
                .or(module_info.dynamic_import_paths.get(specifier))?;

            let target_path = import_path.resolved_path.as_path()?;

            // Skip files not in the module graph
            if !module_db.contains(target_path) {
                return None;
            }

            let target_module = module_db.js_module_info_for_path(target_path)?;

            match target_module
                .find_js_exported_symbol(module_db, local_name)
                .or(target_module.find_js_default_export_symbol(module_db))
            {
                None => {
                    result.store(
                        BiomePath::new(target_path),
                        TextRange::new(TextSize::from(0), TextSize::from(0)),
                    );
                }
                Some(own_export) => match own_export {
                    JsOwnExport::Binding(range) => result.store(BiomePath::new(target_path), range),
                    JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => {}
                },
            }
        }
        ModuleInfoKind::Css(_) => {}
        ModuleInfoKind::Html(module_info) => {
            let resolved_path = module_info
                .static_import_paths
                .get(specifier)
                .or(module_info.dynamic_import_paths.get(specifier))?;

            let target_path = resolved_path.as_path()?;

            // Skip files not in the module graph
            if !module_db.contains(target_path) {
                return None;
            }

            // Check if we need to resolve from a JS file
            if let Some(module) = module_db.js_module_info_for_path(target_path) {
                match module.find_js_exported_symbol(module_db, local_name) {
                    None => {
                        result.store(
                            BiomePath::new(target_path),
                            TextRange::new(TextSize::from(0), TextSize::from(0)),
                        );
                    }
                    Some(own_export) => match own_export {
                        JsOwnExport::Binding(range) => {
                            result.store(BiomePath::new(target_path), range)
                        }
                        JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => {}
                    },
                }
            }
            // if not, it's the whole file as a component
            else {
                result.store(
                    BiomePath::new(target_path),
                    TextRange::new(TextSize::from(0), TextSize::from(0)),
                );
            }
        }
    };

    Some(())
}

/// Extracts the CSS class name at the given cursor offset from a JSX
/// `className` or `class` attribute string value.
///
/// Given `<div className="foo bar baz">` with cursor on `bar`, returns
/// `Some("bar")`.
fn extract_css_class_at_offset(
    jsx_attribute: JsxAttribute,
    cursor_offset: TextSize,
) -> Option<String> {
    let name_token = jsx_attribute.name_value_token().ok()?;
    let name_text = name_token.text_trimmed();

    if name_text != "className" && name_text != "class" {
        return None;
    }

    let initializer = jsx_attribute.initializer()?;
    let value = initializer.value().ok()?;

    let string_literal: JsxString = match value {
        AnyJsxAttributeValue::JsxString(s) => s,
        _ => return None,
    };

    let value_token = string_literal.value_token().ok()?;
    let inner_text = string_literal.inner_string_text().ok()?;
    let inner_source_range = inner_text.source_range(value_token.text_trimmed_range());

    let relative_offset = cursor_offset.checked_sub(inner_source_range.start())?;
    let relative_offset: usize = relative_offset.into();

    let text = inner_text.text();
    if relative_offset > text.len() {
        return None;
    }

    // Find which whitespace-separated class name the cursor falls within
    let mut pos = 0usize;
    for class_name in text.split_ascii_whitespace() {
        // Find actual start (skip whitespace)
        let start = text[pos..].find(class_name).map(|i| i + pos)?;
        let end = start + class_name.len();

        if relative_offset >= start && relative_offset < end {
            return Some(class_name.to_string());
        }

        pos = end;
    }

    None
}
