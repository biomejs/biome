use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{inner_string_text, AnyJsImportSpecifierLike};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};

declare_rule! {
    /// Disallow specified modules when loaded by import or require.
    ///
    /// ## Options
    ///
    /// ```json
    /// {
    ///     "noRestrictedImports": {
    ///         "options": {
    ///             "paths": {
    ///                 "lodash": {
    ///                    "message": "Using lodash is not encouraged",
    ///                    "allowedFrom": ["src/utils"]
    ///                 },
    ///                 "underscore": {
    ///                    "message": "Using underscore is not encouraged",
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub NoRestrictedImports {
        version: "1.6.0",
        name: "noRestrictedImports",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-restricted-imports"),
            RuleSource::EslintTypeScript("no-restricted-imports"),
        ],
        recommended: false,
    }
}

/// Options for each path of rule `noRestrictedImports`.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedModuleConfig {
    pub message: String,
    pub allowed_from: Option<Vec<String>>,
    pub include_all_submodules: Option<bool>,
}

/// Options for the rule `noRestrictedImports`.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedImportsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    paths: FxHashMap<String, RestrictedModuleConfig>,
}

impl Rule for NoRestrictedImports {
    type Query = Ast<AnyJsImportSpecifierLike>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = Box<RestrictedImportsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let file_path = ctx.file_path();

        if node.is_in_ts_module_declaration() {
            return None;
        }

        // "import { merge } from 'lodash';" => get the "lodash"
        let module_name = node.module_name_token()?;
        let inner_text = inner_string_text(&module_name);

        ctx.options()
            .paths
            .iter()
            .filter_map(|(path, config)| {
                if !inner_text.text().starts_with(path) {
                    return None;
                }

                if let Some(allowed_from) = &config.allowed_from {
                    if allowed_from.contains(&file_path.to_string_lossy().to_string()) {
                        return None;
                    }
                }

                if (config.include_all_submodules.is_none()
                    || config.include_all_submodules == Some(false))
                    && inner_text.text().len() != path.len()
                {
                    return None;
                }

                Some((module_name.text_trimmed_range(), config.message.to_string()))
            })
            .next()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (span, text): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            *span,
            markup! {
                {text}
            },
        ))
    }
}
