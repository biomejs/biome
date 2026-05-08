use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTailwindShorthandClassesOptions {
    /// Additional attributes that will be treated as Tailwind class lists.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of functions or tagged templates whose string arguments should be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}

const DEFAULT_FUNCTIONS: [&str; 10] = [
    "clsx", "tw", "twMerge", "twJoin", "cva", "tv", "cn", "cc", "cnb", "ctl",
];

impl biome_deserialize::Merge for UseTailwindShorthandClassesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(attributes) = other.attributes {
            self.attributes = Some(attributes);
        }
        if let Some(functions) = other.functions {
            self.functions = Some(functions);
        }
    }
}

impl UseTailwindShorthandClassesOptions {
    pub fn has_function(&self, name: &str) -> bool {
        self.functions.as_deref().map_or_else(
            || DEFAULT_FUNCTIONS.contains(&name),
            |functions| functions.iter().any(|value| value.as_ref() == name),
        )
    }

    pub fn match_function(&self, name: &str) -> bool {
        self.functions.as_deref().map_or_else(
            || DEFAULT_FUNCTIONS.contains(&name),
            |functions| functions.iter().any(|matcher| matcher.as_ref() == name),
        )
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        CLASS_ATTRIBUTES.contains(&name)
            || self
                .attributes
                .iter()
                .flatten()
                .any(|value| value.as_ref() == name)
    }
}

const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for UseTailwindShorthandClassesOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, TailwindShorthandOptionsVisitor, name)
    }
}

struct TailwindShorthandOptionsVisitor;

impl DeserializationVisitor for TailwindShorthandOptionsVisitor {
    type Output = UseTailwindShorthandClassesOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut result = UseTailwindShorthandClassesOptions::default();

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
