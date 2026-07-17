/// Configuration related to the
/// [eslint-plugin-jest](https://github.com/jest-community/eslint-plugin-jest).
///
/// Also, the module includes implementation to convert rule options to Biome's rule options.
use biome_deserialize_macros::Deserializable;
use biome_rule_options::use_consistent_test_it::{TestFunctionKind, UseConsistentTestItOptions};

/// Options for the [jest/consistent-test-it](https://github.com/jest-community/eslint-plugin-jest/blob/main/docs/rules/consistent-test-it.md) rule.
///
/// Note: ESLint's default for `fn` is `"test"`, while Biome's default is `"it"`.
#[derive(Debug, Default, Deserializable)]
pub(crate) struct ConsistentTestItOptions {
    /// The function to prefer for top-level tests. Defaults to `"test"` in ESLint.
    #[deserializable(rename = "fn")]
    pub function: Option<EslintTestFunctionKind>,
    /// The function to prefer inside `describe` blocks. Defaults to the value of `fn`.
    #[deserializable(rename = "withinDescribe")]
    pub within_describe: Option<EslintTestFunctionKind>,
}

/// ESLint's test function kind — matches `"it"` | `"test"`.
#[derive(Debug, Deserializable)]
pub(crate) enum EslintTestFunctionKind {
    It,
    Test,
}

impl From<EslintTestFunctionKind> for TestFunctionKind {
    fn from(val: EslintTestFunctionKind) -> Self {
        match val {
            EslintTestFunctionKind::It => Self::It,
            EslintTestFunctionKind::Test => Self::Test,
        }
    }
}

impl From<ConsistentTestItOptions> for UseConsistentTestItOptions {
    fn from(val: ConsistentTestItOptions) -> Self {
        // ESLint's defaults: `fn` → "test", `withinDescribe` → "it".
        // Always set both explicitly so the migrated config preserves ESLint behavior
        // regardless of Biome's own defaults.
        let fn_kind = val
            .function
            .map_or(TestFunctionKind::Test, TestFunctionKind::from);
        let within_describe = val
            .within_describe
            .map_or(TestFunctionKind::It, TestFunctionKind::from);
        Self {
            function: Some(fn_kind),
            within_describe: Some(within_describe),
        }
    }
}
