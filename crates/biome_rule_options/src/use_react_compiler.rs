use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReactCompilerOptions {
    /// Which functions React Compiler analyzes. Defaults to `infer`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub compilation_mode: Option<CompilationMode>,
}

impl UseReactCompilerOptions {
    /// Returns [`Self::compilation_mode`] if it is set, and the default mode
    /// (`infer`) otherwise.
    pub fn compilation_mode(&self) -> CompilationMode {
        self.compilation_mode.unwrap_or_default()
    }
}

/// Controls which functions React Compiler analyzes.
#[derive(
    Copy, Clone, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum CompilationMode {
    /// Analyze functions that follow React conventions: components
    /// (capitalized functions that create JSX or call hooks) and hooks
    /// (`use`-prefixed functions).
    #[default]
    Infer,

    /// Analyze only functions annotated with a `"use memo"` directive.
    Annotation,

    /// Analyze every function. This can report React-specific diagnostics in
    /// non-React code, such as utility functions that update module-level
    /// state.
    All,
}

impl CompilationMode {
    /// The mode string expected by React Compiler's plugin options.
    pub const fn as_compiler_mode(self) -> &'static str {
        match self {
            Self::Infer => "infer",
            Self::Annotation => "annotation",
            Self::All => "all",
        }
    }
}
