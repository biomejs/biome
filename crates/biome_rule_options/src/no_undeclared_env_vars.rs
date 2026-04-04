use crate::restricted_regex::RestrictedRegex;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUndeclaredEnvVarsOptions {
    /// Environment variables that should always be allowed.
    /// Use this to specify environment variables that are always available
    /// in your environment, even when not declared in turbo.json.
    /// Supports regular expressions, e.g. `["MY_ENV_.*"]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_env_vars: Option<Box<[RestrictedRegex]>>,
}
