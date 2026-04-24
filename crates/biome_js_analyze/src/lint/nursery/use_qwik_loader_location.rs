use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExportClause, AnyJsExportNamedSpecifier, AnyJsModuleItem, AnyJsRoot, JsCallExpression,
    JsExport, JsVariableDeclaration, JsVariableDeclarationClause, JsVariableDeclarator,
    JsVariableDeclaratorList,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText};
use biome_rule_options::use_qwik_loader_location::UseQwikLoaderLocationOptions;

/// Qwik route functions that must be in route boundary files.
const ROUTE_FNS: &[&str] = &["routeLoader$", "routeAction$"];

/// All Qwik loader/action functions that must be exported with a `use*` name.
const LINTER_FNS: &[&str] = &["routeLoader$", "routeAction$", "globalAction$"];

declare_lint_rule! {
    /// Enforce that Qwik loader functions are declared in the correct location.
    ///
    /// Route functions like `routeLoader$`, `routeAction$` must be declared in route boundary files
    /// (`index`, `layout`, or `plugin` files inside the configured routes directory).
    /// All loader/action functions must also be exported from the module and follow the `use*` naming convention.
    ///
    /// See the [Qwik documentation](https://qwik.dev/docs/route-loader/) for more details.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic,file=src/components/product.jsx
    /// // src/components/product.jsx
    /// import { routeLoader$ } from '@builder.io/qwik-city';
    /// export const useProducts = routeLoader$(async () => {});
    /// ```
    ///
    /// ```jsx,expect_diagnostic,file=src/routes/index.jsx
    /// // src/routes/index.jsx
    /// import { routeLoader$ } from '@builder.io/qwik-city';
    /// export const getProducts = routeLoader$(async () => {});
    /// ```
    ///
    /// ```jsx,expect_diagnostic,file=src/routes/index.jsx
    /// // src/routes/index.jsx
    /// import { routeLoader$ } from '@builder.io/qwik-city';
    /// const useProducts = routeLoader$(async () => {});
    /// ```
    ///
    /// ```jsx,expect_diagnostic,file=src/routes/index.jsx
    /// // src/routes/index.jsx
    /// import { routeLoader$ } from '@builder.io/qwik-city';
    /// async function fetcher() {}
    /// const useProducts = routeLoader$(fetcher);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx,file=src/routes/index.jsx
    /// // src/routes/index.jsx
    /// import { routeLoader$ } from '@builder.io/qwik-city';
    /// export const useProducts = routeLoader$(async () => {});
    /// ```
    ///
    pub UseQwikLoaderLocation {
        version: "2.4.11",
        name: "useQwikLoaderLocation",
        language: "js",
        sources: &[RuleSource::EslintQwik("loader-location").same()],
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseQwikLoaderLocation {
    type Query = Ast<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseQwikLoaderLocationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        let callee = call.callee().ok()?.omit_parentheses();
        let callee_ident_expr = callee.as_js_identifier_expression()?;
        let callee_ref_ident = callee_ident_expr.name().ok()?;
        let callee_name = callee_ref_ident.value_token().ok()?;
        let callee_token_text = callee_name.token_text_trimmed();
        let callee_name_text = callee_token_text.text();

        if !LINTER_FNS.contains(&callee_name_text) {
            return None;
        }

        // Check if callee must be placed in routes dir
        if ROUTE_FNS.contains(&callee_name_text) {
            // Normalize path separators to forward slashes.
            let file_path = ctx.file_path().as_str().replace('\\', "/");
            let file_name = ctx.file_path().file_name()?;

            let is_inside_routes =
                file_path.starts_with("src/routes/") || file_path.contains("/src/routes/");
            let is_route_file = file_name.starts_with("plugin")
                || file_name.starts_with("layout")
                || file_name.starts_with("index");

            if !(is_inside_routes && is_route_file) {
                return Some(RuleState::InvalidLoaderLocation {
                    fn_name: callee_token_text,
                });
            }
        }

        // Check parent structure: must be `const <id> = <fn>()`
        let declarator = call
            .syntax()
            .parent()
            .and_then(|n| n.parent())
            .and_then(JsVariableDeclarator::cast);

        let Some(declarator) = declarator else {
            // The call is not inside a variable declaration — not exported.
            return Some(RuleState::MissingExport {
                fn_name: callee_token_text,
                span: callee_ref_ident.range(),
            });
        };

        // Check that the binding is a plain identifier, not a pattern
        let binding = declarator.id().ok()?;
        let binding = binding
            .as_any_js_binding()
            .and_then(|b| b.as_js_identifier_binding());

        let Some(binding) = binding else {
            // Destructured or otherwise non-identifier binding.
            return Some(RuleState::MissingExport {
                fn_name: callee_token_text,
                span: callee_ref_ident.range(),
            });
        };

        let id_token = binding.name_token().ok()?;
        let id_name = id_token.token_text_trimmed();
        let span = binding.range();

        // Check naming convention
        if !id_name.text().starts_with("use") {
            return Some(RuleState::WrongName {
                fn_name: callee_token_text,
                span,
            });
        }

        // Check if exported
        if !is_exported(&declarator, &id_name, &ctx.root()) {
            return Some(RuleState::MissingExport {
                fn_name: callee_token_text,
                span,
            });
        }

        // Check if call argument is reference
        let args = call.arguments().ok()?.args();
        let first_arg = args.first()?.ok()?;
        let any_js_expr = first_arg.as_any_js_expression()?;
        if let Some(ident) = any_js_expr.as_js_reference_identifier() {
            return Some(RuleState::RecommendedValue {
                span: ident.range(),
            });
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        match state {
            RuleState::InvalidLoaderLocation { fn_name } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.range(),
                    markup! {
                        "The route function "<Emphasis>{fn_name.text()}</Emphasis>"() has been declared outside of the route boundaries."
                    },
                )
                .note(markup! {
                    "Route functions are typically declared within the route boundaries as they are tied to a specific route, if not, Qwik can't associate them with a route."
                })
                .note(markup! {
                    "Route boundary files are "<Emphasis>"index"</Emphasis>", "<Emphasis>"layout"</Emphasis>", and "<Emphasis>"plugin"</Emphasis>" files inside the \""<Emphasis>"src/routes"</Emphasis>"\" directory."
                })
                .note(markup! {
                    "Move the file within the route boundaries or if you want to create reusable logic, you must re-export from within the router boundary.\nSee the "<Hyperlink href="https://qwik.dev/docs/re-exporting-loaders/">"Qwik docs on re-exporting loaders"</Hyperlink>" for details."
                }),
            ),
            RuleState::MissingExport { fn_name, span } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "The loader function "<Emphasis>{fn_name.text()}</Emphasis>"() is not being exported."
                    },
                )
                .note(markup! {
                    "A loader function must be exported, if not, the loader will not run. Make sure to export the loader."
                }),
            ),
            RuleState::WrongName { fn_name, span } => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "The exported name of "<Emphasis>{fn_name.text()}</Emphasis>"() must follow the "<Emphasis>"use*"</Emphasis>" naming convention."
                    },
                )
                .note(markup! {
                    "Rename the declaration to start with \""<Emphasis>"use"</Emphasis>"\"."
                }),
            ),
            RuleState::RecommendedValue { span} => Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    span,
                    markup! {
                        "It is recommended to inline the arrow function instead of passing a reference. "
                    },
                )
                .note(markup! {
                    "An inline arrow function will help the optimizer to make sure that no server code is leaked to the client build. Replace the reference with an inline arrow function."
                }),
            ),
        }
    }
}

pub enum RuleState {
    /// The loader is declared outside of a route boundary file.
    InvalidLoaderLocation { fn_name: TokenText },
    /// The loader return value is not exported from the module.
    MissingExport { fn_name: TokenText, span: TextRange },
    /// The exported name does not follow the `use*` convention.
    WrongName { fn_name: TokenText, span: TextRange },
    /// The first argument is a reference instead of an inlined arrow function.
    RecommendedValue { span: TextRange },
}

/// Returns `true` if the given `declarator` is exported from the module.
///
/// Handles two cases:
/// 1. `export const x = ...` — the declaration is directly under an `export` keyword.
/// 2. `const x = ...; export { x }` — a separate named export specifier.
fn is_exported(declarator: &JsVariableDeclarator, name: &str, root: &AnyJsRoot) -> bool {
    // Case 1: inline `export const x = ...`
    //
    // Parent chain: declarator → declarator_list → variable_declaration → variable_declaration_clause → export
    let inline_exported = declarator
        .parent::<JsVariableDeclaratorList>()
        .and_then(|l| l.parent::<JsVariableDeclaration>())
        .and_then(|d| d.parent::<JsVariableDeclarationClause>())
        .and_then(|c| c.parent::<JsExport>())
        .is_some();

    if inline_exported {
        return true;
    }

    // Case 2: `export { x }` or `export { x as y }` elsewhere in the module.
    let items = match root {
        AnyJsRoot::JsModule(m) => m.items(),
        _ => return false,
    };

    for item in items {
        let AnyJsModuleItem::JsExport(export) = item else {
            continue;
        };
        let Ok(AnyJsExportClause::JsExportNamedClause(named_clause)) = export.export_clause()
        else {
            continue;
        };
        for specifier in named_clause.specifiers() {
            let Ok(specifier) = specifier else { continue };
            let local_name = match &specifier {
                AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(s) => s
                    .name()
                    .ok()
                    .and_then(|r| r.value_token().ok())
                    .map(|t| t.token_text_trimmed()),
                AnyJsExportNamedSpecifier::JsExportNamedSpecifier(s) => s
                    .local_name()
                    .ok()
                    .and_then(|r| r.value_token().ok())
                    .map(|t| t.token_text_trimmed()),
            };
            if local_name.is_some_and(|f| f == name) {
                return true;
            }
        }
    }

    false
}
