use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseNullishCoalescingOptions {
    /// Whether to ignore `||` expressions in conditional test positions
    /// (if/while/for/do-while/ternary conditions).
    ///
    /// When `true` (the default), the rule will not report `||` expressions
    /// that appear in places where the falsy-checking behavior may be intentional.
    ///
    /// Default: `true`
    pub ignore_conditional_tests: Option<bool>,

    /// Whether to ignore ternary expressions that could be simplified
    /// using the nullish coalescing operator.
    ///
    /// Default: `false`
    pub ignore_ternary_tests: Option<bool>,

    /// Whether to ignore `||` expressions that are part of a mixed logical
    /// expression with `&&`.
    ///
    /// When `true`, the rule will not report `||` expressions where the
    /// operator is mixed with `&&` in the same expression without parentheses.
    ///
    /// Default: `false`
    pub ignore_mixed_logical_expressions: Option<bool>,

    /// Whether to ignore `||` expressions based on the primitive type of the
    /// left operand.
    ///
    /// Set to `true` to ignore all primitive types, or an object with
    /// `string`, `number`, `boolean`, and `bigint` fields to ignore specific
    /// types.
    ///
    /// Default: all `false`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_primitives: Option<IgnorePrimitives>,

    /// Whether to ignore `||` expressions inside `Boolean()` calls.
    ///
    /// When `true`, the rule will not report `||` expressions that appear
    /// as arguments to `Boolean()`, where the falsy-coalescing behavior
    /// is intentional.
    ///
    /// Default: `false`
    pub ignore_boolean_coercion: Option<bool>,

    /// Whether to ignore `if` statements that could be simplified by using
    /// the nullish coalescing assignment operator (`??=`).
    ///
    /// When `true`, the rule will not report `if` statements like
    /// `if (foo == null) { foo = bar; }` that could be written as
    /// `foo ??= bar`.
    ///
    /// Default: `false`
    pub ignore_if_statements: Option<bool>,
}

impl UseNullishCoalescingOptions {
    pub fn ignore_conditional_tests(&self) -> bool {
        self.ignore_conditional_tests.unwrap_or(true)
    }

    pub fn ignore_ternary_tests(&self) -> bool {
        self.ignore_ternary_tests.unwrap_or(false)
    }

    pub fn ignore_mixed_logical_expressions(&self) -> bool {
        self.ignore_mixed_logical_expressions.unwrap_or(false)
    }

    pub fn ignore_boolean_coercion(&self) -> bool {
        self.ignore_boolean_coercion.unwrap_or(false)
    }

    pub fn ignore_if_statements(&self) -> bool {
        self.ignore_if_statements.unwrap_or(false)
    }

    pub fn should_ignore_primitive_string(&self) -> bool {
        self.ignore_primitives
            .as_ref()
            .is_some_and(|p| p.should_ignore_string())
    }

    pub fn should_ignore_primitive_number(&self) -> bool {
        self.ignore_primitives
            .as_ref()
            .is_some_and(|p| p.should_ignore_number())
    }

    pub fn should_ignore_primitive_boolean(&self) -> bool {
        self.ignore_primitives
            .as_ref()
            .is_some_and(|p| p.should_ignore_boolean())
    }

    pub fn should_ignore_primitive_bigint(&self) -> bool {
        self.ignore_primitives
            .as_ref()
            .is_some_and(|p| p.should_ignore_bigint())
    }

    pub fn has_any_ignore_primitives(&self) -> bool {
        self.ignore_primitives.is_some()
    }
}

/// Controls which primitive types to ignore in `||` expressions.
///
/// Can be `true` to ignore all primitives, or an object specifying which
/// primitive types to ignore individually.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum IgnorePrimitives {
    /// When `true`, ignore all primitive types (string, number, boolean, bigint).
    All(bool),
    /// Ignore specific primitive types.
    Specific(IgnorePrimitivesOptions),
}

impl Default for IgnorePrimitives {
    fn default() -> Self {
        Self::All(false)
    }
}

impl IgnorePrimitives {
    pub fn should_ignore_string(&self) -> bool {
        match self {
            Self::All(all) => *all,
            Self::Specific(opts) => opts.string.unwrap_or(false),
        }
    }

    pub fn should_ignore_number(&self) -> bool {
        match self {
            Self::All(all) => *all,
            Self::Specific(opts) => opts.number.unwrap_or(false),
        }
    }

    pub fn should_ignore_boolean(&self) -> bool {
        match self {
            Self::All(all) => *all,
            Self::Specific(opts) => opts.boolean.unwrap_or(false),
        }
    }

    pub fn should_ignore_bigint(&self) -> bool {
        match self {
            Self::All(all) => *all,
            Self::Specific(opts) => opts.bigint.unwrap_or(false),
        }
    }
}

impl biome_deserialize::Merge for IgnorePrimitives {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}

impl Deserializable for IgnorePrimitives {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Bool {
            Self::All(<bool as Deserializable>::deserialize(ctx, value, name)?)
        } else {
            Self::Specific(<IgnorePrimitivesOptions as Deserializable>::deserialize(
                ctx, value, name,
            )?)
        })
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for IgnorePrimitives {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("IgnorePrimitives")
    }

    fn json_schema(_generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({
            "oneOf": [
                {
                    "type": "boolean",
                    "description": "When true, ignore all primitive types."
                },
                {
                    "type": "object",
                    "properties": {
                        "string": { "type": "boolean" },
                        "number": { "type": "boolean" },
                        "boolean": { "type": "boolean" },
                        "bigint": { "type": "boolean" }
                    },
                    "additionalProperties": false,
                    "description": "Ignore specific primitive types."
                }
            ]
        })
    }
}

/// Options for ignoring specific primitive types in `||` expressions.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, default)]
pub struct IgnorePrimitivesOptions {
    /// Ignore `string` and string literal types.
    pub string: Option<bool>,
    /// Ignore `number` and number literal types.
    pub number: Option<bool>,
    /// Ignore `boolean` and boolean literal types.
    pub boolean: Option<bool>,
    /// Ignore `bigint` and bigint literal types.
    pub bigint: Option<bool>,
}
