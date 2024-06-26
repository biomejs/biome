use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{inner_string_text, AnyJsImportLike};
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
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged"
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

/// Options for the rule `noRestrictedImports`.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedImportsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    paths: FxHashMap<String, String>,
}

impl Rule for NoRestrictedImports {
    type Query = Ast<AnyJsImportLike>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = Box<RestrictedImportsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.is_in_ts_module_declaration() {
            return None;
        }
        let module_name = node.module_name_token()?;
        let inner_text = inner_string_text(&module_name);

        ctx.options()
            .paths
            .get(inner_text.text())
            .map(|message| (module_name.text_trimmed_range(), message.to_string()))
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
