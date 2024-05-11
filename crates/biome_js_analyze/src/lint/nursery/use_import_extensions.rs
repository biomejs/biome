use std::path::{Component, Path};

use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{inner_string_text, AnyJsImportSpecifierLike, JsLanguage};
use biome_rowan::{BatchMutationExt, SyntaxToken};

use crate::JsRuleAction;

declare_rule! {
    /// Require import extensions for relative imports.
    ///
    /// Browsers and Node.js do not natively support importing files without extensions. This rule
    /// enforces the use of import extensions for relative imports to make the code more consistent.
    ///
    /// Tooling also benefits from explicit import extensions, because they do not need to guess which
    /// file to resolve.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import "./foo";
    /// ```
    /// ```js,expect_diagnostic
    /// import "./bar/";
    /// ```
    /// ```js,expect_diagnostic
    /// import "../";
    /// ```
    /// ```js,expect_diagnostic
    /// import "../.";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import "biome";
    /// ```
    /// ```js
    /// import "./foo.js";
    /// ```
    /// ```js
    /// import "./bar/index.js";
    /// ```
    ///
    /// ## Caveats
    ///
    /// If you are using TypeScript, TypeScript version 5.0 and later is required, also make sure to enable
    /// [allowImportingTsExtensions=true](https://typescriptlang.org/tsconfig#allowImportingTsExtensions) in your `tsconfig.json`.
    ///
    /// Rule does not yet check filesystem for file type. It tries to guess which extension
    /// it should add based on the file extension of the current file and the import path.
    /// When applying the suggested fix, make sure to verify that the file type is correct.
    ///
    pub UseImportExtensions {
        version: "next",
        name: "useImportExtensions",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseImportExtensions {
    type Query = Ast<AnyJsImportSpecifierLike>;
    type State = UseImportExtensionsState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let file_ext = ctx.file_path().extension()?.to_str()?;

        get_extensionless_import(file_ext, node)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.module_name_token.text_range(),
                markup! {
                    "Add a file extension for relative imports."
                },
            )
            .note(markup! {
                "Explicit import improves compatibility with browsers and makes file resolution in tooling faster."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let (suggested_path, extension) = state.suggestion.clone()?;
        let new_module_name = if ctx.as_preferred_quote().is_double() {
            make::js_string_literal(&suggested_path)
        } else {
            make::js_string_literal_single_quotes(&suggested_path)
        };

        mutation.replace_element(
            state.module_name_token.clone().into(),
            new_module_name.into(),
        );

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! {
                "Add potential import extension "<Emphasis>"."{extension}</Emphasis>"."
            }
            .to_owned(),
            mutation,
        })
    }
}

pub struct UseImportExtensionsState {
    suggestion: Option<(String, String)>,
    module_name_token: SyntaxToken<JsLanguage>,
}

fn get_extensionless_import(
    file_ext: &str,
    node: &AnyJsImportSpecifierLike,
) -> Option<UseImportExtensionsState> {
    let module_name_token = node.module_name_token()?;
    let module_path = inner_string_text(&module_name_token);
    let path = Path::new(module_path.text());
    let mut path_components = path.components();
    let first_component = path_components.next()?;

    if !matches!(first_component, Component::CurDir | Component::ParentDir)
        || path.extension().is_some()
    {
        return None;
    }

    let last_component = path_components.last().unwrap_or(first_component);
    let has_query_or_hash = last_component
        .as_os_str()
        .to_str()
        .map_or(false, |last| last.contains('?') || last.contains('#'));

    if has_query_or_hash {
        return Some(UseImportExtensionsState {
            module_name_token,
            suggestion: None,
        });
    }

    let import_ext = resolve_import_extension(file_ext, path);
    let mut path_buf = path.to_path_buf();

    let is_index_file = match last_component {
        Component::ParentDir => true,
        // `import ".././"` is the same as `import "../"`
        // Rust Path does not expose `./` path segment at very end, likely because it does not do anything.
        // To provide proper fix, we need to remove it as well.
        Component::Normal(os_str) if module_path.ends_with("./") || module_path.ends_with('.') => {
            if let Some(base_name) = os_str.to_str() {
                path_buf.set_file_name(base_name);

                true
            } else {
                false
            }
        }
        _ if module_path.ends_with('/') => true,
        _ => false,
    };

    if is_index_file {
        let part = format!("index.{}", import_ext);
        path_buf.push(part);
    } else {
        path_buf.set_extension(import_ext);
    }

    Some(UseImportExtensionsState {
        module_name_token: module_name_token.clone(),
        suggestion: Some((
            path_buf.to_string_lossy().to_string(),
            import_ext.to_string(),
        )),
    })
}

fn resolve_import_extension<'a>(file_ext: &str, path: &Path) -> &'a str {
    // TODO. This is not very accurate. We should use file system access to determine the file type.
    let (potential_ext, potential_component_ext) = match file_ext {
        "ts" | "tsx" | "astro" => ("ts", "tsx"),
        "mts" => ("mts", "tsx"),
        "mjs" => ("mjs", "jsx"),
        "cjs" => ("cjs", "jsx"),
        "cts" => ("cts", "tsx"),
        // Unlikely that these frameworks would import tsx file.
        "svelte" | "vue" => ("ts", "ts"),
        _ => ("js", "jsx"),
    };

    let maybe_is_component = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.chars().next())
        .is_some_and(|c| c.is_uppercase());

    if maybe_is_component {
        return potential_component_ext;
    }

    potential_ext
}
