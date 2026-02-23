use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseMemberOrderingOptions {
    /// Custom order of member groups.
    /// If not provided, the default order is used.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub groups: Option<Vec<MemberGroup>>,
}

impl UseMemberOrderingOptions {
    /// Returns the configured groups or the default order.
    pub fn groups(&self) -> Vec<MemberGroup> {
        self.groups
            .clone()
            .unwrap_or_else(MemberGroup::default_order)
    }
}

/// Represents a category of class member for ordering purposes.
#[derive(
    Clone, Copy, Debug, Deserialize, Deserializable, Eq, Hash, PartialEq, Serialize, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum MemberGroup {
    // Index signatures
    #[serde(rename = "index-signature")]
    IndexSignature,
    #[serde(rename = "static-index-signature")]
    StaticIndexSignature,

    // Properties
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "static-property")]
    StaticProperty,
    #[serde(rename = "protected-property")]
    ProtectedProperty,
    #[serde(rename = "protected-static-property")]
    ProtectedStaticProperty,
    #[serde(rename = "private-property")]
    PrivateProperty,
    #[serde(rename = "private-static-property")]
    PrivateStaticProperty,
    #[serde(rename = "#private-property")]
    HashPrivateProperty,
    #[serde(rename = "#private-static-property")]
    HashPrivateStaticProperty,

    // Combined accessors (get/set).
    // These are reserved for the TC39 auto-accessor proposal (`accessor name;`).
    // Currently, `classify_member()` only produces `GetAccessor` / `SetAccessor` variants.
    // Users should prefer `get-accessor` / `set-accessor` groups in custom configurations.
    #[serde(rename = "accessor")]
    Accessor,
    #[serde(rename = "static-accessor")]
    StaticAccessor,
    #[serde(rename = "protected-accessor")]
    ProtectedAccessor,
    #[serde(rename = "protected-static-accessor")]
    ProtectedStaticAccessor,
    #[serde(rename = "private-accessor")]
    PrivateAccessor,
    #[serde(rename = "private-static-accessor")]
    PrivateStaticAccessor,
    #[serde(rename = "#private-accessor")]
    HashPrivateAccessor,
    #[serde(rename = "#private-static-accessor")]
    HashPrivateStaticAccessor,

    // Getters
    #[serde(rename = "get-accessor")]
    GetAccessor,
    #[serde(rename = "static-get-accessor")]
    StaticGetAccessor,
    #[serde(rename = "protected-get-accessor")]
    ProtectedGetAccessor,
    #[serde(rename = "protected-static-get-accessor")]
    ProtectedStaticGetAccessor,
    #[serde(rename = "private-get-accessor")]
    PrivateGetAccessor,
    #[serde(rename = "private-static-get-accessor")]
    PrivateStaticGetAccessor,
    #[serde(rename = "#private-get-accessor")]
    HashPrivateGetAccessor,
    #[serde(rename = "#private-static-get-accessor")]
    HashPrivateStaticGetAccessor,

    // Setters
    #[serde(rename = "set-accessor")]
    SetAccessor,
    #[serde(rename = "static-set-accessor")]
    StaticSetAccessor,
    #[serde(rename = "protected-set-accessor")]
    ProtectedSetAccessor,
    #[serde(rename = "protected-static-set-accessor")]
    ProtectedStaticSetAccessor,
    #[serde(rename = "private-set-accessor")]
    PrivateSetAccessor,
    #[serde(rename = "private-static-set-accessor")]
    PrivateStaticSetAccessor,
    #[serde(rename = "#private-set-accessor")]
    HashPrivateSetAccessor,
    #[serde(rename = "#private-static-set-accessor")]
    HashPrivateStaticSetAccessor,

    // Constructor
    #[serde(rename = "constructor")]
    Constructor,

    // Methods
    #[serde(rename = "method")]
    Method,
    #[serde(rename = "static-method")]
    StaticMethod,
    #[serde(rename = "protected-method")]
    ProtectedMethod,
    #[serde(rename = "protected-static-method")]
    ProtectedStaticMethod,
    #[serde(rename = "private-method")]
    PrivateMethod,
    #[serde(rename = "private-static-method")]
    PrivateStaticMethod,
    #[serde(rename = "#private-method")]
    HashPrivateMethod,
    #[serde(rename = "#private-static-method")]
    HashPrivateStaticMethod,

    // Static block
    #[serde(rename = "static-block")]
    StaticBlock,
}

impl MemberGroup {
    /// Returns the default order of member groups.
    pub fn default_order() -> Vec<MemberGroup> {
        vec![
            // Index signatures
            MemberGroup::IndexSignature,
            MemberGroup::StaticIndexSignature,
            // Static properties (public -> protected -> private -> #private)
            MemberGroup::StaticProperty,
            MemberGroup::ProtectedStaticProperty,
            MemberGroup::PrivateStaticProperty,
            MemberGroup::HashPrivateStaticProperty,
            // Instance properties (public -> protected -> private -> #private)
            MemberGroup::Property,
            MemberGroup::ProtectedProperty,
            MemberGroup::PrivateProperty,
            MemberGroup::HashPrivateProperty,
            // Static accessors (combined, get, set; each public -> protected -> private -> #private)
            // Note: Combined accessor groups (Accessor, StaticAccessor, etc.) are reserved for
            // the TC39 auto-accessor proposal and are not currently produced by classify_member().
            MemberGroup::StaticAccessor,
            MemberGroup::ProtectedStaticAccessor,
            MemberGroup::PrivateStaticAccessor,
            MemberGroup::HashPrivateStaticAccessor,
            MemberGroup::StaticGetAccessor,
            MemberGroup::ProtectedStaticGetAccessor,
            MemberGroup::PrivateStaticGetAccessor,
            MemberGroup::HashPrivateStaticGetAccessor,
            MemberGroup::StaticSetAccessor,
            MemberGroup::ProtectedStaticSetAccessor,
            MemberGroup::PrivateStaticSetAccessor,
            MemberGroup::HashPrivateStaticSetAccessor,
            // Instance accessors (combined, get, set; each public -> protected -> private -> #private)
            MemberGroup::Accessor,
            MemberGroup::ProtectedAccessor,
            MemberGroup::PrivateAccessor,
            MemberGroup::HashPrivateAccessor,
            MemberGroup::GetAccessor,
            MemberGroup::ProtectedGetAccessor,
            MemberGroup::PrivateGetAccessor,
            MemberGroup::HashPrivateGetAccessor,
            MemberGroup::SetAccessor,
            MemberGroup::ProtectedSetAccessor,
            MemberGroup::PrivateSetAccessor,
            MemberGroup::HashPrivateSetAccessor,
            // Static methods (public -> protected -> private -> #private)
            MemberGroup::StaticMethod,
            MemberGroup::ProtectedStaticMethod,
            MemberGroup::PrivateStaticMethod,
            MemberGroup::HashPrivateStaticMethod,
            // Constructor
            MemberGroup::Constructor,
            // Instance methods (public -> protected -> private -> #private)
            MemberGroup::Method,
            MemberGroup::ProtectedMethod,
            MemberGroup::PrivateMethod,
            MemberGroup::HashPrivateMethod,
            // Static blocks
            MemberGroup::StaticBlock,
        ]
    }

    /// Returns the position (rank) of this member group in the given order.
    /// If the member group is not found in the order, returns `order.len()`.
    pub fn rank(&self, order: &[MemberGroup]) -> usize {
        order.iter().position(|g| g == self).unwrap_or(order.len())
    }

    /// Returns a human-readable label for this member group.
    pub fn label(&self) -> &'static str {
        match self {
            MemberGroup::IndexSignature => "index signature",
            MemberGroup::StaticIndexSignature => "static index signature",
            MemberGroup::Property => "property",
            MemberGroup::StaticProperty => "static property",
            MemberGroup::ProtectedProperty => "protected property",
            MemberGroup::ProtectedStaticProperty => "protected static property",
            MemberGroup::PrivateProperty => "private property",
            MemberGroup::PrivateStaticProperty => "private static property",
            MemberGroup::HashPrivateProperty => "#private property",
            MemberGroup::HashPrivateStaticProperty => "#private static property",
            MemberGroup::Accessor => "accessor",
            MemberGroup::StaticAccessor => "static accessor",
            MemberGroup::ProtectedAccessor => "protected accessor",
            MemberGroup::ProtectedStaticAccessor => "protected static accessor",
            MemberGroup::PrivateAccessor => "private accessor",
            MemberGroup::PrivateStaticAccessor => "private static accessor",
            MemberGroup::HashPrivateAccessor => "#private accessor",
            MemberGroup::HashPrivateStaticAccessor => "#private static accessor",
            MemberGroup::GetAccessor => "get accessor",
            MemberGroup::StaticGetAccessor => "static get accessor",
            MemberGroup::ProtectedGetAccessor => "protected get accessor",
            MemberGroup::ProtectedStaticGetAccessor => "protected static get accessor",
            MemberGroup::PrivateGetAccessor => "private get accessor",
            MemberGroup::PrivateStaticGetAccessor => "private static get accessor",
            MemberGroup::HashPrivateGetAccessor => "#private get accessor",
            MemberGroup::HashPrivateStaticGetAccessor => "#private static get accessor",
            MemberGroup::SetAccessor => "set accessor",
            MemberGroup::StaticSetAccessor => "static set accessor",
            MemberGroup::ProtectedSetAccessor => "protected set accessor",
            MemberGroup::ProtectedStaticSetAccessor => "protected static set accessor",
            MemberGroup::PrivateSetAccessor => "private set accessor",
            MemberGroup::PrivateStaticSetAccessor => "private static set accessor",
            MemberGroup::HashPrivateSetAccessor => "#private set accessor",
            MemberGroup::HashPrivateStaticSetAccessor => "#private static set accessor",
            MemberGroup::Constructor => "constructor",
            MemberGroup::Method => "method",
            MemberGroup::StaticMethod => "static method",
            MemberGroup::ProtectedMethod => "protected method",
            MemberGroup::ProtectedStaticMethod => "protected static method",
            MemberGroup::PrivateMethod => "private method",
            MemberGroup::PrivateStaticMethod => "private static method",
            MemberGroup::HashPrivateMethod => "#private method",
            MemberGroup::HashPrivateStaticMethod => "#private static method",
            MemberGroup::StaticBlock => "static block",
        }
    }

    /// Returns a broad category name for deduplication in diagnostic messages.
    pub fn broad_category(&self) -> &'static str {
        match self {
            MemberGroup::IndexSignature | MemberGroup::StaticIndexSignature => "index signatures",
            MemberGroup::Property
            | MemberGroup::StaticProperty
            | MemberGroup::ProtectedProperty
            | MemberGroup::ProtectedStaticProperty
            | MemberGroup::PrivateProperty
            | MemberGroup::PrivateStaticProperty
            | MemberGroup::HashPrivateProperty
            | MemberGroup::HashPrivateStaticProperty => "properties",
            MemberGroup::Accessor
            | MemberGroup::StaticAccessor
            | MemberGroup::ProtectedAccessor
            | MemberGroup::ProtectedStaticAccessor
            | MemberGroup::PrivateAccessor
            | MemberGroup::PrivateStaticAccessor
            | MemberGroup::HashPrivateAccessor
            | MemberGroup::HashPrivateStaticAccessor
            | MemberGroup::GetAccessor
            | MemberGroup::StaticGetAccessor
            | MemberGroup::ProtectedGetAccessor
            | MemberGroup::ProtectedStaticGetAccessor
            | MemberGroup::PrivateGetAccessor
            | MemberGroup::PrivateStaticGetAccessor
            | MemberGroup::HashPrivateGetAccessor
            | MemberGroup::HashPrivateStaticGetAccessor
            | MemberGroup::SetAccessor
            | MemberGroup::StaticSetAccessor
            | MemberGroup::ProtectedSetAccessor
            | MemberGroup::ProtectedStaticSetAccessor
            | MemberGroup::PrivateSetAccessor
            | MemberGroup::PrivateStaticSetAccessor
            | MemberGroup::HashPrivateSetAccessor
            | MemberGroup::HashPrivateStaticSetAccessor => "accessors",
            MemberGroup::Constructor => "constructors",
            MemberGroup::Method
            | MemberGroup::StaticMethod
            | MemberGroup::ProtectedMethod
            | MemberGroup::ProtectedStaticMethod
            | MemberGroup::PrivateMethod
            | MemberGroup::PrivateStaticMethod
            | MemberGroup::HashPrivateMethod
            | MemberGroup::HashPrivateStaticMethod => "methods",
            MemberGroup::StaticBlock => "static blocks",
        }
    }
}
