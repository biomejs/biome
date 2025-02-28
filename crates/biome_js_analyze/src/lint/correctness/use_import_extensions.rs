use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::path::{Component, Path};

use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{inner_string_text, AnyJsImportLike, JsSyntaxToken};
use biome_rowan::BatchMutationExt;

use crate::JsRuleAction;

#[cfg(feature = "schemars")]
use schemars::JsonSchema;

declare_lint_rule! {
    /// Enforce file extensions for relative imports.
    ///
    /// Browsers and Node.js do not natively support importing files without extensions. This rule
    /// enforces the use of file extensions for relative imports to make the code more consistent.
    ///
    /// Tooling also benefits from explicit file extensions, because they do not need to guess which
    /// file to resolve.
    ///
    /// The rule checks static imports and dynamic imports calls such as `import()` and `require()`.
    ///
    /// To ensure that Visual Studio Code adds the file extension when it automatically imports a variable,
    /// you may set [`javascript.preferences.importModuleSpecifierEnding` and `typescript.preferences.importModuleSpecifierEnding`](https://code.visualstudio.com/docs/getstarted/settings) to the desired file extension.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import "./foo";
    /// ```
    /// ```js,expect_diagnostic
    /// import "./foo/";
    /// ```
    /// ```js,expect_diagnostic
    /// import "../";
    /// ```
    /// ```js,expect_diagnostic
    /// import "../.";
    /// ```
    /// ```js,expect_diagnostic
    /// import("./foo");
    /// ```
    /// ```js,expect_diagnostic
    /// require("./foo");
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
    /// ```js
    /// import("./foo.js");
    /// ```
    /// ```js
    /// require("./foo.js");
    /// ```
    ///
    /// ### Options
    ///
    /// Use the options to specify the correct import extensions for your project based on the linted
    /// file extension. These mappings will override the rule's default logic.
    ///
    /// Currently, Biome determines the import extension based on the inspected file extension.
    /// The `suggestedExtensions` option works as a map, where the key is the source file extension
    /// and the value should provide two possible mappings for imports:
    ///
    ///  - `module` is used for module imports that start with a lower-case character, e.g. `foo.js`
    ///  - `component` is used for component files that start with an upper-case character, e.g. `Foo.jsx` (which is a common convention for React JSX)
    ///
    /// For example, if you want `.ts` files to import other modules as `.js` (or `.jsx`), you should
    /// configure the following options in your Biome config:
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "suggestedExtensions": {
    ///             "ts": {
    ///                 "module": "js",
    ///                 "component": "jsx"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// :::caution
    /// Mainly, this is a temporary workaround that allows Biome to propose correct import extensions
    /// for TypeScript projects that use ES Modules. TypeScript requires you to specify imports to
    /// the actual files used in runtime: `.js` or `.mjs` (see more here: https://github.com/microsoft/TypeScript/issues/49083#issuecomment-1435399267).
    /// :::
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
        version: "1.8.0",
        name: "useImportExtensions",
        language: "js",
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

#[derive(Clone, Debug, Default, Deserializable, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseImportExtensionsOptions {
    /// A map of custom import extension mappings, where the key is the inspected file extension,
    /// and the value is a pair of `module` extension and `component` import extension
    pub suggested_extensions: FxHashMap<Box<str>, SuggestedExtensionMapping>,
}

#[derive(Debug, Clone, Default, Deserializable, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct SuggestedExtensionMapping {
    /// Extension that should be used for module imports
    pub module: Box<str>,
    /// Extension that should be used for component file imports
    pub component: Box<str>,
}

impl Rule for UseImportExtensions {
    type Query = Ast<AnyJsImportLike>;
    type State = UseImportExtensionsState;
    type Signals = Option<Self::State>;
    type Options = Box<UseImportExtensionsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let file_ext = ctx.file_path().extension()?;

        let custom_suggested_imports = &ctx.options().suggested_extensions;

        get_extensionless_import(file_ext, node, custom_suggested_imports)
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

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {
                "Add potential import extension "<Emphasis>"."{extension}</Emphasis>"."
            }
            .to_owned(),
            mutation,
        ))
    }
}

pub struct UseImportExtensionsState {
    suggestion: Option<(String, String)>,
    module_name_token: JsSyntaxToken,
}

fn get_extensionless_import(
    file_ext: &str,
    node: &AnyJsImportLike,
    custom_suggested_imports: &FxHashMap<Box<str>, SuggestedExtensionMapping>,
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
        .is_some_and(|last| last.contains('?') || last.contains('#'));

    if has_query_or_hash {
        return Some(UseImportExtensionsState {
            module_name_token,
            suggestion: None,
        });
    }

    let import_ext = resolve_import_extension(file_ext, path, custom_suggested_imports);

    let mut path_parts = module_path.text().split('/');
    let mut is_index_file = false;

    // Remove trailing slash and useless path segment.
    if module_path.ends_with('/') || module_path.ends_with("/.") {
        path_parts.next_back();

        is_index_file = true;
    }

    match last_component {
        Component::ParentDir | Component::CurDir => {
            is_index_file = true;
        }
        // `import ".././"` is the same as `import "../"`
        // Rust Path does not expose `./` path segment at very end, likely because it does not do anything.
        // To provide proper fix, we need to remove it as well.
        Component::Normal(_) if module_path.ends_with("./") => {
            // Remove useless path segment.
            path_parts.next_back();

            is_index_file = true;
        }
        _ => {}
    };

    // TODO. Once `intersperse` is stabilized, use it instead.
    // https://github.com/rust-lang/rust/issues/79524
    let mut new_path = path_parts.fold(String::new(), |mut output, b| {
        output.push_str(b);
        output.push('/');

        output
    });

    let part = if is_index_file {
        format!("index.{import_ext}")
    } else {
        // fold always adds trailing slash, so we need to remove it.
        new_path.pop();

        format!(".{import_ext}")
    };

    new_path.push_str(&part);

    Some(UseImportExtensionsState {
        module_name_token: module_name_token.clone(),
        suggestion: Some((new_path, import_ext.to_string())),
    })
}

fn resolve_import_extension<'a>(
    file_ext: &str,
    path: &Path,
    custom_suggested_imports: &'a FxHashMap<Box<str>, SuggestedExtensionMapping>,
) -> &'a str {
    let (potential_ext, potential_component_ext): (&str, &str) =
        if let Some(custom_mapping) = custom_suggested_imports.get(file_ext) {
            (&custom_mapping.module, &custom_mapping.component)
        } else {
            // TODO. This is not very accurate. We should use file system access to determine the file type.
            match file_ext {
                "ts" | "tsx" | "astro" => ("ts", "tsx"),
                "mts" => ("mts", "tsx"),
                "mjs" => ("mjs", "jsx"),
                "cjs" => ("cjs", "jsx"),
                "cts" => ("cts", "tsx"),
                // Unlikely that these frameworks would import tsx file.
                "svelte" | "vue" => ("ts", "ts"),
                _ => ("js", "jsx"),
            }
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
