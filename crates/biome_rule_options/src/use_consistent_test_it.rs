use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// The function to use for tests
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TestFunctionKind {
    /// Use `it()` for tests
    #[default]
    It,
    /// Use `test()` for tests
    Test,
}

/// Options for the `useConsistentTestIt` rule
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentTestItOptions {
    /// The function to use for top-level tests (outside describe blocks).
    /// Default: `"it"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<TestFunctionKind>,

    /// The function to use for tests inside describe blocks.
    /// Default: `"it"`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub within_describe: Option<TestFunctionKind>,
}

impl UseConsistentTestItOptions {
    const DEFAULT_FUNCTION: TestFunctionKind = TestFunctionKind::It;
    const DEFAULT_WITHIN_DESCRIBE: TestFunctionKind = TestFunctionKind::It;

    pub fn function(&self) -> TestFunctionKind {
        self.function.unwrap_or(Self::DEFAULT_FUNCTION)
    }

    pub fn within_describe(&self) -> TestFunctionKind {
        self.within_describe
            .unwrap_or(Self::DEFAULT_WITHIN_DESCRIBE)
    }
}
