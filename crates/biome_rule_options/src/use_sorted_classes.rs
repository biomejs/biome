use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct CustomClasses {
    /// Custom component classes to be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Box<str>>>,
    /// Custom utility classes to be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utilities: Option<Vec<Box<str>>>,
}

impl Deserializable for CustomClasses {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, CustomClassesVisitor, name)
    }
}

struct CustomClassesVisitor;
impl DeserializationVisitor for CustomClassesVisitor {
    type Output = CustomClasses;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["components", "utilities"];
        let mut result = CustomClasses::default();

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "components" => {
                    result.components = Deserializable::deserialize(ctx, &value, &key_text)
                        .and_then(|v: Vec<Box<str>>| if v.is_empty() { None } else { Some(v) })
                }
                "utilities" => {
                    result.utilities = Deserializable::deserialize(ctx, &value, &key_text)
                        .and_then(|v: Vec<Box<str>>| if v.is_empty() { None } else { Some(v) })
                }
                unknown_key => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedClassesOptions {
    /// Additional attributes that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of the functions or tagged templates that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Box<str>>>,
    /// Custom utility and component classes to be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classes: Option<CustomClasses>,
}

impl UseSortedClassesOptions {
    pub fn has_function(&self, name: &str) -> bool {
        let iter = self.functions.iter().flatten();
        for v in iter {
            if v.as_ref() == name {
                return true;
            }
        }
        false
    }

    pub fn match_function(&self, name: &str) -> bool {
        self.functions.iter().flatten().any(|matcher| {
            let mut matcher_parts = matcher.split('.');
            let mut name_parts = name.split('.');

            let all_parts_match = matcher_parts
                .by_ref()
                .zip(name_parts.by_ref())
                .all(|(m, p)| m == "*" || m == p);

            all_parts_match && matcher_parts.next().is_none() && name_parts.next().is_none()
        })
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        CLASS_ATTRIBUTES.contains(&name)
            || self.attributes.iter().flatten().any(|v| v.as_ref() == name)
    }
}

/// Attributes that are always targets.
const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions", "classes"];

impl Deserializable for UseSortedClassesOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, UtilityClassSortingOptionsVisitor, name)
    }
}

struct UtilityClassSortingOptionsVisitor;
impl DeserializationVisitor for UtilityClassSortingOptionsVisitor {
    type Output = UseSortedClassesOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut result = UseSortedClassesOptions::default();

        let mut attributes = Vec::new();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(ctx, &value, &key_text)
                    {
                        attributes.extend::<Vec<Box<str>>>(attributes_option);
                    }
                }
                "functions" => {
                    result.functions = Deserializable::deserialize(ctx, &value, &key_text)
                }
                "classes" => result.classes = Deserializable::deserialize(ctx, &value, &key_text),
                unknown_key => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }
        result.attributes = if attributes.is_empty() {
            None
        } else {
            Some(attributes.into_boxed_slice())
        };
        Some(result)
    }
}
