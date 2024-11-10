use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationDiagnostic,
};
use biome_js_syntax::{inner_string_text, AnyJsImportLike};
use biome_rowan::TextRange;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;

declare_lint_rule! {
    /// Disallow specified modules when loaded by import or require.
    ///
    /// ## Options
    ///
    /// ```json,options
    /// {
    ///     "noRestrictedImports": {
    ///         "options": {
    ///             "paths": {
    ///                 "lodash": "Using lodash is not encouraged",
    ///                 "underscore": "Using underscore is not encouraged",
    ///                 "import-foo": {
    ///                     "importNames": ["Bar"],
    ///                     "message": "Please use Bar from /import-bar/baz/ instead."
    ///                 },
    ///                 "import-bar": {
    ///                   "allowImportNames": ["Bar"],
    ///                   "message": "Please use only Bar from import-bar."
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic,use_options
    /// import "lodash";
    /// import "allowed-import";
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const underscore = await import("underscore");
    /// ```
    ///
    /// ```js,expect_diagnostic,use_options
    /// const lodash = require("lodash");
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,use_options
    /// import "allowed-import";
    /// const myImport = await import("allowed-import");
    /// const myImport = require("allowed-import");
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
#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct RestrictedImportsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "FxHashMap::is_empty")]
    paths: FxHashMap<Box<str>, CustomRestrictedImport>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Deserialize,
    biome_deserialize_macros::Deserializable,
    Eq,
    PartialEq,
    Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct CustomRestrictedImportOptions {
    message: String,
    import_names: Box<[Box<str>]>,
    allow_import_names: Box<[Box<str>]>,
}

impl CustomRestrictedImportOptions {
    pub fn has_import_name_patterns(&self) -> bool {
        !self.import_names.is_empty() || !self.allow_import_names.is_empty()
    }

    pub fn is_import_allowed(&self, imported_name: &str) -> bool {
        if !self.allow_import_names.is_empty() {
            // Deny all imports except for the names specified in allow_import_names
            self.allow_import_names
                .iter()
                .any(|name| Borrow::<str>::borrow(name) == imported_name)
        } else if !self.import_names.is_empty() {
            // Allow all imports except for the names specified in import_names
            self.import_names
                .iter()
                .all(|name| Borrow::<str>::borrow(name) != imported_name)
        } else {
            // Deny all imports from this module
            false
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum CustomRestrictedImport {
    Plain(String),
    WithOptions(CustomRestrictedImportOptions),
}

impl From<CustomRestrictedImport> for CustomRestrictedImportOptions {
    fn from(options: CustomRestrictedImport) -> Self {
        match options {
            CustomRestrictedImport::Plain(message) => CustomRestrictedImportOptions {
                message,
                import_names: [].into(),
                allow_import_names: [].into(),
            },
            CustomRestrictedImport::WithOptions(options) => options,
        }
    }
}

impl Deserializable for CustomRestrictedImport {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            biome_deserialize::Deserializable::deserialize(value, name, diagnostics)
                .map(Self::Plain)
        } else {
            biome_deserialize::Deserializable::deserialize(value, name, diagnostics)
                .map(Self::WithOptions)
        }
    }
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
            .map(|restricted_import| {
                let restricted_import_options: CustomRestrictedImportOptions = restricted_import.clone().into();
                (module_name.text_trimmed_range(), restricted_import_options.message.to_string())
            })
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
