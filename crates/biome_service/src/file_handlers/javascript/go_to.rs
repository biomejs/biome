use crate::file_handlers::javascript::extract_css_class_at_offset;
use crate::file_handlers::{ResolveBindingParams, ResolveDefinitionParams};
use crate::workspace::{DefinitionReference, GoToDefinitionResult};
use biome_css_syntax::{TextRange, TextSize};
use biome_fs::BiomePath;
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsRoot, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsVariableDeclarator,
    JsxReferenceIdentifier,
};
use biome_js_type_info::ImportSymbol;
use biome_module_graph::{JsOwnExport, ModuleGraph};
use biome_rowan::{AstNode, AstSeparatedList, TokenAtOffset};
use camino::Utf8Path;

/// Source-side capability: given a cursor position, identify what binding the user clicked on.
pub(crate) fn resolve_binding(params: ResolveBindingParams) -> Option<DefinitionReference> {
    let root: AnyJsRoot = params.parse.tree();

    let token = match root.syntax().token_at_offset(params.cursor_offset) {
        TokenAtOffset::Single(token) => token,
        TokenAtOffset::Between(_, right) => right,
        TokenAtOffset::None => return None,
    };

    // Check if cursor is inside a JSX className/class attribute string.
    // This doesn't need the semantic model, so try it before the model check.
    if let Some(class_name) = extract_css_class_at_offset(&token, params.cursor_offset) {
        return Some(DefinitionReference::CssClass { class_name });
    }

    let semantic_model = params.services.as_js_services()?.semantic_model.as_ref()?;

    for ancestor in token.ancestors() {
        if let Some(reference) = JsReferenceIdentifier::cast_ref(&ancestor)
            && let Some(binding) = semantic_model.binding(&reference)
        {
            let binding_syntax = binding.syntax();
            if is_under_import_clause(&binding_syntax) {
                let name = binding.syntax().text_trimmed().to_string();
                return Some(DefinitionReference::Import { local_name: name });
            }
            if let Some(result) = retrieve_reference_under_dynamic_import(&binding_syntax) {
                return Some(result);
            }
            return Some(DefinitionReference::Local {
                range: binding.syntax().text_trimmed_range(),
            });
        }

        if let Some(reference) = JsxReferenceIdentifier::cast_ref(&ancestor)
            && let Some(binding) = semantic_model.binding(&reference)
        {
            let binding_syntax = binding.syntax();
            if is_under_import_clause(&binding_syntax) {
                let name = binding_syntax.text_trimmed().to_string();
                return Some(DefinitionReference::Import { local_name: name });
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

            if is_under_import_clause(binding_node.syntax()) {
                return Some(DefinitionReference::Import {
                    local_name: binding_text,
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
fn is_under_import_clause(node: &JsSyntaxNode) -> bool {
    node.ancestors().skip(1).any(|ancestor| {
        matches!(
            ancestor.kind(),
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE
                | JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE
                | JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE
                | JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE
        )
    })
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

    Some(DefinitionReference::DynamicImport {
        local_name: identifier.text_trimmed().to_string(),
        specifier: argument.inner_string_text().ok()?.to_string(),
    })
}

/// Destination-side capability: given a binding reference, resolve the definition location.
pub(crate) fn resolve_definition(params: ResolveDefinitionParams) -> Option<GoToDefinitionResult> {
    match params.definition_ref {
        DefinitionReference::Local { range } => Some(GoToDefinitionResult {
            path: BiomePath::new(params.path.as_path().to_string()),
            range: *range,
        }),
        DefinitionReference::Import { local_name } => {
            resolve_import_definition(local_name, params.path.as_path(), params.module_graph)
        }
        DefinitionReference::DynamicImport {
            local_name,
            specifier,
        } => resolve_dynamic_import_definition(
            local_name,
            specifier,
            params.path.as_path(),
            params.module_graph,
        ),
        // CssClass is routed to the CSS handler by the orchestrator
        _ => None,
    }
}

/// Resolves an imported symbol to its definition in the target module.
fn resolve_import_definition(
    local_name: &str,
    current_path: &Utf8Path,
    module_graph: &ModuleGraph,
) -> Option<GoToDefinitionResult> {
    let module_info = module_graph.js_module_info_for_path(current_path)?;
    let js_import = module_info.static_imports.get(local_name)?;

    let target_path = js_import.resolved_path.as_path()?;

    // Skip files not in the module graph
    if !module_graph.contains(target_path) {
        return None;
    }

    let target_module = module_graph.js_module_info_for_path(target_path)?;

    let export_name = match &js_import.symbol {
        ImportSymbol::Named(name) => name.text(),
        ImportSymbol::Default => "default",
        ImportSymbol::All => {
            // Namespace import: navigate to the target module file
            return Some(GoToDefinitionResult {
                path: BiomePath::new(target_path.to_string()),
                range: TextRange::new(TextSize::from(0), TextSize::from(0)),
            });
        }
    };

    let own_export = target_module.find_js_exported_symbol(module_graph, export_name)?;

    match own_export {
        JsOwnExport::Binding(range) => Some(GoToDefinitionResult {
            path: BiomePath::new(target_path.to_string()),
            range,
        }),
        // Type-only exports and namespace exports don't have a binding location
        JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => None,
    }
}

fn resolve_dynamic_import_definition(
    local_name: &str,
    specifier: &str,
    current_path: &Utf8Path,
    module_graph: &ModuleGraph,
) -> Option<GoToDefinitionResult> {
    let module_info = module_graph.js_module_info_for_path(current_path)?;
    let import_path = module_info.dynamic_import_paths.get(specifier)?;
    let target_path = import_path.resolved_path.as_path()?;

    if !module_graph.contains(target_path) {
        return None;
    }

    let target_module = module_graph.js_module_info_for_path(target_path)?;
    match target_module.find_js_exported_symbol(module_graph, local_name) {
        // In this case we found the file, but we don't know the symbol, which means that module
        // imported like `const foo = await import('./foo')`. In this case, we send the user
        // to the top of the file.
        None => Some(GoToDefinitionResult {
            path: BiomePath::new(target_path.to_string()),
            range: TextRange::new(TextSize::from(0), TextSize::from(0)),
        }),
        Some(own_export) => match own_export {
            JsOwnExport::Binding(range) => Some(GoToDefinitionResult {
                path: BiomePath::new(target_path.to_string()),
                range,
            }),
            JsOwnExport::Type(_) | JsOwnExport::Namespace(_) => None,
        },
    }
}
