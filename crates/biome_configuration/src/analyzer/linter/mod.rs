mod rules;

use crate::analyzer::RulePlainConfiguration;
use crate::bool::Bool;
use biome_analyze::RuleDomain;
use biome_deserialize::Merge;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
pub use rules::*;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub type LinterEnabled = Bool<true>;

#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[bpaf(hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<LinterEnabled>,

    /// List of rules
    #[bpaf(pure(Default::default()), optional, hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Rules>,

    /// A list of glob patterns. The analyzer will handle only those files/folders that will
    /// match these patterns.
    #[bpaf(pure(Default::default()), hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::NormalizedGlob>>,

    /// An object where the keys are the names of the domains, and the values are `all`, `recommended`, or `none`.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<RuleDomains>,

    /// Configuration for plugin rules.
    /// An object where keys are plugin rule names and values are their severity levels.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_rules: Option<PluginRules>,
}

/// Configuration for plugin rules.
/// Maps plugin rule names to their severity configuration.
#[derive(Clone, Debug, Default, Eq, PartialEq, Deserialize, Serialize, Deserializable)]
pub struct PluginRules(pub FxHashMap<String, RulePlainConfiguration>);

impl Deref for PluginRules {
    type Target = FxHashMap<String, RulePlainConfiguration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Merge for PluginRules {
    fn merge_with(&mut self, other: Self) {
        for (key, value) in other.0 {
            self.0.insert(key, value);
        }
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for PluginRules {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PluginRules")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let _values = generator.subschema_for::<RulePlainConfiguration>();
        schemars::json_schema!({
            "type": "object",
            "description": "Configuration for plugin rules. Keys are plugin rule names, values are severity levels.",
            "additionalProperties": {
                "$ref": "#/$defs/RulePlainConfiguration"
            }
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum RuleDomainValue {
    /// Enables all the rules that belong to this domain
    All,
    /// Disables all the rules that belong to this domain
    None,
    /// Enables only the recommended rules for this domain
    Recommended,
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize, Merge)]
pub struct RuleDomains(pub FxHashMap<RuleDomain, RuleDomainValue>);

impl Deref for RuleDomains {
    type Target = FxHashMap<RuleDomain, RuleDomainValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for RuleDomains {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("RuleDomains")
    }

    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let _names = generator.subschema_for::<RuleDomain>();
        let _values = generator.subschema_for::<RuleDomainValue>();
        schemars::json_schema!({
            "type": "object",
            "propertyNames": {
                "$ref": "#/$defs/RuleDomain"
            },
            "additionalProperties": {
                "$ref": "#/$defs/RuleDomainValue"
            }
        })
    }
}

impl LinterConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn get_rules(&self) -> Rules {
        self.rules.clone().unwrap_or_default()
    }
}
