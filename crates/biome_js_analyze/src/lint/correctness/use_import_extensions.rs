use biome_diagnostics::Severity;
use biome_module_graph::JsImportPath;
use camino::{Utf8Component, Utf8Path};
use serde::{Deserialize, Serialize};

use crate::{JsRuleAction, services::module_graph::ResolvedImports};
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsImportLike, JsSyntaxToken, inner_string_text};
use biome_rowan::BatchMutationExt;
use biome_rule_options::use_import_extensions::UseImportExtensionsOptions;

declare_lint_rule! {
    /// Enforce file extensions for relative imports.
    ///
    /// Browsers, Deno, and Node.js do not natively support importing files
    /// without extensions from JavaScript modules. This rule enforces the use
    /// of file extensions for relative imports to make the code more
    /// consistent -- and correct.
    ///
    /// In some cases, tooling can also benefit from explicit file extensions,
    /// because they do not need to guess which file to resolve.
    ///
    /// The rule checks both static imports (`import ... from "..."`) as well as
    /// dynamic imports such as `import(...)` and `require(...)`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// The following examples assume these imports will resolve to a file with
    /// an extension. Imports that don't resolve at all will not trigger a
    /// diagnostic.
    ///
    /// ```js
    /// import "./foo";
    /// ```
    /// ```js
    /// import "./foo/";
    /// ```
    /// ```js
    /// import "../";
    /// ```
    /// ```js
    /// import "../.";
    /// ```
    /// ```js
    /// import("./foo");
    /// ```
    /// ```js
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
    /// ## Options
    ///
    /// The rule provides the options described below.
    ///
    /// ### forceJsExtensions
    ///
    /// Normally, this rule suggests to use the extension of the module that is
    /// found in your project. For instance, `.ts` or `.tsx` for a TypeScript
    /// file. If this option is set to `true`, the rule will always suggest to
    /// use `.js` regardless of the extension in your project.
    ///
    /// This is useful if you use the `"module": "node16"` setting when building
    /// your code with `tsc`.
    ///
    /// Default: `false`
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "forceJsExtensions": true
    ///     }
    /// }
    /// ```
    ///
    /// ## Editor Configuration
    ///
    /// If you use Visual Studio Code, you can ensure that it adds the file
    /// extension when automatically importing a variable by configuring
    /// `javascript.preferences.importModuleSpecifierEnding` and
    /// `typescript.preferences.importModuleSpecifierEnding`
    /// in your [settings](https://code.visualstudio.com/docs/getstarted/settings).
    ///
    /// ## Caveats
    ///
    /// If you are using TypeScript, TypeScript version 5.0 or later is
    /// required, also make sure to set
    /// [`allowImportingTsExtensions: true`](https://typescriptlang.org/tsconfig#allowImportingTsExtensions)
    /// in your `tsconfig.json`.
    pub UseImportExtensions {
        version: "1.8.0",
        name: "useImportExtensions",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Project],
    }
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
    type Query = ResolvedImports<AnyJsImportLike>;
    type State = UseImportExtensionsState;
    type Signals = Option<Self::State>;
    type Options = UseImportExtensionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_info = ctx.module_info_for_path(ctx.file_path())?;
        let force_js_extensions = ctx.options().force_js_extensions;

        let node = ctx.query();
        let resolved_path = module_info
            .get_import_path_by_js_node(node)
            .and_then(JsImportPath::as_path)?;

        get_extensionless_import(node, resolved_path, ctx, force_js_extensions)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.module_name_token.text_trimmed_range(),
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
        let new_module_name = if ctx.preferred_quote().is_double() {
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
                "Add import extension "<Emphasis>"."{extension}</Emphasis>"."
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
    node: &AnyJsImportLike,
    resolved_path: &Utf8Path,
    ctx: &RuleContext<UseImportExtensions>,
    force_js_extensions: bool,
) -> Option<UseImportExtensionsState> {
    let module_name_token = node.module_name_token()?;
    let module_path = inner_string_text(&module_name_token);
    let path = Utf8Path::new(module_path.text());
    let mut path_components = path.components();
    let first_component = path_components.next()?;

    if !matches!(
        first_component,
        Utf8Component::CurDir | Utf8Component::ParentDir
    ) {
        // TypeScript path aliases should still be considered.
        // The same does *not* apply for `package.json` aliases, because
        // extensions are not automatically applied to those.
        let matches_path_alias = ctx
            .project_layout()
            .query_tsconfig_for_path(ctx.file_path(), |tsconfig| {
                tsconfig.matches_path_alias(path.as_str())
            })?;
        if !matches_path_alias {
            return None;
        }
    }

    let resolved_stem = resolved_path.file_stem();
    let resolved_extension = resolved_path.extension();
    let resolved_path_sub_extension =
        resolved_stem.and_then(|stem| stem.rfind('.').map(|pos| &stem[pos + 1..]));

    let mut existing_extension = path.extension();

    match (resolved_path_sub_extension, existing_extension) {
        (Some("d"), Some("js")) if resolved_extension.is_some_and(|ext| ext == "ts") => {
            return None; // We resolved a `.d.ts` file, but imported the `.js` file: OK.
        }
        (Some(_), _) if path.file_name()?.starts_with(resolved_path.file_name()?) => {
            return None; // For cases like `./foo.css` -> `./foo.css.ts`
        }
        _ => {}
    }

    let last_component = path_components.next_back().unwrap_or(first_component);
    let query_or_hash = last_component
        .as_str()
        .find(['?', '#'])
        .map(|index| &last_component.as_str()[index..]);

    if let Some(query_or_hash) = query_or_hash
        && let Some(existing_extension) = existing_extension.as_mut()
    {
        *existing_extension = existing_extension
            .strip_suffix(query_or_hash)
            .unwrap_or(existing_extension);
    }

    let extension = if force_js_extensions {
        "js"
    } else {
        resolved_extension?
    };

    // Check if the existing extension matches the desired one.
    if existing_extension.is_some_and(|ext| ext == extension) {
        return None;
    }

    let is_index_file = resolved_stem.is_some_and(|stem| stem == "index");
    let import_path_ends_with_index = path.file_name().is_some_and(|name| name == "index")
        || path
            .with_extension("")
            .file_name()
            .is_some_and(|name| name == "index");

    let new_path = if is_index_file && !import_path_ends_with_index {
        let mut path_parts = path.as_str().split('/');

        // Remove trailing slash and useless path segment.
        if module_path.ends_with('/') || module_path.ends_with("/.") {
            path_parts.next_back();
        }
        if module_path.ends_with("/./") {
            path_parts.next_back();
        }

        let mut new_path = path_parts.fold(String::new(), |mut result, part| {
            result.push_str(part);
            result.push('/');

            result
        });

        new_path.push_str("index.");
        new_path.push_str(extension);
        new_path
    } else {
        let mut new_path = path.to_path_buf();
        let sub_extension = if resolved_path_sub_extension.is_some() {
            existing_extension
        } else {
            None
        };

        if let Some(sub_ext) = sub_extension {
            new_path.set_extension(format!("{sub_ext}.{extension}"));
        } else {
            new_path.set_extension(extension);
        }

        new_path.to_string()
    };

    Some(UseImportExtensionsState {
        module_name_token,
        suggestion: query_or_hash
            .is_none()
            .then(|| (new_path, extension.to_string())),
    })
}
