//! This module includes implementations for deserializing a Stylelint configuration.
//!
//! The defined types follow the Stylelint configuration schema described at
//! <https://stylelint.io/user-guide/configure>.
//!
//! See [super::stylelint_to_biome] for converting a Stylelint config to a Biome config.

use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableTypes, DeserializableValue,
    DeserializationContext, DeserializationVisitor, MapMembers, Merge,
};
use biome_deserialize_macros::Deserializable;
use std::ops::Deref;

use super::eslint_eslint::{IgnorePattern, ShorthandVec};

/// A Stylelint configuration object.
///
/// See <https://stylelint.io/user-guide/configure>
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct StylelintConfigData {
    /// Configurations to extend, resolved recursively.
    pub(crate) extends: ShorthandVec<String>,
    /// The severity applied to rules that don't specify one.
    pub(crate) default_severity: Option<Severity>,
    /// The enabled rules.
    pub(crate) rules: Rules,
    /// Per-glob configuration overrides.
    pub(crate) overrides: Vec<OverrideConfigData>,
    /// The glob patterns that are ignored.
    pub(crate) ignore_files: ShorthandVec<IgnorePattern>,
}
impl Merge for StylelintConfigData {
    fn merge_with(&mut self, mut other: Self) {
        self.extends.merge_with(other.extends);
        if other.default_severity.is_some() {
            self.default_severity = other.default_severity;
        }
        self.rules.merge_with(other.rules);
        self.overrides.append(&mut other.overrides);
        self.ignore_files.merge_with(other.ignore_files);
    }
}

/// A Stylelint config can be embedded in `package.json`.
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct StylelintPackageJson {
    pub(crate) stylelint: Option<StylelintConfigData>,
}

/// A Stylelint `overrides` entry.
///
/// See <https://stylelint.io/user-guide/configure#overrides>
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
pub(crate) struct OverrideConfigData {
    /// The glob patterns for target files.
    pub(crate) files: ShorthandVec<Box<str>>,
    /// The severity applied to rules that don't specify one.
    pub(crate) default_severity: Option<Severity>,
    /// The enabled rules.
    pub(crate) rules: Rules,
}

/// The severity of a Stylelint rule.
///
/// See <https://stylelint.io/user-guide/configure#severity>
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum Severity {
    Warning,
    #[default]
    Error,
}
impl Deserializable for Severity {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        let s = biome_deserialize::Text::deserialize(ctx, value, name)?;
        match s.text() {
            "warning" => Some(Self::Warning),
            "error" => Some(Self::Error),
            _ => None,
        }
    }
}
impl From<Severity> for biome_configuration::RulePlainConfiguration {
    fn from(value: Severity) -> Self {
        match value {
            Severity::Warning => Self::Warn,
            Severity::Error => Self::Error,
        }
    }
}

/// The set of rules configured in a Stylelint configuration.
///
/// Rules are keyed by name because Stylelint configurations, including those
/// pulled in through `extends`, override rules by name.
#[derive(Debug, Default)]
pub(crate) struct Rules(pub(crate) rustc_hash::FxHashMap<Box<str>, RuleData>);
impl Merge for Rules {
    fn merge_with(&mut self, other: Self) {
        // Keep the already-present configuration so that the user's own rules
        // take precedence over the rules pulled in through `extends`.
        for (name, data) in other.0 {
            self.0.entry(name).or_insert(data);
        }
    }
}
impl Deref for Rules {
    type Target = rustc_hash::FxHashMap<Box<str>, RuleData>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Deserializable for Rules {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = Rules;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;
            fn visit_map(
                self,
                ctx: &mut dyn DeserializationContext,
                members: &mut MapMembers<'_>,
                _range: biome_rowan::TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                use biome_deserialize::Text;
                let mut result = rustc_hash::FxHashMap::default();
                for (key, value) in members.flatten() {
                    let Some(rule_name) = Text::deserialize(ctx, &key, "") else {
                        continue;
                    };
                    let Some(data) = RuleData::deserialize(ctx, &value, rule_name.text()) else {
                        continue;
                    };
                    result.insert(rule_name.text().into(), data);
                }
                Some(Rules(result))
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}

/// The configuration of a single Stylelint rule.
#[derive(Debug)]
pub(crate) struct RuleData {
    /// Whether the rule is enabled.
    ///
    /// Stylelint disables a rule when its value is `null` or when its primary
    /// option is `false`.
    pub(crate) enabled: bool,
    /// The severity set in the secondary options, if any.
    pub(crate) severity: Option<Severity>,
}
impl Deserializable for RuleData {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        match value.visitable_type()? {
            DeserializableType::Null => Some(Self {
                enabled: false,
                severity: None,
            }),
            DeserializableType::Bool => {
                let enabled = bool::deserialize(ctx, value, name)?;
                Some(Self {
                    enabled,
                    severity: None,
                })
            }
            DeserializableType::Array => {
                struct Visitor;
                impl DeserializationVisitor for Visitor {
                    type Output = RuleData;
                    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY;
                    fn visit_array(
                        self,
                        ctx: &mut dyn DeserializationContext,
                        items: &mut dyn ExactSizeIterator<
                            Item = Option<Box<dyn DeserializableValue>>,
                        >,
                        _range: biome_rowan::TextRange,
                        name: &str,
                    ) -> Option<Self::Output> {
                        let mut items = items.flatten();
                        let enabled = match items.next() {
                            Some(primary) => match primary.visitable_type() {
                                Some(DeserializableType::Null) => false,
                                Some(DeserializableType::Bool) => {
                                    bool::deserialize(ctx, &primary, name).unwrap_or(true)
                                }
                                _ => true,
                            },
                            None => true,
                        };
                        let severity = items
                            .next()
                            .and_then(|secondary| {
                                SecondaryOptions::deserialize(ctx, &secondary, name)
                            })
                            .and_then(|options| options.severity);
                        Some(RuleData { enabled, severity })
                    }
                }
                value.deserialize(ctx, Visitor, name)
            }
            // Any other primary option (string, number, object) enables the rule.
            _ => Some(Self {
                enabled: true,
                severity: None,
            }),
        }
    }
}

/// The secondary options object of a Stylelint rule.
///
/// Only the fields that have a Biome equivalent are deserialized.
#[derive(Debug, Default, Deserializable)]
#[deserializable(unknown_fields = "allow")]
struct SecondaryOptions {
    severity: Option<Severity>,
}
