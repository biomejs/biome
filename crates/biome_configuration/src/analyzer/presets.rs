use biome_analyze::RulePreset;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_deserialize_macros::Merge;
use biome_rowan::Text;
use bpaf::Bpaf;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::str::FromStr;

#[derive(Clone, Debug, Eq, PartialEq, Bpaf, Merge)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PresetConfig {
    FromAnalyzer(RulePreset),
    All,
    None,
}

impl Serialize for PresetConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::FromAnalyzer(RulePreset::Recommended) => "recommended",
            Self::All => "all",
            Self::None => "none",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for PresetConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = <String as Deserialize>::deserialize(deserializer)?;
        match s.as_str() {
            "recommended" => Ok(Self::FromAnalyzer(RulePreset::Recommended)),
            "all" => Ok(Self::All),
            "none" => Ok(Self::None),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid preset value: \"{s}\". Expected \"recommended\", \"all\", or \"none\""
            ))),
        }
    }
}

impl Default for PresetConfig {
    fn default() -> Self {
        Self::FromAnalyzer(RulePreset::Recommended)
    }
}

impl PresetConfig {
    pub const fn is_all(&self) -> bool {
        matches!(self, Self::All)
    }

    pub const fn is_recommended(&self) -> bool {
        matches!(self, Self::FromAnalyzer(RulePreset::Recommended))
    }

    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl Deserializable for PresetConfig {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        struct Visitor;
        impl DeserializationVisitor for Visitor {
            type Output = PresetConfig;
            const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::STR;
            fn visit_str(
                self,
                ctx: &mut impl DeserializationContext,
                value: Text,
                range: TextRange,
                _name: &str,
            ) -> Option<Self::Output> {
                if value == "none" {
                    return Some(PresetConfig::None);
                } else if value == "all" {
                    return Some(PresetConfig::All);
                } else {
                    let result = RulePreset::from_str(&value);
                    match result {
                        Ok(preset) => return Some(PresetConfig::FromAnalyzer(preset)),
                        Err(err) => {
                            ctx.report(DeserializationDiagnostic::new(err).with_range(range));
                        }
                    }
                }

                None
            }
        }
        value.deserialize(ctx, Visitor, name)
    }
}
