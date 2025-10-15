use biome_console::markup;
use biome_deserialize::{
    DeserializableTypes, DeserializableValidator, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange, non_empty,
};
use biome_deserialize_macros::Deserializable;
use biome_diagnostics::Severity;
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExhaustiveDependenciesOptions {
    /// Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to true.
    #[serde(default = "report_unnecessary_dependencies_default")]
    pub report_unnecessary_dependencies: bool,

    /// Whether to report an error when a hook has no dependencies array.
    #[serde(default)]
    pub report_missing_dependencies_array: bool,

    /// List of hooks of which the dependencies should be validated.
    #[serde(default)]
    #[deserializable(validate = "non_empty")]
    pub hooks: Box<[Hook]>,
}

fn report_unnecessary_dependencies_default() -> bool {
    true
}

impl Default for UseExhaustiveDependenciesOptions {
    fn default() -> Self {
        Self {
            report_unnecessary_dependencies: report_unnecessary_dependencies_default(),
            report_missing_dependencies_array: false,
            hooks: Vec::new().into_boxed_slice(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct Hook {
    /// The name of the hook.
    #[deserializable(validate = "non_empty")]
    pub name: Box<str>,

    /// The "position" of the closure function, starting from zero.
    ///
    /// For example, for React's `useEffect()` hook, the closure index is 0.
    pub closure_index: Option<u8>,

    /// The "position" of the array of dependencies, starting from zero.
    ///
    /// For example, for React's `useEffect()` hook, the dependencies index is 1.
    pub dependencies_index: Option<u8>,

    /// Whether the result of the hook is stable.
    ///
    /// Set to `true` to mark the identity of the hook's return value as stable,
    /// or use a number/an array of numbers to mark the "positions" in the
    /// return array as stable.
    ///
    /// For example, for React's `useRef()` hook the value would be `true`,
    /// while for `useState()` it would be `[1]`.
    pub stable_result: Option<StableHookResult>,
}

impl DeserializableValidator for Hook {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        match (self.closure_index, self.dependencies_index) {
            (Some(closure_index), Some(dependencies_index))
                if closure_index == dependencies_index =>
            {
                ctx.report(
                        DeserializationDiagnostic::new(markup! {
                        <Emphasis>"closureIndex"</Emphasis>" and "<Emphasis>"dependenciesIndex"</Emphasis>" may not be the same"
                    })
                            .with_range(range),
                    );

                self.closure_index = None;
                self.dependencies_index = None;
            }
            _ => {}
        }

        true
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum StableHookResult {
    /// Used to indicate the hook does not have a stable result.
    #[default]
    None,

    /// Used to indicate the identity of the result value is stable.
    ///
    /// Note this does not imply internal stability. For instance, the ref
    /// objects returned by React's `useRef()` always have a stable identity,
    /// but their internal value may be mutable.
    Identity,

    /// Used to indicate the hook returns an array and some of its indices have
    /// stable identities.
    ///
    /// For example, React's `useState()` hook returns a stable function at
    /// index 1.
    Indices(Vec<u8>),

    /// Used to indicate the hook returns an object and some of its properties
    /// have stable identities.
    Keys(Vec<String>),
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for StableHookResult {
    fn schema_name() -> String {
        "StableHookResult".to_owned()
    }

    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;
        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Boolean.into()),
                        metadata: Some(Box::new(Metadata {
                            description: Some("Whether the hook has a stable result.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(SingleOrVec::Single(Box::new(Schema::Object(SchemaObject {
                                instance_type: Some(InstanceType::Integer.into()),
                                format: Some("uint8".to_owned()),
                                number: Some(Box::new(NumberValidation {
                                    minimum: Some(0.),
                                    maximum: Some(255.),
                                    ..Default::default()
                                })),
                                ..Default::default()
                            })))),
                            min_items: Some(1),
                            ..Default::default()
                        })),
                        metadata: Some(Box::new(Metadata {
                            description: Some("Used to indicate the hook returns an array and some of its indices have stable identities.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(SingleOrVec::Single(Box::new(Schema::Object(SchemaObject {
                                instance_type: Some(InstanceType::String.into()),
                                ..Default::default()
                            })))),
                            min_items: Some(1),
                            ..Default::default()
                        })),
                        metadata: Some(Box::new(Metadata {
                            description: Some("Used to indicate the hook returns an object and some of its properties have stable identities.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl biome_deserialize::Deserializable for StableHookResult {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, StableResultVisitor, name)
    }
}

struct StableResultVisitor;
impl DeserializationVisitor for StableResultVisitor {
    type Output = StableHookResult;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::ARRAY
        .union(DeserializableTypes::BOOL)
        .union(DeserializableTypes::NUMBER);

    fn visit_array(
        self,
        ctx: &mut impl DeserializationContext,
        items: impl Iterator<Item = Option<impl DeserializableValue>>,
        range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let (mut keys, mut indices) = (Vec::new(), Vec::new());

        for value in items {
            if let Some(deserialized) = value.and_then(|v| {
                DeserializableValue::deserialize(&v, ctx, StableResultArrayVisitor, "")
            }) {
                match deserialized {
                    StableResultItem::Key(key) => keys.push(key),
                    StableResultItem::Index(index) => indices.push(index),
                }
            }
        }

        if !keys.is_empty() && !indices.is_empty() {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    "Expected either property key names or array indices, not a combination of both"
                })
                .with_range(range),
            );
        }

        if !keys.is_empty() {
            return Some(StableHookResult::Keys(keys));
        }

        if !indices.is_empty() {
            return Some(StableHookResult::Indices(indices));
        }

        Some(StableHookResult::None)
    }

    fn visit_bool(
        self,
        ctx: &mut impl DeserializationContext,
        value: bool,
        range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        match value {
            true => Some(StableHookResult::Identity),
            false => {
                ctx.report(
                    DeserializationDiagnostic::new(
                        markup! { "This hook is configured to not have a stable result" },
                    )
                    .with_custom_severity(Severity::Warning)
                    .with_range(range),
                );
                Some(StableHookResult::None)
            }
        }
    }

    fn visit_number(
        self,
        ctx: &mut impl DeserializationContext,
        value: biome_deserialize::TextNumber,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        StableResultIndexVisitor::visit_number(StableResultIndexVisitor, ctx, value, range, name)
            .map(|index| StableHookResult::Indices(vec![index]))
    }
}

enum StableResultItem {
    Key(String),
    Index(u8),
}

struct StableResultArrayVisitor;
impl DeserializationVisitor for StableResultArrayVisitor {
    type Output = StableResultItem;
    const EXPECTED_TYPE: DeserializableTypes =
        DeserializableTypes::STR.union(DeserializableTypes::NUMBER);

    fn visit_str(
        self,
        _ctx: &mut impl DeserializationContext,
        value: Text,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        Some(StableResultItem::Key(value.to_string()))
    }

    fn visit_number(
        self,
        ctx: &mut impl DeserializationContext,
        value: biome_deserialize::TextNumber,
        range: TextRange,
        name: &str,
    ) -> Option<Self::Output> {
        StableResultIndexVisitor::visit_number(StableResultIndexVisitor, ctx, value, range, name)
            .map(StableResultItem::Index)
    }
}

struct StableResultIndexVisitor;
impl DeserializationVisitor for StableResultIndexVisitor {
    type Output = u8;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::NUMBER;

    fn visit_number(
        self,
        ctx: &mut impl DeserializationContext,
        value: biome_deserialize::TextNumber,
        range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        match value.parse::<u8>() {
            Ok(index) => Some(index),
            Err(_) => {
                ctx.report(DeserializationDiagnostic::new_out_of_bound_integer(
                    0, 255, range,
                ));
                None
            }
        }
    }
}
