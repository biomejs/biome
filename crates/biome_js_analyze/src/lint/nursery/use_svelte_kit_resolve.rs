use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsNamedImportSpecifier,
    AnyTsName, AnyTsType, AnyTsVariableAnnotation, JsCallExpression, JsImport, JsSyntaxNode,
    JsSyntaxToken, TsTypeAnnotation, binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::use_svelte_kit_resolve::UseSvelteKitResolveOptions;

declare_lint_rule! {
    /// Require internal navigation in SvelteKit apps to use paths built with `resolve()`.
    ///
    /// SvelteKit's `resolve()` from `$app/paths` prefixes a route with the app's
    /// [base path](https://svelte.dev/docs/kit/configuration#paths) and checks it against the
    /// app's routes. Passing a hand-written path to `goto()`, `pushState()`, or `replaceState()`
    /// breaks navigation when the app is served from a sub-path, and lets typos in routes go unnoticed.
    ///
    /// `pushState()` and `replaceState()` also accept an empty string, which keeps the current URL
    /// for [shallow routing](https://svelte.dev/docs/kit/shallow-routing).
    ///
    /// Paths are also accepted when they come from a variable whose initializer uses `resolve()`,
    /// or from a variable or parameter annotated with the `ResolvedPathname` type from `$app/types`.
    ///
    /// For navigation to external URLs, use `window.location` instead of `goto()`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { goto } from "$app/navigation";
    ///
    /// goto("/foo");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { goto } from "$app/navigation";
    /// import { resolve } from "$app/paths";
    ///
    /// goto(resolve("/foo") + "/bar");
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { pushState } from "$app/navigation";
    ///
    /// pushState("/foo", {});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { goto, pushState, replaceState } from "$app/navigation";
    /// import { resolve } from "$app/paths";
    ///
    /// goto(resolve("/foo"));
    /// pushState(resolve("/foo"), {});
    /// replaceState(resolve("/foo"), {});
    ///
    /// // Shallow routing
    /// pushState("", {});
    /// replaceState("", {});
    /// ```
    ///
    /// ```ts
    /// import { goto } from "$app/navigation";
    /// import type { ResolvedPathname } from "$app/types";
    ///
    /// function navigate(path: ResolvedPathname) {
    ///     goto(path);
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// ### `ignoreGoto`
    ///
    /// Whether to ignore all `goto()` calls. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreGoto": true
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// import { goto } from "$app/navigation";
    ///
    /// goto("/foo");
    /// ```
    ///
    /// ### `ignorePushState`
    ///
    /// Whether to ignore all `pushState()` calls. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignorePushState": true
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// import { pushState } from "$app/navigation";
    ///
    /// pushState("/foo", {});
    /// ```
    ///
    /// ### `ignoreReplaceState`
    ///
    /// Whether to ignore all `replaceState()` calls. Default: `false`.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "ignoreReplaceState": true
    ///     }
    /// }
    /// ```
    ///
    /// ```js,use_options
    /// import { replaceState } from "$app/navigation";
    ///
    /// replaceState("/foo", {});
    /// ```
    ///
    /// ## References
    ///
    /// - [`resolve()` documentation](https://svelte.dev/docs/kit/$app-paths#resolve)
    /// - [`goto()` documentation](https://svelte.dev/docs/kit/$app-navigation#goto)
    /// - [`pushState()` documentation](https://svelte.dev/docs/kit/$app-navigation#pushState)
    /// - [`replaceState()` documentation](https://svelte.dev/docs/kit/$app-navigation#replaceState)
    pub UseSvelteKitResolve {
        version: "next",
        name: "useSvelteKitResolve",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("no-navigation-without-resolve").same()],
    }
}

const APP_NAVIGATION: &str = "$app/navigation";
const APP_PATHS: &str = "$app/paths";
const APP_TYPES: &str = "$app/types";

/// How many variable initializers to follow before giving up on a path.
const MAX_INITIALIZER_DEPTH: u8 = 8;

impl Rule for UseSvelteKitResolve {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = UseSvelteKitResolveOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        let callee = call.callee().ok()?.omit_parentheses();
        let function = NavigationFunction::from_callee(&callee, model)?;
        if function.is_ignored(ctx.options()) {
            return None;
        }

        let AnyJsCallArgument::AnyJsExpression(path) = call.arguments().ok()?.args().first()?.ok()?
        else {
            return None;
        };

        if is_allowed_path(&path, model, function.allows_empty_path(), MAX_INITIALIZER_DEPTH) {
            return None;
        }

        Some(RuleState {
            function,
            path_range: path.range(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let function = state.function;
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.path_range,
            markup! {
                "The path passed to "<Emphasis>{function}</Emphasis>" isn't built with "<Emphasis>"resolve()"</Emphasis>"."
            },
        )
        .note(markup! {
            "Paths that aren't built with "<Emphasis>"resolve()"</Emphasis>" lack the app's base path, and SvelteKit can't check them against the app's routes."
        });

        let diagnostic = if function.allows_empty_path() {
            diagnostic.note(markup! {
                "Wrap the path in "<Emphasis>"resolve()"</Emphasis>" from "<Emphasis>"$app/paths"</Emphasis>", or pass an empty string to keep the current URL."
            })
        } else {
            diagnostic.note(markup! {
                "Wrap the path in "<Emphasis>"resolve()"</Emphasis>" from "<Emphasis>"$app/paths"</Emphasis>". To navigate to an external URL, use "<Emphasis>"window.location"</Emphasis>" instead."
            })
        };

        Some(diagnostic)
    }
}

#[derive(Debug)]
pub struct RuleState {
    function: NavigationFunction,
    path_range: TextRange,
}

/// A navigation function exported by `$app/navigation`.
#[derive(Debug, Clone, Copy)]
pub enum NavigationFunction {
    Goto,
    PushState,
    ReplaceState,
}

impl NavigationFunction {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "goto" => Some(Self::Goto),
            "pushState" => Some(Self::PushState),
            "replaceState" => Some(Self::ReplaceState),
            _ => None,
        }
    }

    /// Returns the navigation function called by `callee`, if `callee` refers to an
    /// import from `$app/navigation`.
    fn from_callee(callee: &AnyJsExpression, model: &SemanticModel) -> Option<Self> {
        let name = imported_member_name(callee, model, APP_NAVIGATION)?;
        Self::from_name(name.text_trimmed())
    }

    fn is_ignored(self, options: &UseSvelteKitResolveOptions) -> bool {
        match self {
            Self::Goto => options.ignore_goto(),
            Self::PushState => options.ignore_push_state(),
            Self::ReplaceState => options.ignore_replace_state(),
        }
    }

    /// `pushState()` and `replaceState()` accept an empty string to keep the current URL.
    const fn allows_empty_path(self) -> bool {
        matches!(self, Self::PushState | Self::ReplaceState)
    }
}

impl biome_console::fmt::Display for NavigationFunction {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Goto => "goto()",
            Self::PushState => "pushState()",
            Self::ReplaceState => "replaceState()",
        })
    }
}

/// Returns `true` if `path` is known to be built with `resolve()`.
fn is_allowed_path(
    path: &AnyJsExpression,
    model: &SemanticModel,
    allow_empty: bool,
    depth: u8,
) -> bool {
    // Stop following initializers that are too deep or cyclic instead of reporting them.
    let Some(depth) = depth.checked_sub(1) else {
        return true;
    };

    match path.clone().omit_parentheses() {
        AnyJsExpression::JsConditionalExpression(conditional) => {
            conditional.consequent().is_ok_and(|consequent| {
                is_allowed_path(&consequent, model, allow_empty, depth)
            }) && conditional.alternate().is_ok_and(|alternate| {
                is_allowed_path(&alternate, model, allow_empty, depth)
            })
        }
        AnyJsExpression::JsCallExpression(call) => is_resolve_call(&call, model),
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let Some(binding) = identifier
                .name()
                .ok()
                .and_then(|reference| model.binding(&reference))
            else {
                return false;
            };
            match binding.tree().declaration() {
                Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) => {
                    let annotation = declarator.variable_annotation().and_then(|annotation| {
                        match annotation {
                            AnyTsVariableAnnotation::TsTypeAnnotation(annotation) => {
                                Some(annotation)
                            }
                            AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(annotation) => {
                                annotation.type_annotation().ok()
                            }
                        }
                    });
                    if annotation.is_some_and(|annotation| {
                        is_resolved_pathname_annotation(&annotation, model)
                    }) {
                        return true;
                    }
                    declarator
                        .initializer()
                        .and_then(|initializer| initializer.expression().ok())
                        .is_some_and(|initializer| {
                            is_allowed_path(&initializer, model, allow_empty, depth)
                        })
                }
                Some(AnyJsBindingDeclaration::JsFormalParameter(parameter)) => parameter
                    .type_annotation()
                    .is_some_and(|annotation| is_resolved_pathname_annotation(&annotation, model)),
                _ => false,
            }
        }
        path => {
            allow_empty
                && path
                    .as_static_value()
                    .is_some_and(|value| value.as_string_constant() == Some(""))
        }
    }
}

/// Returns `true` if `call` calls `resolve()` or `asset()` from `$app/paths`.
fn is_resolve_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    call.callee()
        .ok()
        .and_then(|callee| imported_member_name(&callee.omit_parentheses(), model, APP_PATHS))
        .is_some_and(|name| matches!(name.text_trimmed(), "resolve" | "asset"))
}

/// Returns `true` if `annotation` is the `ResolvedPathname` type from `$app/types`.
fn is_resolved_pathname_annotation(annotation: &TsTypeAnnotation, model: &SemanticModel) -> bool {
    let Ok(AnyTsType::TsReferenceType(reference_type)) = annotation.ty() else {
        return false;
    };
    let Ok(AnyTsName::JsReferenceIdentifier(name)) = reference_type.name() else {
        return false;
    };
    model
        .binding(&name)
        .and_then(|binding| named_import_name(&binding, APP_TYPES))
        .is_some_and(|name| name.text_trimmed() == "ResolvedPathname")
}

/// Returns the name that `expression` imports from `module`.
///
/// This supports named imports, such as `goto` in `import { goto } from "$app/navigation"`,
/// and members of namespace imports, such as `navigation.goto` in
/// `import * as navigation from "$app/navigation"`.
fn imported_member_name(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    module: &str,
) -> Option<JsSyntaxToken> {
    match expression {
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let binding = model.binding(&identifier.name().ok()?)?;
            named_import_name(&binding, module)
        }
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let object = member.object().ok()?.omit_parentheses();
            let binding = model.binding(&object.as_js_reference_identifier()?)?;
            let Some(AnyJsBindingDeclaration::JsNamespaceImportSpecifier(specifier)) =
                binding.tree().declaration()
            else {
                return None;
            };
            if !is_imported_from(specifier.syntax(), module) {
                return None;
            }
            member.member().ok()?.as_js_name()?.value_token().ok()
        }
        _ => None,
    }
}

/// Returns the imported name of `binding` if it's a named import from `module`.
fn named_import_name(binding: &Binding, module: &str) -> Option<JsSyntaxToken> {
    let specifier: AnyJsNamedImportSpecifier = match binding.tree().declaration()? {
        AnyJsBindingDeclaration::JsNamedImportSpecifier(specifier) => specifier.into(),
        AnyJsBindingDeclaration::JsShorthandNamedImportSpecifier(specifier) => specifier.into(),
        _ => return None,
    };
    if !is_imported_from(specifier.syntax(), module) {
        return None;
    }
    specifier.imported_name()
}

fn is_imported_from(specifier: &JsSyntaxNode, module: &str) -> bool {
    specifier
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| source.text() == module)
}
