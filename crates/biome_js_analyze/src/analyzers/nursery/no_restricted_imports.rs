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
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged"
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub NoRestrictedImports {
        version: "next",
        name: "noRestrictedImports",
        source: RuleSource::Eslint("no-restricted-imports"),
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
    type Query = Ast<AnyJsImportSpecifierLike>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = Box<RestrictedImportsOptions>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let module_name = ctx.query().module_name_token()?;
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
